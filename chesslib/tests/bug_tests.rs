// Make a test for this position:
// rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1

use chesslib::fen::load_fen;

#[test]
pub fn test_bug_position() {
    let mut board = load_fen("rn4k1/pppb1Q2/6B1/6p1/1P6/P1N5/1BPq2PP/R3R2K b - - 0 1").unwrap();
    let (mv, _score) = board.find_best_move(2);
    assert_eq!(mv.unwrap().to_string(), "g8h8");
}

#[test]
pub fn test_bug_position2() {
    // I think we've fixed the bug now... previously the castling bonus was not being applied correctly
    // Eval values updated after PSTs landed (eval baseline shifted up by
    // ~70 cp due to per-piece positional bonuses). Relative order between
    // moves is what matters: castling (e8g8) is still by far the best
    // option for black; e8f8 (king walk) is a clear regression.
    let mut board = load_fen("rn2k2r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 b kq - 0 1").unwrap();
    let score = board.evaluate();
    assert_eq!(score, 216);
    let board_after_e8f8 = load_fen("rn3k1r/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 w - - 0 1").unwrap();
    let score_after_e8f8 = board_after_e8f8.evaluate();
    assert_eq!(score_after_e8f8, 292); // worse for black (king walked, lost castling)

    let board_after_e8g8 = load_fen("rn3rk1/ppp2ppp/4bn2/q1b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 w - - 0 1").unwrap();
    let score_after_e8g8 = board_after_e8g8.evaluate();
    assert_eq!(score_after_e8g8, 132); // best for black: castled bonus + king PST g1 reward

    // Moving the queen in line with bishop, attacking the king
    let board_after_a5b6 = load_fen("rn2k2r/ppp2pp1/1q2bn1p/2b1N3/8/2NB4/PPPP1PPP/R1BQR1K1 w kq - 0 1").unwrap();
    let score_after_a5b6 = board_after_a5b6.evaluate();
    assert_eq!(score_after_a5b6, 195);

    // Pre-quiescence, the engine picked e8g8 (castling) because the static eval
    // gives it a +35cp bonus over the start position. With quiescence resolving
    // exchanges past the depth-4 horizon, the engine now finds tactical play
    // from a5b6 (queen attacks the kingside diagonal) that outweighs the
    // castling bonus.
    let (mv, _score) = board.find_best_move(4);
    let mv_str = mv.unwrap().to_string();
    assert!(
        mv_str == "e8g8" || mv_str == "a5b6",
        "expected castling or the quiescence-found queen attack, got {}",
        mv_str
    );
}