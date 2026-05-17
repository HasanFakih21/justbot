use crate::board::{Board, Castling, Piece, Side, Square};

//Starting Position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//[pieces] [turn to move] [castling rights] [enpassant] [halfmove clock] [fullmove clock]

//FEN String Parser
impl Board {
    pub fn from_fen(fen_string: &str) -> Self {
        let mut board = Board::new();
        let mut fen = fen_string.split(" ");

        let piece_string = fen.next().unwrap();

        let ranks = piece_string.split('/');
        for (rank, r_str) in ranks.rev().enumerate() {
            let mut file: usize = 0;
            for p in r_str.chars() {
                if let Some(num) = p.to_digit(10) {
                    file += num as usize;
                    continue;
                }

                let side = if p.is_ascii_uppercase() {Side::White} else {Side::Black};
                board.place_piece(side, Piece::from_char(p).unwrap(), Square::from_rank_and_file(rank, file));

                file += 1;
            }
        }

        let turn = fen.next().unwrap();
        match turn {
            "w" => board.board_state.side_to_move = Side::White,
            "b" => board.board_state.side_to_move = Side::Black,
            _ => eprintln!("Invalid side to move")
        }

        let castling_rights = fen.next().unwrap();
        for c in castling_rights.chars() {
            if c == '-' {
                continue;
            }

            board.board_state.castling_rights.set(Castling::from(c) as u8);
        }

        let enpassant = fen.next().unwrap();
        if let Ok(square) = Square::try_from(enpassant) {
            board.board_state.enpassant = Some(square);
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use crate::board::constants::STARTING_FEN;
    use super::*;

    #[test]
    fn test_from_fen() {
        let board = Board::from_fen(STARTING_FEN);
        println!("{board}");

        let board2 = Board::from_fen("8/8/8/8/8/8/8/8 w - - ");
        println!("{board2}");

        let board3 = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        println!("{board3}");

        let board4 = Board::from_fen("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
        println!("{board4}");

        let board5 = Board::from_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9");
        println!("{board5}");
        
        let board5 = Board::from_fen("rnbqkbnr/pp3ppp/4p3/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq c6 0 4");
        println!("{board5}");

        let board6 = Board::from_fen("rnb1kbnr/pp1q1pp1/4p2p/2p1N3/3Pp3/2P5/PP2BPPP/RNBQK1R1 b Qkq - 1 7");
        println!("{board6}");
    }
}