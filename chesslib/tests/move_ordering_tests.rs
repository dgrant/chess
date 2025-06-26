use chesslib::board::Board;
use chesslib::fen::load_fen;
use chesslib::Square;
use chesslib::types::Move;

// #[test]
// fn test_move_ordering_captures_first_white() {
//     // Position with multiple captures available
//     // White can capture black's queen or a pawn
//     let fen = "rnb1k2r/ppp2ppp/3p4/4n3/1b1qP3/2N5/PPP2PPP/R1B1KBNR w KQkq - 0 1";
//     let mut board = load_fen(fen).unwrap();
//
//     let moves = board.get_raw_moves(-1); // Get all moves in sorted order
//
//     // First moves should be captures (queen capture should be first)
//     let mut found_queen_capture = false;
//     let mut found_pawn_capture = false;
//     let mut first_non_capture_idx = 0;
//
//     for (i, mv) in moves.iter().enumerate() {
//         if board.get_piece_at_square_fast(mv.target.to_bit_index()).is_some() {
//             if board.get_piece_at_square_fast(mv.target.to_bit_index()).unwrap().to_fen() == "q" {
//                 found_queen_capture = true;
//                 // Queen capture should be one of the first moves
//                 assert!(i < 3, "Queen capture should be among the first moves");
//             }
//             if board.get_piece_at_square_fast(mv.target.to_bit_index()).unwrap().to_fen() == "p" {
//                 found_pawn_capture = true;
//             }
//         } else if first_non_capture_idx == 0 {
//             first_non_capture_idx = i;
//         }
//     }
//
//     assert!(found_queen_capture, "Should find queen capture");
//     assert!(found_pawn_capture, "Should find pawn capture");
//     assert!(first_non_capture_idx > 0, "Captures should come before non-captures");
// }

#[test]
fn test_move_ordering_captures_first_black() {
    // Similar position but from black's perspective
    let fen = "rnb1k2r/ppp2ppp/3p4/4n3/1b1qP3/2N5/PPPB1PPP/R3KBNR b KQkq - 0 1";
    let mut board = load_fen(fen).unwrap();

    let moves = board.get_next_moves(-1); // Get all moves in sorted order

    // Assert that these are in the first 4 moves, in this order:
    // d4d2 (queen takes bishop and leaves center and checks king)
    // b4c3 (bishop takes knight)
    // d4c3 (queen takes knight and leaves center)
    // d4e4 (queen takes pawn and stays in center, and checks king)
    // d4f2 (queen takes pawn and leaves center and checks king)
    // e5d3 (knight checks king and leaves center)
    // e5f3 (knight checks king and leaves center)
    // d4e3 (queen checks king and leaves center)
    // d6d5 (pawn pushes to center)
    assert_eq!(moves[0], "d4d2");
    assert_eq!(moves[1], "b4c3");

}

#[test]
fn test_move_ordering_center_control() {
    // Test that moves to control center squares are preferred over other quiet moves
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = load_fen(fen).unwrap();

    let moves = board.get_raw_moves(-1);

    // Find moves to e4 and d4
    let e4_moves: Vec<&Move> = moves.iter()
        .filter(|m| m.target == Square::try_from("e4").unwrap())
        .collect();
    let d4_moves: Vec<&Move> = moves.iter()
        .filter(|m| m.target == Square::try_from("d4").unwrap())
        .collect();

    assert!(!e4_moves.is_empty(), "Should find e4 move");
    assert!(!d4_moves.is_empty(), "Should find d4 move");

    // These central moves should be among the first moves (after any captures)
    let e4_index = moves.iter().position(|m| m.target == Square::try_from("e4").unwrap()).unwrap();
    let d4_index = moves.iter().position(|m| m.target == Square::try_from("d4").unwrap()).unwrap();

    // Should be in the first half of all moves
    assert!(e4_index < moves.len() / 2, "e4 should be among the first moves");
    assert!(d4_index < moves.len() / 2, "d4 should be among the first moves");
}

#[test]
fn test_move_selection_count() {
    let mut board = Board::new();

    // Test selecting different numbers of moves
    let no_moves = board.get_raw_moves(0);
    assert!(no_moves.is_empty(), "Should get no moves when n=0");

    let one_move = board.get_raw_moves(1);
    assert_eq!(one_move.len(), 1, "Should get exactly one move when n=1");

    let five_moves = board.get_raw_moves(5);
    assert_eq!(five_moves.len(), 5, "Should get exactly five moves when n=5");

    let all_moves = board.get_raw_moves(-1);
    assert!(all_moves.len() > 15, "Should get all legal moves when n=-1");
}

#[test]
fn test_check_ordering() {
    // Position where white can give check
    let fen = "rnbqkbnr/ppp3pp/3p1p2/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
    let mut board = load_fen(fen).unwrap();

    let moves = board.get_raw_moves(-1);

    // Find moves that give check (Qh5+ should be among them)
    let mut found_check_move = false;

    for (i, mv) in moves.iter().enumerate() {
        // Apply move and see if it gives check
        board.apply_move(mv);
        if board.black_king_in_check {
            found_check_move = true;
            // Check-giving moves should be among the first moves
            assert!(i < moves.len() / 2, "Check-giving moves should be among the first moves");
        }
        board.undo_last_move();
    }

    assert!(found_check_move, "Should find at least one check-giving move");
}
