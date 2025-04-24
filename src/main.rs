pub mod board;
use board::board::*;
pub mod move_gen;
use move_gen::pseudo_gen::*;

fn main() {
    let bd = Board::init_new();

    bd.print();
}
