pub const fn knight_attacks_from(sq: u8) -> Bitboard {
    let bb = 1u64 << sq;
    let mut attacks = 0u64;

    attacks |= (bb << 17) & !FILE_A; 
    attacks |= (bb << 10) & !FILE_AB; 

    attacks |= (bb << 15) & !FILE_H;   
    attacks |= (bb << 6)  & !FILE_GH;  

    attacks |= (bb >> 15) & !FILE_A;  
    attacks |= (bb >> 6)  & !FILE_AB; 

    attacks |= (bb >> 17) & !FILE_H;  
    attacks |= (bb >> 10) & !FILE_GH; 

    attacks
}

pub const fn king_attacks_from(sq: u8) -> Bitboard {
    let bb = 1u64 << sq;

    let mut attacks = (bb << 8) | (bb >> 8);

    let left_right = ((bb << 1) & !FILE_A) | ((bb >> 1) & !FILE_H);
    attacks |= left_right;

    attacks |= (left_right << 8) | (left_right >> 8);

    attacks
}

pub const KNIGHT_ATTACKS: [Bitboard; 64] = {
    let mut table = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = knight_attacks_from(sq as u8);
        sq += 1;
    }
    table
};

pub const KING_ATTACKS: [Bitboard; 64] = {
    let mut table = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = king_attacks_from(sq as u8);
        sq += 1;
    }
    table
};


//


pub const fn make_diag_masks() -> ([u64;64], [u64;64]) {
    let mut diags = [0u64; 64];
    let mut antis = [0u64; 64];
    let mut sq = 0usize;
    while sq < 64 {
        let rank = (sq / 8) as i32;
        let file = (sq % 8) as i32;
        let mut mask = 0u64;

        let mut r = rank;
        let mut f = file;

        mask |= 1u64 << sq;

        r = rank + 1; f = file + 1;
        while r < 8 && f < 8 {
            mask |= 1u64 << (r * 8 + f) as usize;
            r += 1; f += 1;
        }

        r = rank - 1; f = file - 1;
        while r >= 0 && f >= 0 {
            mask |= 1u64 << (r * 8 + f) as usize;
            r -= 1; f -= 1;
        }
        diags[sq] = mask;

        let mut amask = 0u64;
        amask |= 1u64 << sq;

        r = rank + 1; f = file - 1;
        while r < 8 && f >= 0 {
            amask |= 1u64 << (r * 8 + f) as usize;
            r += 1; f -= 1;
        }

        r = rank - 1; f = file + 1;
        while r >= 0 && f < 8 {
            amask |= 1u64 << (r * 8 + f) as usize;
            r -= 1; f += 1;
        }
        antis[sq] = amask;

        sq += 1;
    }
    (diags, antis)
}

// Masks
pub const BITMASKS: [u64; 64] = {
    let mut m = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        m[i] = 1u64 << i;
        i += 1;
    }
    m
};

pub const FILE_AB: Bitboard = FILE_A | (FILE_A << 1);
pub const FILE_GH: Bitboard = FILE_H | (FILE_H >> 1);

pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_H: Bitboard = 0x8080_8080_8080_8080;
 
pub const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
pub const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
pub const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
pub const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
pub const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
pub const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
pub const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

pub type Bitboard = u64;
pub const EMPTY: u8 = 0xFF;