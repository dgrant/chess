use chesslib::board::Board;
use chesslib::fen::load_fen;

#[test]
fn test_board_equality() {
    let board1 = Board::new();
    let board2 = Board::new();

    // Two newly created boards should be equal
    assert_eq!(board1, board2);

    // After a move, boards should be different
    let mut board3 = Board::new();
    board3.apply_move_from_string("e2e4");
    assert_ne!(board1, board3);

    // Loading same FEN should create equal boards
    let board4 = load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let board5 = load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(board4, board5);
}
