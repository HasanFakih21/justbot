use std::time::Instant;

use crate::board::Board;
use crate::board::movegen::MoveGenKind;
use crate::search::data::{SearchData, SearchKind};
use crate::search::{search, search_runner};
use crate::types::*;

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
                _ => (),
            }
        }

        let move_list = self.generate_moves(MoveGenKind::All);
        if let Some(m) = move_list.iter().find(|e| {
            e.get_from() == from && e.get_to() == to && e.get_promoted_piece() == promotion_piece
        }) {
            Ok(*m)
        } else {
            Err("Invalid move string")
        }
    }
}

pub fn input_loop() {
    let mut board = Board::new();
    let mut data = SearchData::default();
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
            "ucinewgame" => {
                board = Board::from_fen(STARTING_FEN);
                data = SearchData::default();
            }
            "go" => go(args, &mut board, &mut data),
            "quit" => break,
            "d" => println!("{board}"),
            _ => eprintln!("Not a valid command"),
        }

        input_buffer.clear();
    }
}

pub fn position(args: &str, board: &mut Board) {
    if args.trim().is_empty() {
        eprintln!("Need to provide a valid argument!");
        return;
    }

    let (command, args) = args.split_once(" ").unwrap_or((args, ""));
    let (args, moves) = args.split_once("moves").unwrap_or((args, ""));

    match command.trim() {
        "startpos" => {
            *board = Board::from_fen(STARTING_FEN);
        }
        "fen" => {
            if args.trim().is_empty() {
                eprintln!("Please provide a fen string");
                return;
            }
            *board = Board::from_fen(args);
        }
        _ => eprintln!("Not a valid position argument!"),
    }

    if !moves.trim().is_empty() {
        for m_str in moves.split_ascii_whitespace() {
            let result = board.parse_move(m_str);
            if let Ok(m) = result
                && board.make_move(m).is_err()
            {
                eprintln!("Illegal Move! {m}");
                return;
            }
        }
    }
    //println!("{board}");
}

pub fn go(args: &str, board: &mut Board, data: &mut SearchData) {
    if args.trim().is_empty() {
        eprintln!("Need to provide a valid argument!");
        return;
    }

    let (command, args) = args.split_once(" ").unwrap_or((args, ""));

    match command.trim() {
        "depth" => {
            let depth = args.trim().parse::<usize>().unwrap();
            let best_move = search(data, depth, board, -INFINITY, INFINITY);
            if let Some((m, _)) = best_move {
                println!("bestmove {m}");
            }
        }
        "perft" => {
            println!("{args}");
            if let Ok(depth) = args.trim().parse::<usize>() {
                let clock = Instant::now();
                let nodes_count = crate::perft::perft(depth, board);
                println!(
                    "Number of nodes: {nodes_count}\nTime: {}ms",
                    clock.elapsed().as_millis()
                );
            } else {
                eprintln!("Enter a valid depth!")
            }
        }
        "wtime" => {
            //Example: go wtime 900000 btime 900000 winc 0 binc 0
            let args: Vec<&str> = args.split_ascii_whitespace().collect();
            let times: Vec<u128> = args.iter().filter_map(|e| e.parse::<u128>().ok()).collect();

            println!("{:?}", times);
            let wtime = times.first().unwrap_or(&300000);
            let btime = times.get(1).unwrap_or(&300000);

            let winc = times.get(2).unwrap_or(&0);
            let binc = times.get(3).unwrap_or(&0);

            match board.board_state.side_to_move {
                Side::White => data.set_limit(SearchKind::Normal(*wtime, *winc)),
                Side::Black => data.set_limit(SearchKind::Normal(*btime, *binc)),
            };

            let best_move = search_runner(board, data);
            if let Some((m, _)) = best_move {
                println!("bestmove {m}");
            }
        }
        "movetime" => {
            let time = args.trim().parse::<u128>().unwrap();
            data.set_limit(SearchKind::Exact(time));
            let best_move = search_runner(board, data);
            if let Some((m, _)) = best_move {
                println!("bestmove {m}");
            }
        }
        _ => {
            //eprintln!("Not a valid go argument!")
            let best_move = search_runner(board, data);
            if let Some((m, _)) = best_move {
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
    use super::*;
    use crate::types::constants::STARTING_FEN;

    #[test]
    fn test_parse_move() {
        let board = Board::from_fen(STARTING_FEN);
        if let Ok(m) = board.parse_move("e2e4") {
            println!("bestmove {m}");
        }
    }

    #[test]
    fn test_parse_times() {
        let mut board = Board::from_fen(STARTING_FEN);
        go(
            "wtime 5000 btime 5000 winc 0 binc 0",
            &mut board,
            &mut SearchData::default(),
        );
    }
}
