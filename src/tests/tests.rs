use crate::board::board::PieceType;
use std::time::Instant;
use crate::move_gen::pseudo_gen::rook_attacks;
use crate::board::board::*;

#[test]
fn bench_rook_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        rook_attacks(sq, bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        rook_attacks(sq, bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_board_manipulation() {

    let mut bd = Board::init_empty();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        bd.set_square(sq, Colour::White,PieceType::Pawn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        bd.set_square(sq, Colour::White,PieceType::Pawn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}
