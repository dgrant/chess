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
        let myboard = board::get_starting_board();
        assert_eq!(board::get_piece_at_coordinate(&myboard, "b2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "c2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "d2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "e2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "a2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "f2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "g2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "h2"), board::W_PAWN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "a1"), board::W_ROOK);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "h1"), board::W_ROOK);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "b1"), board::W_KNIGHT);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "g1"), board::W_KNIGHT);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "c1"), board::W_BISHOP);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "f1"), board::W_BISHOP);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "d1"), board::W_QUEEN);
        assert_eq!(board::get_piece_at_coordinate(&myboard, "e1"), board::W_KING);

        board::print_board(&myboard);

        assert_eq!(myboard.w_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
    }
}
