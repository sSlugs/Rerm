pub type Bitboard = u64;
pub const EMPTY: Bitboard = 0;

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

// board

pub struct Board {
    pieces: [[Bitboard; 6];2], // 6 pieces 2 colours || 0:white,1:black || 0:pawn,1:knight,2:bishop,3:rook,4:queen,5:king

    occupancy: [Bitboard; 3], // 0:white piece, 1:black piece, 3:any piece
}

impl Board { // Board manipulation functions
    pub fn set_square(&mut self,sq: u64,piece:) {

    }
}

impl Board { // Init functions
    pub fn init_new() -> Board { //creates board with default values
        Board {
            pieces: [[  0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000],[

                        0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
                        ]],

            occupancy: [0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111],
        }
    }
}

// bit manipulation 
#[inline(always)]
pub fn set_bit(bb: &mut u64, sq: u8) {
    // safe, bounds-checked
    *bb |= BITMASKS[sq as usize];
}

#[inline(always)]
pub fn clear_bit(bb: &mut u64, sq: u8) {
    *bb &= !BITMASKS[sq as usize];
}

#[inline(always)]
pub fn toggle_bit(bb: &mut u64, sq: u8) {
    *bb ^= BITMASKS[sq as usize];
}

#[inline(always)]
pub fn is_set(bb: u64, sq: u8) -> bool {
    bb & BITMASKS[sq as usize] != 0
}