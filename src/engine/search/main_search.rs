//! the main iterative deepening search, that calls several [`negamax`] searches
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::bail;
use anyhow::Result;
use chess::ChessMove;
use lockfree::channel::spsc::Receiver;

use crate::optlog;
use crate::search::exit_condition;
use crate::search::info;
use crate::search::moveordering::ordered_moves;
use crate::search::moveordering::pv_ordered_moves;
use crate::search::negamax::negamax;
use crate::search::negamax::search_to;
use crate::search::search_until;
use crate::search::send;
use crate::search::Message;
use crate::search::RootNode;
use crate::search::MV;
use crate::search::SEARCH_THREADS;
use crate::setup::depth::Depth;
use crate::setup::depth::ONE_PLY;
use crate::setup::values::Value;
use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TranspositionTable;
use crate::uci::UCI_LISTENING_FREQUENCY;
use crate::Engine;

impl<K: TKey, E: TEntry, TT: TranspositionTable<K, E>> Engine<K, E, TT> {
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
            board: self.board,
            pv: Vec::new(),
            eval: Value::MIN,
            previous_eval: Value::MIN,
        };

        thread::spawn(move || {
            let mut best_move: Option<ChessMove> = None;
            let mut best_value: Value = Value::MIN;

            let mut target_depth = Depth(0);
            let mut total_nodes = 0;
            let start_time = Instant::now();

            while !exit_condition() && target_depth < search_to() {
                // record the time it takes to reach this depth to see if it's worth it to go
                // deeper
                let cur_depth_start = Instant::now();
                // go one level deeper
                target_depth += ONE_PLY;
                optlog!(search;debug;"iterative deepening searching to depth {:?}", target_depth);

                // reset best move
                best_value = Value::MIN;
                let mut alpha = Value::MIN;
                let beta = Value::MAX;

                // get an ordered sequence of moves from this position
                let moves = if let Some(first_move) = root.pv.first() {
                    pv_ordered_moves(&root.board, &first_move.0)
                } else {
                    ordered_moves(&root.board)
                };

                optlog!(search;trace;"ordered moves: {}", moves);

                // iterate through all the possible moves from [`RootNode`]
                for mv in moves {
                    // recursively search the next position
                    let search_result = -negamax(
                        root.board.make_move_new(mv),
                        target_depth - 1,
                        -beta,
                        -alpha,
                    );

                    optlog!(
                        search;
                        debug;
                        "move {mv} has value {} ({} nodes)",
                        search_result.next_position_value,
                        search_result.nodes_searched
                    );

                    // add up all the recursively searched nodes, and the one the search begun from
                    total_nodes += search_result.nodes_searched + 1;

                    // we found a better match, update:
                    // * best available value for a next position
                    // * best move to get to that position
                    // * principal variation from that position
                    // * alpha value
                    // + check if we should stop searching
                    // + send info to the UCI thread
                    if search_result.next_position_value > best_value {
                        best_value = search_result.next_position_value;
                        best_move = Some(mv);

                        root.pv = vec![MV(mv, search_result.next_position_value)];
                        root.pv.extend(search_result.pv);

                        // UCI guess, not final move but have one ready in case stop is received
                        if let Some(mv) = best_move {
                            if let Err(e) = publisher.send(Message::BestGuess(MV(mv, best_value))) {
                                optlog!(comm;debug;"error sending best guess: {:?}", e);
                                break;
                            }
                        }
                    }
                    // always keep the best evaluation to prune worse ones
                    alpha = alpha.max(search_result.next_position_value);

                    // check on [`SEARCHING`] and [`SEARCH_UNTIL`] to see if we need to quit this
                    // search
                    if exit_condition() {
                        return;
                    }
                } // we have checked all moves for this depth

                // save previous evaluation of the root node
                root.previous_eval = root.eval;
                root.eval = best_value;

                // new depth info
                info(
                    &mut publisher,
                    target_depth,
                    best_value,
                    total_nodes,
                    start_time.elapsed(),
                    0,
                    0,
                    target_depth,
                    1,
                    &root.pv,
                );

                if let Some(mv) = best_move {
                    send(&mut publisher, Message::BestMove(MV(mv, best_value)));
                }
                if let Some(ponder) = root.pv.get(1) {
                    send(&mut publisher, Message::Ponder(*ponder));
                }

                // check if we should even try to go deeper
                if search_until().is_some_and(|u| u < Instant::now() + cur_depth_start.elapsed()) {
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
