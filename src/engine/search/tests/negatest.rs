use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::time::Duration;

use chess::Board;
use chess::BoardStatus;
use chess::ChessMove;
use chess::Color;
use chess::MoveGen;

use crate::Engine;
use crate::move_generation::prio_iterator;
use crate::position::FIFTY_MOVE_LIMIT;
use crate::position::Position;
use crate::position::is_irreversible;
use crate::search::SEARCH_PATH_LEN;
use crate::search::SEARCHING;
use crate::search::SearchOptions;
use crate::search::negamax::Opts;
use crate::search::negamax::negamax;
use crate::search::negamax::ng_test;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::transposition_table::TT;
use crate::util::Print;
use crate::util::short_benches;

#[test]
fn startpos_is_positive() {
    let pos = Board::default();
    SEARCHING.store(true, Ordering::Relaxed);
    let val = ng_test(pos, Depth(4), Value::MIN, Value::MAX, Opts::new()).unwrap();
    assert!(
        val.next_position_value > Value::ZERO,
        "startpos was {}",
        val.next_position_value
    );
}

#[test]
fn mate_is_mate() {
    let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
    for x in 1..10 {
        SEARCHING.store(true, Ordering::Relaxed);
        // println!("x: {}", x);
        assert_eq!(pos.side_to_move(), Color::Black);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            -Value::MATE
        );
    }
}

#[test]
fn mate_in_1_is_mate() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..5 {
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 1,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn will_mate_in_1_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    // NOTE: this test used to start at depth 1,
    // but after implementing quiescence search it fails for
    // depth <= 1. This is (as far as i can imagine) because
    // quiescence only searches captures and not checks. See
    // note in quiescence.rs for details.
    for d in 2..5 {
        let mut engine = Engine::new().unwrap();
        engine.board = pos.into();
        engine.eng_opts.use_ab = false;
        engine.eng_opts.use_pv = false;
        engine.eng_opts.threads = 1;

        eprintln!(
            "all possible moves: {:?}",
            prio_iterator(MoveGen::new_legal(&pos), &pos, &[])
                .map(|cm| cm.to_string())
                .collect::<Vec<_>>() //ordered_moves(&engine.board.chessboard)
        );

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(2000))
            .unwrap_or_else(|e| panic!("died at depth {d}: {e}"));
        engine.board = engine.board.make_move(mv);

        assert_eq!(
            engine.board.chessboard.status(),
            BoardStatus::Checkmate,
            "depth={d} mv={mv} pos={}",
            pos.print()
        );
    }
}

#[test]
fn mate_in_1_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..5 {
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 1,
            "depth = {x} pos={}",
            Position::from(pos).print()
        );
    }
}

#[test]
fn mate_in_2_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/4K2k w - - 0 1").unwrap();
    SEARCHING.store(true, Ordering::Relaxed);
    assert_ne!(
        ng_test(pos, Depth(1), Value::MIN, Value::MAX, Opts::new())
            .unwrap()
            .next_position_value,
        Value::MATE,
        "depth = 1 pos={}",
        pos.print()
    );
    assert_ne!(
        ng_test(pos, Depth(2), Value::MIN, Value::MAX, Opts::new())
            .unwrap()
            .next_position_value,
        Value::MATE,
        "depth = 2 pos={}",
        pos.print()
    );
    for x in 3..5 {
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 3,
            "depth = {x} pos={}",
            pos.print(),
        );
    }
}

#[test]
fn will_mate_in_2_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k b - - 0 1").unwrap();
    for d in 5..6 {
        let mut engine = Engine::new().unwrap();

        // setopts(Opts::new().tt(true).search(debug)).unwrap();
        engine.eng_opts.use_tt = true;
        engine.board = pos.into();

        let mv1 = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        engine.board = engine.board.make_move(mv1);

        eprintln!("made first move in mating sequence: {mv1}");

        assert_eq!(
            engine.board.chessboard.status(),
            BoardStatus::Ongoing,
            "depth=1 mv={mv1} pos={}",
            pos.print()
        );

        let mv2 = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        let board_before = engine.board.clone();
        engine.board = engine.board.make_move(mv2);
        let board_after = engine.board.clone();

        assert_ne!(board_before, board_after);
        assert_ne!(board_before.chessboard, board_after.chessboard);

        // panic!("{}", engine.table);

        assert_eq!(
            engine.board.chessboard.status(),
            BoardStatus::Checkmate,
            "depth=1 mv2={mv2} pos={}",
            engine.board.print()
        );
    }
}

#[test]
fn score_same_with_or_without_ab_pv() {
    for pos in short_benches().into_iter() {
        for x in 1..4 {
            SEARCHING.store(true, Ordering::SeqCst);
            // println!("testing pos_{p_idx}_depth_{x}");
            assert_eq!(
                ng_test(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(false).pv(false)
                )
                .unwrap()
                .next_position_value,
                ng_test(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(true).pv(true)
                )
                .unwrap()
                .next_position_value,
                "depth = {x} pos={}",
                pos.print()
            );
        }
    }
}

#[test]
fn checkmate_the_author() {
    // crate::util::setup_logging();
    let pos = Board::from_str("1n1k4/r1pp1p2/7p/8/1p1q4/6r1/4q3/1K6 b - - 0 1").unwrap();
    let mut engine = Engine::new().unwrap();
    engine.board = pos.into();

    let mv = engine
        .best_move(Depth(2), Duration::from_millis(10000))
        .unwrap();
    engine.board = engine.board.make_move(mv);

    assert_eq!(
        engine.board.chessboard.status(),
        BoardStatus::Checkmate,
        "depth=2 move={mv} pos={}",
        pos.print()
    );
}

/// the positions actually played in the game must stay visible however deep
/// the search goes. before, game history and search path shared one small
/// array, so the root position was evicted after a few plies and a line
/// returning to it could never be recognised.
#[test]
fn game_history_survives_descent() {
    let root_hash = 0xdead_beef_u64;
    let history = [root_hash];
    let mut options = SearchOptions {
        extensions: Depth::ZERO,
        game_history: &history,
        path: [0; SEARCH_PATH_LEN],
    };

    assert_eq!(options.repetition_count(root_hash), 1);

    for ply in 1..(SEARCH_PATH_LEN as u64 * 4) {
        options = options.descend(ply);
        assert_eq!(
            options.repetition_count(root_hash),
            1,
            "root position was forgotten after {ply} plies",
        );
    }

    // the search path itself is remembered too, for the most recent plies
    assert_eq!(options.repetition_count(SEARCH_PATH_LEN as u64 * 4 - 1), 1);
}

/// a repeated position is scored as a draw, not as a win for whoever is ahead.
#[test]
fn repetition_scores_as_draw() {
    // white is a queen up, but this position has already been played
    let board = Board::from_str("4k3/8/8/8/8/8/8/Q6K w - - 0 1").unwrap();
    let history = [board.get_hash()];

    SEARCHING.store(true, Ordering::Relaxed);
    let tt = TT::new();
    let result = negamax(
        Position::from(board),
        Depth(6),
        Value::MIN,
        Value::MAX,
        SearchOptions {
            extensions: Depth::ZERO,
            game_history: &history,
            path: [0; SEARCH_PATH_LEN],
        },
        &Opts::new().engine_opts,
        &tt.get(),
    );

    assert_eq!(
        result.next_position_value,
        Value::DRAW,
        "a repetition should be a draw, not {}",
        result.next_position_value
    );
}

/// the fifty-move counter is a draw once it reaches its limit, and any capture
/// or pawn move restarts it.
#[test]
fn fifty_move_rule_is_a_draw() {
    // white is a rook up, but has shuffled for fifty moves
    let board = Board::from_str("4k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
    let mut pos = Position::from(board);
    pos.halfmove_clock = FIFTY_MOVE_LIMIT;

    SEARCHING.store(true, Ordering::Relaxed);
    let tt = TT::new();
    let result = negamax(
        pos,
        Depth(4),
        Value::MIN,
        Value::MAX,
        SearchOptions::default(),
        &Opts::new().engine_opts,
        &tt.get(),
    );

    assert_eq!(
        result.next_position_value,
        Value::DRAW,
        "the fifty-move limit should be a draw, not {}",
        result.next_position_value
    );
}

/// only pawn moves and captures reset the counter; losing castling rights
/// makes earlier positions unreachable but leaves the clock running.
#[test]
fn halfmove_clock_resets_on_pawn_moves_and_captures() {
    let board = Board::from_str("r3k3/8/8/8/8/8/1P6/R3K3 w Q - 0 1").unwrap();
    let mut pos = Position::from(board);
    pos.halfmove_clock = 20;

    // a quiet rook move keeps counting, even though it gives up the castling
    // rights, so it still has to clear the repetition history
    let quiet = ChessMove::from_str("a1b1").unwrap();
    assert_eq!(pos.make_move(quiet).halfmove_clock, 21);
    assert!(is_irreversible(
        &pos.chessboard,
        &pos.make_move(quiet).chessboard,
        quiet
    ));

    // a pawn move restarts the count
    let pawn = ChessMove::from_str("b2b3").unwrap();
    assert_eq!(pos.make_move(pawn).halfmove_clock, 0);

    // and so does a capture
    let capture = ChessMove::from_str("a1a8").unwrap();
    assert!(board.piece_on(capture.get_dest()).is_some());
    assert_eq!(pos.make_move(capture).halfmove_clock, 0);
}

/// neither side can mate, so the position is drawn however good the search
/// thinks the material looks.
#[test]
fn insufficient_material_is_a_draw() {
    for fen in [
        "4k3/8/8/8/8/8/8/4K3 w - - 0 1",  // bare kings
        "4k3/8/8/8/8/8/8/3BK3 w - - 0 1", // king and bishop
        "4k3/8/8/8/8/8/8/3NK3 w - - 0 1", // king and knight
    ] {
        let board = Board::from_str(fen).unwrap();
        assert!(
            Position::from(board).is_insufficient_material(),
            "{fen} should be a draw by insufficient material"
        );

        SEARCHING.store(true, Ordering::Relaxed);
        let tt = TT::new();
        let result = negamax(
            Position::from(board),
            Depth(3),
            Value::MIN,
            Value::MAX,
            SearchOptions::default(),
            &Opts::new().engine_opts,
            &tt.get(),
        );
        assert_eq!(result.next_position_value, Value::DRAW, "{fen}");
    }

    // a single pawn can promote, so it is not insufficient
    let with_pawn = Board::from_str("4k3/8/8/8/8/8/P7/4K3 w - - 0 1").unwrap();
    assert!(!Position::from(with_pawn).is_insufficient_material());
}
