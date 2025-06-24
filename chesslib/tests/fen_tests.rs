use chesslib::board_utils::get_starting_board;

#[test]
fn test_starting_position_fen() {
    let board = get_starting_board();
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}
