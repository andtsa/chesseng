// let wp : BitBoard = board.pieces(Pawn).bitand(board.color_combined(White));
// let r = wp.to_size(0) as f64;
// println!("{r}");
// return r;

// pub const FULL_SEARCH_DEPTH: u32 = 1;

//
// fn mov(board : Board) -> ChessMove {
//     // let mg = board.
//     //let e : f64 = eval(board);
//     return ChessMove::new(Square::make_square(Second, D), Square::make_square(Third, D), None);
// }

// let bb = board.combined();
// println!("{}",bb.to_string());
// let b = board.make_move_new(mov(board));
// println!("{}",fen_to_str(b.to_string()));
//
// pub fn squares_covered_by_side(board : Board, side : Color) -> f64 {
//     let mut bb = BitBoard::new(0);
//
//     // first, pawns.
//     for s in board.pieces(Pawn).bitand(board.color_combined(side)).into_iter() {
//         let mut a = s.forward(side);
//         if a!=None{
//             let a1 = BitBoard::from_maybe_square(a.unwrap().left());
//             let a2 = BitBoard::from_maybe_square(a.unwrap().right());
//             if a1!=None {bb.bitor_assign(a1.unwrap());}
//             if a2!=None {bb.bitor_assign(a2.unwrap());}
//         }
//     }
//
//     // second, king (easiest ones)
//     let ks= board.king_square(side);
//     let mut s = ks.up();
//     if s!=None {
//         bb.bitor_assign(BitBoard::from_maybe_square(s).unwrap());
//         if s.unwrap().left()!=None {bb.bitor_assign(BitBoard::from_maybe_square(s.unwrap().left()).unwrap())}
//         if s.unwrap().right()!=None {bb.bitor_assign(BitBoard::from_maybe_square(s.unwrap().right()).unwrap())}
//     }
//     s = ks.down();
//     if s!=None {
//         bb.bitor_assign(BitBoard::from_maybe_square(s).unwrap());
//         if s.unwrap().left()!=None {bb.bitor_assign(BitBoard::from_maybe_square(s.unwrap().left()).unwrap())}
//         if s.unwrap().right()!=None {bb.bitor_assign(BitBoard::from_maybe_square(s.unwrap().right()).unwrap())}
//     }
//     s = ks.right();
//     if s!=None {bb.bitor_assign(BitBoard::from_square(s.unwrap()))}
//     s = ks.left();
//     if s!=None {bb.bitor_assign(BitBoard::from_square(s.unwrap()))}
//
//
//     // next, Rooks, and Queen verticals. Here we need to repeat adding squares in each direction until piece or wall is in the way
//     for rs in board.color_combined(side).bitand(board.pieces(Rook).bitor(board.pieces(Queen))).into_iter() {
//         let mut pointer = Some(rs.clone());
//         while pointer!=None && (pointer==Some(rs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().up();
//         }
//         pointer = Some(rs.clone());
//         while pointer!=None && (pointer==Some(rs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().down();
//         }
//         pointer = Some(rs.clone());
//         while pointer!=None && (pointer==Some(rs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().left();
//         }
//         pointer = Some(rs.clone());
//         while pointer!=None && (pointer==Some(rs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().right();
//         }
//     }
//
//     // almost done, now Bishops, and queen diagonals. two checks needed to see if the square up/down & left/right exists
//     for bs in board.color_combined(side).bitand(board.pieces(Bishop).bitor(board.pieces(Queen))).into_iter() {
//         let mut pointer = Some(bs.clone());
//         while pointer!=None && (pointer==Some(bs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().up();
//             if pointer == None|| board.piece_on(pointer.unwrap())==None {break}
//             pointer = pointer.unwrap().left();
//         }
//         pointer = Some(bs.clone());
//         while pointer!=None && (pointer==Some(bs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().down();
//             if pointer == None || board.piece_on(pointer.unwrap())==None{break}
//             pointer = pointer.unwrap().left();
//         }
//         pointer = Some(bs.clone());
//         while pointer!=None && (pointer==Some(bs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().up();
//             if pointer == None || board.piece_on(pointer.unwrap())==None{break}
//             pointer = pointer.unwrap().right();
//         }
//         pointer = Some(bs.clone());
//         while pointer!=None && (pointer==Some(bs) || board.piece_on(pointer.unwrap())==None) {
//             bb.bitor_assign(BitBoard::from_square(pointer.unwrap()));
//             pointer = pointer.unwrap().down();
//             if pointer == None || board.piece_on(pointer.unwrap())==None{break}
//             pointer = pointer.unwrap().right();
//         }
//     }
//
//     // // finally, knights... a little more complicated
//     // for ks in board.pieces(Knight).bitand(board.color_combined(side)).into_iter() {
//     //     let mut pointer = Some(ks.clone());
//     //
//     //     // first up, checking along the way and adding left & right squares
//     //     pointer = pointer.unwrap().up();
//     //     if pointer!=None
//     // }
//
//
//     // println!("{}", bb.reverse_colors().to_string());
//     bb.bitand_assign(board.color_combined(side).not());//.bitxor(BitBoard::new(u64::MAX)));
//     println!("{}", bb.reverse_colors().to_string());
//
//     return bb.popcnt() as f64;
// }

fn computer_move(board : &Board) -> ChessMove {
    let now = Instant::now();

    let sd = get_search_depth(board.combined().popcnt());
    println!("{} pieces on the board, search depth of {}", board.combined().popcnt(), sd);
    let mut mv = compute_best_move(board, sd, None); //full_search(board, FULL_SEARCH_DEPTH).moves.pop_front().unwrap();
    let mut repeat_q : bool = !board.legal(mv);
    while repeat_q {
        println!("Computer made illegal move {}, retrying", mv.to_string());
        mv = compute_best_move(board, sd, None);
        repeat_q = !board.legal(mv);
    }

    println!("computed move in {} milliseconds", now.elapsed().as_millis());
    return mv;
}

pub fn get_search_depth(d : u32) -> u32 {
    match d {
        22..=32 => {4}
        9..=21 => {5}
        6 ..=8 => {6}
        4 | 5 => {7}
        2 | 3 => {8}
        _=>{4}
    }
}

/// evaluate a board. positive means white is winning, negative means black is winning
pub fn eval(board : Board) -> f64 {
    match board.status() {
        Stalemate => {return STALEMATE_VAL * piece_values(board);}
        Checkmate => {return (board.side_to_move().to_index()as f64 - 0.5) * 2.0 * CHECKMATE_VAL}
        _ => {}
    }

    let mut e : f64 = 0.0;
    e += piece_value_weight(board, PIECE_VALUE_WEIGHT) * piece_values(board);
    e += N_OF_MOVES_WEIGHT * possible_moves_val(board);
    e += pinning_val(board, PINNING_VAL, single_piece_value());
    e += check_val(board, CHECK_PIECE_MULTIPLIER);
    // println!("{e}");
    return e;
}




pub fn bfs_get_leaves(board : &Board, depth : u32) -> Vec<Vec<ChessMove>> {
    let mut paths : VecDeque<(Vec<ChessMove>, u32)> = VecDeque::new();
    let mut results : Vec<Vec<ChessMove>> = Vec::new();
    let am = all_moves(board);
    for m in am {
        paths.push_back((vec!(m), 1));
    }
    while !paths.is_empty() {
        let cur = paths.pop_front().unwrap();
        if cur.1 >= depth {
            results.push(cur.0);
        } else {
            let mut b = *board;
            for cm in &cur.0 {
                b = b.make_move_new(*cm);
            }
            for nm in all_moves(&b) {
                let vv = &mut cur.0.to_vec();
                vv.push(nm);
                paths.push_back((vv.to_owned(), cur.1+1));
            }
        }
    }
    return results;
}



/// returns moveset with optimal result up to certain depth
///
/// TODO: stop it from assuming im stupid <=> opponent's move should be optimal every time.
pub fn full_search(board : &Board, depth : u32) -> MoveSet {
    let mut heap : BinaryHeap<MoveSet> = BinaryHeap::new();
    // let mut deque : VecDeque<(ChessMove, u32)> = VecDeque::new();
    let paths = bfs_get_leaves(board, depth);

    for p in paths {
        let mut b = *board;
        for m in &p {
            b = b.make_move_new(*m);
        }
        heap.push(MoveSet {
            moves: VecDeque::from(p),
            end_eval: eval(b),
        });
    }
    return heap.pop().unwrap();
}

fn single_piece_value() -> HashMap<Piece, f64> {
    HashMap::from([(Pawn, PAWN_VAL),(Rook, ROOK_VAL),(Knight, KNIGHT_VAL),(Bishop, BISHOP_VAL),(Queen, QUEEN_VAL),(King, KING_VAL)])
}

// pub fn printout(&self) -> String {
//     let mut r = String::new();
//     r += &*("            piece_value_weight: ".to_owned() + &self.piece_value_weight.to_string() + ",
//             n_of_moves_weight: " + &self.n_of_moves_weight.to_string() + ",
//             covered_squares_weight: " + &self.covered_squares_weight.to_string() + ",
// 
//             pawn_val: " + &self.pawn_val.to_string() + ",
//             rook_val: " + &self.rook_val.to_string() + ",
//             bishop_val: " + &self.bishop_val.to_string() + ",
//             knight_val: " + &self.knight_val.to_string() + ",
//             king_val: " + &self.king_val.to_string() + ",
//             queen_val: " + &self.queen_val.to_string() + ",
// 
//             piece_values_map: HashMap::new(),
// 
//             pinning_val: " + &self.pinning_val.to_string() + ",
//             check_val: " + &self.check_val.to_string() + ",
//             check_piece_multiplier: " + &self.check_piece_multiplier.to_string() + ",
// 
//             knight_movement_base: " + &self.knight_movement_base.to_string() + ",
//             knight_movement_coef: " + &self.knight_movement_coef.to_string() + ",
// 
//             stalemate_val: " + &self.stalemate_val.to_string() + ",
//             checkmate_val: " + &self.checkmate_val.to_string() + ",");
//     return r;
// }



// println!("{}\n{:b}",board.combined().to_string(), board.combined().to_size(0));
//
// let mut bd : u64 = 0;
// for p in board.pieces(Pawn).into_iter() {
//     // println!("{}", p);
//     // println!("{:b}", (1 as u64)<<p.to_index());
//     bd |= (1 as u64)<<p.to_index();
// }
//
// println!("{}\n{:b}", board.pieces(Pawn).to_string(), board.pieces(Pawn).to_size(0));

// for i in 0..8 {
//     println!("{:b}", board.combined().to_size(8*i) as u8);
// }
// println!("\n--\n{}\n{}",fen_to_str(board.to_string()) , eval(board));

// match board.status() {
//     Checkmate => {println!("checkmate for {}", board.side_to_move().to_index() as f64 - 0.5)}
//     Stalemate => {println!("stalemate for {}", board.side_to_move().to_index())}
//     _=>{ println!("other");}
// }


// move set:
// use std::cmp::Ordering;
// use std::collections::VecDeque;
// use chess::ChessMove;
// use chess::Color::Black;
// use crate::COMPUTER_PLAYER;
//
// pub struct MoveSet {
//     pub(crate) moves: VecDeque<ChessMove>,
//     pub(crate) end_eval: f64
// }
//
// impl Eq for MoveSet {}
//
// impl PartialEq<Self> for MoveSet {
//     fn eq(&self, other: &Self) -> bool {
//         self.end_eval == other.end_eval
//     }
// }
// impl Ord for MoveSet {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // Reverse the ordering to get a min-heap (smallest value has highest priority)
//         if COMPUTER_PLAYER == Black {
//             // other.end_eval  self.end_eval
//             if other.end_eval < self.end_eval {
//                 Ordering::Greater
//             } else if other.end_eval > self.end_eval{
//                 Ordering::Less
//             } else {
//                 Ordering::Equal
//             }
//         } else {
//             if other.end_eval < self.end_eval {
//                 Ordering::Less
//             } else if other.end_eval > self.end_eval{
//                 Ordering::Greater
//             } else {
//                 Ordering::Equal
//             }
//         }
//     }
// }
// impl PartialOrd<Self> for MoveSet {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
//
//
// pub fn compute_best_move_untheaded(board : &Board, depth : u32, bot : &Bot) -> ChessMove {
//     if !IS_THREADED {
//         return compute_best_move_untheaded(board, depth, bot);
//     }
//     let maximizing = board.side_to_move() == White;
//     let mut best_move = None;
//     let mut best_value = if maximizing {f64::MIN} else {f64::MAX};
//     let legal_moves = all_moves(board);
//
//     for &mv in &legal_moves {
//         let bd = board.make_move_new(mv);
//
//         let value = minimax(&bd, depth - 1, maximizing, f64::MIN, f64::MAX, bot);
//
//         if (value > best_value && maximizing) || (value < best_value && !maximizing) {
//             best_value = value;
//             best_move = Some(mv);
//         }
//     }
//
//     best_move.unwrap() // Return the best move found
// }


