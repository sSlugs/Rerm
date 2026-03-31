use crate::{board::{board::{Board, Colour, PieceType, inverse_index_change}, moves::Move}, constants::Bitboard, move_gen::pseudo_gen::{bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks}};

impl Board {
    pub fn gen_psuedo_legal_moves(&mut self,pseudo_legal_moves: &mut Vec<Move>) {
        pseudo_legal_moves.clear();
        let mut occ_bb = self.occupancy[self.turn as usize] & !self.pieces[self.turn as usize][0]; // occupancy of all piece EXCEPT pawns
        let mut piece_bb: u64;

        while occ_bb != 0 { // iteratre over the set bits, ignore unset bits (non pawn moves vec pushes)
            let sq = occ_bb.trailing_zeros() as usize; //the lsb, which is the sq of the first piece on the occ.
            match self.piece_at_square(sq).unwrap().0 {
                PieceType::Rook => piece_bb = rook_attacks(sq, &self.occupancy, self.turn),
                PieceType::Knight => piece_bb = knight_attacks(sq, &self.occupancy, self.turn),
                PieceType::Bishop => piece_bb = bishop_attacks(sq, &self.occupancy, self.turn),
                PieceType::Queen => piece_bb = queen_attacks(sq, &self.occupancy, self.turn),
                PieceType::King => piece_bb = king_attacks(sq, &self.occupancy, self.turn),
                _ => unreachable!()
            }

            if piece_bb == 0 {
               occ_bb &= occ_bb - 1;
               continue 
            }

            while piece_bb != 0 {
                let msq = piece_bb.trailing_zeros() as usize;
                pseudo_legal_moves.push(Move { from: sq as u8, to: msq as u8, promotion_piece: None, flags: 0 });
                piece_bb &= piece_bb - 1
            }

            occ_bb &= occ_bb - 1
        }

        // creating pawn moves and pushing into vector logic
        if self.pieces[self.turn as usize][0] != 0 { // if there is atleast 1 pawn on the board
            let pawn_bbs= pawn_attacks(&self.pieces, &self.occupancy, self.turn, self.ep_sq); // single pushes, double pushes, captures,promos, ep attacks, all pawns (of colour)
            for i in 0..5 { // iterate over all the bitboards in pawn_bbs
                let mut pawn_bb = pawn_bbs[i];
                if pawn_bb == 0 { // if pawn bb is empty just go to next
                    continue;
                }
                while pawn_bb != 0 {
                    let sq = pawn_bb.trailing_zeros() as usize; // iterate through all the possible moves then push it to vec depending on the type of bb it is

                    match i {
                        0 => { // single pushes
                            pseudo_legal_moves.push(Move { from: (inverse_index_change(self.turn, sq, 8)) as u8, to: sq as u8, promotion_piece: None, flags: 0 }); // move obj is u8, the index of square not physical square itself
                        },
                        1 => { // double pushes
                            pseudo_legal_moves.push(Move { from: (inverse_index_change(self.turn, sq, 16)) as u8, to: sq as u8, promotion_piece: None, flags: 0 });
                        },
                        2 => { // captures
                            let col = sq % 8;
                            let down_l = inverse_index_change(self.turn, sq, 9);
                            let down_r = inverse_index_change(self.turn, sq, 7);
                            if col == 0 { // a file
                                pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: None, flags: 0 });
                            } else if col == 8 { //h file
                                pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: None, flags: 0 });
                            } else {
                                if (1 << down_l) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: None, flags: 0 });
                                }
                                if (1 << down_r) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: None, flags: 0 });
                                }
                            }
                        },
                        3 => { // promotions
                            let col = sq % 8;
                            let down_l = inverse_index_change(self.turn, sq, 9);
                            let down_r = inverse_index_change(self.turn, sq, 7);
                            let down_1 = inverse_index_change(self.turn, sq, 8);
                            if (1 << down_1) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_1 as u8, to: sq as u8, promotion_piece: Some(PieceType::Queen), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_1 as u8, to: sq as u8, promotion_piece: Some(PieceType::Rook), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_1 as u8, to: sq as u8, promotion_piece: Some(PieceType::Bishop), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_1 as u8, to: sq as u8, promotion_piece: Some(PieceType::Knight), flags: 0 });
                                }
                            if col == 0 { // A file
                                if (1 << down_r) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Queen), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Rook), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Bishop), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Knight), flags: 0 });
                                }
                            } else if col == 8 { // H file
                                if (1 << down_l) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Queen), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Rook), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Bishop), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Knight), flags: 0 });
                                }
                            } else {
                                if (1 << down_l) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Queen), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Rook), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Bishop), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: Some(PieceType::Knight), flags: 0 });
                                }
                                if (1 << down_r) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Queen), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Rook), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Bishop), flags: 0 });
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: Some(PieceType::Knight), flags: 0 });
                                }
                            }
                        },
                        4 => { // en passants
                            let col = sq % 8;
                            let down_l = inverse_index_change(self.turn, sq, 9);
                            let down_r = inverse_index_change(self.turn, sq, 7);
                            if col == 0 { // a file
                                if self.turn == Colour::White {
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: None, flags: 1 });
                                } else {
                                    pseudo_legal_moves.push(Move { from: 25 as u8, to: sq as u8, promotion_piece: None, flags: 1 });
                                }
                            } else if col == 8 { //h file
                                pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: None, flags: 1 });
                            } else {
                                if (1 << down_l) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_l as u8, to: sq as u8, promotion_piece: None, flags: 1 });
                                }
                                if (1 << down_r) & pawn_bbs[5] != 0 {
                                    pseudo_legal_moves.push(Move { from: down_r as u8, to: sq as u8, promotion_piece: None, flags: 1 });
                                }
                            }
                        },
                        _ => unreachable!()
                    }

                    pawn_bb &= pawn_bb - 1;
                }
            }
        }

        // all logic for pseudo castling (just checks if u have castling rights, and theres no pieces in way)
        let mut cr = self.castle_rights;
        while cr != 0 {
            
        }

    }
}