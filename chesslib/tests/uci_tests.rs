#[cfg(test)]
mod tests {
    use chesslib::handle_uci_command;
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

        // First character should be either a pawn move from rank 7 or a knight move from rank 8
        let rank = black_move.chars().nth(1).unwrap();
        assert!(rank == '7' || rank == '8', "Should be Black's move from rank 7 (pawn) or rank 8 (knight)");
    }
}
