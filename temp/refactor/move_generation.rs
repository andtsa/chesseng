
pub fn ordered_moves(board : &Board) -> Vec<ChessMove> {
    let mut mg = MoveGen::new_legal(board);
    let masks = vec![
        board.pieces(Queen).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Rook).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Bishop).bitor(board.pieces(Knight)).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Pawn).bitand(board.color_combined(board.side_to_move().not())),
        BitBoard::new(0b1100000011000000000000000000000000000),
        BitBoard::new(0b1111000011110000111100001111000000000000000000),
        !EMPTY
    ];
    let mut r : Vec<ChessMove> = Vec::new();
    for m in masks {
        mg.set_iterator_mask(m);
        let mvs = mg.by_ref().collect::<Vec<ChessMove>>();
        r.append(&mut mvs.clone());
    }
    return r;
}


pub fn compute_best_move(b : &Board, depth : u32, bb : &Bot) -> ChessMove {
    // only ran once per move generation, clone the passed parameters so the search threads
    // don't reference the main thread's board
    let board = b.clone();
    let bot = bb.clone();
    let maximizing = board.side_to_move() == White;
    let mut best_move = None;
    let mut best_value = if maximizing {f64::MIN} else {f64::MAX};
    let legal_moves = all_moves(&board);

    let mut threads = Vec::new();
    // a locked vector of end-of-branch evaluations and moves, each thread adds its own result to
    // the shared vector which is processed at the end
    let moves: Mutex<Vec<(f64, ChessMove)>> = Mutex::new(Vec::new());

    // store the information each thread needs in an arc that can be safely passed between them
    let thread_data = (board, bot, maximizing, moves);
    let arc_thread_data = Arc::new(thread_data);

    for &mv in &legal_moves {
        let arc_thread_data = Arc::clone(&arc_thread_data);
        threads.push(thread::spawn(move ||{
            {
                let bd = arc_thread_data.0.make_move_new(mv);
                let value = minimax(&bd, depth - 1, arc_thread_data.2, f64::MIN, f64::MAX, &arc_thread_data.1);
                arc_thread_data.3.lock().unwrap().push((value, mv));
            }
        }));
    }

    // wait for all the threads to finish
    for t in threads {
        t.join().unwrap();
    }

    // each pair is end-of-branch evaluation, and the move at the top of the branch.
    // loop through all to pick the best one.
    let binding = arc_thread_data.3.lock().unwrap();
    for (value, mv) in binding.deref() {
        if (value > &best_value && maximizing) || (value < &best_value && !maximizing) {
            best_value = *value;
            best_move = Some(mv);
        }
    }

    *best_move.unwrap() // Return the best move found
}



fn minimax(board : &Board, depth : u32, maximizing : bool, alpha : f64, beta : f64, bot : &Bot) -> f64 {
    if depth <= 0 || board.status()!=Ongoing {
        return bot.eval(*board);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    return if maximizing {
        let mut best_value = f64::MIN;
        let legal_moves = all_moves(board);

        for mv in legal_moves {
            let bd = board.make_move_new(mv);

            let value = minimax(&bd, depth - 1, false, alpha, beta, bot);
            best_value = if best_value < value {value} else {best_value};
            alpha = if best_value < alpha {alpha} else {best_value};
            if beta <= alpha {
                break
            }
        }

        best_value
    } else {
        let mut best_value = f64::MAX;
        let legal_moves = all_moves(board);

        for mv in legal_moves {
            let bd = board.make_move_new(mv);

            let value = minimax(&bd, depth - 1, true, alpha, beta, bot);
            best_value = if best_value > value {value} else {best_value};
            beta = if best_value > beta {beta} else {best_value};
            if beta <= alpha {
                break
            }
        }

        best_value
    }
}


pub fn start_thinking(board : &Board, duration : Duration, bot : &Bot) -> ChessMove {
    let end = Instant::now() + duration;

    let depths: Vec<u32> = vec![6,7,8,9,10];

    let maximizing = board.side_to_move() == White;
    let mut best_move = None;
    let mut best_value = if maximizing {f64::MIN} else {f64::MAX};
    let moves: Arc<Mutex<Vec<(f64, ChessMove)>>> = Arc::new(Mutex::new(Vec::new()));
    let move_list = &all_moves(board);
    if move_list.len() <= 1 {
        return move_list[0];
    }
    let deepest_reached_boards : Arc<Mutex<Vec<(ChessMove, Board)>>> = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for &mv in move_list {
            let moves = Arc::clone(&moves);
            // let cur_depth = depths[0].clone();
            let cur_depth = bot.get_search_depth(board);
            s.spawn(move || {
                let bd = board.make_move_new(mv);
                let value = timed_minimax(&bd, end, cur_depth - 1, maximizing, f64::MIN, f64::MAX, bot);
                moves.lock().unwrap().push((value, mv));
            });
        }
    });

    // each pair is end-of-branch evaluation, and the move at the top of the branch.
    // loop through all to pick the best one.
    let binding = moves.lock().unwrap();
    for (value, mv) in binding.deref() {
        if (value > &best_value && maximizing) || (value < &best_value && !maximizing) {
            best_value = *value;
            best_move = Some(mv);
        }
    }
    *best_move.unwrap() // Return the best move found
}

pub fn timed_minimax(board : &Board, end : Instant, depth : u32, maximizing : bool, alpha : f64, beta : f64, bot : &Bot) -> f64 {
    if Instant::now()>end || depth <= 0 || board.status()!=Ongoing {
        return bot.eval(*board);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    return if maximizing {
        let mut best_value = f64::MIN;
        let legal_moves = all_moves(board);
        for mv in legal_moves {
            let bd = board.make_move_new(mv);
            let value = timed_minimax(&bd, end, depth-1, false, alpha, beta, bot);
            best_value = if best_value < value {value} else {best_value};
            alpha = if best_value < alpha {alpha} else {best_value};
            if beta <= alpha {
                break
            }
        }
        best_value
    } else {
        let mut best_value = f64::MAX;
        let legal_moves = all_moves(board);
        for mv in legal_moves {
            let bd = board.make_move_new(mv);
            let value = timed_minimax(&bd, end, depth-1, true, alpha, beta, bot);
            best_value = if best_value > value {value} else {best_value};
            beta = if best_value > beta {beta} else {best_value};
            if beta <= alpha {
                break
            }
        }
        best_value
    }
}

//
// pub fn iterative_deepening(board : &Board, duration : Duration, bot : &Bot) {
//     let end = Instant::now() + duration;
//
//     let depths: Vec<u32> = vec![3,4,5,6,7];
//
//     let maximizing = board.side_to_move() == White;
//     let mut best_move = None;
//     let mut best_value = if maximizing {f64::MIN} else {f64::MAX};
//     let moves: Arc<Mutex<Vec<(f64, ChessMove)>>> = Arc::new(Mutex::new(Vec::new()));
//
//     let deepest_reached_boards : Arc<Mutex<Vec<(ChessMove, Board)>>> = Arc::new(Mutex::new(Vec::new()));
//
//     thread::scope(|s| {
//         for &mv in &all_moves(board) {
//             let deepest_reached_boards = Arc::clone(&deepest_reached_boards);
//             let cur_depth = depths[0].clone();
//             s.spawn(move || {
//                 let bd = board.make_move_new(mv);
//                 let value = id_look_further(&bd, cur_depth - 1, maximizing, f64::MIN, f64::MAX, bot, deepest_reached_boards);
//             });
//
//         }
//     });
//
//
// }