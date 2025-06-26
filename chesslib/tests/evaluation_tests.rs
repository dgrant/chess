use chesslib::board::Board;
use chesslib::fen::load_fen;

#[test]
fn test_starting_position_evaluation() {
    let board = Board::new();
    assert_eq!(board.evaluate(), 0); // Starting position should be equal
}

#[test]
fn test_material_advantage() {
    // Position where White is up a queen
    let board = load_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
    assert_eq!(board.evaluate(), 900); // White is up a queen (900 centipawns)

    // Position where Black is up a rook
    let board = load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBNR w Kkq - 0 1").unwrap();
    assert_eq!(board.evaluate(), -500); // Black is up a rook (500 centipawns)

    // Position where White is up a bishop
    let board = load_fen("rnbqk1nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
    assert_eq!(board.evaluate(), 300); // White is up a bishop (300 centipawns)

    // Position where Black is up a knight
    let board = load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R b KQkq - 0 1").unwrap();
    assert_eq!(board.evaluate(), -300); // Black is up a knight (300 centipawns)
}

#[test]
fn test_pawn_counting() {
    // Position where White is up two pawns
    let board = load_fen("rnbqkbnr/pppp1p1p/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(board.evaluate(), 200); // White is up two pawns (200 centipawns)
}
