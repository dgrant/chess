use board::Color::{White, Black};
use board;

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
            board::Piece::WhitePawn => White,
            board::Piece::WhiteRook => White,
            board::Piece::WhiteKnight => White,
            board::Piece::WhiteBishop => White,
            board::Piece::WhiteQueen => White,
            board::Piece::WhiteKing => White,
            board::Piece::BlackPawn => Black,
            board::Piece::BlackRook => Black,
            board::Piece::BlackKnight => Black,
            board::Piece::BlackBishop => Black,
            board::Piece::BlackQueen => Black,
            board::Piece::BlackKing => Black,
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


pub struct Board {
    /// White pieces
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queen: u64,
    pub white_king: u64,
    pub any_white: u64,

    /// Black pieces
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queen: u64,
    pub black_king: u64,
    pub any_black: u64,

    pub empty: u64,
    pub occupied: u64,
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
}


pub fn get_starting_board() -> Board {
    Board {
        white_pawns: (1 << (8 + 0)) + (1 << (8 + 1)) + (1 << (8 + 2)) + (1 << (8 + 3)) + (1 << (8 + 4)) + (1 << (8 + 5)) + (1 << (8 + 6)) + (1 << (8 + 7)),
        white_knights: (1 << (0 + 1)) + (1 << (0 + 6)),
        white_bishops: (1 << (0 + 2)) + (1 << (0 + 5)),
        white_rooks: (1 << (0 + 0)) + (1 << (0 + 7)),
        white_queen: (1 << (0 + 3)),
        white_king: (1 << (0 + 4)),
        black_pawns: (1 << (6 * 8 + 0)) + (1 << (6 * 8 + 1)) + (1 << (6 * 8 + 2)) + (1 << (6 * 8 + 3)) + (1 << (6 * 8 + 4)) + (1 << (6 * 8 + 5)) + (1 << (6 * 8 + 6)) + (1 << (6 * 8 + 7)),
        black_knights: (1 << (7 * 8 + 1)) + (1 << (7 * 8 + 6)),
        black_bishops: (1 << (7 * 8 + 2)) + (1 << (7 * 8 + 5)),
        black_rooks: (1 << (7 * 8 + 0)) + (1 << (7 * 8 + 7)),
        black_queen: (1 << (7 * 8 + 3)),
        black_king: (1 << (7 * 8 + 4)),
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

pub fn print_board(_board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let coordinate = &format!("{}{}", int_file_to_string(file), (rank + 1).to_string());
//            print!("coordinate:{}\n", coordinate);
            let piece = _board.get_piece_at_coordinate(coordinate);
            print!("{}", piece);
        }
        print!("\n");
    }
}