#[cfg(test)]
mod pawn_promotion_tests {
    use chesslib::board_utils::get_starting_board;

    #[test]
    fn test_white_pawn_promotion() {
        let mut board = get_starting_board();

        // Move a pawn to 7th rank
        board.apply_move_from_string("e2e4"); // white
        board.apply_move_from_string("d7d5"); // black
        board.apply_move_from_string("e4d5"); // white takes black pawn
        board.apply_move_from_string("h7h6"); // black
        board.apply_move_from_string("d5d6"); // white
        board.apply_move_from_string("h6h5"); // black
        board.apply_move_from_string("d6c7"); // white
        board.apply_move_from_string("h5h4"); // black
        board.apply_move_from_string("c7b8q");  // Promote to queen

        // Verify promotion
        // TODO: Fix this to now use fen, just use existing enums
        assert_eq!(board.get_piece_at_coordinate_as_fen("b8"), "Q");
    }

    #[test]
    fn test_black_pawn_promotion() {
        let mut board = get_starting_board();

        // Move a pawn to 2nd rank and promote
        board.apply_move_from_string("e2e4"); // white
        board.apply_move_from_string("d7d5"); // black
        board.apply_move_from_string("a2a3"); // white
        board.apply_move_from_string("d5e4"); // black
        board.apply_move_from_string("b2b3"); // white
        board.apply_move_from_string("e4e3"); // black
        board.apply_move_from_string("c2c3"); // white
        board.apply_move_from_string("e3f2"); // black
        board.apply_move_from_string("d2d3"); // white
        board.apply_move_from_string("f2g1q"); // black

        // Verify promotion
        // TODO: Fix this to now use fen, just use existing enums
        assert_eq!(board.get_piece_at_coordinate_as_fen("g1"), "q");  // lowercase for black pieces
    }
}
