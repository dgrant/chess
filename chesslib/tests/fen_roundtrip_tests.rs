// Round-trip tests for FEN serialization. Previously load_fen ignored
// the en-passant target, halfmove clock, and fullmove counter, and
// to_fen emitted hardcoded "-", 0, and 1 for them. These tests pin down
// the fix so a FEN string survives load → to_fen unchanged.

use chesslib::board_utils::get_starting_board;
use chesslib::fen::load_fen;
use chesslib::types::Square;

#[test]
fn load_fen_parses_en_passant_target() {
    // Position with active en passant on e6: white d5 pawn can play
    // d5xe6 capturing the black pawn that just pushed e7-e5.
    let fen = "4k3/8/8/3Pp3/8/8/8/4K3 w - e6 0 1";
    let b = load_fen(fen).expect("FEN should parse");
    assert_eq!(
        b.en_passant_target,
        Some(Square::try_from("e6").unwrap()),
        "en passant target should be parsed from FEN"
    );
}

#[test]
fn load_fen_parses_no_en_passant() {
    let fen = "4k3/8/8/3Pp3/8/8/8/4K3 w - - 0 1";
    let b = load_fen(fen).expect("FEN should parse");
    assert_eq!(b.en_passant_target, None);
}

#[test]
fn load_fen_parses_halfmove_and_fullmove_clocks() {
    // Halfmove clock 17, fullmove number 42 — arbitrary values.
    let fen = "4k3/8/8/8/8/8/8/4K3 w - - 17 42";
    let b = load_fen(fen).expect("FEN should parse");
    assert_eq!(b.halfmove_clock, 17, "halfmove clock should parse");
    assert_eq!(b.fullmove_number, 42, "fullmove number should parse");
}

#[test]
fn to_fen_emits_en_passant_target() {
    let mut b = get_starting_board();
    b.apply_move_from_string("e2e4");
    let fen = b.to_fen();
    assert!(
        fen.contains(" e3 "),
        "after 1.e4 the FEN should contain en-passant target e3, got: {}",
        fen
    );
}

#[test]
fn to_fen_emits_no_en_passant_when_none() {
    let b = get_starting_board();
    let fen = b.to_fen();
    assert!(
        fen.contains(" - 0 "),
        "starting position FEN should have '-' for en passant, got: {}",
        fen
    );
}

#[test]
fn to_fen_emits_halfmove_clock_after_quiet_moves() {
    let mut b = get_starting_board();
    // Knight moves are quiet (not pawn, not capture) — halfmove ticks up.
    b.apply_move_from_string("g1f3");
    b.apply_move_from_string("g8f6");
    let fen = b.to_fen();
    assert!(
        fen.ends_with(" 2 2"),
        "expected halfmove=2 fullmove=2 after 1.Nf3 Nf6, got tail of: {}",
        fen
    );
}

#[test]
fn to_fen_resets_halfmove_clock_on_pawn_move() {
    let mut b = get_starting_board();
    b.apply_move_from_string("g1f3"); // Nf3 — halfmove becomes 1
    b.apply_move_from_string("e7e5"); // pawn move — halfmove resets to 0
    let fen = b.to_fen();
    let trailing = fen.split_whitespace().rev().take(2).collect::<Vec<_>>();
    // [fullmove, halfmove] reversed
    assert_eq!(
        trailing,
        vec!["2", "0"],
        "after 1.Nf3 e5 expect halfmove=0 fullmove=2, got: {}",
        fen
    );
}

#[test]
fn fen_full_roundtrip_preserves_state() {
    // A canonical position with all interesting fields populated.
    let original = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    let b = load_fen(original).expect("FEN should parse");
    let emitted = b.to_fen();
    assert_eq!(emitted, original, "FEN round-trip should be identity");
}

#[test]
fn fen_roundtrip_with_en_passant() {
    let original = "rnbqkbnr/pp1ppppp/8/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3";
    let b = load_fen(original).expect("FEN should parse");
    let emitted = b.to_fen();
    assert_eq!(emitted, original, "FEN round-trip with en-passant target");
}
