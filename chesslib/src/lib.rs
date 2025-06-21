use crate::board::{Board, get_starting_board, bitboard_to_moves, Color};
use crate::move_generation::{b_single_push_targets};
use rand::seq::IteratorRandom;
use std::sync::Mutex;

pub mod board;
pub mod move_generation;

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
            let mut board_state = BOARD_STATE.lock().unwrap();
            if let Some(board) = board_state.as_mut() {
                let moves = command.strip_prefix("position startpos moves ").unwrap_or("").split_whitespace();
                for mv in moves {
                    board.apply_move(mv); // Apply each move to the board
                }
            } else {
                *board_state = Some(get_starting_board());
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

                // Example logic using parsed parameters (can be replaced with actual move calculation logic)
                let black_pawn_moves = b_single_push_targets(board.black_pawns, board.empty) & board.black_pawns;
                if board.side_to_move == Color::Black {
                    let possible_moves: Vec<String> = bitboard_to_moves(black_pawn_moves, true);
                    if let Some(random_move) = possible_moves.into_iter().choose(&mut rand::thread_rng()) {
                        return format!("bestmove {}", random_move);
                    }
                    return "bestmove e7e6".to_string();
                }
                return "bestmove e2e4".to_string();
            }
            "bestmove e2e4".to_string() // Default move if no position is set
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
}
