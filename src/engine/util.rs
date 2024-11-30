//! Legacy code, looks garbage but it works
#![allow(dead_code)]

use std::str::FromStr;

use chess::Board;
use chess::ChessMove;
use chess::Piece;

/// positions from rustfish github
pub fn bench_positions() -> Vec<Board> {
    let positions = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 11",
        "4rrk1/pp1n3p/3q2pQ/2p1pb2/2PP4/2P3N1/P2B2PP/4RRK1 b - - 7 19",
        "rq3rk1/ppp2ppp/1bnpb3/3N2B1/3NP3/7P/PPPQ1PP1/2KR3R w - - 7 14",
        "r1bq1r1k/1pp1n1pp/1p1p4/4p2Q/4Pp2/1BNP4/PPP2PPP/3R1RK1 w - - 2 14",
        "r3r1k1/2p2ppp/p1p1bn2/8/1q2P3/2NPQN2/PPP3PP/R4RK1 b - - 2 15",
        "r1bbk1nr/pp3p1p/2n5/1N4p1/2Np1B2/8/PPP2PPP/2KR1B1R w kq - 0 13",
        "r1bq1rk1/ppp1nppp/4n3/3p3Q/3P4/1BP1B3/PP1N2PP/R4RK1 w - - 1 16",
        "4r1k1/r1q2ppp/ppp2n2/4P3/5Rb1/1N1BQ3/PPP3PP/R5K1 w - - 1 17",
        "2rqkb1r/ppp2p2/2npb1p1/1N1Nn2p/2P1PP2/8/PP2B1PP/R1BQK2R b KQ - 0 11",
        "r1bq1r1k/b1p1npp1/p2p3p/1p6/3PP3/1B2NN2/PP3PPP/R2Q1RK1 w - - 1 16",
        "3r1rk1/p5pp/bpp1pp2/8/q1PP1P2/b3P3/P2NQRPP/1R2B1K1 b - - 6 22",
        "r1q2rk1/2p1bppp/2Pp4/p6b/Q1PNp3/4B3/PP1R1PPP/2K4R w - - 2 18",
        "4k2r/1pb2ppp/1p2p3/1R1p4/3P4/2r1PN2/P4PPP/1R4K1 b - - 3 22",
        "3q2k1/pb3p1p/4pbp1/2r5/PpN2N2/1P2P2P/5PP1/Q2R2K1 b - - 4 26",
        "6k1/6p1/6Pp/ppp5/3pn2P/1P3K2/1PP2P2/3N4 b - - 0 1",
        "3b4/5kp1/1p1p1p1p/pP1PpP1P/P1P1P3/3KN3/8/8 w - - 0 1",
        "2K5/p7/7P/5pR1/8/5k2/r7/8 w - - 0 1",
        "8/6pk/1p6/8/PP3p1p/5P2/4KP1q/3Q4 w - - 0 1",
        "7k/3p2pp/4q3/8/4Q3/5Kp1/P6b/8 w - - 0 1",
        "8/2p5/8/2kPKp1p/2p4P/2P5/3P4/8 w - - 0 1",
        "8/1p3pp1/7p/5P1P/2k3P1/8/2K2P2/8 w - - 0 1",
        "8/pp2r1k1/2p1p3/3pP2p/1P1P1P1P/P5KR/8/8 w - - 0 1",
        "8/3p4/p1bk3p/Pp6/1Kp1PpPp/2P2P1P/2P5/5B2 b - - 0 1",
        "5k2/7R/4P2p/5K2/p1r2P1p/8/8/8 b - - 0 1",
        "6k1/6p1/P6p/r1N5/5p2/7P/1b3PP1/4R1K1 w - - 0 1",
        "1r3k2/4q3/2Pp3b/3Bp3/2Q2p2/1p1P2P1/1P2KP2/3N4 w - - 0 1",
        "6k1/4pp1p/3p2p1/P1pPb3/R7/1r2P1PP/3B1P2/6K1 w - - 0 1",
        "8/3p3B/5p2/5P2/p7/PP5b/k7/6K1 w - - 0 1",
        // 5-man positions
        "8/8/8/8/5kp1/P7/8/1K1N4 w - - 0 1",  // Kc2 - mate
        "8/8/8/5N2/8/p7/8/2NK3k w - - 0 1",   // Na2 - mate
        "8/3k4/8/8/8/4B3/4KB2/2B5 w - - 0 1", // draw
        // 6-man positions
        "8/8/1P6/5pr1/8/4R3/7k/2K5 w - - 0 1",  // Re5 - mate
        "8/2p4P/8/kr6/6R1/8/8/1K6 w - - 0 1",   // Ka2 - mate
        "8/8/3P3k/8/1p6/8/1P6/1K3n2 b - - 0 1", // Nd2 - draw
        // 7-man positions
        "8/R7/2q5/8/6k1/8/1P5p/K6R w - - 0 124", // Draw
        // Mate and stalemate positions
        "6k1/3b3r/1p1p4/p1n2p2/1PPNpP1q/P3Q1p1/1R1RB1P1/5K2 b - - 0 1",
        "r2r1n2/pp2bk2/2p1p2p/3q4/3PN1QP/2P3R1/P4PP1/5RK1 w - - 0 1",
        "8/8/8/8/8/6k1/6p1/6K1 w - -",
        "7k/7P/6K1/8/3B4/8/8/8 b - -",
        // positions from sandy_bot games
        "3rr1k1/2pq1p1p/p2b1np1/Pp3b2/8/N2n1Q1P/1P1PNPP1/R1BK1B1R b - - 0 23",
        "r1bk3r/1ppq1pp1/p2p1n1p/4p3/2PnP3/Q2PB1P1/P3BPKP/1R4NR b - - 2 17",
        // mate in 5
        "R1bq1r1k/6pp/2pP1b2/5P2/4pPB1/1r1nP2K/NP2Q2P/2B3NR b - - 10 24",
        "r1b3k1/p7/Pp1nqP2/3p1r1p/7Q/1PR1RN1P/5PB1/7K w - - 1 33",
        "1rbr3k/8/2pq1bpp/2n2P2/2Q1pPB1/R3P3/1PN1K2P/2B3NR b - - 7 28",
        "5r1k/8/b1pq1b1p/R1n2p2/2Q1pP2/1r2P2B/1PN4P/2B2KNR b - - 3 28",
        "rn1qkb2/pbpp1ppr/1p6/P3p3/4n1pP/2N2P1B/1PPP4/R1BQK1NR b KQq - 1 9",
        // mate in 10
        "8/1pB1rnbk/6pn/7q/P3B2P/1P6/6P1/2Q1R2K b - - 0 34",
        "rn3rk1/p7/bp2p2p/1q1pPp1Q/P2Nn3/1Pb2NPP/5PB1/3RR1K1 w - - 2 23",
    ];

    positions
        .iter()
        .map(|p| Board::from_str(p).unwrap())
        .collect()
}

/// A shorter collection of positions for benchmarking
pub fn short_benches() -> Vec<Board> {
    let positions = [
        // starting position
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // 5-man positions
        "8/8/8/8/5kp1/P7/8/1K1N4 w - - 0 1", // Kc2 - mate
        // 6-man positions
        "8/8/1P6/5pr1/8/4R3/7k/2K5 w - - 0 1", // Re5 - mate
        // 7 man position
        "8/R7/2q5/8/6k1/8/1P5p/K6R w - - 0 124",
    ];
    positions
        .iter()
        .map(|p| Board::from_str(p).unwrap())
        .collect()
}

/// Trait for printing the board
pub trait Print {
    /// Print the board
    fn print(&self) -> String;
    /// Print the board with the last move highlighted
    fn print_move(&self, mv: ChessMove, capture: bool) -> String;
}

impl Print for Board {
    fn print(&self) -> String {
        format!(
            "\n+     {:?} to move\n{}",
            self.side_to_move(),
            fen_to_str(self.to_string())
        )
    }
    fn print_move(&self, mv: ChessMove, capture: bool) -> String {
        fen_to_string_highlighted(
            self.to_string(),
            mv,
            capture,
            self.piece_on(mv.get_dest())
                .is_some_and(|p| matches!(p, Piece::Pawn)),
        )
    }
}

/// Convert a FEN string to a visual representation of the board
pub fn fen_to_str(fen: String) -> String {
    let mut r: String = String::new();
    let mut rows: Vec<&str> = fen.split("/").collect();
    rows[7] = (rows[7].split(" ")).collect::<Vec<&str>>()[0];
    for (i, row) in rows.iter().enumerate() {
        r += &*(8 - i).to_string();
        for c in row.chars() {
            if c.is_numeric() {
                r += &*" . ".repeat(c.to_digit(10).unwrap() as usize).to_string();
            } else {
                r += &*(" ".to_owned() + &c.to_string() + " ");
            }
        }
        r += "\n";
    }

    r = r.replace("p", "♟");
    r = r.replace("P", "♙");
    r = r.replace("r", "♜");
    r = r.replace("R", "♖");
    r = r.replace("n", "♞");
    r = r.replace("N", "♘");
    r = r.replace("b", "♝");
    r = r.replace("B", "♗");
    r = r.replace("k", "♚");
    r = r.replace("K", "♔");
    r = r.replace("q", "♛");
    r = r.replace("Q", "♕");

    r += "+ a  b  c  d  e  f  g  h";

    r.to_string()
}

/// Highlight the last played move in the visual representation of the board
pub fn fen_to_string_highlighted(
    full_fen: String,
    mv: ChessMove,
    capture: bool,
    pawn_move: bool,
) -> String {
    let esc: String = if capture {
        "\x1b[1;31m".parse().unwrap()
    } else if pawn_move {
        "\x1b[1;34m".parse().unwrap()
    } else {
        "\x1b[1;94m".parse().unwrap()
    };
    let rst: String = ("\x1b[0m").parse().unwrap();

    let dest = [
        mv.get_dest().get_rank().to_index(),
        mv.get_dest().get_file().to_index(),
    ];
    let source = [
        mv.get_source().get_rank().to_index(),
        mv.get_source().get_file().to_index(),
    ];
    let mut r: String = String::new();
    let fen: Vec<char> = (full_fen.split(" ")).collect::<Vec<&str>>()[0]
        .chars()
        .collect();

    let mut index = 0;
    let mut row = 7;
    let mut col = 0;
    r += "8";
    while index < fen.len() {
        if fen[index].is_numeric() {
            let cols = fen[index].to_digit(10).unwrap() as usize;
            for _ in 0..cols {
                if row == dest[0] && col == dest[1] {
                    r += &*esc;
                }
                if row == source[0] && col == source[1] {
                    r += &*esc;
                }
                r += " . ";
                if (row == dest[0] && col == dest[1]) || (row == source[0] && col == source[1]) {
                    r += &*rst;
                }
                col += 1;
            }
            index += 1;
        } else if fen[index] == '/' {
            r += &*("\n".to_owned() + &*(row).to_string());
            index += 1;
            col = 0;
            row -= 1;
        } else {
            if row == dest[0] && col == dest[1] {
                r += &*esc;
            }
            if row == source[0] && col == source[1] {
                r += &*esc;
            }
            r += &*(" ".to_owned() + &*fen[index].to_string() + " ");
            if (row == dest[0] && col == dest[1]) || (row == source[0] && col == source[1]) {
                r += &*rst;
            }
            index += 1;
            col += 1;
        }
    }

    r = r.replace("p", "♟");
    r = r.replace("P", "♙");
    r = r.replace("r", "♜");
    r = r.replace("R", "♖");
    r = r.replace("n", "♞");
    r = r.replace("N", "♘");
    r = r.replace("b", "♝");
    r = r.replace("B", "♗");
    r = r.replace("k", "♚");
    r = r.replace("K", "♔");
    r = r.replace("q", "♛");
    r = r.replace("Q", "♕");

    r += "\n+ a  b  c  d  e  f  g  h";

    r
}

/// convert a bitboard (u64) to a FEN string, displaying 1s as # and
/// 0s as empty squares.
pub fn bitboard_to_fen(bb: u64) -> String {
    let mut r: String = String::new();
    for i in 0..64 {
        if i % 8 == 0 {
            r += &*(8 - i / 8).to_string();
        }
        r += if (bb >> i) & 1 == 1 { " #" } else { " ." };
        if i % 8 == 7 {
            r += "\n";
        }
    }
    r += "+ a b c d e f g h";
    r
}
