use macroquad::prelude::*;
use crate::board::{BitBoard, Side, Piece, print_board};

pub mod board;

#[macroquad::main("MyGame")]
async fn main() {
    let board_texture = load_texture("assets/board.png").await.unwrap();
    let piece_textures = generate_piece_texture_arrays().await;
    let mut bit_board = BitBoard::new();

    bit_board.set_bit(Side::Black, Piece::Pawns, board::Square::D5);
    bit_board.clear_bit(Side::Black, Piece::Pawns, board::Square::D7);
    print_board(&bit_board.bit_board_pieces[Side::White as usize][Piece::Pawns as usize]);

    loop {
        request_new_screen_size(768.0, 768.0);

        draw_texture_ex(
            &board_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );

        piece_textures
            .iter()
            .enumerate()
            .for_each(|(side_index, pieces)| {
                pieces
                    .iter()
                    .enumerate()
                    .for_each(|(piece_index, texture)| {
                        draw_board(&bit_board.bit_board_pieces[side_index][piece_index], texture);
                    });
            });

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let (file, rank) = ((mouse_pos.0 / 96.0).floor() as usize, 7 - (mouse_pos.1 / 96.0).floor() as usize);
            let square_index = rank * 8 + file;

            let piece_present = bit_board.get_piece_at_square(Side::White, board::Square::try_from(square_index).unwrap());
            println!("Clicked on square: {:?}, Piece present: {:?}", board::Square::try_from(square_index).unwrap(), piece_present);
        }

        next_frame().await
    }
}


fn draw_piece(x: f32, y: f32, texture: &Texture2D) {
    texture.set_filter(FilterMode::Linear);
    let params = DrawTextureParams {
        dest_size: Some(vec2(96.0, 96.0)),
        ..Default::default()
    };

    draw_texture_ex(texture, x, y, WHITE, params);
}


fn draw_board(bit_board: &u64, texture: &Texture2D) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let board_index = (rank * 8) + file; 
            let bit_state = bit_board & (1u64 << board_index);

            if bit_state != 0 {
                draw_piece(file as f32 * 96.0, (7 - rank) as f32 * 96.0, texture);
            }
        }
    }
}

async fn generate_piece_texture_arrays() -> [[Texture2D; 6]; 2] {
    let pw_texture = load_texture("assets/pw.png").await.unwrap();
    let nw_texture = load_texture("assets/nw.png").await.unwrap();
    let bw_texture = load_texture("assets/bw.png").await.unwrap();
    let rw_texture = load_texture("assets/rw.png").await.unwrap();
    let qw_texture = load_texture("assets/qw.png").await.unwrap();
    let kw_texture = load_texture("assets/kw.png").await.unwrap();

    let pb_texture = load_texture("assets/pb.png").await.unwrap();
    let nb_texture = load_texture("assets/nb.png").await.unwrap();
    let bb_texture = load_texture("assets/bb.png").await.unwrap();
    let rb_texture = load_texture("assets/rb.png").await.unwrap();
    let qb_texture = load_texture("assets/qb.png").await.unwrap();
    let kb_texture = load_texture("assets/kb.png").await.unwrap();
    
    build_textures_atlas();

    [
        [pw_texture, nw_texture, bw_texture, rw_texture, qw_texture, kw_texture],
        [pb_texture, nb_texture, bb_texture, rb_texture, qb_texture, kb_texture]
    ]
}