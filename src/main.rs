pub mod board;
use board::board::*;
use std::hint::black_box;
use std::arch::x86_64::_rdtsc;

fn main() {
    let mut bd = Board::init_empty();
    bd.print();

    // Warm up (avoid cold-cache & branch‐predictor effects)
    for _ in 0..10_000 {

        bd.set_square(42, Colour::White, PieceType::Knight);

    }

    // Benchmark via RDTSC
    const RUNS: u64 = 1_000_000;
    let start = unsafe { _rdtsc() };
    for _ in 0..RUNS {

        bd.set_square(42, Colour::Black, PieceType::King);

        black_box(&bd);
    }
    let end = unsafe { _rdtsc() };

    // Compute averages
    let avg_cycles = (end - start) as f64 / RUNS as f64;
    let cpu_ghz    = 4.892; 
    let avg_ns     = avg_cycles / cpu_ghz;

    println!("Avg cycles: {:.2}", avg_cycles);
    println!("Avg time:   {:.3} ns", avg_ns);
    bd.print();
}
