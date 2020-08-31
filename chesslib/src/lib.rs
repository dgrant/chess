pub mod board;

#[cfg(test)]
mod tests {
    use board;

    #[test]
    fn convert_coordinate_to_bitboard_index_valid() {
        // a file
        assert_eq!(board::convert_coordinate_to_bitboard_index("a1"), 0);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a2"), 8);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a3"), 16);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a4"), 24);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a5"), 32);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a6"), 40);
        assert_eq!(board::convert_coordinate_to_bitboard_index("a7"), 48);

        assert_eq!(board::convert_coordinate_to_bitboard_index("a8"), 56);
        // Center squares
        assert_eq!(board::convert_coordinate_to_bitboard_index("d4"), 27);
        assert_eq!(board::convert_coordinate_to_bitboard_index("e4"), 28);
        assert_eq!(board::convert_coordinate_to_bitboard_index("d5"), 35);
        assert_eq!(board::convert_coordinate_to_bitboard_index("e5"), 36);

        // h file
        assert_eq!(board::convert_coordinate_to_bitboard_index("h8"), 63);
    }

    #[test]
    fn test() {
        let myboard = board::get_starting_board();
        assert_eq!(myboard.get_piece_at_coordinate("a2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("b2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("c2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("d2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("e2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("f2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("g2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("h2"), board::W_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("a1"), board::W_ROOK);
        assert_eq!(myboard.get_piece_at_coordinate("h1"), board::W_ROOK);
        assert_eq!(myboard.get_piece_at_coordinate("b1"), board::W_KNIGHT);
        assert_eq!(myboard.get_piece_at_coordinate("g1"), board::W_KNIGHT);
        assert_eq!(myboard.get_piece_at_coordinate("c1"), board::W_BISHOP);
        assert_eq!(myboard.get_piece_at_coordinate("f1"), board::W_BISHOP);
        assert_eq!(myboard.get_piece_at_coordinate("d1"), board::W_QUEEN);
        assert_eq!(myboard.get_piece_at_coordinate("e1"), board::W_KING);
        assert_eq!(myboard.get_piece_at_coordinate("b7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("c7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("d7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("e7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("a7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("f7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("g7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("h7"), board::B_PAWN);
        assert_eq!(myboard.get_piece_at_coordinate("a8"), board::B_ROOK);
        assert_eq!(myboard.get_piece_at_coordinate("h8"), board::B_ROOK);
        assert_eq!(myboard.get_piece_at_coordinate("b8"), board::B_KNIGHT);
        assert_eq!(myboard.get_piece_at_coordinate("g8"), board::B_KNIGHT);
        assert_eq!(myboard.get_piece_at_coordinate("c8"), board::B_BISHOP);
        assert_eq!(myboard.get_piece_at_coordinate("f8"), board::B_BISHOP);
        assert_eq!(myboard.get_piece_at_coordinate("d8"), board::B_QUEEN);
        assert_eq!(myboard.get_piece_at_coordinate("e8"), board::B_KING);

        board::print_board(&myboard);

        assert_eq!(myboard.w_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
        assert_eq!(myboard.b_pawns, 0b0000000011111111000000000000000000000000000000000000000000000000);
    }
}
