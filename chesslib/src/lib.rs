pub mod board;

#[cfg(test)]
mod tests {
    use board;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        let board = board::get_starting_board();
        assert_eq!(board::get_piece_at_coordinate(&board, "b2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "c2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "d2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "e2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "a2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "f2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "g2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&board, "h2"), board::W_PAWN);

        assert_eq!(board::get_piece_at_coordinate(&board, "a1"), board::W_ROOK);
        assert_eq!(board::get_piece_at_coordinate(&board, "h1"), board::W_ROOK);

        assert_eq!(board::get_piece_at_coordinate(&board, "b1"), board::W_KNIGHT);
        assert_eq!(board::get_piece_at_coordinate(&board, "g1"), board::W_KNIGHT);

        assert_eq!(board::get_piece_at_coordinate(&board, "c1"), board::W_BISHOP);
        assert_eq!(board::get_piece_at_coordinate(&board, "f1"), board::W_BISHOP);

        assert_eq!(board::get_piece_at_coordinate(&board, "d1"), board::W_QUEEN);
        assert_eq!(board::get_piece_at_coordinate(&board, "e1"), board::W_KING);

        board::print_board(&board);
    }
}
