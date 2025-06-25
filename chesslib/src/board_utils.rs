use crate::board::Board;
use crate::types::{Color, A, B, C, D, E, F, G, H};

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

/// Convert a source and target bitboard (each with exactly one bit set) to a move string
pub fn bitboard_squares_to_move(src: u64, target: u64) -> String {
    // Get indices
    let src_idx = src.trailing_zeros() as u8;
    let target_idx = target.trailing_zeros() as u8;

    // Convert to algebraic notation
    let from_file = int_file_to_string(src_idx % 8);
    let from_rank = (src_idx / 8 + 1).to_string();
    let to_file = int_file_to_string(target_idx % 8);
    let to_rank = (target_idx / 8 + 1).to_string();

    format!("{}{}{}{}", from_file, from_rank, to_file, to_rank)
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
        move_history: Vec::new(),
    };
    board.update_composite_bitboards();
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
    };
    board.update_composite_bitboards();
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
            let square = 1 << (rank * 8 + file);
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

pub fn bitboard_to_pawn_single_moves(moveable_pawns: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    bitboard_to_pawn_single_moves_append(&mut moves, moveable_pawns, is_black);
    moves
}

/// Convert a bitboard of pawn single moves into a list of move strings
pub fn bitboard_to_pawn_single_moves_append(possible_moves: &mut Vec<String>, moveable_pawns: u64, is_black: bool) {
    let mut working_pawns = moveable_pawns;

    while working_pawns != 0 {
        let from_square = working_pawns.trailing_zeros() as u8;
        working_pawns &= working_pawns - 1;  // Clear the processed bit

        let to_square = if is_black {
            from_square - 8  // Black pawns move down
        } else {
            from_square + 8  // White pawns move up
        };

        // Convert indices to coordinates (0-7)
        let (from_file, from_rank) = (from_square & 7, from_square >> 3);
        let (to_file, to_rank) = (to_square & 7, to_square >> 3);

        if (is_black && to_rank == 0) || (!is_black && to_rank == 7) {
            // Pre-allocate string with enough capacity for longest possible move
            let mut move_str = String::with_capacity(5);
            move_str.push_str(int_file_to_string(from_file));
            move_str.push((from_rank + b'1') as char);
            move_str.push_str(int_file_to_string(to_file));
            move_str.push((to_rank + b'1') as char);

            // Reuse the base move string for all promotions
            for promotion in ['q', 'r', 'b', 'n'] {
                let mut promoted = move_str.clone();
                promoted.push(promotion);
                possible_moves.push(promoted);
            }
        } else {
            let mut move_str = String::with_capacity(4);
            move_str.push_str(int_file_to_string(from_file));
            move_str.push((from_rank + b'1') as char);
            move_str.push_str(int_file_to_string(to_file));
            move_str.push((to_rank + b'1') as char);
            possible_moves.push(move_str);
        }
    }

}

/// Convert a bitboard of pawn double moves into a list of move strings
pub fn bitboard_to_pawn_double_moves(moveable_pawns: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    let mut working_pawns = moveable_pawns;

    while working_pawns != 0 {
        let from_square = working_pawns.trailing_zeros() as u8;
        working_pawns &= working_pawns - 1;  // Clear the processed bit

        let to_square = if is_black {
            from_square - 16  // Black pawns move down two squares
        } else {
            from_square + 16  // White pawns move up two squares
        };

        // Convert to algebraic notation
        let from_file = int_file_to_string(from_square % 8);
        let from_rank = (from_square / 8 + 1).to_string();
        let to_file = int_file_to_string(to_square % 8);
        let to_rank = (to_square / 8 + 1).to_string();

        moves.push(format!("{}{}{}{}", from_file, from_rank, to_file, to_rank));
    }

    moves
}

/// Convert a bitboard of pawn capture moves into a list of move strings
pub fn bitboard_to_pawn_capture_moves(source_pawns: u64, target_squares: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    let mut working_pawns = source_pawns;

    // For each pawn
    while working_pawns != 0 {
        let from_square = working_pawns.trailing_zeros() as u8;
        working_pawns &= working_pawns - 1;  // Clear the processed bit

        // Get this pawn's possible captures
        let pawn = 1u64 << from_square;
        let mut captures = if is_black {
            crate::move_generation::b_pawns_attack_targets(pawn, target_squares)
        } else {
            crate::move_generation::w_pawns_attack_targets(pawn, target_squares)
        };

        // For each capture
        while captures != 0 {
            let to_square = captures.trailing_zeros() as u8;
            captures &= captures - 1;  // Clear the processed bit

            // Convert to algebraic notation
            let from_file = int_file_to_string(from_square % 8);
            let from_rank = (from_square / 8 + 1).to_string();
            let to_file = int_file_to_string(to_square % 8);
            let to_rank_int = to_square / 8 + 1;
            let to_rank = to_rank_int.to_string();

            if (is_black && to_rank_int == 1) || (!is_black && to_rank_int == 8) {
                moves.push(format!("{}{}{}{}q", from_file, from_rank, to_file, to_rank));
                moves.push(format!("{}{}{}{}r", from_file, from_rank, to_file, to_rank));
                moves.push(format!("{}{}{}{}b", from_file, from_rank, to_file, to_rank));
                moves.push(format!("{}{}{}{}n", from_file, from_rank, to_file, to_rank));
            } else {
                moves.push(format!("{}{}{}{}", from_file, from_rank, to_file, to_rank));
            }
        }
    }

    moves
}
