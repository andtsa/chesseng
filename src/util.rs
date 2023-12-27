use chess::{Board, ChessMove, Color, File, MoveGen, Rank, Square};
use chess::File::{A,B,C,D,E,F,G,H};
use chess::Piece::Queen;
use chess::Rank::{First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth};

pub fn all_moves(board : &Board) -> Vec<ChessMove> {
    let mg = MoveGen::new_legal(board);
    return mg.collect();
}
pub fn make_cm(s : &str) -> ChessMove {
    let spl : Vec<&str> = s.split(" ").collect();
    return as_cm(spl[0].parse().unwrap(), spl[1].parse().unwrap());
}
pub fn as_cm(source : String, dest : String) -> ChessMove {
    let s: Vec<char> = source.chars().collect();
    let d : Vec<char> = dest.chars().collect();

    let mut source_rank: Rank = Rank::First;
    let mut source_file: File = File::A;
    let mut dest_rank: Rank = Rank::First;
    let mut dest_file: File = File::A;
    match s[0] {
        'a'=> { source_file =A}
        'b'=> { source_file =B}
        'c'=> { source_file =C}
        'd'=> { source_file =D}
        'e'=> { source_file =E}
        'f'=> { source_file =F}
        'g'=> { source_file =G}
        'h'=> { source_file =H}
        _ => {}
    }
    match s[1] {
        '1'=> { source_rank =First}
        '2'=> { source_rank =Second}
        '3'=> { source_rank =Third}
        '4'=> { source_rank =Fourth}
        '5'=> { source_rank =Fifth}
        '6'=> { source_rank =Sixth}
        '7'=> { source_rank =Seventh}
        '8'=> { source_rank =Eighth}
        _ => {}
    }
    match d[0] {
        'a'=> { dest_file =A}
        'b'=> { dest_file =B}
        'c'=> { dest_file =C}
        'd'=> { dest_file =D}
        'e'=> { dest_file =E}
        'f'=> { dest_file =F}
        'g'=> { dest_file =G}
        'h'=> { dest_file =H}
        _ => {}
    }
    match d[1] {
        '1'=> { dest_rank =First}
        '2'=> { dest_rank =Second}
        '3'=> { dest_rank =Third}
        '4'=> { dest_rank =Fourth}
        '5'=> { dest_rank =Fifth}
        '6'=> { dest_rank =Sixth}
        '7'=> { dest_rank =Seventh}
        '8'=> { dest_rank =Eighth}
        _ => {}
    }

    let mut mv = ChessMove::new(Square::make_square(source_rank, source_file), Square::make_square(dest_rank, dest_file), None);
    if d.len() == 3 {
        mv = ChessMove::new(Square::make_square(source_rank, source_file), Square::make_square(dest_rank, dest_file), Option::from(Queen));
    }

    return mv;
}

pub fn fen_to_str(fen : String) -> String {
    let mut r : String = String::new();
    let mut rows : Vec<&str> = fen.split("/").collect();
    rows[7] = (rows[7].split(" ")).collect::<Vec<&str>>()[0];
    for i in 0..8 {
        r += &*(8-i).to_string();
        for c in rows[i].chars() {
            if c.is_numeric() {
                r += &*" . ".repeat(c.to_digit(10).unwrap() as usize).to_string();
            } else {
                r += &*(" ".to_owned() + &c.to_string()+" ");
            }
        }
        r += &*"\n";

    }

    r = r.replace("p","♟");
    r = r.replace("P","♙");
    r = r.replace("r","♜");
    r = r.replace("R","♖");
    r = r.replace("n","♞");
    r = r.replace("N","♘");
    r = r.replace("b","♝");
    r = r.replace("B","♗");
    r = r.replace("k","♚");
    r = r.replace("K","♔");
    r = r.replace("q","♛");
    r = r.replace("Q","♕");

    r += &*"+ a  b  c  d  e  f  g  h";

    return r.to_string();
}


pub fn fen_to_string_highlighted(full_fen : String, mv : ChessMove, capture : bool, pawn_move : bool) -> String {

    let esc: String = if capture {"\x1b[1;31m".parse().unwrap()} else if pawn_move {"\x1b[1;34m".parse().unwrap()} else {"\x1b[1;94m".parse().unwrap()};
    let rst: String = ("\x1b[0m").parse().unwrap();

    let dest = [mv.get_dest().get_rank().to_index(), mv.get_dest().get_file().to_index()];
    let source = [mv.get_source().get_rank().to_index(), mv.get_source().get_file().to_index()];
    let mut r : String = String::new();
    let fen : Vec<char> = (full_fen.split(" ")).collect::<Vec<&str>>()[0].chars().collect();

    let mut index = 0;
    let mut row = 7;
    let mut col = 0;
    r += "8";
    while index < fen.len() {
        if fen[index].is_numeric() {
            let cols = fen[index].to_digit(10).unwrap() as usize;
            for _ in 0..cols {
                if row == dest[0] && col == dest[1] {r += &*esc;}
                if row == source[0] && col == source[1] {r += &*esc;}
                r += " . ";
                if (row == dest[0] && col == dest[1]) || (row == source[0] && col == source[1]) {r += &*rst;}
                col += 1;
            }
            index += 1;
        } else if fen[index] == '/' {
            r += &*("\n".to_owned() + &*(row).to_string());
            index+=1;
            col = 0;
            row -= 1;
        } else {
            if row == dest[0] && col == dest[1] {r += &*esc;}
            if row == source[0] && col == source[1] {r += &*esc;}
            r += &*(" ".to_owned() + &*fen[index].to_string()+" ");
            if (row == dest[0] && col == dest[1]) || (row == source[0] && col == source[1]) {r += &*rst;}
            index += 1;
            col += 1;
        }
    }

    r = r.replace("p","♟");
    r = r.replace("P","♙");
    r = r.replace("r","♜");
    r = r.replace("R","♖");
    r = r.replace("n","♞");
    r = r.replace("N","♘");
    r = r.replace("b","♝");
    r = r.replace("B","♗");
    r = r.replace("k","♚");
    r = r.replace("K","♔");
    r = r.replace("q","♛");
    r = r.replace("Q","♕");

    r += &*"\n+ a  b  c  d  e  f  g  h";

    return r;
}

pub trait Stringify {
    fn stringify(&self) -> String;
}

impl Stringify for Color {
    fn stringify(&self) -> String {
        match *self {
            Color::White => "White",
            Color::Black => "Black",
        }.parse().unwrap()
    }
}

/// return the same board but set the side_to_move arg[1]
/// note: will return none if current player is in check.
pub fn to_side(board : &Board, side : Color) -> Option<Board> {
    return if board.side_to_move() == side {
        Some(Board::from(*board))
    } else {
        board.null_move()
    }
}

pub fn pick_max(a : f64, b : f64) -> f64 {
    return if a > b {
        a
    } else {
        b
    }
}
pub fn pick_min(a : f64, b : f64) -> f64 {
    return if a < b {
        a
    } else {
        b
    }
}
