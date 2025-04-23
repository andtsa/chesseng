//! the main iterative deepening search, that calls several [`negamax`] searches
use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use anyhow::bail;
use chess::ChessMove;
use chess::MoveGen;
use lockfree::channel::spsc::Receiver;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::Engine;
use crate::evaluation::evaluate;
use crate::move_generation::prio_iterator;
use crate::optlog;
use crate::opts::opts;
use crate::search::MV;
use crate::search::Message;
use crate::search::RootNode;
use crate::search::SEARCH_THREADS;
use crate::search::SearchOptions;
use crate::search::SearchResult;
use crate::search::exit_condition;
use crate::search::info;
use crate::search::negamax::negamax;
use crate::search::negamax::search_to;
use crate::search::search_until;
use crate::search::send;
use crate::setup::depth::Depth;
use crate::setup::depth::ONE_PLY;
use crate::setup::values::Value;
use crate::transposition_table::TranspositionTable;
use crate::uci::UCI_LISTENING_FREQUENCY;

impl Engine {
    /// Begin the search for the best move, spawns a new thread to actually do
    /// the search, and returns a listener for [`Message`]s.
    pub fn begin_search(&mut self) -> Result<Receiver<Message>> {
        optlog!(search;debug;"begin_search called with depth {:?}", search_to());
        self.set_search(true);
        if exit_condition() {
            bail!("begin_search called with exit_condition true!");
        }

        let (mut publisher, receiver) = lockfree::channel::spsc::create();
        let mut root = RootNode {
            board: self.board.clone(),
            pv: Vec::new(),
            eval: Value::MIN,
            previous_eval: Value::MIN,
        };

        let tt = self.table.get();

        let engine_history = self
            .history
            .make_contiguous()
            .iter_mut()
            .map(|p| p.chessboard.get_hash())
            .collect::<Vec<_>>();

        thread::spawn(move || {
            let mut best_move: Option<ChessMove> = None;
            let mut best_value: Value = Value::MIN;

            let mut target_depth = Depth(0);
            let mut total_nodes = 0;
            let mut max_depth = Depth::ZERO;
            let mut min_depth = Depth::MAX;

            // for now I'm using tablebase_hits to refer to transposition table hits,
            // because it is displayed more prominently on cutechess UI, and I
            // don't have an endgame tablebase yet.
            let mut tb_hits = 0;
            let start_time = Instant::now();

            let initial_options = SearchOptions {
                extensions: Depth::ZERO,
                history: [
                    *engine_history.first().unwrap_or(&0),
                    *engine_history.get(1).unwrap_or(&0),
                    *engine_history.get(2).unwrap_or(&0),
                    *engine_history.get(3).unwrap_or(&0),
                    *engine_history.get(4).unwrap_or(&0),
                    *engine_history.get(5).unwrap_or(&0),
                    *engine_history.get(6).unwrap_or(&0),
                ],
            };

            optlog!(search;warn;"history:{:?}", initial_options.history);

            // SAFETY: if it fails it's due to poison,
            // and that means another thread panicked,
            // so we should panic as well anyway
            let search_options = opts().unwrap();

            // iterative deepening loop
            while !exit_condition() && target_depth < search_to() {
                // record the time it takes to reach this depth to see if it's worth it to go
                // deeper
                let cur_depth_start = Instant::now();
                // go one level deeper
                target_depth += ONE_PLY;
                optlog!(search;debug;"iterative deepening searching to depth {:?}", target_depth);

                // reset best move
                best_value = Value::MIN;

                let base_gen = MoveGen::new_legal(&root.board.chessboard);
                // get an ordered sequence of moves from this position
                let moves = if search_options.use_pv {
                    if let Some(first_move) = root.pv.first() {
                        prio_iterator(base_gen, &root.board.chessboard, &[first_move.0])
                    } else {
                        prio_iterator(base_gen, &root.board.chessboard, &[])
                    }
                } else {
                    prio_iterator(base_gen, &root.board.chessboard, &[])
                }
                .collect::<Vec<_>>();

                // optlog!(search;trace;"ordered moves: {}", moves);

                // in case of parallel search, use the same (thread-safe) alpha value across all
                // searches. when one finishes it will update for all that haven't run yet (this
                // is unimpactful if all searches are run in parallel, but 30+
                // threads for complex positions are impractical).
                let par_alpha = AtomicI16::new(Value::MIN.0);

                // call the [`negamax`] search, update the alpha value and return the
                // [`SearchResult`]
                let search_fn = |mv: &ChessMove| {
                    let next_position = root.board.make_move(*mv);
                    if next_position.causes_threefold(&engine_history) {
                        SearchResult {
                            pv: vec![],
                            next_position_value: -evaluate(&next_position, true),
                            nodes_searched: 1,
                            tb_hits: 0,
                            depth: ONE_PLY,
                        }
                    } else {
                        let partial = -negamax(
                            next_position,
                            target_depth - 1,
                            Value(par_alpha.load(Ordering::Relaxed)),
                            Value::MAX,
                            initial_options,
                            &search_options,
                            &tt,
                        );
                        par_alpha.store(
                            par_alpha
                                .load(Ordering::Acquire)
                                .max(partial.next_position_value.0),
                            Ordering::Release,
                        );
                        partial
                    }
                };

                // if we want the search to be single-threaded, we use the current thread and a
                // normal iterator.
                // fine-grained control of the nuber of threads is not implemented yet, mostly
                // because the current implementation trades that off for instead being very
                // easy to implement and rely on
                let all_results = if search_options.threads <= 1 {
                    moves.iter().map(search_fn).collect::<Vec<SearchResult>>()
                } else {
                    moves
                        .par_iter()
                        .map(search_fn)
                        .collect::<Vec<SearchResult>>()
                };

                // iterate through all the possible moves from [`RootNode`]
                for (mv, search_result) in moves.iter().zip(all_results.into_iter()) {
                    optlog!(
                        search;
                        debug;
                        "move {mv} has value {} ({} nodes)",
                        search_result.next_position_value,
                        search_result.nodes_searched
                    );

                    // add up all the recursively searched nodes, and the one the search begun from
                    total_nodes += search_result.nodes_searched + 1;
                    // add up all the transposition table hits
                    tb_hits += search_result.tb_hits;

                    max_depth = max_depth.max(search_result.depth);
                    min_depth = min_depth.min(search_result.depth);

                    // we found a better match, update:
                    // * best available value for a next position
                    // * best move to get to that position
                    // * principal variation from that position
                    // * alpha value
                    // + check if we should stop searching
                    // + send info to the UCI thread
                    if search_result.next_position_value > best_value {
                        best_value = search_result.next_position_value;
                        best_move = Some(*mv);

                        root.pv = vec![MV(*mv, search_result.next_position_value)];
                        root.pv.extend(search_result.pv);

                        // UCI guess, not final move but have one ready in case stop is received
                        if let Some(mv) = best_move {
                            if let Err(e) = publisher.send(Message::BestGuess(MV(mv, best_value))) {
                                optlog!(comm;debug;"error sending best guess: {:?}", e);
                                break;
                            }
                        }
                    }

                    // check on [`SEARCHING`] and [`SEARCH_UNTIL`] to see if we need to quit this
                    // search
                    if exit_condition() {
                        return;
                    }
                } // we have checked all moves for this depth

                // save previous evaluation of the root node
                root.previous_eval = root.eval;
                root.eval = best_value;

                {
                    // new depth info
                    info(
                        &mut publisher,
                        target_depth,
                        best_value,
                        total_nodes,
                        start_time.elapsed(),
                        tt.read().map_or(0, |l| l.hashfull()),
                        tb_hits,
                        max_depth,
                        1,
                        &root.pv,
                    );
                } // ensure lock is dropped asap

                if let Some(mv) = best_move {
                    send(&mut publisher, Message::BestMove(MV(mv, best_value)));
                }
                if let Some(ponder) = root.pv.get(1) {
                    send(&mut publisher, Message::Ponder(*ponder));
                }

                // check if we should even try to go deeper.
                let next_search_estimate =
                    cur_depth_start.elapsed() * ((1 + target_depth.0) / 3) as u32;
                // we expect the next depth to take much longer than the current depth.
                // this may be pessimistic, but that's offset by a generous time allocation
                if search_until().is_some_and(|u| u < Instant::now() + next_search_estimate) {
                    optlog!(
                        search;
                        debug;
                        "not enough time for depth {} ({}ms/{}ms), breaking early at move {}",
                        target_depth.0 + 1,
                        start_time.elapsed().as_millis(),
                        (search_until().unwrap_or_else(Instant::now) - Instant::now()).as_millis(),
                        MV(best_move.unwrap_or_default(), best_value)
                    );
                    break;
                }
            }

            optlog!(search;debug;"sending best move {:?}", best_move);
            optlog!(comm;debug;"sending best move {:?}", best_move);

            if let Some(mv) = best_move {
                send(&mut publisher, Message::BestMove(MV(mv, best_value)))
            }

            // looks sketchy, but it's to prevent dropping the sender before the receiver
            // has gotten the best move.
            thread::sleep(Duration::from_millis(
                (SEARCH_THREADS * 2 * UCI_LISTENING_FREQUENCY) as u64,
            ));
        });

        Ok(receiver)
    }
}
