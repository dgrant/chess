use chesslib::board::Board;
use chesslib::types::BISHOP_VALUE;

#[test]
fn test_castling_bonus() {
    let mut board = Board::new();
    let initial_eval = board.evaluate();

    // Castle white kingside
    board.apply_move_from_string("e2e4");
    board.apply_move_from_string("e7e5");
    board.apply_move_from_string("g1f3");
    board.apply_move_from_string("b8c6");
    board.apply_move_from_string("f1e2");
    board.apply_move_from_string("g8f6");
    board.apply_move_from_string("e1g1"); // White castles kingside

    let after_white_castle = board.evaluate();
    assert!(after_white_castle > initial_eval, "Evaluation should increase after white castles");

    // Castle black kingside
    board.apply_move_from_string("f8e7");
    board.apply_move_from_string("d2d3");
    board.apply_move_from_string("e8g8"); // Black castles kingside

    let after_both_castle = board.evaluate();
    assert!(after_both_castle < after_white_castle, "Evaluation should decrease after black also castles");
}

#[test]
fn test_starting_position_evaluation() {
    let board = Board::new();
    assert_eq!(board.evaluate(), 0, "Starting position should be equal");
}

#[test]
fn test_material_advantage() {
    let mut board = Board::new();

    // Numbers updated after PSTs landed; relative directions all match expectations.
    assert_eq!(board.evaluate(), 0);
    board.apply_move_from_string("e2e4");
    assert_eq!(board.evaluate(), 32); // white pawn advance, PST bonus for centralized pawn
    board.apply_move_from_string("d7d5");
    assert_eq!(board.evaluate(), -3); // nearly mirrored
    // Capture a black pawn
    board.apply_move_from_string("e4d5");
    assert_eq!(board.evaluate(), 113); // white up a pawn after capture

    board.apply_move_from_string("d8d5"); // queen captures pawn
    assert_eq!(board.evaluate(), 12); // material restored; black's queen on d5 has slight PST cost
}

#[test]
fn test_bishop_pair_bonus() {
    let mut board = Board::new();

    // Set up a position where white has both bishops but black lost one
    board.apply_move_from_string("e2e4");
    board.apply_move_from_string("d7d5");
    board.apply_move_from_string("f1c4"); // Develop white bishop
    board.apply_move_from_string("c8d7"); // Develop black bishop
    board.apply_move_from_string("e4d5"); // Capture pawn
    board.apply_move_from_string("d7c6"); // Move black bishop
    board.apply_move_from_string("d5c6"); // Capture black bishop

    let eval = board.evaluate();
    // Should be worth more than just the bishop material difference
    assert!(eval > BISHOP_VALUE);
}
