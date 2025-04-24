pub type Bitboard = u64;

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

// Enums
#[repr(usize)]
#[derive(Copy, Clone,PartialEq,Debug)]
pub enum Colour { White = 0, Black = 1 }

#[repr(usize)]
#[derive(Copy, Clone,PartialEq,Debug)]
pub enum PieceType {
    Pawn   = 0,
    Knight = 1,
    Bishop = 2,
    Rook   = 3,
    Queen  = 4,
    King   = 5,
}

// board

pub struct Board {
    pub pieces: [[Bitboard; 6];2], // 6 pieces 2 colours || 0:white,1:black || 0:pawn,1:knight,2:bishop,3:rook,4:queen,5:king

    pub occupancy: [Bitboard; 3], // 0:white piece, 1:black piece, 3:any piece

    pub mailbox: [u8;64],

    turn: Colour,
    castle_rights: u8,
}

impl Board { // Board manipulation functions
    #[inline(always)]
    pub fn set_square(&mut self, sq: usize, colour: Colour, piece: PieceType) {
        let m       = BITMASKS[sq];
        let idx_new = (colour as usize)*6 + (piece as usize);


        let old = self.mailbox[sq];
        if old != 0xFF && old as usize != idx_new {

            self.pieces[old as usize / 6][old as usize % 6] ^= m;

            let col_old = if old < 6 { 0 } else { 1 };
            self.occupancy[col_old] ^= m;
            self.occupancy[2]       ^= m;
        }

        if old as usize != idx_new {
            self.pieces[colour as usize][piece as usize] ^= m;
            self.occupancy[colour as usize] ^= m;
            self.occupancy[2]               ^= m;
            self.mailbox[sq] = idx_new as u8;
        }
    }

    #[inline(always)]
    pub fn clear_square(&mut self,sq: usize) { //clears square on board
        let old = self.mailbox[sq];

        if old == 0xFF { return; }

        let nm       = !BITMASKS[sq];
        self.pieces[old as usize / 6][old as usize % 6] ^= nm;
        self.occupancy[old as usize / 6] ^= nm;
        self.occupancy[2]               ^= nm;

        self.mailbox[sq] = 0xFF;
    }

    #[inline(always)]
    pub fn piece_at_square(&mut self,sq: usize) -> Option<(PieceType,Colour)> { // returns piecetype and colour of square of board
        let square = self.mailbox[sq];

        if square == 0xFF {return None;}

        let colour = if square < 6 { Colour::White } else { Colour::Black };
        let piece = match square % 6 {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!(),
        };

        return Some((piece,colour));
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
                        0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111],

            mailbox: [
                3, 1, 2, 4, 5, 2, 1, 3,
                0, 0, 0, 0, 0, 0, 0, 0,
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF, 
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                6, 6, 6, 6, 6, 6, 6, 6,
                9, 7, 8, 10,11, 8, 7, 9,
            ],
            turn: Colour::White,
            castle_rights: 0b0000_1111,
        }
    }

    pub fn init_empty() -> Self {
        Board {
            pieces: [[0;6];2],
            occupancy: [0;3],
            mailbox: [0xFF; 64],  // 0xFF means “no piece”

            turn: Colour::White,
            castle_rights: 0b0000_1111,
        }
    }
}

impl Board { // UI / UX
    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let glyph = match self.mailbox[sq] {
                    0xFF => '.',
                    idx => {
                        let idx = idx as usize;
                        let color = if idx < 6 { Colour::White } else { Colour::Black };
                        let pt = idx % 6;
                        let ch = match pt {
                            0 => 'P', 
                            1 => 'N', 
                            2 => 'B', 
                            3 => 'R', 
                            4 => 'Q', 
                            5 => 'K',
                            _ => '?',
                        };
                        if color == Colour::Black {
                            ch.to_ascii_lowercase()
                        } else {
                            ch
                        }
                    }
                };
                print!("{} ", glyph);
            }
            println!();
        }
        print!("  ");
        for file in 0..8 {
            let letter = (b'a' + file as u8) as char;
            print!("{} ", letter);
        }
        println!();
    }
}