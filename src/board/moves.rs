use crate::board::board::PieceType;
use crate::board::board::*;

#[inline(always)]
fn abs_diff(a: u8, b: u8) -> u8 {
    a.max(b) - a.min(b)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion_piece: Option<PieceType>, 
    pub flags: u8, // per-bit. 0st:enpassant , 
}

#[derive(Copy, Clone, Debug)]
pub struct Undo {
    pub captured_piece: Option<PieceType>,
    pub captured_square: Option<u8>,

    pub castled: u8, // similiar to castling rights in board. first 4 bits, determine if the move was a castle respectively. wqc wkc bqc bkc : 0001 0010 0100 1000

    pub old_castle_rights: u8,
    pub old_ep_square: Option<u64>, //
    pub old_halfmove_clock: u8,

    pub moved_piece: PieceType, 

    pub from: u8, //Exact same as from in move object,just the starting square
    pub to: u8,
    pub promotion_piece: Option<PieceType>,
}

impl Board {
    #[inline(always)]
    pub fn make_move(&mut self, mv: Move) -> Undo {
        let from = mv.from as u8;
        let to   = mv.to   as u8;

        let mut castled = 0;

        let old_castle_rights   = self.castle_rights;
        let old_ep_square       = self.ep_sq; 
        let old_halfmove_clock  = self.halfmove_clock;

        let mut captured_piece: Option<PieceType> = None;
        let mut captured_square: Option<u8>       = None;

        self.ep_sq = None;

        let (moving_pt, side) = self.piece_at_square(from as usize).expect("no piece on from");

        #[inline(always)]
        fn clear_rights_on_rook_move(rights: &mut u8, from: u8) { // clears castling rights when given the starting square of the rook, ex. (63/h8) cleres black kingside casting reights
            match from {
                0  => *rights &= 0b0000_1110, // a1: clear WQ
                7  => *rights &= 0b0000_1101, // h1: clear WK
                56 => *rights &= 0b0000_1011, // a8: clear BQ
                63 => *rights &= 0b0000_0111, // h8: clear BK
                _  => {}
            }
        }

        #[inline(always)]
        fn clear_rights_on_rook_capture(rights: &mut u8, sq: u8) { // same as above just if your rook is captured
            match sq {
                0  => *rights &= 0b0000_1110, // a1 rook gone -> no WQ
                7  => *rights &= 0b0000_1101, // h1 rook gone -> no WK
                56 => *rights &= 0b0000_1011, // a8 rook gone -> no BQ
                63 => *rights &= 0b0000_0111, // h8 rook gone -> no BK
                _  => {}
            }
        }

        if mv.flags & 1 == 1 {

            let cap_sq = match self.turn {
                Colour::White => to.wrapping_sub(8),
                Colour::Black => to.wrapping_add(8),
            };

            self.clear_square(cap_sq as usize);
            captured_piece  = Some(PieceType::Pawn);
            captured_square = Some(cap_sq);


            self.clear_square(from as usize);
            self.set_square(to as usize, side, moving_pt);

        } else {

            if let Some(cap) = self.piece_at_square(to as usize) {
                captured_piece  = Some(cap.0);
                captured_square = Some(to);
                self.clear_square(to as usize);

                if cap.0 == PieceType::Rook {
                    clear_rights_on_rook_capture(&mut self.castle_rights, to);
                }
            }

            match (moving_pt, from, to) { // if the move is castling then move rook
                (PieceType::King, 4, 6) => {  // White O-O: h1 -> f1
                    self.clear_square(7);
                    self.set_square(5, side, PieceType::Rook);
                    self.castle_rights &= 0b0000_1100; // clear WK|WQ
                    castled = 0b0000_0010
                }
                (PieceType::King, 4, 2) => {  // White O-O-O: a1 -> d1
                    self.clear_square(0);
                    self.set_square(3, side, PieceType::Rook);
                    self.castle_rights &= 0b0000_1100;
                    castled = 0b0000_0001
                }
                (PieceType::King, 60, 62) => { // Black O-O: h8 -> f8
                    self.clear_square(63);
                    self.set_square(61, side, PieceType::Rook);
                    self.castle_rights &= 0b0000_0011; // clear BK|BQ
                    castled = 0b0000_1000
                }
                (PieceType::King, 60, 58) => { // Black O-O-O: a8 -> d8
                    self.clear_square(56);
                    self.set_square(59, side, PieceType::Rook);
                    self.castle_rights &= 0b0000_0011;
                    castled = 0b0000_0100
                }
                _ => {}
            }

            if let Some(promo) = mv.promotion_piece {

                self.clear_square(from as usize);
                self.set_square(to as usize, side, promo);
            } else {

                self.clear_square(from as usize);
                self.set_square(to as usize, side, moving_pt);
            }

            if moving_pt == PieceType::King {
                if from == 4  { self.castle_rights &= 0b0000_1100; } 
                if from == 60 { self.castle_rights &= 0b0000_0011; } 
            }

            if moving_pt == PieceType::Rook {
                clear_rights_on_rook_move(&mut self.castle_rights, from);
            }
        }


        if moving_pt == PieceType::Pawn && (from as i16 - to as i16).abs() == 16 {
            let mid = ((from as i16 + to as i16) / 2) as u8;
            self.ep_sq = Some(1u64 << mid);
        } else {
            self.ep_sq = None;
        }

        self.halfmove_clock = if moving_pt == PieceType::Pawn || captured_piece.is_some() {
            0
        } else {
            self.halfmove_clock.saturating_add(1)
        };

        self.turn = !self.turn;

        Undo {
            captured_piece,
            captured_square,
            old_castle_rights,
            old_ep_square,
            castled,
            old_halfmove_clock,
            moved_piece: moving_pt,

            from: mv.from,
            to: mv.to,
            promotion_piece: mv.promotion_piece,
        }
    }

    pub fn unmake_move(&mut self, u: &Undo) {
        // 1) restore meta first
        self.turn            = !self.turn;            // or: self.turn = !self.turn;
        self.castle_rights   = u.old_castle_rights;
        self.ep_sq           = u.old_ep_square;
        self.halfmove_clock  = u.old_halfmove_clock;

        let from = u.from as usize;
        let to   = u.to   as usize;
        let side = self.turn; // after restoring turn, this is the mover’s colour

        // 2) special cases in reverse order of make()

        // a) If promotion, turn promoted piece at 'to' back into pawn first
        if u.promotion_piece.is_some() {
            // at 'to' there’s the promoted piece; replace with pawn of 'side'
            self.clear_square(to);
            self.set_square(to, side, PieceType::Pawn);
        }

        // b) If castling, move rook back
        if u.castled != 0 {
            match (u.moved_piece, u.from, u.to) {
                (PieceType::King, 4, 6)   => { self.clear_square(5);  self.set_square(7,  side, PieceType::Rook); }
                (PieceType::King, 4, 2)   => { self.clear_square(3);  self.set_square(0,  side, PieceType::Rook); }
                (PieceType::King, 60, 62) => { self.clear_square(61); self.set_square(63, side, PieceType::Rook); }
                (PieceType::King, 60, 58) => { self.clear_square(59); self.set_square(56, side, PieceType::Rook); }
                _ => {}
            }
        }

        // 3) move the moved piece back: 'to' -> 'from'
        //    (if promo, we already changed the piece at 'to' into a pawn above)
        self.clear_square(to);
        self.set_square(from, side, u.moved_piece);

        // 4) restore captured piece (normal or EP) at the recorded square
        if let (Some(cpt), Some(cs)) = (u.captured_piece, u.captured_square) {
            // captured piece belongs to the opposite side
            self.set_square(cs as usize, !side, cpt);
        }       

    }
}
