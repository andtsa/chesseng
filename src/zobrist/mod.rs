use std::collections::HashMap;
use chess::Board;
use rand::{random, Rng, thread_rng};

/// Zobrist table for generating zobrist hashes
/// but wtf is any of this? why don't we use regular hashes?
/// -> because even though the hashes here are less elegant and
/// slower to initialise, you can get the hash of a board state
/// by only knowing the previous and the move that led to it.
/// This allows for really fast computations on the fly.
/// the Zobrist table maps combinations of pieces & positions to random numbers
/// we can then use these to generate hashes.
///
/// A little explanation as to what everything means:
/// keys look like (piece,(x,y))
/// where pieces are represented as ints:
/// white pawn: 1
/// white knight: 2
/// white bishop: 3
/// white rook: 4
/// white queen: 5
/// white king: 6
/// black pawn: 7
/// black knight: 8
/// black bishop: 9
/// black rook: 10
/// black queen: 11
/// black king: 12
pub fn generate_zobrist_table() -> HashMap<(i32,i32,i32),u64> {
    let pieces = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let mut map : HashMap<(i32,i32,i32),u64> = HashMap::new();
    let mut gen = thread_rng();
    for x in 0..8 {
        for y in 0..8 {
            for p in &pieces {
                map.insert((*p,x,y),gen.gen());
            }
        }
    }
    map
}

pub fn compute_zobrist_hash(board : &Board, table : &HashMap<(i32,i32,i32),u64>) -> u64 {

    todo!()
}