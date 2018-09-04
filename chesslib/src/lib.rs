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
        assert_eq!(board::get_piece_at_coordinate(&board, "b2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "c2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "d2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "e2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "a2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "f2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "g2"), "p");
        assert_eq!(board::get_piece_at_coordinate(&board, "h2"), "p");

        assert_eq!(board::get_piece_at_coordinate(&board, "a1"), "R");
        assert_eq!(board::get_piece_at_coordinate(&board, "h1"), "R");

        assert_eq!(board::get_piece_at_coordinate(&board, "b1"), "N");
        assert_eq!(board::get_piece_at_coordinate(&board, "g1"), "N");

        assert_eq!(board::get_piece_at_coordinate(&board, "c1"), "B");
        assert_eq!(board::get_piece_at_coordinate(&board, "f1"), "B");

        assert_eq!(board::get_piece_at_coordinate(&board, "d1"), "Q");
        assert_eq!(board::get_piece_at_coordinate(&board, "e1"), "K");
    }
}
