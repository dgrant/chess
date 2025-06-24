pub static SPACE: &'static str = " ";
pub static A: &'static str = "a";
pub static B: &'static str = "b";
pub static C: &'static str = "c";
pub static D: &'static str = "d";
pub static E: &'static str = "e";
pub static F: &'static str = "f";
pub static G: &'static str = "g";
pub static H: &'static str = "h";

use crate::types::Square;

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
    pub fn to_unicode(&self) -> &'static str {
        match self {
            Piece::WhitePawn => "♙",
            Piece::WhiteRook => "♖",
            Piece::WhiteKnight => "♘",
            Piece::WhiteBishop => "♗",
            Piece::WhiteQueen => "♕",
            Piece::WhiteKing => "♔",
            Piece::BlackPawn => "♟",
            Piece::BlackRook => "♜",
            Piece::BlackKnight => "♞",
            Piece::BlackBishop => "♝",
            Piece::BlackQueen => "♛",
            Piece::BlackKing => "♚",
        }
    }
}

// Define a Move struct using the Square enum.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub target: Square,
}


// Example conversion of a move string into a Move struct.
impl TryFrom<&str> for Move {
    type Error = &'static str;
    fn try_from(mv: &str) -> Result<Self, Self::Error> {
        if mv.len() != 4 {
            return Err("Invalid move format");
        }
        let src = Square::try_from(&mv[0..2])?;
        let target = Square::try_from(&mv[2..4])?;
        Ok(Move { src, target })
    }
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

    /// State of the board
    pub any_white: u64,
    pub any_black: u64,
    pub empty: u64,
    pub side_to_move: Color
}

impl Board {

    /// Returns the Unicode character representation of the chess piece at the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - A string slice containing the algebraic chess notation coordinate (e.g., "e4", "a1")
    ///
    /// # Returns
    ///
    /// A static string slice containing the Unicode character representation of the piece
    /// or a space character if the square is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use chesslib::board::get_starting_board;
    /// let board = get_starting_board();
    ///
    /// assert_eq!(board.get_piece_at_coordinate("e1"), "♔"); // White king
    /// assert_eq!(board.get_piece_at_coordinate("d8"), "♛"); // Black queen
    /// assert_eq!(board.get_piece_at_coordinate("e4"), " "); // Empty square
    /// ```
    pub fn get_piece_at_coordinate(&self, coordinate: &str) -> &'static str {
        let bitboard_index = convert_coordinate_to_bitboard_index(coordinate);
        match self.get_piece_at_square(bitboard_index) {
            Some(piece) => piece.to_unicode(),
            None => SPACE
        }
    }

    fn get_piece_at_square(&self, square_index: u8) -> Option<Piece> {
        if is_bit_set(self.white_pawns, square_index) {
            Some(Piece::WhitePawn)
        } else if is_bit_set(self.black_pawns, square_index) {
            Some(Piece::BlackPawn)
        } else if is_bit_set(self.white_knights, square_index) {
            Some(Piece::WhiteKnight)
        } else if is_bit_set(self.black_knights, square_index) {
            Some(Piece::BlackKnight)
        } else if is_bit_set(self.white_bishops, square_index) {
            Some(Piece::WhiteBishop)
        } else if is_bit_set(self.black_bishops, square_index) {
            Some(Piece::BlackBishop)
        } else if is_bit_set(self.white_rooks, square_index) {
            Some(Piece::WhiteRook)
        } else if is_bit_set(self.black_rooks, square_index) {
            Some(Piece::BlackRook)
        } else if is_bit_set(self.white_queen, square_index) {
            Some(Piece::WhiteQueen)
        } else if is_bit_set(self.black_queen, square_index) {
            Some(Piece::BlackQueen)
        } else if is_bit_set(self.white_king, square_index) {
            Some(Piece::WhiteKing)
        } else if is_bit_set(self.black_king, square_index) {
            Some(Piece::BlackKing)
        } else {
            None
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        let from_bit = mv.src.to_bitboard();
        let to_bit = mv.target.to_bitboard();
        let src_idx = mv.src.to_bit_index();
        let target_idx = mv.target.to_bit_index();

        // Get the piece at the source square
        let piece = self.get_piece_at_square(src_idx);

        if let Some(piece) = piece {
            // First, if there's a piece on the target square, remove it from its bitboard
            if let Some(captured_piece) = self.get_piece_at_square(target_idx) {
                let captured_bitboard = match captured_piece {
                    Piece::WhitePawn => &mut self.white_pawns,
                    Piece::BlackPawn => &mut self.black_pawns,
                    Piece::WhiteRook => &mut self.white_rooks,
                    Piece::WhiteKnight => &mut self.white_knights,
                    Piece::WhiteBishop => &mut self.white_bishops,
                    Piece::WhiteQueen => &mut self.white_queen,
                    Piece::WhiteKing => &mut self.white_king,
                    Piece::BlackRook => &mut self.black_rooks,
                    Piece::BlackKnight => &mut self.black_knights,
                    Piece::BlackBishop => &mut self.black_bishops,
                    Piece::BlackQueen => &mut self.black_queen,
                    Piece::BlackKing => &mut self.black_king,
                };
                *captured_bitboard &= !to_bit;  // Clear the captured piece's bit
            }

            // Then move the piece from source to target
            let piece_bitboard = match piece {
                Piece::WhitePawn => &mut self.white_pawns,
                Piece::BlackPawn => &mut self.black_pawns,
                Piece::WhiteRook => &mut self.white_rooks,
                Piece::WhiteKnight => &mut self.white_knights,
                Piece::WhiteBishop => &mut self.white_bishops,
                Piece::WhiteQueen => &mut self.white_queen,
                Piece::WhiteKing => &mut self.white_king,
                Piece::BlackRook => &mut self.black_rooks,
                Piece::BlackKnight => &mut self.black_knights,
                Piece::BlackBishop => &mut self.black_bishops,
                Piece::BlackQueen => &mut self.black_queen,
                Piece::BlackKing => &mut self.black_king,
            };
            *piece_bitboard ^= from_bit;  // Clear the source square
            *piece_bitboard |= to_bit;    // Set the target square

            self.update_composite_bitboards();
            self.side_to_move = match self.side_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }
    }

    /// Updates the composite bitboards that represent the state of the board.
    /// This includes the combined bitboards for all white pieces, all black pieces,
    /// and the empty squares.
    /// This method should be called after any change to the individual piece bitboards
    /// to ensure that the composite bitboards reflect the current state of the board.
    fn update_composite_bitboards(&mut self) {
        self.any_white = self.white_pawns | self.white_knights | self.white_bishops |
                        self.white_rooks | self.white_queen | self.white_king;
        self.any_black = self.black_pawns | self.black_knights | self.black_bishops |
                        self.black_rooks | self.black_queen | self.black_king;
        self.empty = !(self.any_white | self.any_black);
    }

    pub fn apply_move_from_string(&mut self, mv_str: &str) {
        if let Ok(mv) = Move::try_from(mv_str) {
            self.apply_move(&mv);
        }
    }

    pub fn apply_moves(&mut self, moves: impl Iterator<Item = Move>) {
        for mv in moves {
            self.apply_move(&mv);
        }
    }

    pub fn apply_moves_from_strings(&mut self, moves: impl Iterator<Item = String>) {
        for mv in moves {
            self.apply_move_from_string(&mv);
        }
    }

    pub fn convert_moves(moves: impl Iterator<Item = String>) -> impl Iterator<Item = Result<Move, &'static str>> {
        moves.map(|mv| Move::try_from(mv.as_str()))
    }

    pub fn get_next_move(&self) -> String {
        use crate::move_generation::{w_pawns_able_to_push, b_pawns_able_to_push,
                                   w_pawns_able_to_double_push, b_pawns_able_to_double_push,
                                   w_pawns_attack_targets, b_pawns_attack_targets,
                                   knight_legal_moves, knight_attack_targets};
        use rand::seq::IteratorRandom;

        if self.side_to_move == Color::Black {
            // Get all possible pawn moves
            let moveable_pawns = b_pawns_able_to_push(self.black_pawns, self.empty);
            let double_moveable_pawns = b_pawns_able_to_double_push(self.black_pawns, self.empty);
            let attacking_pawns = b_pawns_attack_targets(self.black_pawns, self.any_white);

            let mut possible_moves = bitboard_to_pawn_single_moves(moveable_pawns, true);
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, true));
            possible_moves.extend(bitboard_to_pawn_capture_moves(self.black_pawns, attacking_pawns, true));

            // Process each black knight separately
            let mut working_knights = self.black_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the processed bit

                let single_knight = 1u64 << knight_pos;
                // Get legal moves and attacks for this specific knight
                let moves = knight_legal_moves(single_knight, self.any_black);
                let attacks = knight_attack_targets(single_knight, self.any_white);

                possible_moves.extend(self.bitboard_to_moves(single_knight, moves));
                possible_moves.extend(self.bitboard_to_moves(single_knight, attacks));
            }

            possible_moves.into_iter().choose(&mut rand::thread_rng())
                .expect("No moves found for black, which should be impossible in current state")
        } else {
            // Get all possible pawn moves
            let moveable_pawns = w_pawns_able_to_push(self.white_pawns, self.empty);
            let double_moveable_pawns = w_pawns_able_to_double_push(self.white_pawns, self.empty);
            let attacking_pawns = w_pawns_attack_targets(self.white_pawns, self.any_black);

            let mut possible_moves = bitboard_to_pawn_single_moves(moveable_pawns, false);
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, false));
            possible_moves.extend(bitboard_to_pawn_capture_moves(self.white_pawns, attacking_pawns, false));

            // Process each white knight separately
            let mut working_knights = self.white_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the processed bit

                let single_knight = 1u64 << knight_pos;
                // Get legal moves and attacks for this specific knight
                let moves = knight_legal_moves(single_knight, self.any_white);
                let attacks = knight_attack_targets(single_knight, self.any_black);

                possible_moves.extend(self.bitboard_to_moves(single_knight, moves));
                possible_moves.extend(self.bitboard_to_moves(single_knight, attacks));
            }

            possible_moves.into_iter().choose(&mut rand::thread_rng())
                .expect("No moves found for white, which should be impossible in current state")
        }
    }

    // Generic helper function to convert a source bitboard and target bitboard into a list of moves
    fn bitboard_to_moves(&self, source_pieces: u64, target_squares: u64) -> Vec<String> {
        // Assert that source_pieces contains exactly one piece (one bit set)
        debug_assert_eq!(source_pieces.count_ones(), 1,
            "bitboard_to_moves should be called with exactly one source piece, got {} pieces",
            source_pieces.count_ones());

        let mut moves = Vec::new();
        let mut working_source = source_pieces;

        // For each source piece
        while working_source != 0 {
            let from_square = working_source.trailing_zeros() as u8;
            working_source &= working_source - 1;  // Clear the processed bit

            // For each target square
            let mut current_targets = target_squares;
            while current_targets != 0 {
                let to_square = current_targets.trailing_zeros() as u8;
                current_targets &= current_targets - 1;  // Clear the processed bit

                // Convert to algebraic notation
                let from_file = int_file_to_string(from_square % 8);
                let from_rank = (from_square / 8 + 1).to_string();
                let to_file = int_file_to_string(to_square % 8);
                let to_rank = (to_square / 8 + 1).to_string();

                moves.push(format!("{}{}{}{}", from_file, from_rank, to_file, to_rank));
            }
        }

        moves
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
        side_to_move: Color::White
    };
    board.update_composite_bitboards();
    board
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
        _ => SPACE
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

pub fn bitboard_to_pawn_single_moves(bitboard: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = 1 << (rank * 8 + file);
            if bitboard & square != 0 {
                let to_rank = if is_black {
                    rank - 1 // Black pawns move downward by decreasing rank
                } else {
                    rank + 1 // White pawns move upward by increasing rank
                };
                let from = format!("{}{}", int_file_to_string(file), rank + 1);
                let to = format!("{}{}", int_file_to_string(file), to_rank + 1);
                moves.push(format!("{}{}", from, to));
            }
        }
    }
    moves
}

pub fn bitboard_to_pawn_double_moves(bitboard: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = 1 << (rank * 8 + file);
            if bitboard & square != 0 {
                let from = format!("{}{}", int_file_to_string(file), rank + 1);
                let to_rank = if is_black {
                    rank - 2 // Black pawns move down two ranks
                } else {
                    rank + 2 // White pawns move up two ranks
                };
                let to = format!("{}{}", int_file_to_string(file), to_rank + 1);
                moves.push(format!("{}{}", from, to));
            }
        }
    }
    moves
}

pub fn bitboard_to_pawn_capture_moves(from_bitboard: u64, target_bitboard: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    let mut working_board = target_bitboard;

    while working_board != 0 {
        // Get the target square (least significant 1-bit)
        let to_square = working_board.trailing_zeros() as u8;
        // Clear the processed bit
        working_board &= working_board - 1;

        // Find the source pawn that can attack this square
        let from_square = if is_black {
            // Check both possible source squares for black pawns (one rank up, one file left or right)
            let possible_from_east = to_square + 7;
            let possible_from_west = to_square + 9;
            if from_bitboard & (1 << possible_from_east) != 0 {
                possible_from_east
            } else {
                possible_from_west
            }
        } else {
            // Check both possible source squares for white pawns (one rank down, one file left or right)
            let possible_from_east = to_square - 9;
            let possible_from_west = to_square - 7;
            if from_bitboard & (1 << possible_from_east) != 0 {
                possible_from_east
            } else {
                possible_from_west
            }
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
        assert_eq!(board.get_piece_at_coordinate("a2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("b2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("c2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("d2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("e2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("f2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("g2"), "♙");
        assert_eq!(board.get_piece_at_coordinate("h2"), "♙");

        // Back rank pieces
        assert_eq!(board.get_piece_at_coordinate("a1"), "♖");
        assert_eq!(board.get_piece_at_coordinate("h1"), "♖");
        assert_eq!(board.get_piece_at_coordinate("b1"), "♘");
        assert_eq!(board.get_piece_at_coordinate("g1"), "♘");
        assert_eq!(board.get_piece_at_coordinate("c1"), "♗");
        assert_eq!(board.get_piece_at_coordinate("f1"), "♗");
        assert_eq!(board.get_piece_at_coordinate("d1"), "♕");
        assert_eq!(board.get_piece_at_coordinate("e1"), "♔");
    }

    #[test]
    fn test_starting_board_black_piece_positions() {
        let board = get_starting_board();

        // Black pieces
        // Pawns on rank 7
        assert_eq!(board.get_piece_at_coordinate("a7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("b7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("c7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("d7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("e7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("f7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("g7"), "♟");
        assert_eq!(board.get_piece_at_coordinate("h7"), "♟");

        // Back rank pieces
        assert_eq!(board.get_piece_at_coordinate("a8"), "♜");
        assert_eq!(board.get_piece_at_coordinate("h8"), "♜");
        assert_eq!(board.get_piece_at_coordinate("b8"), "♞");
        assert_eq!(board.get_piece_at_coordinate("g8"), "♞");
        assert_eq!(board.get_piece_at_coordinate("c8"), "♝");
        assert_eq!(board.get_piece_at_coordinate("f8"), "♝");
        assert_eq!(board.get_piece_at_coordinate("d8"), "♛");
        assert_eq!(board.get_piece_at_coordinate("e8"), "♚");
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
        board.apply_move(&Move { src: Square::E2, target: Square::E4 });
        assert!(is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e4")));
        assert!(!is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e2")));

        // Test moving a black pawn from d7 to d5
        board.apply_move(&Move { src: Square::D7, target: Square::D5 });
        assert!(is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d5")));
        assert!(!is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d7")));

        // Verify empty squares are updated correctly
        assert_eq!(board.empty, !(board.any_white | board.any_black));
    }

    #[test]
    fn test_side_to_move_after_sequence() {
        let mut board = get_starting_board();
        assert_eq!(board.side_to_move, Color::White);

        // First move: White e2e4
        board.apply_move(&Move { src: Square::E2, target: Square::E4 });
        assert_eq!(board.side_to_move, Color::Black);

        // Second move: Black d7d6
        board.apply_move(&Move { src: Square::D7, target: Square::D6 });
        assert_eq!(board.side_to_move, Color::White);

        // Third move: White g2g4
        board.apply_move(&Move { src: Square::G2, target: Square::G4 });
        assert_eq!(board.side_to_move, Color::Black);

        // Get next move - should suggest a black move
        let next_move = board.get_next_move();
        // Now allow for both pawn and knight moves
        assert!(next_move.starts_with("a7") || next_move.starts_with("b7") ||
               next_move.starts_with("c7") || next_move.starts_with("d6") ||
               next_move.starts_with("e7") || next_move.starts_with("f7") ||
               next_move.starts_with("g7") || next_move.starts_with("h7") ||
               next_move.starts_with("b8") || next_move.starts_with("g8"),
               "Move {} should be a black pawn or knight move", next_move);
    }

    #[test]
    fn test_knight_moves() {
        let mut board = get_starting_board();

        // Move white knight from b1 to c3
        board.apply_move(&Move { src: Square::B1, target: Square::C3 });
        assert!(is_bit_set(board.white_knights, convert_coordinate_to_bitboard_index("c3")));
        assert!(!is_bit_set(board.white_knights, convert_coordinate_to_bitboard_index("b1")));
        assert_eq!(board.side_to_move, Color::Black);

        // Move black knight from g8 to f6
        board.apply_move(&Move { src: Square::G8, target: Square::F6 });
        assert!(is_bit_set(board.black_knights, convert_coordinate_to_bitboard_index("f6")));
        assert!(!is_bit_set(board.black_knights, convert_coordinate_to_bitboard_index("g8")));
        assert_eq!(board.side_to_move, Color::White);

        // Test a capture: white knight takes black pawn
        board.apply_move(&Move { src: Square::C3, target: Square::D5 });
        assert!(is_bit_set(board.white_knights, convert_coordinate_to_bitboard_index("d5")));
        assert!(!is_bit_set(board.white_knights, convert_coordinate_to_bitboard_index("c3")));
        assert!(!is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d5")));
    }

    #[test]
    fn test_bitboard_to_moves() {
        let board = get_starting_board();

        // Test with a single source and multiple targets
        let source = 1u64 << convert_coordinate_to_bitboard_index("e4");  // Knight on e4
        let targets = (1u64 << convert_coordinate_to_bitboard_index("f6")) |  // Target squares f6, d6, c5
                     (1u64 << convert_coordinate_to_bitboard_index("d6")) |
                     (1u64 << convert_coordinate_to_bitboard_index("c5"));

        let moves = board.bitboard_to_moves(source, targets);

        // Verify the moves are generated correctly
        assert!(moves.contains(&"e4f6".to_string()));
        assert!(moves.contains(&"e4d6".to_string()));
        assert!(moves.contains(&"e4c5".to_string()));
        assert_eq!(moves.len(), 3);

        // Test with a different single source and single target
        let g1_source = 1u64 << convert_coordinate_to_bitboard_index("g1");  // Knight on g1
        let f3_target = 1u64 << convert_coordinate_to_bitboard_index("f3");  // Target square f3

        let moves = board.bitboard_to_moves(g1_source, f3_target);

        // Verify move is generated correctly
        assert!(moves.contains(&"g1f3".to_string()));
        assert_eq!(moves.len(), 1);

        // Test b1 knight separately
        let b1_source = 1u64 << convert_coordinate_to_bitboard_index("b1");  // Knight on b1
        let c3_target = 1u64 << convert_coordinate_to_bitboard_index("c3");  // Target square c3

        let moves = board.bitboard_to_moves(b1_source, c3_target);

        // Verify move is generated correctly
        assert!(moves.contains(&"b1c3".to_string()));
        assert_eq!(moves.len(), 1);

        // Test with no target squares (should produce empty move list)
        assert!(board.bitboard_to_moves(source, 0).is_empty());
    }
}
