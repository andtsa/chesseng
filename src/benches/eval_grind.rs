use chess::Board;
use iai::black_box;
use sandy_engine::Opts;
use sandy_engine::search::moveordering::ordered_moves;

fn board_setup() {
    let _ = Board::default();
}

fn move_gen() {
    let _ = ordered_moves(&Board::default());
}

fn evaluation_benches() {
    let pos = Board::default();
    sandy_engine::evaluation::evaluate(black_box(&pos), &ordered_moves(&pos), Opts::new());
}

iai::main!(board_setup, move_gen, evaluation_benches);
