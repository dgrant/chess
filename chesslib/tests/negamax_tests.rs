use chesslib::board::Board;
use chesslib::search::Searcher;
use chesslib::types::Square;

#[test]
fn test_negamax_basic_tactics() {
    // Test 1: Capture a hanging piece
    let mut board = Board::new();
    // Move white pawn to e4
    board.apply_move_from_string("f2f4");
    // Move black pawn to d5
    board.apply_move_from_string("e7e5");
    // White can capture the undefended pawn
    let (best_move, _score) = Searcher::new().find_best_move(&mut board, 3);
    assert!(best_move.is_some());
    assert_eq!(best_move.unwrap().to_string(), "f4e5");

    // Test 2: Avoid losing a piece
    let mut board = Board::new();
    // Setup a position where a piece needs to move away from attack
    board.apply_move_from_string("e2e4");
    board.apply_move_from_string("e7e5");
    board.apply_move_from_string("d2d4");
    board.apply_move_from_string("d7d6");
    board.apply_move_from_string("f1b5"); // Bishop to b5
    board.apply_move_from_string("c7c6"); // Threatens the bishop

    let (best_move, _score) = Searcher::new().find_best_move(&mut board, 3);
    assert!(best_move.is_some());
    // Bishop should move away to safety
    assert_eq!(best_move.unwrap().src, Square::B5);
}

#[test]
fn test_negamax_checkmate() {
    // Test a simple checkmate in one
    let mut board = Board::new();
    // Setup Scholar's mate position
    board.apply_move_from_string("e2e4");
    board.apply_move_from_string("e7e5");
    board.apply_move_from_string("d1h5");
    board.apply_move_from_string("b8c6");
    board.apply_move_from_string("f1c4");
    board.apply_move_from_string("g8f6");

    // White can checkmate with Qxf7#
    let (best_move, _score) = Searcher::new().find_best_move(&mut board, 3);
    assert!(best_move.is_some());
    assert_eq!(best_move.unwrap().to_string(), "h5f7");
}
