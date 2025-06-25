#[cfg(test)]
mod castling_tests {
    use chesslib::board_utils::get_starting_board;

    #[test]
    fn test_initial_castling_rights() {
        let board = get_starting_board();
        assert!(board.white_kingside_castle_rights);
        assert!(board.white_queenside_castle_rights);
        assert!(board.black_kingside_castle_rights);
        assert!(board.black_queenside_castle_rights);
    }

    #[test]
    fn test_castling_rights_after_king_move() {
        let mut board = get_starting_board();

        // Move white king and verify castling rights are lost
        board.apply_move_from_string("e2e4");  // Move pawn to allow king movement
        board.apply_move_from_string("e7e5");  // Black response
        board.apply_move_from_string("e1e2");  // Move white king

        assert!(!board.white_kingside_castle_rights);
        assert!(!board.white_queenside_castle_rights);
        assert!(board.black_kingside_castle_rights);
        assert!(board.black_queenside_castle_rights);
    }

    #[test]
    fn test_castling_rights_after_rook_moves() {
        let mut board = get_starting_board();

        // Move kingside rook and verify only kingside rights are lost
        board.apply_move_from_string("h2h4");  // Move pawn to allow rook movement
        board.apply_move_from_string("e7e6");  // Black response
        board.apply_move_from_string("h1h3");  // Move kingside rook

        assert!(!board.white_kingside_castle_rights);
        assert!(board.white_queenside_castle_rights);
        assert!(board.black_kingside_castle_rights);
        assert!(board.black_queenside_castle_rights);
    }

    #[test]
    fn test_white_kingside_castle() {
        let mut board = get_starting_board();

        // Clear the path for kingside castle
        board.apply_move_from_string("e2e4");
        board.apply_move_from_string("e7e5");
        board.apply_move_from_string("g1f3");
        board.apply_move_from_string("g8f6");
        board.apply_move_from_string("f1e2");
        board.apply_move_from_string("f8e7");

        // Get legal moves and verify castling is included
        let moves = board.get_next_moves(-1);
        assert!(moves.contains(&"e1g1".to_string()));

        // Perform the castle
        board.apply_move_from_string("e1g1");

        // Verify king and rook positions
        assert_eq!(board.get_piece_at_coordinate_as_fen("g1"), "K");
        assert_eq!(board.get_piece_at_coordinate_as_fen("f1"), "R");
        assert_eq!(board.get_piece_at_coordinate_as_fen("e1"), " ");
        assert_eq!(board.get_piece_at_coordinate_as_fen("h1"), " ");
    }

    #[test]
    fn test_black_queenside_castle() {
        let mut board = get_starting_board();

        // Clear the path for queenside castle
        board.apply_move_from_string("a2a3"); // white moving pawn
        board.apply_move_from_string("b8c6"); // black moving knight out
        board.apply_move_from_string("b2b3"); // white moving pawn
        board.apply_move_from_string("d7d6"); // black moving pawn in front of queen
        board.apply_move_from_string("c2c3"); // white moving pawn
        board.apply_move_from_string("c8g4"); // black moving bishop out
        board.apply_move_from_string("d2d3"); // white moving pawn out
        board.apply_move_from_string("d8e7"); // black moving queen out
        board.apply_move_from_string("e2e3"); // white moving pawn out

        // Get legal moves and verify castling is included
        let moves = board.get_next_moves(-1);
        assert!(moves.contains(&"e8c8".to_string()));

        // Perform the castle
        board.apply_move_from_string("e8c8");

        // Verify king and rook positions
        assert_eq!(board.get_piece_at_coordinate_as_fen("c8"), "k");
        assert_eq!(board.get_piece_at_coordinate_as_fen("d8"), "r");
        assert_eq!(board.get_piece_at_coordinate_as_fen("e8"), " ");
        assert_eq!(board.get_piece_at_coordinate_as_fen("a8"), " ");
    }

    #[test]
    fn test_castling_not_allowed_through_check() {
        let mut board = get_starting_board();

        // Set up a position where white can't castle through check
        board.apply_move_from_string("e2e4");
        board.apply_move_from_string("e7e5");
        board.apply_move_from_string("g1f3");
        board.apply_move_from_string("d8h4"); // Queen attacks f1, preventing kingside castle

        let moves = board.get_next_moves(-1);
        assert!(!moves.contains(&"e1g1".to_string()));
    }

    #[test]
    fn test_castling_not_allowed_in_check() {
        let mut board = get_starting_board();

        // Set up a position where white is in check
        board.apply_move_from_string("f2f3");
        board.apply_move_from_string("e7e6");
        board.apply_move_from_string("g2g4");
        board.apply_move_from_string("d8h4"); // Check!

        let moves = board.get_next_moves(-1);
        assert!(!moves.contains(&"e1g1".to_string()));
        assert!(!moves.contains(&"e1c1".to_string()));
    }
}
