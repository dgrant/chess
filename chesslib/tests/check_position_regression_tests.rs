// Regression tests for the position originally noted in chesslib/TODO.md as
// "has trouble getting a next move":
//
//   rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1
//
// Black is in check from Qf7+. Only legal move is Kh8 (Kf8 attacked by
// queen, Kh7 attacked by Bg6, Kxf7 attacked by Bg6, no blocks/captures).
// After 1...Kh8 white has Qh7# (queen to h7, defended by Bg6, mate).
//
// Current state: engine reliably returns Kh8 at every depth and within
// any reasonable time budget. These tests pin that down.

use chesslib::board::Board;
use chesslib::fen::load_fen;
use chesslib::types::Square;

const FEN: &str = "rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1";

fn board() -> Board {
    load_fen(FEN).unwrap()
}

#[test]
fn black_is_in_check_from_qf7() {
    let b = board();
    assert!(b.black_king_in_check, "black should be in check");
}

#[test]
fn kh8_is_the_only_legal_move() {
    let mut b = board();
    let mut moves = Vec::new();
    b.get_all_raw_moves_append(&mut moves);
    assert_eq!(moves.len(), 1, "only Kh8 should be legal, got: {moves:?}");
    let m = &moves[0];
    assert_eq!(m.src, Square::G8);
    assert_eq!(m.target, Square::H8);
}

#[test]
fn engine_returns_kh8_at_every_depth() {
    for depth in 1..=4 {
        let mut b = board();
        let (mv, _score) = b.find_best_move(depth);
        let m = mv.unwrap_or_else(|| panic!("engine returned None at depth {depth}"));
        assert_eq!(
            (m.src, m.target),
            (Square::G8, Square::H8),
            "engine returned wrong move at depth {depth}: {m:?}"
        );
    }
}

#[test]
fn engine_returns_kh8_within_time_budget() {
    use std::time::Duration;
    let mut b = board();
    let (mv, _score, _depth) = b.find_best_move_within(Duration::from_millis(500));
    let m = mv.expect("engine should always return a move within budget");
    assert_eq!((m.src, m.target), (Square::G8, Square::H8));
}

/// After 1...Kh8 white has at least two mate-in-1 moves: Qh7# (queen to
/// h7, defended by Bg6) and Qf8# (queen to f8, blocked-back-rank mate
/// since Kh8 isn't adjacent to f8). At any search depth ≥ 1 the engine
/// must play one of those mating moves — never a non-mating move like
/// Nb1 that just delays.
///
/// Pins down the mate-distance fix: previously the eval returned
/// CHECKMATE_BONUS=100000 at any leaf where the side-to-move was mated
/// without depth/ply awareness. The search's own mate detection used
/// -30000+depth. The 100000 dominated and was constant, so the engine
/// couldn't tell mate-in-1 from mate-in-N. After the fix, mate scores
/// are ply-aware (closer mates score higher) and consistent across
/// eval and search.
#[test]
fn after_kh8_white_plays_a_mate_in_1_at_every_depth() {
    use chesslib::types::Move;

    let kh8 = Move {
        src: Square::G8,
        target: Square::H8,
        promotion: None,
    };

    let mating_targets = [Square::H7, Square::F8];

    for depth in 1..=4 {
        // Run a few times to defeat random tie-break flakiness.
        for _ in 0..6 {
            let mut b = board();
            b.apply_move(&kh8);
            let (mv, _score) = b.find_best_move(depth);
            let m = mv.unwrap();
            assert!(
                m.src == Square::F7 && mating_targets.contains(&m.target),
                "at depth {} white should play a mate-in-1 (Qh7# or Qf8#) but played {:?}->{:?}",
                depth,
                m.src,
                m.target
            );
        }
    }
}

/// After 1...Kh8 2.Qf8 IS checkmate: black king on h8 isn't adjacent to f8
/// (so Kxf8 is impossible); queen attacks h8 along rank 8 with g8 the only
/// square between (and nothing can block or capture there). Confirmed
/// against multiple engines.
#[test]
fn qf8_is_also_a_mate_in_1() {
    use chesslib::types::Move;
    let kh8 = Move {
        src: Square::G8,
        target: Square::H8,
        promotion: None,
    };
    let qf8 = Move {
        src: Square::F7,
        target: Square::F8,
        promotion: None,
    };
    let mut b = board();
    b.apply_move(&kh8);
    b.apply_move(&qf8);
    assert!(b.is_checkmate(), "Qf8# is mate");
}
