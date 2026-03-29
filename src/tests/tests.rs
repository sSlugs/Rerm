use crate::board::board::PieceType;
use crate::board::moves::Move;
use std::time::Instant;
use crate::move_gen::pseudo_gen::{bishop_attacks, pawn_attacks, queen_attacks, rook_attacks,knight_attacks,king_attacks};
use crate::board::board::*;

#[test]
fn bench_rook_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        rook_attacks(sq, &bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        rook_attacks(sq, &bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_bishop_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        bishop_attacks(sq, &bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        bishop_attacks(sq, &bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_queen_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        queen_attacks(sq, &bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        queen_attacks(sq, &bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_knight_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        knight_attacks(sq, &bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        knight_attacks(sq, &bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_king_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        king_attacks(sq, &bd.occupancy,bd.turn);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        king_attacks(sq, &bd.occupancy,bd.turn);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]

fn bench_pawn_attacks() {

    let bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        pawn_attacks(&bd.pieces,&bd.occupancy,bd.turn,bd.ep_sq);
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        pawn_attacks(&bd.pieces, &bd.occupancy,bd.turn,bd.ep_sq);
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}


#[test]
fn bench_board_manipulation() {

    let mut bd = Board::init_new();

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        bd.in_check();
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        bd.in_check();
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}

#[test]
fn bench_push_pseudo_move() {

    let mut bd = Board::init_new();

    let mv: Move = Move{
        from: 10,
        to: 18,
        promotion_piece: None, 
        flags: 0,
    };

    let sq = 27;
    let runs = 1_000_000;

    // warm up
    for _ in 0..100 {
        bd.make_move(mv);
        bd = Board::init_new();
    }

    // time measurement
    let start = Instant::now();
    for _ in 0..runs {
        bd.make_move(mv);
        bd = Board::init_new();
    }
    let dur = start.elapsed();

    let ns_per_call = (dur.as_secs_f64() * 1e9) / (runs as f64);
    println!("rook_attacks avg: {:.3} ns/call", ns_per_call);
}
