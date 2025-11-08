pub mod board;
use board::board::*;
pub mod move_gen;
use move_gen::pseudo_gen::*;

fn main() {
    let mut bd = Board::init_new();

    let x = 123;

    if let Some((piece, colour)) = bd.piece_at_square(x) {
    println!("There’s a {:?} {:?} on square {}",colour, piece,x);
    } else {
        println!("Square {} does not exist",x);
    }

    bd.print();
}