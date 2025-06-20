extern crate chesslib;
use chesslib::board::{get_starting_board, convert_coordinate_to_bitboard_index, is_bit_set};
use chesslib::move_generation::{w_single_push_targets, w_double_push_targets, b_single_push_targets, b_double_push_targets, w_pawns_able_to_push, w_pawns_able_to_double_push, b_pawns_able_to_push, b_pawns_able_to_double_push};

#[test]
fn test_initial_board_pawns() {
    let board = get_starting_board();

    // Test white pawns are in correct position (second rank)
    for file in 0..8 {
        let index = convert_coordinate_to_bitboard_index(&format!("{}{}", ('a' as u8 + file) as char, 2));
        assert!(is_bit_set(board.white_pawns, index), "White pawn should be present at {}{}", ('a' as u8 + file) as char, 2);
    }

    // Test black pawns are in correct position (seventh rank)
    for file in 0..8 {
        let index = convert_coordinate_to_bitboard_index(&format!("{}{}", ('a' as u8 + file) as char, 7));
        assert!(is_bit_set(board.black_pawns, index), "Black pawn should be present at {}{}", ('a' as u8 + file) as char, 7);
    }
}

#[test]
fn test_coordinate_conversion() {
    assert_eq!(convert_coordinate_to_bitboard_index("a1"), 0);
    assert_eq!(convert_coordinate_to_bitboard_index("h1"), 7);
    assert_eq!(convert_coordinate_to_bitboard_index("a8"), 56);
    assert_eq!(convert_coordinate_to_bitboard_index("h8"), 63);
    assert_eq!(convert_coordinate_to_bitboard_index("e4"), 28);
}



#[test]
fn test_w_single_push_targets() {
    let board = get_starting_board();

    // Test white single push targets
    let white_single_push = w_single_push_targets(board.white_pawns, board.empty);
    let expected_white_single_push = 0x0000000000FF0000;
    let diff_white_single_push = white_single_push ^ expected_white_single_push;
    assert!(diff_white_single_push == 0, "White single push targets mismatch: diff = {:064b}", diff_white_single_push);
}

#[test]
fn test_w_double_push_targets() {
    let board = get_starting_board();

    // Test white double push targets
    let white_double_push = w_double_push_targets(board.white_pawns, board.empty);
    let expected_white_double_push = 0x00000000FF000000;
    let diff_white_double_push = white_double_push ^ expected_white_double_push;
    assert!(diff_white_double_push == 0, "White double push targets mismatch: diff = {:064b}", diff_white_double_push);
}

#[test]
fn test_b_single_push_targets() {
    let board = get_starting_board();

    // Test black single push targets
    let black_single_push = b_single_push_targets(board.black_pawns, board.empty);
    let expected_black_single_push = 0x0000FF0000000000; // Adjusted expected value
    let diff_black_single_push = black_single_push ^ expected_black_single_push;
    assert!(diff_black_single_push == 0, "Black single push targets mismatch: diff = {:064b}", diff_black_single_push);
}

#[test]
fn test_b_double_push_targets() {
    let board = get_starting_board();

    // Test black double push targets
    let black_double_push = b_double_push_targets(board.black_pawns, board.empty);
    let expected_black_double_push = 0x000000FF00000000; // Adjusted expected value
    let diff_black_double_push = black_double_push ^ expected_black_double_push;
    assert!(diff_black_double_push == 0, "Black double push targets mismatch: diff = {:064b}", diff_black_double_push);
}

#[test]
fn test_w_pawns_able_to_push() {
    let board = get_starting_board();

    // Test white pawns able to push
    let white_pawns_push = w_pawns_able_to_push(board.white_pawns, board.empty);
    let expected_white_pawns_push = 0x000000000000FF00;
    let diff_white_pawns_push = white_pawns_push ^ expected_white_pawns_push;
    assert!(diff_white_pawns_push == 0, "White pawns able to push mismatch: diff = {:064b}", diff_white_pawns_push);
}

#[test]
fn test_w_pawns_able_to_double_push() {
    let board = get_starting_board();

    // Test white pawns able to double push
    let white_pawns_double_push = w_pawns_able_to_double_push(board.white_pawns, board.empty);
    let expected_white_pawns_double_push = 0x000000000000FF00; // Adjusted expected value
    let diff_white_pawns_double_push = white_pawns_double_push ^ expected_white_pawns_double_push;
    assert!(diff_white_pawns_double_push == 0, "White pawns able to double push mismatch: diff = {:064b}", diff_white_pawns_double_push);
}

#[test]
fn test_b_pawns_able_to_push() {
    let board = get_starting_board();

    // Test black pawns able to push
    let black_pawns_push = b_pawns_able_to_push(board.black_pawns, board.empty);
    let expected_black_pawns_push = 0x00FF000000000000;
    let diff_black_pawns_push = black_pawns_push ^ expected_black_pawns_push;
    assert!(diff_black_pawns_push == 0, "Black pawns able to push mismatch: diff = {:064b}", diff_black_pawns_push);
}

#[test]
fn test_b_pawns_able_to_double_push() {
    let board = get_starting_board();

    // Test black pawns able to double push
    let black_pawns_double_push = b_pawns_able_to_double_push(board.black_pawns, board.empty);
    let expected_black_pawns_double_push = 0x00FF000000000000; // Adjusted expected value
    let diff_black_pawns_double_push = black_pawns_double_push ^ expected_black_pawns_double_push;
    assert!(diff_black_pawns_double_push == 0, "Black pawns able to double push mismatch: diff = {:064b}", diff_black_pawns_double_push);
}
