use crate::board::{Board, get_starting_board};
use std::sync::Mutex;

use lazy_static::lazy_static;

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
                        board.apply_moves_from_strings(moves.map(|s| s.to_string()));
                    }
                }
            } else {
                // Handle other position commands (like FEN) here if needed
                if board_state.is_none() {
                    *board_state = Some(get_starting_board());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_uci_command() {
        assert_eq!(handle_uci_command("uci"), "id name ChessEngine\nid author YourName\nuciok");
        assert_eq!(handle_uci_command("isready"), "readyok");
        assert_eq!(handle_uci_command("quit"), "");
        assert_eq!(handle_uci_command("unknown"), "Unknown command");
        assert_eq!(handle_uci_command("position"), "position set");
    }

    #[test]
    fn test_handle_uci_newgame() {
        assert_eq!(handle_uci_command("ucinewgame"), "");
    }

    #[test]
    fn test_handle_uci_position() {
        assert_eq!(handle_uci_command("position startpos moves e2e4"), "position set");
    }

    #[test]
    fn test_handle_uci_go() {
        handle_uci_command("position startpos moves e2e4"); // Set position
        let response = handle_uci_command("go");
        assert!(response.starts_with("bestmove"), "Response should start with 'bestmove'");
    }

    #[test]
    fn test_handle_uci_stop() {
        assert_eq!(handle_uci_command("stop"), "calculation stopped");
    }

    #[test]
    fn test_position_startpos_resets_board() {
        // Make some moves
        handle_uci_command("position startpos moves e2e4 e7e5");
        
        // Start a new position - this should reset
        handle_uci_command("position startpos moves d2d4");
        
        // Get next move - should be Black to move after d2d4
        let response = handle_uci_command("go");
        assert!(response.starts_with("bestmove"));
        let black_move = response.split_whitespace().nth(1).unwrap();
        assert!(black_move.chars().nth(1).unwrap() == '7', "Should be Black's move from rank 7");
    }
}
