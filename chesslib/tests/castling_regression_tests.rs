// Regression: from this position black should never prefer Kf8 over O-O.
//
// Position: rn2k2r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 b kq - 0 1
//
// History: prior to MVV-LVA capture ordering, quiescence search, and the
// revert of buggy PST middlegame values, the engine picked Kf8 from this
// position — which loses castling rights for no positional gain. With
// the current search and eval, O-O (e8g8) ties for best at depth 4 and
// Kf8 is ~80cp worse. These tests pin that down so the bug can't quietly
// come back.

use chesslib::board::Board;
use chesslib::fen::load_fen;
use chesslib::types::{Move, Square};

const FEN: &str = "rn2k2r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 b kq - 0 1";

fn board() -> Board {
    load_fen(FEN).unwrap()
}

fn root_moves(b: &mut Board) -> Vec<Move> {
    let mut v = Vec::new();
    b.get_all_raw_moves_append(&mut v);
    v
}

#[test]
fn castling_rights_are_set() {
    let b = board();
    assert!(b.black_kingside_castle_rights);
    assert!(b.black_queenside_castle_rights);
    assert!(!b.black_king_in_check);
}

#[test]
fn kingside_castle_is_in_root_move_list() {
    let mut b = board();
    let moves = root_moves(&mut b);
    let castle = moves
        .iter()
        .find(|m| m.src == Square::E8 && m.target == Square::G8);
    assert!(castle.is_some(), "O-O must be a generated legal move");
}

/// At depth 4, the search-returned score for O-O (e8g8) must be at least
/// as good for black as the score for Kf8 (e8f8). Concretely: the
/// negamax-returned score for the position *after* O-O should be ≤ the
/// score after Kf8 (lower = better for black; scores reported from
/// white's POV).
#[test]
fn castle_scores_at_least_as_well_as_kf8() {
    fn score_after(mv: Move) -> i64 {
        let mut b = board();
        b.apply_move(&mv);
        let (_resp, score_for_white) = b.find_best_move(3);
        score_for_white
    }

    let castle = Move {
        src: Square::E8,
        target: Square::G8,
        promotion: None,
    };
    let kf8 = Move {
        src: Square::E8,
        target: Square::F8,
        promotion: None,
    };

    let s_castle = score_after(castle);
    let s_kf8 = score_after(kf8);

    assert!(
        s_castle <= s_kf8,
        "O-O (e8g8) should not score worse than Kf8 (e8f8): \
         s_castle={s_castle}, s_kf8={s_kf8}"
    );
}

/// Slightly stronger: at depth 4, the engine should not pick the bare
/// king walk e8f8 as best move. Three other moves (Nc6, Qb6, O-O) tie
/// for best at score 125; Kf8 is at 205. Random tie-break can't pick
/// Kf8 because it's not tied. Run multiple times to defeat any
/// stochastic flakiness.
#[test]
fn engine_does_not_pick_kf8_at_depth_4() {
    for _ in 0..5 {
        let mut b = board();
        let (mv, _score) = b.find_best_move(4);
        let mv = mv.unwrap();
        assert!(
            !(mv.src == Square::E8 && mv.target == Square::F8),
            "engine picked Kf8 — castling-vs-king-walk regression"
        );
    }
}
