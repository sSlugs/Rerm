use crate::board::board::{Colour};
use crate::constants::*;

static DIAG_MASKS: [u64;64] = {
    let (d, _) = make_diag_masks();
    d
};
static ANTI_MASKS: [u64;64] = {
    let (_, a) = make_diag_masks();
    a
};


#[inline(always)]
pub fn rook_attacks(sq: usize,occ: &[Bitboard;3],colour: Colour) -> Bitboard{
    let bb = 1u64 << sq;

    let rank = (sq / 8) as u64;
    let file = (sq % 8) as u64;

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

#[inline(always)]
pub fn bishop_attacks(sq: usize,occ: &[Bitboard;3],colour: Colour) -> Bitboard {

    let test_occ = occ[2] & !BITMASKS[sq]; // doing this to avoid the problem where it doesnt generate moves below the bishop if there is a bishop on the sqare in the occ is 7ns delay, please fix!!!

    // Precomputed masks used 
    let bb = 1u64 << sq;
    let two_bb = bb << 1;

    // use const arrays (no loops)
    let diag_mask = DIAG_MASKS[sq];
    let anti_diag_mask = ANTI_MASKS[sq];

    let occ_diag      = test_occ & diag_mask;
    let occ_anti_diag = test_occ & anti_diag_mask;

    let left_diag  = occ_diag.wrapping_sub(two_bb);
    let right_diag = (occ_diag.reverse_bits()
        .wrapping_sub(two_bb.reverse_bits()))
        .reverse_bits();
    let attacks_diag = (left_diag ^ right_diag) & diag_mask;

    let left_anti  = occ_anti_diag.wrapping_sub(two_bb);
    let right_anti = (occ_anti_diag.reverse_bits()
        .wrapping_sub(two_bb.reverse_bits()))
        .reverse_bits();
    let attacks_anti = (left_anti ^ right_anti) & anti_diag_mask;

    // mask out own square and friendly pieces (friendly_occ passed in)
    ((attacks_diag | attacks_anti) & !bb) & !occ[colour as usize]
}

#[inline(always)]
pub fn queen_attacks(sq: usize,occ: &[Bitboard;3],colour: Colour) -> Bitboard {
    rook_attacks(sq, &occ, colour) | bishop_attacks(sq, &occ, colour)
}



//              ||
// Non Sliders  ||
// Non Sliders \  /
//              \/

#[inline(always)]
pub fn pawn_attacks(pieces: &[[Bitboard; 6]],occ: &[Bitboard; 3],colour: Colour,ep_square: Option<Bitboard>) -> [Bitboard;6] {
    let empty = !occ[2]; // occupancys of where every piece isnt
    let pawns = pieces[colour as usize][0]; // curent turns colour pawns occ
    let enemy = occ[colour.opposite() as usize]; // enemy occupancy

    let mut single_pushes = 0;
    let mut double_pushes = 0;
    let mut captures = 0;
    let mut promotions = 0;
    let mut ep_attack = 0;


    if colour == Colour::White {
        // WHITE

        let sp = (pawns << 8) & empty;

        promotions |= sp & RANK_8;
        single_pushes |= sp & !RANK_8;

        let rank2_pawns = pawns & RANK_2;
        let one_forward = (rank2_pawns << 8) & empty;
        double_pushes = (one_forward << 8) & empty & RANK_4;

        let left_attacks  = (pawns << 7) & !FILE_H;
        let right_attacks = (pawns << 9) & !FILE_A;

        let cap_left  = left_attacks & enemy;
        let cap_right = right_attacks & enemy;

        promotions |= (cap_left | cap_right) & RANK_8;
        captures   |= (cap_left | cap_right) & !RANK_8;

        if let Some(ep_sq) = ep_square {
            ep_attack = (left_attacks & ep_sq) | (right_attacks & ep_sq) // unfinished
        }

    } else {
        // BLACK 
        let sp = (pawns >> 8) & empty;
        promotions |= sp & RANK_1;
        single_pushes |= sp & !RANK_1;

        let rank7_pawns = pawns & RANK_7;
        let one_forward = (rank7_pawns >> 8) & empty;
        double_pushes = (one_forward >> 8) & empty & RANK_5;

        let left_attacks  = (pawns >> 9) & !FILE_H;
        let right_attacks = (pawns >> 7) & !FILE_A;

        let cap_left  = left_attacks & enemy;
        let cap_right = right_attacks & enemy;

        promotions |= (cap_left | cap_right) & RANK_1;
        captures   |= (cap_left | cap_right) & !RANK_1;

        if let Some(ep_sq) = ep_square {
            ep_attack = (left_attacks & ep_sq) | (right_attacks & ep_sq)
        }
    }
    [single_pushes, double_pushes, captures, promotions, ep_attack,pawns]

}

#[inline(always)]
pub fn knight_attacks(sq: usize,occ: &[Bitboard;3],colour: Colour) -> Bitboard {
    KNIGHT_ATTACKS[sq] & !occ[colour as usize]
}

#[inline(always)]
pub fn king_attacks(sq: usize,occ: &[Bitboard;3],colour: Colour) -> Bitboard {
    KING_ATTACKS[sq] & !occ[colour as usize]
}