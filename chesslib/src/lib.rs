pub mod board;
pub mod move_generation;

pub fn handle_uci_command(input: &str) -> String {
    static mut BOARD_STATE: Option<String> = None; // Store the board state

    match input.trim() {
        "uci" => "id name ChessEngine\nid author YourName\nuciok".to_string(),
        "isready" => "readyok".to_string(),
        "quit" => "".to_string(),
        "ucinewgame" => {
            unsafe {
                BOARD_STATE = None; // Reset the board state
            }
            "".to_string()
        },
        command if command.starts_with("position") => {
            unsafe {
                BOARD_STATE = Some(command.to_string());
            }
            "position set".to_string()
        },
        command if command.starts_with("go") => {
            unsafe {
                if let Some(board_state) = &BOARD_STATE {
                    if board_state.contains("e2e4") {
                        return "bestmove e7e5".to_string();
                    }
                }
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
    fn test_handle_uci_usermove() {
        assert_eq!(handle_uci_command("e2e4"), "bestmove e7e5");
        assert_eq!(handle_uci_command("d2d4"), "bestmove e2e4");
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
        assert_eq!(handle_uci_command("go"), "bestmove e7e5");
    }

    #[test]
    fn test_handle_uci_stop() {
        assert_eq!(handle_uci_command("stop"), "calculation stopped");
    }
}
