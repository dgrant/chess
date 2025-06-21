use crate::board::Color::{White, Black};

pub static W_PAWN: &'static str = "♙";
pub static W_ROOK: &'static str = "♖";
pub static W_KNIGHT: &'static str = "♘";
pub static W_BISHOP: &'static str = "♗";
pub static W_QUEEN: &'static str = "♕";
pub static W_KING: &'static str = "♔";
pub static W_SPACE: &'static str = " ";

pub static B_PAWN: &'static str = "♟";
pub static B_ROOK: &'static str = "♜";
pub static B_KNIGHT: &'static str = "♞";
pub static B_BISHOP: &'static str = "♝";
pub static B_QUEEN: &'static str = "♛";
pub static B_KING: &'static str = "♚";
pub static B_SPACE: &'static str = " ";

pub static A: &'static str = "a";
pub static B: &'static str = "b";
pub static C: &'static str = "c";
pub static D: &'static str = "d";
pub static E: &'static str = "e";
pub static F: &'static str = "f";
pub static G: &'static str = "g";
pub static H: &'static str = "h";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn => White,
            Piece::WhiteRook => White,
            Piece::WhiteKnight => White,
            Piece::WhiteBishop => White,
            Piece::WhiteQueen => White,
            Piece::WhiteKing => White,
            Piece::BlackPawn => Black,
            Piece::BlackRook => Black,
            Piece::BlackKnight => Black,
            Piece::BlackBishop => Black,
            Piece::BlackQueen => Black,
            Piece::BlackKing => Black,
        }
    }
}

#[derive(PartialEq)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}


#[derive(Debug)]
pub struct Board {
    /// White pieces
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queen: u64,
    pub white_king: u64,
    /// Black pieces
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queen: u64,
    pub black_king: u64,
    pub any_white: u64,
    pub any_black: u64,
    pub empty: u64,
    pub side_to_move: Color
}

impl Board {
    // pub fn get_piece_at(&self, position: Square) -> &'static str {
    // }

    pub fn get_piece_at_coordinate(&self, coordinate: &str) -> &'static str {
        let bitboard_index = convert_coordinate_to_bitboard_index(coordinate);
        if is_bit_set(self.white_pawns, bitboard_index) {
            return W_PAWN;
        } else if is_bit_set(self.white_knights, bitboard_index) {
            return W_KNIGHT;
        } else if is_bit_set(self.white_bishops, bitboard_index) {
            return W_BISHOP;
        } else if is_bit_set(self.white_queen, bitboard_index) {
            return W_QUEEN;
        } else if is_bit_set(self.white_rooks, bitboard_index) {
            return W_ROOK;
        } else if is_bit_set(self.white_king, bitboard_index) {
            return W_KING;
        } else if is_bit_set(self.black_pawns, bitboard_index) {
            return B_PAWN;
        } else if is_bit_set(self.black_knights, bitboard_index) {
            return B_KNIGHT;
        } else if is_bit_set(self.black_bishops, bitboard_index) {
            return B_BISHOP;
        } else if is_bit_set(self.black_queen, bitboard_index) {
            return B_QUEEN;
        } else if is_bit_set(self.black_king, bitboard_index) {
            return B_KING;
        } else if is_bit_set(self.black_rooks, bitboard_index) {
            return B_ROOK;
        } else {
            return W_SPACE;
        }
    }

    pub fn apply_move(&mut self, mv: &str) {
        // Parse the move (e.g., "e2e4") and update the board state
        let from = convert_coordinate_to_bitboard_index(&mv[0..2]);
        let to = convert_coordinate_to_bitboard_index(&mv[2..4]);

        if is_bit_set(self.white_pawns, from) {
            self.white_pawns ^= 1 << from; // Remove pawn from the original square
            self.white_pawns |= 1 << to;  // Add pawn to the new square
        } else if is_bit_set(self.black_pawns, from) {
            self.black_pawns ^= 1 << from; // Remove pawn from the original square
            self.black_pawns |= 1 << to;  // Add pawn to the new square
        }

        self.empty = !(self.any_white | self.any_black); // Update empty squares

        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}


pub fn get_starting_board() -> Board {
    let white_pawns = (1 << (8 + 0)) + (1 << (8 + 1)) + (1 << (8 + 2)) + (1 << (8 + 3)) +
                     (1 << (8 + 4)) + (1 << (8 + 5)) + (1 << (8 + 6)) + (1 << (8 + 7));
    let white_knights = (1 << (0 + 1)) + (1 << (0 + 6));
    let white_bishops = (1 << (0 + 2)) + (1 << (0 + 5));
    let white_rooks = (1 << (0 + 0)) + (1 << (0 + 7));
    let white_queen = 1 << (0 + 3);
    let white_king = 1 << (0 + 4);
    let black_pawns = (1 << (6 * 8 + 0)) + (1 << (6 * 8 + 1)) + (1 << (6 * 8 + 2)) +
                     (1 << (6 * 8 + 3)) + (1 << (6 * 8 + 4)) + (1 << (6 * 8 + 5)) +
                     (1 << (6 * 8 + 6)) + (1 << (6 * 8 + 7));
    let black_knights = (1 << (7 * 8 + 1)) + (1 << (7 * 8 + 6));
    let black_bishops = (1 << (7 * 8 + 2)) + (1 << (7 * 8 + 5));
    let black_rooks = (1 << (7 * 8 + 0)) + (1 << (7 * 8 + 7));
    let black_queen = 1 << (7 * 8 + 3);
    let black_king = 1 << (7 * 8 + 4);

    let any_white = white_pawns | white_knights | white_bishops | white_rooks | white_queen | white_king;
    let any_black = black_pawns | black_knights | black_bishops | black_rooks | black_queen | black_king;
    let empty = !(any_white | any_black);

    Board {
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
        any_white,
        any_black,
        empty,
        side_to_move: Color::White
    }
}

pub fn file_name_to_int(file: &str) -> u8 {
    match file {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
//        TODO(dgrant): Handle this differently
        _ => 0
    }
}

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
//        TODO(dgrant): Handle this differently
        _ => W_SPACE
    }
}

/// Converts a coordinate like a1 into a bitboard index. ie. a1->0, h8->63
///
/// # Arguments
/// * `coordinate` - a chess board coordinate like f6
pub fn convert_coordinate_to_bitboard_index(coordinate: &str) -> u8 {
    let file = &coordinate[0..1];
    let file_number = file_name_to_int(file);
    let rank: u8 = (&coordinate[1..2]).parse().unwrap();
    return (rank - 1) * 8 + file_number;
}

pub fn is_bit_set(bitboard: u64, bit: u8) -> bool {
    (1 << bit) & bitboard != 0
}

pub fn board_to_string(board: &Board) -> String {
    let mut result = String::new();
    for rank in (0..8).rev() {
        for file in 0..8 {
            let coordinate = &format!("{}{}", int_file_to_string(file), (rank + 1).to_string());
            let piece = board.get_piece_at_coordinate(coordinate);
            result.push_str(piece);
        }
        result.push('\n');
    }
    result
}

pub fn print_board(board: &Board) {
    let board_string = board_to_string(board);
    print!("{}", board_string);
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

pub fn bitboard_to_moves(bitboard: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = 1 << (rank * 8 + file);
            if bitboard & square != 0 {
                let from = format!("{}{}", int_file_to_string(file), rank + 1);
                let to = if is_black {
                    format!("{}{}", int_file_to_string(file), rank) // Black pawns move downward
                } else {
                    format!("{}{}", int_file_to_string(file), rank + 2) // White pawns move upward
                };
                moves.push(format!("{}{}", from, to));
            }
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_board_side_to_move() {
        let board = get_starting_board();
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_starting_board_piece_counts() {
        let board = get_starting_board();

        // Count bits set in each piece bitboard
        let white_pawn_count = board.white_pawns.count_ones();
        let black_pawn_count = board.black_pawns.count_ones();
        let white_knight_count = board.white_knights.count_ones();
        let black_knight_count = board.black_knights.count_ones();

        assert_eq!(white_pawn_count, 8, "Should be 8 white pawns");
        assert_eq!(black_pawn_count, 8, "Should be 8 black pawns");
        assert_eq!(white_knight_count, 2, "Should be 2 white knights");
        assert_eq!(black_knight_count, 2, "Should be 2 black knights");
    }

    #[test]
    fn test_starting_board_white_piece_positions() {
        let board = get_starting_board();

        // White pieces
        // Pawns on rank 2
        assert_eq!(board.get_piece_at_coordinate("a2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("b2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("c2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("d2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("e2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("f2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("g2"), W_PAWN);
        assert_eq!(board.get_piece_at_coordinate("h2"), W_PAWN);

        // Back rank pieces
        assert_eq!(board.get_piece_at_coordinate("a1"), W_ROOK);
        assert_eq!(board.get_piece_at_coordinate("h1"), W_ROOK);
        assert_eq!(board.get_piece_at_coordinate("b1"), W_KNIGHT);
        assert_eq!(board.get_piece_at_coordinate("g1"), W_KNIGHT);
        assert_eq!(board.get_piece_at_coordinate("c1"), W_BISHOP);
        assert_eq!(board.get_piece_at_coordinate("f1"), W_BISHOP);
        assert_eq!(board.get_piece_at_coordinate("d1"), W_QUEEN);
        assert_eq!(board.get_piece_at_coordinate("e1"), W_KING);
    }

    #[test]
    fn test_starting_board_black_piece_positions() {
        let board = get_starting_board();

        // Black pieces
        // Pawns on rank 7
        assert_eq!(board.get_piece_at_coordinate("a7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("b7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("c7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("d7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("e7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("f7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("g7"), B_PAWN);
        assert_eq!(board.get_piece_at_coordinate("h7"), B_PAWN);

        // Back rank pieces
        assert_eq!(board.get_piece_at_coordinate("a8"), B_ROOK);
        assert_eq!(board.get_piece_at_coordinate("h8"), B_ROOK);
        assert_eq!(board.get_piece_at_coordinate("b8"), B_KNIGHT);
        assert_eq!(board.get_piece_at_coordinate("g8"), B_KNIGHT);
        assert_eq!(board.get_piece_at_coordinate("c8"), B_BISHOP);
        assert_eq!(board.get_piece_at_coordinate("f8"), B_BISHOP);
        assert_eq!(board.get_piece_at_coordinate("d8"), B_QUEEN);
        assert_eq!(board.get_piece_at_coordinate("e8"), B_KING);
    }

    #[test]
    fn test_starting_board_bitboard_representation() {
        let board = get_starting_board();

        // Verify exact bitboard representations
        assert_eq!(board.white_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
        assert_eq!(board.black_pawns, 0b0000000011111111000000000000000000000000000000000000000000000000);
    }

    #[test]
    fn test_coordinate_conversion() {
        // a file
        assert_eq!(convert_coordinate_to_bitboard_index("a1"), 0);
        assert_eq!(convert_coordinate_to_bitboard_index("a2"), 8);
        assert_eq!(convert_coordinate_to_bitboard_index("a3"), 16);
        assert_eq!(convert_coordinate_to_bitboard_index("a4"), 24);
        assert_eq!(convert_coordinate_to_bitboard_index("a5"), 32);
        assert_eq!(convert_coordinate_to_bitboard_index("a6"), 40);
        assert_eq!(convert_coordinate_to_bitboard_index("a7"), 48);
        assert_eq!(convert_coordinate_to_bitboard_index("a8"), 56);

        // Center squares
        assert_eq!(convert_coordinate_to_bitboard_index("d4"), 27);
        assert_eq!(convert_coordinate_to_bitboard_index("e4"), 28);
        assert_eq!(convert_coordinate_to_bitboard_index("d5"), 35);
        assert_eq!(convert_coordinate_to_bitboard_index("e5"), 36);

        // h file
        assert_eq!(convert_coordinate_to_bitboard_index("h8"), 63);
    }

    #[test]
    fn test_piece_colors() {
        assert_eq!(Piece::WhitePawn.color(), Color::White);
        assert_eq!(Piece::BlackPawn.color(), Color::Black);
        assert_eq!(Piece::WhiteKing.color(), Color::White);
        assert_eq!(Piece::BlackKing.color(), Color::Black);
    }

    #[test]
    fn test_complete_bitboard_representation() {
        let board = get_starting_board();

        // White pieces bitboard patterns
        assert_eq!(board.white_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
        assert_eq!(board.white_knights, 0b0000000000000000000000000000000000000000000000000000000001000010);
        assert_eq!(board.white_bishops, 0b0000000000000000000000000000000000000000000000000000000000100100);
        assert_eq!(board.white_rooks, 0b0000000000000000000000000000000000000000000000000000000010000001);
        assert_eq!(board.white_queen, 0b0000000000000000000000000000000000000000000000000000000000001000);
        assert_eq!(board.white_king, 0b0000000000000000000000000000000000000000000000000000000000010000);

        // Black pieces bitboard patterns
        assert_eq!(board.black_pawns, 0b0000000011111111000000000000000000000000000000000000000000000000);
        assert_eq!(board.black_knights, 0b0100001000000000000000000000000000000000000000000000000000000000);
        assert_eq!(board.black_bishops, 0b0010010000000000000000000000000000000000000000000000000000000000);
        assert_eq!(board.black_rooks, 0b1000000100000000000000000000000000000000000000000000000000000000);
        assert_eq!(board.black_queen, 0b0000100000000000000000000000000000000000000000000000000000000000);
        assert_eq!(board.black_king, 0b0001000000000000000000000000000000000000000000000000000000000000);

        // Composite bitboards
        assert_eq!(board.any_white,
            board.white_pawns | board.white_knights | board.white_bishops |
            board.white_rooks | board.white_queen | board.white_king);

        assert_eq!(board.any_black,
            board.black_pawns | board.black_knights | board.black_bishops |
            board.black_rooks | board.black_queen | board.black_king);

        assert_eq!(board.empty, !(board.any_white | board.any_black));

        // Verify the relationship between the bitboards
        assert_eq!(board.any_white & board.any_black, 0); // No overlap between white and black pieces
        assert_eq!(board.any_white | board.any_black | board.empty, !0u64); // All squares are accounted for
    }

    #[test]
    fn test_bitboard_layout_matches_chess_coordinates() {
        // This test verifies that the bit layout matches the chess board layout
        // according to the Wisconsin CS page convention

        // Test rank-by-rank layout (8 bits per rank)
        assert_eq!(convert_coordinate_to_bitboard_index("a1"), 0);
        assert_eq!(convert_coordinate_to_bitboard_index("h1"), 7);
        assert_eq!(convert_coordinate_to_bitboard_index("a2"), 8);
        assert_eq!(convert_coordinate_to_bitboard_index("h2"), 15);
        assert_eq!(convert_coordinate_to_bitboard_index("a8"), 56);
        assert_eq!(convert_coordinate_to_bitboard_index("h8"), 63);

        // Verify specific squares from the Wisconsin CS page examples
        // These are the bit positions as shown in their diagrams
        let board = get_starting_board();

        // White king is at e1 (bit 4)
        assert!(is_bit_set(board.white_king, 4));

        // Black queen is at d8 (bit 59)
        assert!(is_bit_set(board.black_queen, 59));
    }

    #[test]
    fn test_apply_move() {
        let mut board = get_starting_board();

        // Test moving a white pawn from e2 to e4
        board.apply_move("e2e4");
        assert!(is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e4")));
        assert!(!is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e2")));

        // Test moving a black pawn from d7 to d5
        board.apply_move("d7d5");
        assert!(is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d5")));
        assert!(!is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d7")));

        // Verify empty squares are updated correctly
        assert_eq!(board.empty, !(board.any_white | board.any_black));
    }
}
