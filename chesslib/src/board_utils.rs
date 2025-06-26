use crate::board::Board;
use crate::types::{Color, Move, Square, A, B, C, D, E, F, G, H};

pub fn int_file_to_string(file: u8) -> &'static str {
    match file {
        0 => A,
        1 => B,
        2 => C,
        3 => D,
        4 => E,
        5 => F,
        6 => G,
        7 => H,
        _ => panic!("Invalid file number"),
    }
}

/// Convert a source and target bitboard (each with exactly one bit set) to a move
pub fn bitboard_squares_to_move(src: u64, target: u64) -> Move {
    // Get indices
    let src_idx = src.trailing_zeros() as u8;
    let target_idx = target.trailing_zeros() as u8;
    Move {
        src: Square::from_bit_index(src_idx),
        target: Square::from_bit_index(target_idx),
        promotion: None,
    }
}

pub fn get_empty_board() -> Board {
    let white_pawns = 0;
    let white_knights = 0;
    let white_bishops = 0;
    let white_rooks = 0;
    let white_queen = 0;
    let white_king = 0;
    let black_pawns = 0;
    let black_knights = 0;
    let black_bishops = 0;
    let black_rooks = 0;
    let black_queen = 0;
    let black_king = 0;

    let mut board = Board {
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queen,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queen,
        black_king,
        any_white: 0,
        any_black: 0,
        empty: 0,
        side_to_move: Color::White,
        white_king_in_check: false,
        black_king_in_check: false,
        white_kingside_castle_rights: false,
        white_queenside_castle_rights: false,
        black_kingside_castle_rights: false,
        black_queenside_castle_rights: false,
        en_passant_target: None,
        move_history: Vec::with_capacity(10),
        piece_map: [None; 64],
    };
    board.update_composite_bitboards();
    board.rebuild_piece_map();
    board
}

pub fn get_starting_board() -> Board {
    let white_pawns = (1 << (8 + 0)) + (1 << (8 + 1)) + (1 << (8 + 2)) + (1 << (8 + 3)) +
                     (1 << (8 + 4)) + (1 << (8 + 5)) + (1 << (8 + 6)) + (1 << (8 + 7));
    let white_knights = (1 << (0 + 1)) + (1 << (0 + 6));
    let white_bishops = (1 << (0 + 2)) + (1 << (0 + 5));
    let white_rooks = (1 << (0 + 0)) + (1 << (0 + 7));
    let white_queen = 1 << (0 + 3);
    let white_king = 1 << (0 + 4);

    let black_pawns = (1 << (48 + 0)) + (1 << (48 + 1)) + (1 << (48 + 2)) + (1 << (48 + 3)) +
                     (1 << (48 + 4)) + (1 << (48 + 5)) + (1 << (48 + 6)) + (1 << (48 + 7));
    let black_knights = (1 << (56 + 1)) + (1 << (56 + 6));
    let black_bishops = (1 << (56 + 2)) + (1 << (56 + 5));
    let black_rooks = (1 << (56 + 0)) + (1 << (56 + 7));
    let black_queen = 1 << (56 + 3);
    let black_king = 1 << (56 + 4);

    let mut board = Board {
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queen,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queen,
        black_king,
        any_white: 0,
        any_black: 0,
        empty: 0,
        side_to_move: Color::White,
        white_king_in_check: false,
        black_king_in_check: false,
        white_kingside_castle_rights: true,
        white_queenside_castle_rights: true,
        black_kingside_castle_rights: true,
        black_queenside_castle_rights: true,
        en_passant_target: None,
        move_history: Vec::new(),
        piece_map: [None; 64],
    };
    board.update_composite_bitboards();
    board.rebuild_piece_map();
    board
}

/// Returns true if the given bit is set in the bitboard
pub fn is_bit_set(bitboard: u64, bit: u8) -> bool {
    (bitboard & (1u64 << bit)) != 0
}

pub fn bitboard_to_string(bitboard: u64) -> String {
    let mut result = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = 1u64 << (rank * 8 + file);  // Use 1u64 to ensure 64-bit shift
            if bitboard & square != 0 {
                result.push('1'); // Occupied square
            } else {
                result.push('.'); // Empty square
            }
        }
        result.push('\n'); // Newline after each rank
    }
    result
}

