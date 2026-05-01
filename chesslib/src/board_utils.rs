use crate::board::Board;
use crate::types::{Color, Move, PieceType, Square, A, B, C, D, E, F, G, H};

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

/// An empty 8x8 board with no pieces placed and no castling rights —
/// the default starting point for `load_fen`, which then fills the
/// piece bitboards rank by rank.
pub fn get_empty_board() -> Board {
    Board {
        // No pieces yet; all six per-type and both per-colour bitboards
        // start at zero.
        pieces: [0; 6],
        colors: [0; 2],

        side_to_move: Color::White,
        white_king_in_check: false,
        black_king_in_check: false,

        // No castling rights for an empty board — they're set
        // explicitly by the FEN parser when it reads the castling
        // field.
        white_kingside_castle_rights: false,
        white_queenside_castle_rights: false,
        black_kingside_castle_rights: false,
        black_queenside_castle_rights: false,

        en_passant_target: None,
        halfmove_clock: 0,
        fullmove_number: 1,
        move_history: Vec::with_capacity(10),
        piece_map: [None; 64],
    }
}

/// The standard starting position of a chess game.
///
/// Bitboards are laid out so that bit 0 = a1, bit 7 = h1, bit 56 = a8,
/// bit 63 = h8 (little-endian rank-file mapping). Constants below match
/// that layout: white's back rank is on rank 1 (bits 0..8), pawns on
/// rank 2 (bits 8..16); black's back rank is on rank 8 (bits 56..64),
/// pawns on rank 7 (bits 48..56).
pub fn get_starting_board() -> Board {
    // White pieces on ranks 1-2.
    const WHITE_PAWNS: u64 = 0xFF << 8; // a2..h2
    const WHITE_ROOKS: u64 = (1 << 0) | (1 << 7); // a1, h1
    const WHITE_KNIGHTS: u64 = (1 << 1) | (1 << 6); // b1, g1
    const WHITE_BISHOPS: u64 = (1 << 2) | (1 << 5); // c1, f1
    const WHITE_QUEEN: u64 = 1 << 3; // d1
    const WHITE_KING: u64 = 1 << 4; // e1

    // Black pieces on ranks 7-8 (mirror of white, shifted by 56 bits).
    const BLACK_PAWNS: u64 = 0xFF << 48; // a7..h7
    const BLACK_ROOKS: u64 = (1 << 56) | (1 << 63); // a8, h8
    const BLACK_KNIGHTS: u64 = (1 << 57) | (1 << 62); // b8, g8
    const BLACK_BISHOPS: u64 = (1 << 58) | (1 << 61); // c8, f8
    const BLACK_QUEEN: u64 = 1 << 59; // d8
    const BLACK_KING: u64 = 1 << 60; // e8

    // pieces[pt] holds both colours' pieces of type pt.
    let mut pieces = [0u64; 6];
    pieces[PieceType::Pawn.idx()] = WHITE_PAWNS | BLACK_PAWNS;
    pieces[PieceType::Knight.idx()] = WHITE_KNIGHTS | BLACK_KNIGHTS;
    pieces[PieceType::Bishop.idx()] = WHITE_BISHOPS | BLACK_BISHOPS;
    pieces[PieceType::Rook.idx()] = WHITE_ROOKS | BLACK_ROOKS;
    pieces[PieceType::Queen.idx()] = WHITE_QUEEN | BLACK_QUEEN;
    pieces[PieceType::King.idx()] = WHITE_KING | BLACK_KING;

    // colors[c] holds every piece of colour c regardless of type.
    let mut colors = [0u64; 2];
    colors[Color::White.idx()] =
        WHITE_PAWNS | WHITE_KNIGHTS | WHITE_BISHOPS | WHITE_ROOKS | WHITE_QUEEN | WHITE_KING;
    colors[Color::Black.idx()] =
        BLACK_PAWNS | BLACK_KNIGHTS | BLACK_BISHOPS | BLACK_ROOKS | BLACK_QUEEN | BLACK_KING;

    let mut board = Board {
        pieces,
        colors,
        side_to_move: Color::White,
        white_king_in_check: false,
        black_king_in_check: false,
        // All castling rights present at the start of a normal game.
        white_kingside_castle_rights: true,
        white_queenside_castle_rights: true,
        black_kingside_castle_rights: true,
        black_queenside_castle_rights: true,
        en_passant_target: None,
        halfmove_clock: 0,
        fullmove_number: 1,
        move_history: Vec::new(),
        piece_map: [None; 64],
    };
    // The bitboards above are correct; the mailbox `piece_map` mirrors
    // them so the per-square lookup `get_piece_at_square_fast` works.
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
            let square = 1u64 << (rank * 8 + file); // Use 1u64 to ensure 64-bit shift
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
