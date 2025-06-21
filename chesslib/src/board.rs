use crate::board::Color::{White, Black};

pub static SPACE: &'static str = " ";
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

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Square {
    pub fn to_bit_index(&self) -> u8 {
        *self as u8
    }

    pub fn to_bitboard(&self) -> u64 {
        1u64 << self.to_bit_index()
    }
}

// Define a Move struct using the Square enum.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub target: Square,
}

// Implement a conversion from a string coordinate (e.g., "a1") to a Square.
use std::convert::TryFrom;

impl TryFrom<&str> for Square {
    type Error = &'static str;
    fn try_from(coordinate: &str) -> Result<Self, Self::Error> {
        match coordinate {
            "a1" => Ok(Square::A1),
            "b1" => Ok(Square::B1),
            "c1" => Ok(Square::C1),
            "d1" => Ok(Square::D1),
            "e1" => Ok(Square::E1),
            "f1" => Ok(Square::F1),
            "g1" => Ok(Square::G1),
            "h1" => Ok(Square::H1),
            "a2" => Ok(Square::A2),
            "b2" => Ok(Square::B2),
            "c2" => Ok(Square::C2),
            "d2" => Ok(Square::D2),
            "e2" => Ok(Square::E2),
            "f2" => Ok(Square::F2),
            "g2" => Ok(Square::G2),
            "h2" => Ok(Square::H2),
            "a3" => Ok(Square::A3),
            "b3" => Ok(Square::B3),
            "c3" => Ok(Square::C3),
            "d3" => Ok(Square::D3),
            "e3" => Ok(Square::E3),
            "f3" => Ok(Square::F3),
            "g3" => Ok(Square::G3),
            "h3" => Ok(Square::H3),
            "a4" => Ok(Square::A4),
            "b4" => Ok(Square::B4),
            "c4" => Ok(Square::C4),
            "d4" => Ok(Square::D4),
            "e4" => Ok(Square::E4),
            "f4" => Ok(Square::F4),
            "g4" => Ok(Square::G4),
            "h4" => Ok(Square::H4),
            "a5" => Ok(Square::A5),
            "b5" => Ok(Square::B5),
            "c5" => Ok(Square::C5),
            "d5" => Ok(Square::D5),
            "e5" => Ok(Square::E5),
            "f5" => Ok(Square::F5),
            "g5" => Ok(Square::G5),
            "h5" => Ok(Square::H5),
            "a6" => Ok(Square::A6),
            "b6" => Ok(Square::B6),
            "c6" => Ok(Square::C6),
            "d6" => Ok(Square::D6),
            "e6" => Ok(Square::E6),
            "f6" => Ok(Square::F6),
            "g6" => Ok(Square::G6),
            "h6" => Ok(Square::H6),
            "a7" => Ok(Square::A7),
            "b7" => Ok(Square::B7),
            "c7" => Ok(Square::C7),
            "d7" => Ok(Square::D7),
            "e7" => Ok(Square::E7),
            "f7" => Ok(Square::F7),
            "g7" => Ok(Square::G7),
            "h7" => Ok(Square::H7),
            "a8" => Ok(Square::A8),
            "b8" => Ok(Square::B8),
            "c8" => Ok(Square::C8),
            "d8" => Ok(Square::D8),
            "e8" => Ok(Square::E8),
            "f8" => Ok(Square::F8),
            "g8" => Ok(Square::G8),
            "h8" => Ok(Square::H8),
            _ => Err("Invalid coordinate"),
        }
    }
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

        // Find which piece is on the source square and move it
        if let Some(piece) = self.get_piece_at_square(src_idx) {
            let bitboard = match piece {
                Piece::WhitePawn => &mut self.white_pawns,
                Piece::WhiteKnight => &mut self.white_knights,
                Piece::WhiteBishop => &mut self.white_bishops,
                Piece::WhiteRook => &mut self.white_rooks,
                Piece::WhiteQueen => &mut self.white_queen,
                Piece::WhiteKing => &mut self.white_king,
                Piece::BlackPawn => &mut self.black_pawns,
                Piece::BlackKnight => &mut self.black_knights,
                Piece::BlackBishop => &mut self.black_bishops,
                Piece::BlackRook => &mut self.black_rooks,
                Piece::BlackQueen => &mut self.black_queen,
                Piece::BlackKing => &mut self.black_king,
            };
            *bitboard ^= from_bit;
            *bitboard |= to_bit;
        }

        self.update_composite_bitboards();
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
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
        use crate::move_generation::{w_pawns_able_to_push, b_pawns_able_to_push, w_pawns_able_to_double_push, b_pawns_able_to_double_push};
        use rand::seq::IteratorRandom;

        if self.side_to_move == Color::Black {
            let moveable_pawns = b_pawns_able_to_push(self.black_pawns, self.empty);
            let double_moveable_pawns = b_pawns_able_to_double_push(self.black_pawns, self.empty);
            
            let mut possible_moves = bitboard_to_pawn_single_moves(moveable_pawns, true);
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, true));
            
            possible_moves.into_iter().choose(&mut rand::thread_rng())
                .expect("No moves found for black, which should be impossible in current state")
        } else {
            let moveable_pawns = w_pawns_able_to_push(self.white_pawns, self.empty);
            let double_moveable_pawns = w_pawns_able_to_double_push(self.white_pawns, self.empty);
            
            let mut possible_moves = bitboard_to_pawn_single_moves(moveable_pawns, false);
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, false));
            
            possible_moves.into_iter().choose(&mut rand::thread_rng())
                .expect("No moves found for white, which should be impossible in current state")
        }
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

pub fn bitboard_to_pawn_single_moves(bitboard: u64, is_black: bool) -> Vec<String> {
    let mut moves = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = 1 << (rank * 8 + file);
            if bitboard & square != 0 {
                let from = format!("{}{}", int_file_to_string(file), rank + 1);
                let to_rank = if is_black {
                    rank - 1 // Black pawns move downward by decreasing rank
                } else {
                    rank + 1 // White pawns move upward by increasing rank
                };
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
        assert!(next_move.starts_with("a7") || next_move.starts_with("b7") || next_move.starts_with("c7") ||
               next_move.starts_with("d6") || next_move.starts_with("e7") || next_move.starts_with("f7") ||
               next_move.starts_with("g7") || next_move.starts_with("h7"),
               "Move {} should be a black pawn move", next_move);
    }
}
