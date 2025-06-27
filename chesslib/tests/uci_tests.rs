#[cfg(test)]
mod tests {
    use chesslib::handle_uci_command;

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

    #[test]
    fn test_fen_position() {
        // Test loading a simple FEN position
        let response = handle_uci_command("position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(response, "position set");

        // Verify the position by making a move
        let move_response = handle_uci_command("go");
        assert!(move_response.starts_with("bestmove"));
    }

    #[test]
    fn test_fen_position_with_moves() {
        // Test loading a FEN position and applying moves
        let response = handle_uci_command("position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 moves e7e5");
        assert_eq!(response, "position set");

        // Verify it's white to move after black's e7e5
        let move_response = handle_uci_command("go");
        assert!(move_response.starts_with("bestmove"));
        let white_move = move_response.split_whitespace().nth(1).unwrap();
        assert!(white_move.chars().nth(1).unwrap() == '1' || white_move.chars().nth(1).unwrap() == '2',
               "Should be White's move from rank 1 or 2");
    }

    #[test]
    fn test_invalid_fen_position() {
        // Test loading an invalid FEN position
        let response = handle_uci_command("position fen invalid/fen/string");
        assert_eq!(response, "position set");

        // Should still be able to make moves (from starting position)
        let move_response = handle_uci_command("go");
        assert!(move_response.starts_with("bestmove"));
    }

    #[test]
    fn test_fen_position_with_spaces() {
        // Test loading a FEN position that contains spaces - first validate the position loads
        let response = handle_uci_command("position fen 8/8/8/4k3/4P3/4K3/8/8 b - - 0 1");
        assert_eq!(response, "position set");

        // Verify we can query the position
        let go_response = handle_uci_command("go");
        assert!(go_response.starts_with("bestmove"));

        // Now try the move
        let response = handle_uci_command("position fen 8/8/8/4k3/4P3/4K3/8/8 b - - 0 1 moves e5e4");
        assert_eq!(response, "position set");

        // Verify it's white to move after black's e5e4
        let move_response = handle_uci_command("go");
        assert!(move_response.starts_with("bestmove"));
    }
}
