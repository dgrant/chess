use chesslib::board_utils::get_starting_board;
use chesslib::fen::load_fen;

#[test]
fn test_to_fen_starting_position() {
    let board = get_starting_board();
    assert_eq!(
        board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}

#[test]
fn test_load_fen_starting_position() {
    let board = get_starting_board();
    assert_eq!(
        load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
        board
    );
}

#[test]
fn test_random_positions() {
    use std::panic::AssertUnwindSafe;
    let mut board = get_starting_board();
    for _ in 0..10000 {
        while board.move_history.len() < 600 && !board.is_checkmate() && !board.is_stalemate() {
            let next_move_str =
                std::panic::catch_unwind(AssertUnwindSafe(|| board.get_next_move_random()));
            match next_move_str {
                Ok(mv) => board.apply_move_from_string(&*mv),
                Err(_) => panic!("get_next_move panicked, board: {}", board.to_fen()),
            }
        }
        assert_eq!(
            load_fen(board.to_fen().as_str()).unwrap(),
            board,
            "Failed with this position: {}",
            board.to_fen()
        );
    }
}
