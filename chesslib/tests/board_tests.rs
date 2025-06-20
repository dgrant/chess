extern crate chesslib;
use chesslib::board::{get_starting_board, convert_coordinate_to_bitboard_index, is_bit_set};

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
