use std::cmp::Reverse;

use crate::board::Board;
use crate::board::movegen::MoveGenKind;
use crate::search::data::SearchData;
use crate::types::*;

pub mod data;
pub mod time;

pub const FAIL_INCREMENTS: [i32; 5] = [25, 50, 150, 300, INFINITY];

impl Board {
    pub fn detect_repetitions(&self) -> usize {
        let half_moves = self.board_state.half_move_clock as usize;
        let mut count = 0;

        if self.game_history.len() < half_moves {
            return 0
        }

        let last_halfmove_ply = self.game_history.len() - half_moves;
        for position in self.game_history[last_halfmove_ply..].iter() {
            if self.board_state.hash == *position {
                count += 1
            }
        }

        count
    }
}

pub fn search_runner(board: &mut Board, data: &mut SearchData) -> Option<(Move, i32)> {
    data.clear_node_count();
    data.reset_pv();
    data.start_time();
    //data.clear_table(); //Clears Transposition Table for some reason it plays better when clearing the TT before the start of every search prob cause it gets filled up with old + deep entries that never get replaced....
    let mut depth = 1;

    //Initialize with move from first depth
    let mut best_move = search(data, depth, board, -INFINITY, INFINITY)?;
    depth += 1;

    //Aspiration Window
    let mut score = best_move.1;
    let mut alpha_window = score - (100 / 4);
    let mut beta_window = score + (100 / 4);
    let mut alpha_fail = 0;
    let mut beta_fail = 0;

    //All infos belonging to the pv should be sent together e.g. info depth 2 score cp 214 time 1242 nodes 2124 nps 34928 pv e2e4 e7e5 g1f3
    println!(
        "info depth {} time {} score cp {} nodes {} nps {} pv {} hashfull {}",
        depth - 1,
        data.time.elapsed().as_millis(),
        score,
        data.get_total_nodes_searched(),
        data.nodes_per_second(),
        data.get_pv(),
        data.tt.hashfull(),
    );

    //Iterative Deepening
    loop {
        let deeper_move = search(data, depth, board, alpha_window, beta_window);
        if data.over_limit() {
            println!("Searched for: {}ms\nTime Limit: {}ms", data.time.elapsed().as_millis(), data.time.get_limit());
            break;
        }
        let new_score = deeper_move.unwrap().1;
        if new_score <= alpha_window {
            //Failed Low
            alpha_window -= FAIL_INCREMENTS[alpha_fail];
            alpha_fail += 1;
            continue;
        } else if new_score > beta_window {
            //Failed High
            beta_window += FAIL_INCREMENTS[beta_fail];
            beta_fail += 1;
            continue;
        }

        depth += 1;

        best_move = deeper_move?;
        score = new_score;
        alpha_fail = 0;
        beta_fail = 0;
        alpha_window = score - (100 / 4);
        beta_window = score + (100 / 4);
        println!(
            "info depth {} time {} score cp {} nodes {} nps {} pv {} hashfull {}",
            depth - 1,
            data.time.elapsed().as_millis(),
            score,
            data.get_total_nodes_searched(),
            data.nodes_per_second(),
            data.get_pv(),
            data.tt.hashfull()
        );
    }

    Some(best_move)
}

pub fn search(
    data: &mut SearchData,
    depth: usize,
    board: &mut Board,
    alpha: i32,
    beta: i32,
) -> Option<(Move, i32)> {
    //Root Search
    let mut best_score = -INFINITY;
    let mut best_move: Option<(Move, i32)> = None;
    let ply = 0;
    data.clear_pv(0);

    for m in order_moves(board, data).iter() {
        if board.make_move(*m).is_ok() {
            let score = -negamax(data, depth - 1, board, -beta, -alpha, ply + 1);
            board.unmake_move();
            if data.over_limit() {
                return None;
            }
            //println!("{m}: {score}");
            if score >= best_score {
                best_score = score;
                best_move = Some((*m, score));
                data.add_pv_move(*m, ply);
            }
        }
    }

    if let Some((m, s)) = best_move {
        data.tt
            .add_entry(m, s, Bound::Exact, board.board_state.hash, depth);
    }
    best_move
}

pub fn negamax(
    data: &mut SearchData,
    depth: usize,
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    ply: usize,
) -> i32 {
    if depth == 0 {
        if board.king_in_check() {
            return search_checks(data, board, alpha, beta, ply);
        } else {
            return quiesce(data, board, alpha, beta, ply); //Horizon Node
        }
    }

    data.add_nodes(1);
    data.clear_pv(ply);

    if board.board_state.half_move_clock > 4 {
        //50 move rule
        if board.board_state.half_move_clock >= 50 {
            return 0;
        }
        //We need to check history if positions were repeated only for the side to move.
        let count = board.detect_repetitions();
        if count >= 2 {
            return 0;
        }
    }

    //TT Cutoffs only if depth of entry is greater or equal to the depth of the current node
    if let Some(e) = data.tt.get_entry(board.board_state.hash)
        && board.board_state.hash == e.get_key()
        && e.get_depth() >= depth
    {
        let mut tt_score = e.get_score();
        //Adjust mate scores
        if tt_score == MATE_SCORE {
            tt_score -= ply as i32;
        } else if tt_score == -MATE_SCORE {
            tt_score += ply as i32;
        }

        match e.get_bound() {
            Bound::Exact => return tt_score,
            Bound::Lower => {
                if tt_score >= beta {
                    return tt_score;
                }
            }
            Bound::Upper => {
                if tt_score < alpha {
                    return tt_score;
                }
            }
        }
    }

    let mut legal_moves = 0;
    let mut best_score = -INFINITY;
    let mut best_move: Option<Move> = None;
    let mut bound = Bound::Upper; //Fail-high means score is atleast this good so lower-bound/Fail-low means the score is an upper bound

    for m in order_moves(board, data).iter() {
        if board.make_move(*m).is_ok() {
            legal_moves += 1;
            let mut score;

            //PVS
            if legal_moves == 1 {
                //First Move
                score = -negamax(data, depth - 1, board, -beta, -alpha, ply + 1);
            } else {
                score = -negamax(data, depth - 1, board, -alpha - 1, -alpha, ply + 1);
                if score > alpha && score < beta {
                    score = -negamax(data, depth - 1, board, -beta, -alpha, ply + 1); //We want to search again
                }
            }

            board.unmake_move();
            if data.over_limit() {
                return TIMEOUT_SCORE;
            }

            if score > alpha {
                bound = Bound::Exact;
                alpha = score;
                data.add_pv_move(*m, ply);
            }

            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }

            if score >= beta {
                if let Some(m) = best_move {
                    let mut tt_score = best_score;
                    //Adjust mate scores
                    if tt_score >= MATE_CUTOFF {
                        tt_score = MATE_SCORE;
                    } else if tt_score <= -MATE_CUTOFF {
                        tt_score = -MATE_SCORE;
                    }
                    data.tt
                        .add_entry(m, tt_score, Bound::Lower, board.board_state.hash, depth);
                }
                return best_score;
            }
        }
    }

    if legal_moves == 0 {
        if board.is_king_in_attack(board.board_state.side_to_move) {
            return -MATE_SCORE + ply as i32;
        } else {
            return 0;
        }
    }

    if let Some(m) = best_move {
        let mut tt_score = best_score;
        //Adjust mate scores
        if tt_score >= MATE_CUTOFF {
            tt_score = MATE_SCORE;
        } else if tt_score <= -MATE_CUTOFF {
            tt_score = -MATE_SCORE;
        }
        data.tt
            .add_entry(m, tt_score, bound, board.board_state.hash, depth);
    }

    best_score
}

pub fn quiesce(
    data: &mut SearchData,
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    _ply: usize,
) -> i32 {
    data.add_nodes(1);
    let static_eval = board.evaluate();
    // if board.is_king_in_attack(board.board_state.side_to_move) {
    //     return search_checks(data, board, alpha, beta, _ply);
    // }

    if board.board_state.half_move_clock > 4 {
        //We need to check history if positions were repeated only for the side to move.
        let count = board.detect_repetitions();
        if count >= 2 {
            return 0;
        }
    }

    let mut best_score = static_eval;
    //let mut bound = Bound::Upper;
    //let mut best_move: Option<Move> = None;

    if best_score >= beta {
        return best_score;
    }
    if best_score > alpha {
        alpha = best_score;
    }

    for m in order_moves(board, data).iter() {
        if !m.get_kind().is_quiet() && board.make_move(*m).is_ok() {
            let score = -quiesce(data, board, -beta, -alpha, _ply + 1);
            board.unmake_move();
            if data.over_limit() {
                return TIMEOUT_SCORE;
            }

            if score >= beta {
                // if let Some(m) = best_move {
                //     board
                //         .tt
                //         .add_entry(m, best_score, Bound::Lower, board.board_state.hash, 0);
                // }
                return score;
            }
            if score > best_score {
                best_score = score;
                //best_move = Some(*m);
            }
            if score > alpha {
                alpha = score;
                //bound = Bound::Exact;
            }
        }
    }

    // if let Some(m) = best_move {
    //     board
    //         .tt
    //         .add_entry(m, best_score, bound, board.board_state.hash, 0);
    // }

    best_score
}

pub fn search_checks(
    data: &mut SearchData,
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    ply: usize,
) -> i32 {
    let mut best_score = -INFINITY;
    let mut legal_moves = 0;
    data.add_nodes(1);

    if !board.king_in_check() {
        return quiesce(data, board, alpha, beta, ply);
    }

    for m in order_moves(board, data).iter() {
        if board.make_move(*m).is_ok() {
            legal_moves += 1;
            let score = -search_checks(data, board, -beta, -alpha, ply + 1);
            board.unmake_move();
            if data.over_limit() {
                return TIMEOUT_SCORE;
            }

            if score >= beta {
                return score;
            }
            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
        }
    }

    if legal_moves == 0 {
        if board.is_king_in_attack(board.board_state.side_to_move) {
            return -MATE_SCORE + ply as i32;
        } else {
            return 0;
        }
    }

    best_score
}

pub fn order_moves(board: &mut Board, data: &mut SearchData) -> MoveList {
    //We want to sort the moves based on most valuable victim / least valuable attacker
    //Sort captures based on (attacked piece value - attacking piece value)
    let mut full_list = MoveList::new();
    let mut captures: MoveList = board.generate_moves(MoveGenKind::Captures);
    let mut best_move: Option<Move> = None;

    //Want to add the best move from the transposition table if it exists to the beginning of the list
    if let Some(e) = data.tt.get_entry(board.board_state.hash)
        && board.board_state.hash == e.get_key()
    {
        let bm = e.get_best_move();
        full_list.push(bm);
        best_move = Some(bm);
    }

    captures.iter_mut().into_slice().sort_by_key(|m| {
        let attacker = board.get_piece_at_square(m.get_from()).unwrap().1;
        let victim = match m.get_kind() {
            MoveKind::EnPassant => {
                board
                    .get_piece_at_square(Square::from(m.get_to() as usize ^ 8))
                    .unwrap()
                    .1
            }
            _ => board.get_piece_at_square(m.get_to()).unwrap().1,
        };
        Reverse(victim.value() - attacker.value())
    });

    let quiet = board.generate_moves(MoveGenKind::Quiet);
    let pawn_promos = board.generate_moves(MoveGenKind::NonCapturePromotions);

    for m in captures
        .iter()
        .chain(pawn_promos.iter())
        .chain(quiet.iter())
    {
        if let Some(bm) = best_move
            && *m == bm
        {
            continue;
        }
        full_list.push(*m);
    }

    full_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_search() {
        let mut board = Board::from_fen(STARTING_FEN);
        let mut data = SearchData::default();
        let best_move = search(&mut data, 5, &mut board, -INFINITY, INFINITY);
        if let Some(m) = best_move {
            println!("Best move: {}", m.0);
        }
    }

    #[test]
    fn test_order_moves() {
        let mut board = Board::from_fen(STARTING_FEN);
        let move_list = order_moves(&mut board, &mut SearchData::default());
        println!("{board}");
        println!("{move_list}");
        println!();

        let mut board =
            Board::from_fen("rnbqkb1r/pp3p2/4pnpp/1p1p2N1/1Q1P4/BP2P3/P1PN1PPP/R3K2R b KQkq - 0 1");
        let move_list = order_moves(&mut board, &mut SearchData::default());

        println!("{board}");
        println!("{move_list}");
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(
            *first_move,
            Move::new(Square::F8, Square::B4, MoveKind::Capture)
        );

        let mut board =
            Board::from_fen("rnbq1rk1/pN1p1ppp/4n2b/2p1p3/N1BP3R/2P2Q2/PP3PPP/2B1K2R w K - 0 1");
        let move_list = order_moves(&mut board, &mut SearchData::default());

        println!("{board}");
        for m in move_list.iter() {
            print!("{m}, ");
        }
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(
            *first_move,
            Move::new(Square::B7, Square::D8, MoveKind::Capture)
        );
    }

    #[test]
    fn test_repetion_detection() {
        use MoveKind::*;
        use Square::*;

        let mut board = Board::from_fen("8/6K1/3N4/8/5Q2/8/1kr5/8 w - - 0 1");
        let _ = board.make_move(Move::new(F4, E4, QuietMove));
        let _ = board.make_move(Move::new(C2, C1, QuietMove));
        let _ = board.make_move(Move::new(E4, F4, QuietMove));
        let _ = board.make_move(Move::new(C1, C2, QuietMove));
        let _ = board.make_move(Move::new(F4, E4, QuietMove));
        let _ = board.make_move(Move::new(C2, C1, QuietMove));
        let _ = board.make_move(Move::new(E4, F4, QuietMove));

        let mut data = SearchData::default();
        let (m, score) = search(&mut data, 3, &mut board, -INFINITY, INFINITY).unwrap();
        println!(
            "{:?}\nCurrent Hash: {}",
            board.game_history, board.board_state.hash
        );
        println!("Repetions counted: {}", board.detect_repetitions());
        assert_eq!(score, 0);
        assert_eq!(m, Move::new(C1, C2, QuietMove));
    }

    #[test]
    fn test_mate_in_one() {
        let mut data = SearchData::default();
        let mut board =
            Board::from_fen("r1b4r/p1p1q3/1bppk3/4pp2/3PP1Q1/2P1R3/PP3PPP/RN4K1 w - - 0 18");
        data.set_playing_as(board.board_state.side_to_move);
        let best_move = search(&mut data, 1, &mut board, -INFINITY, INFINITY);
        println!("Best Move: {}", best_move.unwrap().0);
        assert_eq!(
            Move::new(Square::G4, Square::F5, MoveKind::Capture),
            best_move.unwrap().0
        );
    }

    #[test]
    fn test_mate_in_four() {
        let mut data = SearchData::default();
        let mut board = Board::from_fen("6k1/5pp1/5n1p/8/5P1q/2RQ3P/B5PK/8 b - - 0 36");
        data.set_playing_as(board.board_state.side_to_move);
        let best_move = search(&mut data, 4, &mut board, -INFINITY, INFINITY);
        println!("Best Move: {}", best_move.unwrap().0);
        assert_eq!(
            Move::new(Square::F6, Square::G4, MoveKind::QuietMove),
            best_move.unwrap().0
        );
    }

    #[test]
    fn test_pv_line() {
        use MoveKind::*;
        use Square::*;

        let mut data = SearchData::default();
        let mut board = Board::from_fen("6k1/5pp1/5n1p/8/5P1q/2RQ3P/B5PK/8 b - - 0 36");
        data.set_playing_as(board.board_state.side_to_move);
        data.get_time_settings().btime = 1000000;
        data.start_time();
        let best_move = search(&mut data, 7, &mut board, -INFINITY, INFINITY);
        println!("PV: {}", data.get_pv());
        let mut pv_line = MoveList::new();
        pv_line.push(Move::new(F6, G4, QuietMove));
        pv_line.push(Move::new(H2, G1, QuietMove));
        pv_line.push(Move::new(H4, F2, QuietMove));
        pv_line.push(Move::new(G1, H1, QuietMove));
        pv_line.push(Move::new(F2, E1, QuietMove));
        pv_line.push(Move::new(D3, F1, QuietMove));
        pv_line.push(Move::new(E1, F1, Capture));

        assert_eq!(pv_line.to_string(), data.get_pv().to_string());

        println!("Best Move: {}", best_move.unwrap().0);
        assert_eq!(
            Move::new(Square::F6, Square::G4, MoveKind::QuietMove),
            best_move.unwrap().0
        );
    }

    #[test]
    fn test_bugged_position() {
        let mut board = Board::from_fen("6k1/5pp1/7p/8/5Pn1/2R4P/B5P1/4qQ1K b - - 6 39");
        println!("Hash: {}", board.board_state.hash);
        //Position hash: 6128121706435820836

        board = Board::from_fen("6k1/5pp1/7p/8/5Pn1/2RQ3P/B4qP1/6K1 w - - 3 38");
        println!("Hash 2: {}", board.board_state.hash);
        //Position hash: 16381162810209017462

        board = Board::from_fen("6k1/5pp1/7p/8/5Pnq/2RQ3P/B5P1/6K1 b - - 2 37");
        println!("Hash 3: {}", board.board_state.hash);
        //Position hash: 3246015867840709621
    }

    #[test]
    fn test_transposition_timeout() {
        let mut data = SearchData::new();
        data.set_playing_as(Side::Black);
        data.get_time_settings().btime = 8080;
        let mut board = Board::from_fen("6k1/2p5/4R1pp/1p1r4/pP1p4/P5PP/2P2P2/6K1 b - - 0 32");
        let _ = search_runner(&mut board, &mut data);
        println!();
        // let best_move1 = search_runner(&mut board, &mut data);
        // println!();
        // let best_move2 = search_runner(&mut board, &mut data);
        // println!();
        // let best_move3 = search_runner(&mut board, &mut data);
        // println!();
        // let best_move4 = search_runner(&mut board, &mut data);
        // println!();

        assert!(!data.tt.0.iter().any(|i| {
            if let Some(e) = i {
                e.get_score() == TIMEOUT_SCORE
            } else {
                false
            }
        }));

        //I want to count the number of entries in the table
        let total_size = data.tt.0.len();
        assert_eq!(total_size, ENTRIES);
        let count = data.tt.0.iter().filter(|e|e.is_some()).count();

        println!("Total Size: {total_size} Number of Entries: {count}");
        println!("Hashfull: {}", (count as f32/total_size as f32) * 1000.0);
        println!("{}", data.tt.hashfull());
    }
}
