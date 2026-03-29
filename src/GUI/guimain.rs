use macroquad::prelude::*;

const SQ_LENGTH: f32 = 100.0;

const CALM_GREEN: Color = Color {
    r: 0.4118,
    g: 0.5725,
    b: 0.2431,
    a: 1.0,
};

fn draw_chess_grid() {
    let padding = 0.0;
    let mut colour: Color;

    for i in 0..64 {
        let sq = i as f32;
        let col = sq % 8.0;
        let row = (sq / 8.0).floor();

        if (i + row as i32) % 2 == 0 {
            colour = WHITE;
        } else {
            colour = CALM_GREEN;
        }
        draw_rectangle((col*SQ_LENGTH) + padding, (row*SQ_LENGTH) + padding, SQ_LENGTH, SQ_LENGTH, colour);
    }
}

#[macroquad::main("Demo")]
pub async fn main() {
    loop {
        clear_background(BLACK);
        draw_chess_grid();
        let (x, y) = mouse_position();
        
        next_frame().await;
    }
}