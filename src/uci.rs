use std::time::Instant;

use crate::{board::{Board, Piece, Square, constants::STARTING_FEN, moves::Move}, perft::perft, search::best_move};

impl Board {
    pub fn parse_move(&self, move_string: &str) -> Result<Move, &str> {
        let from = Square::try_from(&move_string[0..2]).unwrap();
        let to = Square::try_from(&move_string[2..4]).unwrap();
        let mut promotion_piece: Option<Piece> = None;
        if move_string.len() > 4 {
            match &move_string[4..] {
                "n" => promotion_piece = Some(Piece::Knight),
                "b" => promotion_piece = Some(Piece::Bishop),
                "r" => promotion_piece = Some(Piece::Rook),
                "q" => promotion_piece = Some(Piece::Queen),
                _ => ()
            }
        }

        let move_list = self.generate_all_moves();
        if let Some(m) = move_list.iter().find(|e| e.get_from() == from && e.get_to() == to && e.get_promoted_piece() == promotion_piece) {
            Ok(*m)
        } else {
            Err("Invalid move string")
        }
    }
}

pub fn input_loop() {
    let mut board = Board::new();

    let mut input_buffer = String::new();
    loop {
        if std::io::stdin().read_line(&mut input_buffer).unwrap() == 0 {
            break;   
        }

        let (command, args) = input_buffer.split_once(" ").unwrap_or((&input_buffer, ""));

        match command.trim() {
            "position" => position(args, &mut board),
            "uci" => uci(),
            "isready" => println!("readyok"),
            "ucinewgame" => board = Board::from_fen(STARTING_FEN),
            "go" => go(args, &mut board),
            "quit" => break,
            _=> eprintln!("Not a valid command"),
        }
        
        input_buffer.clear();
    }
}

pub fn position(args: &str, board: &mut Board) {
    if args.trim().is_empty() {
        eprintln!("Need to provide a valid argument!");
        return
    }

    let (command, args) = args.split_once(" ").unwrap_or((args, ""));
    let (args, moves) = args.split_once("moves").unwrap_or((args, ""));

    match command.trim() {
        "startpos" => {
            *board = Board::from_fen(STARTING_FEN);
        },
        "fen" => {
            if args.trim().is_empty() {
                eprintln!("Please provide a fen string");
                return;
            }
            *board = Board::from_fen(args);
        },
        _ => eprintln!("Not a valid position argument!")
    }   

    if !moves.trim().is_empty() {
        for m_str in moves.split_ascii_whitespace() {
            let result = board.parse_move(m_str);
            if let Ok(m) = result && board.make_move(m).is_err() {
                eprintln!("Illegal Move! {m}");
                return;
            } 
        }
    }

    println!("{board}");
}

pub fn go(args: &str, board: &mut Board) {
    if args.trim().is_empty() {
        eprintln!("Need to provide a valid argument!");
        return
    }

    let (command, args) = args.split_once(" ").unwrap_or((args, ""));
    
    match command.trim() {
        "depth" => {
            let best_move = best_move(args.trim().parse::<usize>().unwrap(), board);
            if let Some((m, i)) = best_move {
                println!("info score cp {i}");
                println!("bestmove {m}");
            }
        },
        "perft" => {
            println!("{args}");
            if let Ok(depth) = args.trim().parse::<usize>() {
                let clock = Instant::now();
                let nodes_count = perft(depth, board);
                println!("Number of nodes: {nodes_count}\nTime: {}ms", clock.elapsed().as_millis()); 
            } else {eprintln!("Enter a valid depth!")}
        },
        _=> {
            //eprintln!("Not a valid go argument!")
            let best_move = best_move(6, board);
            if let Some((m, i)) = best_move {
                println!("info score cp {i}");
                println!("bestmove {m}");
            }
        }
    }
}

pub fn uci() {
    println!("id name JustBot 1.0");
    println!("id author Hasan Fakih");
    println!("uciok");
}

#[cfg(test)]
pub mod tests {
    use crate::board::constants::STARTING_FEN;
    use super::*;

    #[test]
    fn test_parse_move() {
        let board = Board::from_fen(STARTING_FEN);
        if let Ok(m) = board.parse_move("e2e4") {
            println!("bestmove {m}");
        }
    }
}