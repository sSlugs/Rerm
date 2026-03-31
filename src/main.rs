pub mod board;
use board::board::*;
pub mod move_gen;
use move_gen::pseudo_gen::*;
use crate::move_gen::legal_move_gen::*;
use crate::GUI::guimain;
pub mod tests;
pub mod GUI;
pub mod constants;

fn main() {
    let mut bd = Board::init_new();

    //let n = pawn_attacks(&bd.pieces,&bd.occupancy,bd.turn,bd.ep_sq).2;

    let n = bishop_attacks(27,&bd.occupancy,Colour::Black);

    let bin = format!("{:064b}", n);

    // rank 8 printed first, rank 1 printed last
    for chunk in bin.as_bytes().chunks(8) {
        // flip bits in the rank so A-file is on the left
        let line: String = chunk.iter().rev().map(|&c| c as char).collect();
        println!("{}", line);
    }

    println!("{}",bd.in_check());
    
    bd.print();

    guimain::main();
}