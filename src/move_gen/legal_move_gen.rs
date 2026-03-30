use crate::{board::board::{Board, Colour, PieceType}, constants::Bitboard, move_gen::pseudo_gen::pawn_attacks};

impl Board {
    pub fn gen_psuedo_legal_moves(bd: &mut Board) ->  Vec<(u8,u8)> {
        let mut psuedo_legal_moves: Vec<(u8,u8)> = Vec::new();

        for i in 0..64 { //0,0 bottom left
            let move_bitboard: Bitboard;
            let (single_pushes, double_pushes, captures, promotions, ep_attack): (Bitboard,Bitboard,Bitboard,Bitboard,Bitboard);

            if let Some(piece) = bd.piece_at_square(i) {
                if piece.1 == bd.turn {
                   match piece.0 {
                    PieceType::Pawn => (single_pushes, double_pushes, captures, promotions, ep_attack) = pawn_attacks(&bd.pieces, &bd.occupancy, bd.turn, bd.ep_sq),
                    _ => unreachable!()
                } 
                }
            }
        }

        psuedo_legal_moves
    }
}