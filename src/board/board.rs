use crate::{constants::*, move_gen::pseudo_gen::{bishop_attacks, rook_attacks}};
use std::ops::Not;

impl Not for Colour {
    type Output = Colour;

    #[inline(always)]
    fn not(self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

// Enums
#[repr(usize)]
#[derive(Copy, Clone,PartialEq,Debug,Eq)]
pub enum Colour { White = 0, Black = 1 }

impl Colour {
    pub fn opposite(self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone,PartialEq,Debug,Eq)]
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

    pub occupancy: [Bitboard; 3], // 0:white piece, 1:black piece, 2:any piece

    pub mailbox: [u8;64],

    pub turn: Colour,
    pub castle_rights: u8, // first 4 bits respecetivley 0000. wqc wkc bqc bkc 0001 0010 0100 1000, 1 means castling allowed

    pub ep_sq: Option<u64>,

    pub halfmove_clock: u8,
}

impl Board { // Board manipulation functions
    #[inline(always)]
    pub fn set_square(&mut self, sq: usize, colour: Colour, piece: PieceType) {
        let m       = BITMASKS[sq];
        let idx_new = (colour as usize)*6 + (piece as usize);


        let old = self.mailbox[sq];
        if old != EMPTY && old as usize != idx_new {

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

        if old == EMPTY { return; }

        let nm       = !BITMASKS[sq];
        self.pieces[old as usize / 6][old as usize % 6] ^= nm;
        self.occupancy[old as usize / 6] ^= nm;
        self.occupancy[2]               ^= nm;

        self.mailbox[sq] = EMPTY;
    }

    #[inline(always)]
    pub fn piece_at_square(&mut self,sq: usize) -> Option<(PieceType,Colour)> { // returns piecetype and colour of square of board
        if sq > 63 {
            panic!("piece_at_square: OOB error, sq given does not exist on the board!")
        }
        let square = self.mailbox[sq];

        if square == EMPTY {return None;}

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

impl Board { // Check/Help functions

    #[inline(always)]
    fn king_square(&self) -> usize {
        let king_bb = self.pieces[self.turn as usize][5];
        king_bb.trailing_zeros() as usize
        //ifu r getting errors here or at is_square_attacked its bc ur using an empty board and there isnt a king so its overflowing at 64 and u didnt wanna add error handling
    }

    #[inline(always)]
    pub fn in_check(&self) -> bool {
        let ks = self.king_square();
        self.is_square_attacked(ks, self.turn.opposite())
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, sq: usize, by: Colour) -> bool {
        let enemy  = by as usize;
        let occ    = &self.occupancy;
        let pieces = &self.pieces;
        let bb_sq: Bitboard = 1u64 << sq;

        // 1) pawn attacks
        if by as usize == 0 {
            // white pawns attack up-left / up-right
            let pawns = pieces[enemy][0];
            let attacks =
                ((pawns << 7) & !FILE_H) |
                ((pawns << 9) & !FILE_A);
            if attacks & bb_sq != 0 {
                return true;
            }
        } else {
            // black pawns attack down-left / down-right
            let pawns = pieces[enemy][0];
            let attacks =
                ((pawns >> 7) & !FILE_A) |
                ((pawns >> 9) & !FILE_H);
            if attacks & bb_sq != 0 {
                return true;
            }
        }

        // 2) knights
        let knights = pieces[enemy][1];
        if KNIGHT_ATTACKS[sq] & knights != 0 {
            return true;
        }

        // 3) bishops / queens (diagonals)
        let bishops_queens =
            pieces[enemy][3] | pieces[enemy][4];
        if bishop_attacks(sq, &occ,self.turn) & bishops_queens != 0 {
            return true;
        }

        // 4) rooks / queens (orthogonal)
        let rooks_queens =
            pieces[enemy][3] | pieces[enemy][4];
        if rook_attacks(sq, &occ,self.turn) & rooks_queens != 0 {
            return true;
        }

        // 5) king
        let enemy_king = pieces[enemy][5];
        if KING_ATTACKS[sq] & enemy_king != 0 {
            return true;
        }

        false
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
                EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,
                EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,
                EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY, 
                EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,
                6, 6, 6, 6, 6, 6, 6, 6,
                9, 7, 8, 10,11, 8, 7, 9,
            ],
            turn: Colour::White,
            castle_rights: 0b0000_1111,

            ep_sq: None,

            halfmove_clock: 0,
        }
    }

    pub fn init_empty() -> Self {
        Board {
            pieces: [[0;6];2],
            occupancy: [0;3],
            mailbox: [EMPTY; 64],  // EMPTY means “no piece”

            turn: Colour::White,
            castle_rights: 0b0000_1111,
            ep_sq: None,

            halfmove_clock: 0,
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
                    EMPTY => '.',
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