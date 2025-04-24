pub mod board;
use board::board::*;

fn main() {
    let mut bd = Board::init_new();

    bd.print();
}
