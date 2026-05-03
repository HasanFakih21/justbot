use macroquad::prelude::*;
use crate::board::{BitBoard, print_board};

pub mod board;

#[macroquad::main("MyGame")]
async fn main() {
    let board_texture = load_texture("assets/board.png").await.unwrap();

    let bb_texture = load_texture("assets/bb.png").await.unwrap();
    let bw_texture = load_texture("assets/bw.png").await.unwrap();
    let kb_texture = load_texture("assets/kb.png").await.unwrap();
    let kw_texture = load_texture("assets/kw.png").await.unwrap();
    let nb_texture = load_texture("assets/nb.png").await.unwrap();
    let nw_texture = load_texture("assets/nw.png").await.unwrap();
    let pb_texture = load_texture("assets/pb.png").await.unwrap();
    let pw_texture = load_texture("assets/pw.png").await.unwrap();
    let qb_texture = load_texture("assets/qb.png").await.unwrap();
    let qw_texture = load_texture("assets/qw.png").await.unwrap();
    let rb_texture = load_texture("assets/rb.png").await.unwrap();
    let rw_texture = load_texture("assets/rw.png").await.unwrap();
    build_textures_atlas();

    let mut bit_board = BitBoard::new();
    bit_board.set_bit(board::Piece::WhitePawns, board::Square::A5);
    bit_board.clear_bit(board::Piece::WhitePawns, board::Square::A2);
    bit_board.clear_bit(board::Piece::WhitePawns, board::Square::A2);

    print_board(&bit_board.black_bishops);

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

        draw_board(&bit_board.white_bishops, &bw_texture);
        draw_board(&bit_board.black_bishops, &bb_texture);
        draw_board(&bit_board.white_king, &kw_texture);
        draw_board(&bit_board.black_king, &kb_texture);
        draw_board(&bit_board.white_queens, &qw_texture);
        draw_board(&bit_board.black_queens, &qb_texture);
        draw_board(&bit_board.white_pawns, &pw_texture);
        draw_board(&bit_board.black_pawns, &pb_texture);
        draw_board(&bit_board.white_knights, &nw_texture);
        draw_board(&bit_board.black_knights, &nb_texture);
        draw_board(&bit_board.white_rooks, &rw_texture);
        draw_board(&bit_board.black_rooks, &rb_texture);

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