use crate::board::Board;
use crate::logger::log_to_file;
use crate::search::Searcher;
use crate::types::Color;
use std::sync::Mutex;
use std::time::Duration;

use crate::board_utils::get_starting_board;
use crate::fen::load_fen;
use lazy_static::lazy_static;

lazy_static! {
    static ref BOARD_STATE: Mutex<Option<Board>> = Mutex::new(None);

    /// One Searcher per process. Killer/history (and the future
    /// transposition table) survive across `go` commands within a
    /// session — a depth-N search after a depth-(N-1) search reuses
    /// the previous iteration's move-ordering signals. This is also
    /// the reason `Searcher` is stateful (see `chesslib::search` for
    /// the design rationale). Reset on `ucinewgame`.
    static ref SEARCHER: Mutex<Searcher> = Mutex::new(Searcher::new());
}

pub fn handle_uci_command(input: &str) -> String {
    match input.trim() {
        "uci" => "id name ChessEngine\nid author YourName\nuciok".to_string(),
        "isready" => "readyok".to_string(),
        "quit" => "".to_string(),
        "ucinewgame" => {
            let mut board_state = BOARD_STATE.lock().unwrap();
            *board_state = Some(get_starting_board()); // Reset the board state
                                                       // Fresh game → fresh searcher. Drops killer/history from
                                                       // the previous game so stale move-ordering signals don't
                                                       // contaminate the new one.
            *SEARCHER.lock().unwrap() = Searcher::new();
            "".to_string()
        }
        command if command.starts_with("position") => {
            let mut board_state = BOARD_STATE
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            // Always reset to starting position when "startpos" is used
            if command.contains("startpos") {
                *board_state = Some(get_starting_board());
                if let Some(board) = board_state.as_mut() {
                    if let Some(moves_str) = command.strip_prefix("position startpos moves ") {
                        let moves = moves_str.split_whitespace();
                        for move_str in moves {
                            board.apply_moves_from_strings(std::iter::once(move_str.to_string()));
                            log_to_file(
                                &format!("Position after {}: {}", move_str, board.to_fen()),
                                true,
                            );
                        }
                    } else {
                        // Log initial position
                        log_to_file(&format!("Initial position: {}", board.to_fen()), true);
                    }
                }
            } else if command.starts_with("position fen ") {
                let command = command.trim();
                // Find where the moves part begins, if it exists
                let (fen_part, moves_part) = match command.find(" moves ") {
                    Some(moves_index) => {
                        let (fen, moves) = command.split_at(moves_index);
                        (
                            fen.strip_prefix("position fen ").unwrap(),
                            Some(moves.strip_prefix(" moves ").unwrap()),
                        )
                    }
                    None => (command.strip_prefix("position fen ").unwrap(), None),
                };

                // Load the FEN position
                match load_fen(fen_part.trim()) {
                    Ok(new_board) => {
                        *board_state = Some(new_board);

                        // Log the initial FEN position
                        if let Some(board) = board_state.as_ref() {
                            log_to_file(&format!("Initial FEN position: {}", board.to_fen()), true);
                        }

                        // Handle any moves after the FEN position
                        if let Some(moves_str) = moves_part {
                            if let Some(board) = board_state.as_mut() {
                                let moves = moves_str.split_whitespace();
                                for move_str in moves {
                                    board.apply_moves_from_strings(std::iter::once(
                                        move_str.to_string(),
                                    ));
                                    log_to_file(
                                        &format!("Position after {}: {}", move_str, board.to_fen()),
                                        true,
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log_to_file(&format!("Error loading FEN position: {e}"), true);
                        *board_state = Some(get_starting_board());
                    }
                }
            } else {
                // Handle other position commands (like FEN) here if needed
                if board_state.is_none() {
                    *board_state = Some(get_starting_board());
                    if let Some(board) = board_state.as_ref() {
                        log_to_file(&format!("Initial position: {}", board.to_fen()), true);
                    }
                }
            }
            "position set".to_string()
        }
        command if command.starts_with("go") => {
            let mut board_state = BOARD_STATE.lock().unwrap();
            if let Some(board) = board_state.as_mut() {
                let mut wtime: Option<u32> = None;
                let mut btime: Option<u32> = None;
                let mut movetime: Option<u32> = None;
                let mut fixed_depth: Option<i32> = None;

                let params: Vec<&str> = command.split_whitespace().collect();
                for i in 0..params.len() {
                    match params[i] {
                        "wtime" => wtime = params.get(i + 1).and_then(|v| v.parse().ok()),
                        "btime" => btime = params.get(i + 1).and_then(|v| v.parse().ok()),
                        "movetime" => movetime = params.get(i + 1).and_then(|v| v.parse().ok()),
                        "depth" => fixed_depth = params.get(i + 1).and_then(|v| v.parse().ok()),
                        _ => {}
                    }
                }

                // 'go depth N' is exact. Otherwise use a time budget from movetime
                // or 1/30th of our remaining clock, default 1s if no info.
                let mut searcher = SEARCHER.lock().unwrap();
                let result = if let Some(d) = fixed_depth {
                    let (mv, score) = searcher.find_best_move(board, d);
                    mv.map(|m| (m.to_string(), score, d))
                } else {
                    let budget_ms: u64 = movetime.map(|t| t as u64).unwrap_or_else(|| {
                        let our_ms = match board.side_to_move {
                            Color::White => wtime,
                            Color::Black => btime,
                        };
                        our_ms.map(|t| (t as u64 / 30).max(50)).unwrap_or(1000)
                    });
                    let (mv, score, depth) =
                        searcher.find_best_move_within(board, Duration::from_millis(budget_ms));
                    mv.map(|m| (m.to_string(), score, depth))
                };

                match result {
                    Some((mv, score, depth)) => {
                        // UCI: 'score cp' is from the engine's perspective (side to
                        // move), positive = engine is winning. Our find_best_move
                        // returns it in White's POV, so flip when Black is to move.
                        let cp = if board.side_to_move == Color::Black {
                            -score
                        } else {
                            score
                        };
                        println!("info depth {depth} score cp {cp} pv {mv}");
                        format!("bestmove {mv}")
                    }
                    None => "bestmove 0000".to_string(),
                }
            } else {
                "bestmove e2e4".to_string() // Default move if no position is set
            }
        }
        "stop" => "calculation stopped".to_string(),
        _ => "Unknown command".to_string(),
    }
}
