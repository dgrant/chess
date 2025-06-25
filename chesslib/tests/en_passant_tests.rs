#[cfg(test)]
mod tests {
    use chesslib::types::Square;
    use chesslib::board_utils::get_starting_board;

    #[test]
    fn test_white_en_passant_after_black_double_push() {
        let mut board = get_starting_board();

        // Move white pawn to e4
        board.apply_move_from_string("e2e4");
        // Verify en-passant target is set correctly
        assert_eq!(board.en_passant_target, Some(Square::try_from("e3").unwrap()));

        // Move black pawn to f7 to f5 (double push)
        board.apply_move_from_string("f7f5");
        // Verify en-passant target is set correctly
        assert_eq!(board.en_passant_target, Some(Square::try_from("f6").unwrap()));

        // Get all moves for white
        let moves = board.get_next_moves(-1);
        assert!(moves.contains(&"e4f5".to_string()));

        board.apply_move_from_string("e4e5"); // white move e4 to e5
        assert_eq!(board.en_passant_target, None);
        board.apply_move_from_string("d7d5"); // black move d7 to d5 (this pawn can be captured en-passant)

        assert_eq!(board.en_passant_target, Some(Square::try_from("d6").unwrap()));
        // Now white can capture en-passant from e5 to d6 (capturing the black pawn on d5)
        board.apply_move_from_string("e5d6");

        // Make sure the black pawn on d5 is removed
        assert_eq!(board.get_piece_at_coordinate_as_fen("d5"), " ");
    }

    #[test]
    fn test_black_en_passant_after_white_double_push() {
        let mut board = get_starting_board();

        // Move white pawn from e2 to e4 (double push)
        board.apply_move_from_string("e2e4");
        // Verify en-passant target is set correctly
        assert_eq!(board.en_passant_target, Some(Square::try_from("e3").unwrap()));

        // Move black pawn to f5 (double push)
        board.apply_move_from_string("f7f5");
        board.apply_move_from_string("a2a3");
        // Move black pawn to f4
        board.apply_move_from_string("f5f4");
        board.apply_move_from_string("a3a4");
        // Move black pawn to h4 (double push)
        board.apply_move_from_string("h7h5");
        board.apply_move_from_string("a4a5");
        // Move black pawn to h4
        board.apply_move_from_string("h5h4");

        // Move white pawn g2 to g4 (double push), this pawn can be captured en-passant by black h or f pawns
        board.apply_move_from_string("g2g4");

        // Verify en-passant target is set correctly
        assert_eq!(board.en_passant_target, Some(Square::try_from("g3").unwrap()));

        // Get all moves for black
        let moves = board.get_next_moves(-1);

        // Verify both e4 pawns can capture en-passant
        assert!(moves.contains(&"h4g3".to_string()));
        assert!(moves.contains(&"f4g3".to_string()));
    }

    #[test]
    fn test_en_passant_expires() {
        let mut board = get_starting_board();

        // Move white pawn to e4
        board.apply_move_from_string("e2e4");
        board.apply_move_from_string("h7h6");
        board.apply_move_from_string("e4e5");
        // Move black pawn to f7 to f5 (double push)
        board.apply_move_from_string("f7f5");

        // Verify en-passant is available for white
        assert!(board.get_next_moves(-1).contains(&"e5f6".to_string()));

        // Make a different move
        board.apply_move_from_string("d2d3"); // move by white

        // Verify en-passant target is cleared
        assert_eq!(board.en_passant_target, None);

        // Get all moves for white
        let black_moves = board.get_next_moves(-1);

        // Verify en-passant is no longer possible
        assert!(!black_moves.contains(&"e5f6".to_string()));
    }

    #[test]
    fn test_en_passant_capture_removes_correct_pawn() {
        let mut board = get_starting_board();

        // Setup position
        board.apply_move_from_string("e2e4"); // white
        board.apply_move_from_string("a7a6"); // black
        board.apply_move_from_string("e4e5"); // white
        board.apply_move_from_string("f7f5"); // black, double push can be captured en-passant

        // Do en-passant capture
        board.apply_move_from_string("e5f6");

        // Verify the captured pawn is removed
        assert_eq!(board.get_piece_at_coordinate_as_fen("f5"), " ");
        // Verify capturing pawn is on f6
        assert_eq!(board.get_piece_at_coordinate_as_fen("f6"), "P");
    }
}
