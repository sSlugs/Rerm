use macroquad::prelude::*;

use crate::{board::{board::{Board, PieceType}, moves::{Move, Undo}}, move_gen::pseudo_gen::pawn_attacks};

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


#[macroquad::main("Sprite")]
pub async fn main() {
    let textures = load_texture("assets/ChessPiecesArray.png").await.unwrap();
    let mut selected_square: Option<usize> = Option::None;
    let mut bd = Board::init_new();
    let mut move_list: Vec<Undo> = Vec::new();

    loop {
        clear_background(BLACK);
        draw_chess_grid();
        let (x, y) = mouse_position();
        let col = (x/SQ_LENGTH).floor();
        let row = 7.0 - (y/SQ_LENGTH).floor();
        //let (sq_y,sq_x) = row_col_to_x_y(&row, &col);

        draw_pieces(&mut bd,&textures);

    
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(sq) = selected_square { 
                if let Some(_) = bd.piece_at_square(sq) {
                    if sq != (((col + 1.0) + row*8.0 )-1.0) as usize {
                        let undo = bd.make_move(Move{
                        from: sq as u8,
                        to: (((col + 1.0) + row*8.0 )-1.0) as u8,
                        promotion_piece: None,
                        flags: 0,
                        });
                        move_list.push(undo);
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


        let n = pawn_attacks(&bd.pieces,&bd.occupancy,bd.turn,bd.ep_sq);

        let bin = format!("{:064b}", n.0);

        // rank 8 printed first, rank 1 printed last
        for chunk in bin.as_bytes().chunks(8) {
            // flip bits in the rank so A-file is on the left
            let line: String = chunk.iter().rev().map(|&c| c as char).collect();
            println!("{}", line);
        }
        println!("------------------------------");
        
        next_frame().await;
    }
}