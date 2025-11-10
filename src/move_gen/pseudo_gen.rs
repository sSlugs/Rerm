use crate::board::board::{Bitboard, Colour};

pub fn rook_attacks(sq: usize,occ: [Bitboard;3],colour: Colour) -> Bitboard{
    let bb = 1u64 << sq;

    let rank = (sq / 8) as u64;
    let file = (sq % 8) as u64;

    const FILE_A: u64 = 0x0101_0101_0101_0101;
    const RANK_1: u64 = 0x0000_0000_0000_00FF;

    let rank_mask = RANK_1 << (rank * 8);
    let file_mask = FILE_A << file;

    let occ_rank = occ[2] & rank_mask;
    let occ_file = occ[2] & file_mask;

    let bb_rev = bb.reverse_bits();

    let left_rank = occ_rank.wrapping_sub(bb << 1);

    let occ_rank_rev = occ_rank.reverse_bits();
    let right_rank = (occ_rank_rev
        .wrapping_sub(bb_rev << 1))
        .reverse_bits();

    let horiz = (left_rank ^ right_rank) & rank_mask;

    let left_file = occ_file.wrapping_sub(bb << 1);

    let occ_file_rev = occ_file.reverse_bits();
    let right_file = (occ_file_rev
        .wrapping_sub(bb_rev << 1))
        .reverse_bits();

    let vert = (left_file ^ right_file) & file_mask;

    ((horiz | vert) & !bb) & !occ[colour as usize]
}

pub fn bishop_attacks(sq: usize,occ: [Bitboard;3],colour: Colour) -> Bitboard {
    
}