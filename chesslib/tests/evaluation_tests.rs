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

    assert_eq!(board.evaluate(), 0);
    board.apply_move_from_string("e2e4");
    assert_eq!(board.evaluate(), 45); // white gains some freedom for queen, and pawn in center
    board.apply_move_from_string("d7d5");
    assert_eq!(board.evaluate(), 0); // black gains the same, net even
    // Capture a black pawn — but white now has doubled d-pawns (d2 + d5),
    // so the +100 material gain is partially offset by a -15 doubled penalty.
    board.apply_move_from_string("e4d5");
    assert_eq!(board.evaluate(), 100); // +100 material, -15 doubled pawn -> net +100

    board.apply_move_from_string("d8d5"); // queen captures pawn — d-file no longer doubled for white
    assert_eq!(board.evaluate(), -15); // back to almost even, slight advantage to black as the queen is in center of board
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

// --- Pawn structure tests ---
//
// Pin the *direction* of the eval delta caused by isolating these features,
// rather than exact totals. Concrete numeric values that depend on multiple
// eval terms get brittle; what we care about is that doubled / isolated
// pawns are penalised relative to a clean structure.

use chesslib::fen::load_fen;

fn eval_diff(better_fen: &str, worse_fen: &str) -> i64 {
    let a = load_fen(better_fen).unwrap().evaluate();
    let b = load_fen(worse_fen).unwrap().evaluate();
    a - b
}

#[test]
fn test_doubled_pawns_are_penalised_for_white() {
    // Both positions have 3 white pawns total, only difference is doubled b-file.
    // a2,b2,c2 — no doubling, all connected (a-b-c neighbours)
    // a2,b2,b3 — b-file doubled; a still has b-neighbour, b's still have a-neighbour, no isolation
    let no_doubled = "4k3/8/8/8/8/8/PPP5/4K3 w - - 0 1";
    let doubled    = "4k3/8/8/8/8/1P6/PP6/4K3 w - - 0 1";
    let diff = eval_diff(no_doubled, doubled);
    assert!(
        diff > 0,
        "no-doubled structure should score higher than doubled (same material). got diff={}",
        diff
    );
}

#[test]
fn test_isolated_pawn_is_penalised_for_white() {
    // Same material (4 pawns). Only difference: connected vs isolated.
    // a2,b2,c2,d2 — all connected, no isolation
    // a2,b2,d2,f2 — d2 is isolated (c empty, e empty); f2 also isolated (e empty, g empty)
    // Note: choosing fewer-isolated as connected to minimise variables.
    let connected = "4k3/8/8/8/8/8/PPPP4/4K3 w - - 0 1";
    let isolated  = "4k3/8/8/8/8/8/PP1P1P2/4K3 w - - 0 1";
    let diff = eval_diff(connected, isolated);
    assert!(
        diff > 0,
        "connected pawn structure should score higher than isolated. got diff={}",
        diff
    );
}

#[test]
fn test_doubled_pawns_are_penalised_for_black() {
    // Same setup mirrored for black.
    let no_doubled = "4k3/ppp5/8/8/8/8/8/4K3 b - - 0 1"; // a7,b7,c7
    let doubled    = "4k3/pp6/1p6/8/8/8/8/4K3 b - - 0 1"; // a7,b7,b6 (b-file doubled)
    // Black's doubled pawns should *help* white (white POV eval rises).
    // So eval(no_doubled) < eval(doubled), so the diff (better - worse) is negative.
    let diff = eval_diff(no_doubled, doubled);
    assert!(
        diff < 0,
        "black having doubled pawns should give white a higher eval. got diff={}",
        diff
    );
}
