use macroquad::prelude::*;

use crate::board::{Piece, Side};

pub fn draw_piece(x: f32, y: f32, texture: &Texture2D) {
    texture.set_filter(FilterMode::Linear);
    let params = DrawTextureParams {
        dest_size: Some(vec2(96.0, 96.0)),
        ..Default::default()
    };

    draw_texture_ex(texture, x, y, WHITE, params);
}

pub fn draw_board(mailbox: [Option<(Side, Piece)>; 64], texture_array: &[[Texture2D; 6]; 2]) {
    for (i, e) in mailbox.iter().enumerate() {
        if let Some((side, piece)) = e {
            let rank = i / 8;
            let file = i % 8;
            draw_piece(file as f32 * 96.0, (7 - rank) as f32 * 96.0, &texture_array[*side as usize][*piece as usize]);
        }
    }
}

pub async fn generate_piece_texture_arrays() -> [[Texture2D; 6]; 2] {
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