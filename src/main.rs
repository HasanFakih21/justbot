use macroquad::prelude::*;
use crate::board::moves::Move;
use crate::board::{Board, Piece, Side, Square, constants::STARTING_FEN};
use crate::draw::{draw_board, draw_piece, generate_piece_texture_arrays};

pub mod board;
pub mod attacks;
pub mod occupancy;
pub mod magics;
pub mod perft;

pub mod draw;

#[macroquad::main("MyGame")]
async fn main() {
    //Setting up textures
    let board_texture = load_texture("assets/board.png").await.unwrap();
    let piece_textures = generate_piece_texture_arrays().await;

    //Initialilze Board class
    let mut board = Board::from_fen(STARTING_FEN);
    let mut move_list = board.generate_all_moves();
    let mut selected_piece: Option<(Side, Piece, Square)> = None;
    let mut selected_piece_moves: Vec<Move> = Vec::new();
    let mut mailbox = board.pieces_on_squares;

    loop {
        request_new_screen_size(768.0, 768.0);
        let mouse_pos = mouse_position();
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

        draw_board(mailbox, &piece_textures);  

        if is_mouse_button_pressed(MouseButton::Left) && selected_piece.is_none() {
            let square = get_square_from_mouse_position(mouse_pos);
            let piece_present = board.get_piece_at_square(square);
            selected_piece_moves = move_list.into_iter().filter(|e| e.get_from() == square).collect();

            if let Some(piece) = piece_present && piece.0 == board.side_to_move {
                    selected_piece = Some((board.side_to_move, piece.1, square));
                    mailbox[square as usize] = None;
                }
        }

        if is_mouse_button_down(MouseButton::Left) && let Some((side, piece, _)) = selected_piece {
            draw_piece(mouse_pos.0 - 48.0, mouse_pos.1 - 48.0, &piece_textures[side as usize][piece as usize]);
        } else if selected_piece.is_some() {
            let new_square = get_square_from_mouse_position(mouse_pos);
            let selected_move = selected_piece_moves.iter().find(|e| e.get_to() == new_square);
            if let Some(m) = selected_move {
                let _ = board.make_move(*m);
                move_list = board.generate_all_moves();
            }
            selected_piece = None;
            selected_piece_moves.clear();
            mailbox = board.pieces_on_squares;
        }

        next_frame().await
    }
}

fn get_square_from_mouse_position(mouse_pos: (f32, f32)) -> Square {
    let file = (mouse_pos.0 / 96.0).floor() as usize;
    let rank = 7 - (mouse_pos.1 / 96.0).floor() as usize;
    Square::from_rank_and_file(rank, file)
}