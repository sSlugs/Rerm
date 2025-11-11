pub mod board;
use board::board::*;
pub mod move_gen;
use move_gen::pseudo_gen::*;
pub mod tests;

fn main() {
    let mut bd = Board::init_new();

    bd.set_square(0, Colour::White, PieceType::Pawn);
    bd.set_square(9, Colour::Black, PieceType::Pawn);

    //let n = pawn_attacks(&bd.pieces,&bd.occupancy,bd.turn,bd.ep_sq).2;

    let n = knight_attacks(35,&bd.occupancy,bd.turn);

    let bin = format!("{:064b}", n);

    // rank 8 printed first, rank 1 printed last
    for chunk in bin.as_bytes().chunks(8) {
        // flip bits in the rank so A-file is on the left
        let line: String = chunk.iter().rev().map(|&c| c as char).collect();
        println!("{}", line);
    }

}