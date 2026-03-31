use macroquad::prelude::*;
use std::io::{self, Write};

use crate::{board::{board::{Board, Colour, PieceType, inverse_index_change}, moves::{Move, Undo}}, move_gen::pseudo_gen::pawn_attacks};

const SQ_LENGTH: f32 = 100.0;

const CALM_GREEN: Color = Color {
    r: 0.4118,
    g: 0.5725,
    b: 0.2431,
    a: 1.0,
};

fn draw_chess_grid() {
    let mut colour: Color;

    for i in 0..64 {
        let sq = i as f32;
        let col = sq % 8.0;
        let row = (7.0 - (sq / 8.0).floor());

        if (i + row as i32) % 2 == 0 {
            colour = WHITE;
        } else {
            colour = CALM_GREEN;
        }
        draw_rectangle((col*SQ_LENGTH), (row*SQ_LENGTH), SQ_LENGTH, SQ_LENGTH, colour);
    }
}

fn row_col_to_x_y(row: &f32,col:&f32) -> (f32,f32) {
    return (((row * SQ_LENGTH) + SQ_LENGTH/2.0),((col * SQ_LENGTH) + SQ_LENGTH/2.0));
}

fn draw_pieces(bd: &mut Board,textures: &Texture2D) {
    for i in 0..64 {
        let (sq_y,sq_x) = row_col_to_x_y(&(7.0 - ((i as f32) /8.0).floor()), &((i as f32) % 8.0));
        if let Some(piece) = bd.piece_at_square(i)  {

            let tile_size = 60.0;
            let x_offset: f32;

            match piece.0 {
                PieceType::Queen => x_offset = 0.0,
                PieceType::King => x_offset = 1.0,
                PieceType::Rook => x_offset = 2.0,
                PieceType::Knight => x_offset = 3.0,
                PieceType::Bishop => x_offset = 4.0,
                PieceType::Pawn => x_offset = 5.0,
            }

            let y_offset = (!piece.1 as i32) as f32;
            let source = Rect::new(x_offset * tile_size, y_offset * tile_size, tile_size,tile_size);

            draw_texture_ex(
                textures,
                sq_x - 52.5,
                sq_y - 50.0,
                WHITE,
                DrawTextureParams {
                    source: Some(source),
                    dest_size: Some(vec2(100.0, 100.0)), // scaled up
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_pseudo_legal_moves(bd: &mut Board,sq:u8) {
    let mut moves: Vec<Move>= Vec::with_capacity(1024);
    let mut pseudo_legal_sqs: Vec<u8> = Vec::new();
    let row = (sq % 8) as f32;
    let col = (7 - (sq / 8)) as f32;
    let (sq_x,sq_y) = row_col_to_x_y(&row, &col);
    draw_circle(sq_x, sq_y, 25.0, RED);
    bd.gen_psuedo_legal_moves(&mut moves);

    for mv in moves {
        if mv.from == sq {
            pseudo_legal_sqs.push(mv.to);
        }
    }

    if pseudo_legal_sqs.is_empty() {
        return;
    }

    for msq in pseudo_legal_sqs {
        let row = (msq % 8) as f32;
        let col = (7 - (msq / 8)) as f32;
        let (sq_x,sq_y) = row_col_to_x_y(&row, &col);
        draw_circle(sq_x, sq_y, 25.0, BLACK);
    }
}

#[macroquad::main("Sprite")]
pub async fn main() {
    let textures = load_texture("assets/ChessPiecesArray.png").await.unwrap();
    let mut selected_square: Option<usize> = Option::None;
    let mut bd = Board::init_new();
    let mut move_list: Vec<Undo> = Vec::new();
    //bd.set_square(54,Colour::White, PieceType::Pawn);

    loop {
        clear_background(BLACK);
        draw_chess_grid();
        let (x, y) = mouse_position();
        let col = (x/SQ_LENGTH).floor();
        let row = 7.0 - (y/SQ_LENGTH).floor();
        let mut ps_moves: Vec<Move> = Vec::with_capacity(1024);
        bd.gen_psuedo_legal_moves(&mut ps_moves);

        draw_pieces(&mut bd,&textures);
        if let Some(ssq) = selected_square {
            draw_pseudo_legal_moves(&mut bd,ssq as u8);
        }
    
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(sq) = selected_square { 
                if let Some(piece) = bd.piece_at_square(sq) {
                    let sq_to = col + row*8.0;
                    if sq != sq_to as usize {
                        let mut promo_piece = None;
                        let mut flags = 0;

                        if piece.0 == PieceType::Pawn {
                            println!("-{}-",sq_to);
                            if (sq_to/8.0).floor() == 7.0 {
                                print!("\nPlease Input Promotion Piece (Q,R,B,N | Default Queen if none): ");
                                let _ = io::stdout().flush();
                                let mut input = String::new();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read line");

                                match input.to_ascii_lowercase().trim() {
                                    "q" => promo_piece = Some(PieceType::Queen),
                                    "r" => promo_piece = Some(PieceType::Rook),
                                    "b" => promo_piece = Some(PieceType::Bishop),
                                    "n" => promo_piece = Some(PieceType::Knight),
                                    _ => promo_piece = Some(PieceType::Queen),
                                }
                            } else if let Some(e_piece) = bd.piece_at_square(inverse_index_change(bd.turn, sq_to as usize, 8)) {
                                if e_piece.0 == PieceType::Pawn && e_piece.1 == !bd.turn {
                                    if bd.piece_at_square(sq_to as usize) == None {
                                        flags = 1;
                                    }
                                }
                            }
                        }
                        let mv = Move{
                        from: sq as u8,
                        to: sq_to as u8,
                        promotion_piece: promo_piece,
                        flags,
                        };

                        if ps_moves.contains(&mv) {
                            let undo = bd.make_move(mv);
                            move_list.push(undo);
                        }
                    }
                }
                selected_square = None
            } else if selected_square == Some((((col + 1.0) + row*8.0 )-1.0) as usize) {
                selected_square = None
            } else {
                selected_square = Some((((col + 1.0) + row*8.0 )-1.0) as usize)
            }

        }
        

        if is_key_pressed(KeyCode::R) {
            if let Some(last) = move_list.last() {
                bd.unmake_move(&last);
                move_list.pop();
            }
        }


        
        
        next_frame().await;
    }
}