use crate::board::Board;
use crate::logger::log_to_file;
use std::sync::Mutex;

use lazy_static::lazy_static;
use crate::board_utils::get_starting_board;

lazy_static! {
    static ref BOARD_STATE: Mutex<Option<Board>> = Mutex::new(None);
}

pub fn handle_uci_command(input: &str) -> String {
    match input.trim() {
        "uci" => "id name ChessEngine\nid author YourName\nuciok".to_string(),
        "isready" => "readyok".to_string(),
        "quit" => "".to_string(),
        "ucinewgame" => {
            let mut board_state = BOARD_STATE.lock().unwrap();
            *board_state = Some(get_starting_board()); // Reset the board state
            "".to_string()
        },
        command if command.starts_with("position") => {
            let mut board_state = BOARD_STATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            // Always reset to starting position when "startpos" is used
            if command.contains("startpos") {
                *board_state = Some(get_starting_board());
                if let Some(board) = board_state.as_mut() {
                    if let Some(moves_str) = command.strip_prefix("position startpos moves ") {
                        let moves = moves_str.split_whitespace();
                        for move_str in moves {
                            board.apply_moves_from_strings(std::iter::once(move_str.to_string()));
                            log_to_file(&format!("Position after {}: {}", move_str, board.to_fen()), true);
                        }
                    } else {
                        // Log initial position
                        log_to_file(&format!("Initial position: {}", board.to_fen()), true);
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
        },
        command if command.starts_with("go") => {
            let board_state = BOARD_STATE.lock().unwrap();
            if let Some(board) = board_state.as_ref() {
                let mut _wtime: Option<u32> = None;
                let mut _btime: Option<u32> = None;
                let mut _movestogo: Option<u32> = None;

                // Parse parameters
                let params: Vec<&str> = command.split_whitespace().collect();
                for i in 0..params.len() {
                    match params[i] {
                        "wtime" => _wtime = params.get(i + 1).and_then(|v| v.parse::<u32>().ok()),
                        "btime" => _btime = params.get(i + 1).and_then(|v| v.parse::<u32>().ok()),
                        "movestogo" => _movestogo = params.get(i + 1).and_then(|v| v.parse::<u32>().ok()),
                        _ => {}
                    }
                }

                format!("bestmove {}", board.get_next_move())
            } else {
                "bestmove e2e4".to_string() // Default move if no position is set
            }
        },
        "stop" => "calculation stopped".to_string(),
        _ => "Unknown command".to_string(),
    }
}
