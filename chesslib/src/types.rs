#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn to_bit_index(&self) -> u8 {
        *self as u8
    }

    pub fn to_bitboard(&self) -> u64 {
        1u64 << self.to_bit_index()
    }
}

// Implement a conversion from a string coordinate (e.g., "a1") to a Square.
use std::convert::TryFrom;

impl TryFrom<&str> for Square {
    type Error = &'static str;
    fn try_from(coordinate: &str) -> Result<Self, Self::Error> {
        match coordinate {
            "a1" => Ok(Square::A1), "b1" => Ok(Square::B1), "c1" => Ok(Square::C1), "d1" => Ok(Square::D1),
            "e1" => Ok(Square::E1), "f1" => Ok(Square::F1), "g1" => Ok(Square::G1), "h1" => Ok(Square::H1),
            "a2" => Ok(Square::A2), "b2" => Ok(Square::B2), "c2" => Ok(Square::C2), "d2" => Ok(Square::D2),
            "e2" => Ok(Square::E2), "f2" => Ok(Square::F2), "g2" => Ok(Square::G2), "h2" => Ok(Square::H2),
            "a3" => Ok(Square::A3), "b3" => Ok(Square::B3), "c3" => Ok(Square::C3), "d3" => Ok(Square::D3),
            "e3" => Ok(Square::E3), "f3" => Ok(Square::F3), "g3" => Ok(Square::G3), "h3" => Ok(Square::H3),
            "a4" => Ok(Square::A4), "b4" => Ok(Square::B4), "c4" => Ok(Square::C4), "d4" => Ok(Square::D4),
            "e4" => Ok(Square::E4), "f4" => Ok(Square::F4), "g4" => Ok(Square::G4), "h4" => Ok(Square::H4),
            "a5" => Ok(Square::A5), "b5" => Ok(Square::B5), "c5" => Ok(Square::C5), "d5" => Ok(Square::D5),
            "e5" => Ok(Square::E5), "f5" => Ok(Square::F5), "g5" => Ok(Square::G5), "h5" => Ok(Square::H5),
            "a6" => Ok(Square::A6), "b6" => Ok(Square::B6), "c6" => Ok(Square::C6), "d6" => Ok(Square::D6),
            "e6" => Ok(Square::E6), "f6" => Ok(Square::F6), "g6" => Ok(Square::G6), "h6" => Ok(Square::H6),
            "a7" => Ok(Square::A7), "b7" => Ok(Square::B7), "c7" => Ok(Square::C7), "d7" => Ok(Square::D7),
            "e7" => Ok(Square::E7), "f7" => Ok(Square::F7), "g7" => Ok(Square::G7), "h7" => Ok(Square::H7),
            "a8" => Ok(Square::A8), "b8" => Ok(Square::B8), "c8" => Ok(Square::C8), "d8" => Ok(Square::D8),
            "e8" => Ok(Square::E8), "f8" => Ok(Square::F8), "g8" => Ok(Square::G8), "h8" => Ok(Square::H8),
            _ => Err("Invalid coordinate"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    White,
    Black,
}

/// Represents the castling rights for both players
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastlingRights {
    /// White kingside castling right (h1 rook)
    pub white_kingside: bool,
    /// White queenside castling right (a1 rook)
    pub white_queenside: bool,
    /// Black kingside castling right (h8 rook)
    pub black_kingside: bool,
    /// Black queenside castling right (a8 rook)
    pub black_queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        // At the start of a game, all castling is allowed
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl CastlingRights {
    /// Creates a new CastlingRights with all castling allowed
    pub fn new() -> Self {
        Self::default()
    }

    /// Remove castling rights for a given color's kingside
    pub fn remove_kingside(&mut self, is_white: bool) {
        if is_white {
            self.white_kingside = false;
        } else {
            self.black_kingside = false;
        }
    }

    /// Remove castling rights for a given color's queenside
    pub fn remove_queenside(&mut self, is_white: bool) {
        if is_white {
            self.white_queenside = false;
        } else {
            self.black_queenside = false;
        }
    }

    /// Remove all castling rights for a given color
    pub fn remove_all_for_color(&mut self, is_white: bool) {
        if is_white {
            self.white_kingside = false;
            self.white_queenside = false;
        } else {
            self.black_kingside = false;
            self.black_queenside = false;
        }
    }
}

pub static SPACE: &'static str = " ";
pub static A: &'static str = "a";
pub static B: &'static str = "b";
pub static C: &'static str = "c";
pub static D: &'static str = "d";
pub static E: &'static str = "e";
pub static F: &'static str = "f";
pub static G: &'static str = "g";
pub static H: &'static str = "h";

#[derive(PartialEq, Clone, Copy, Debug)]
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

    pub fn to_fen(&self) -> &'static str {
        match self {
            Piece::WhitePawn => "P",
            Piece::WhiteRook => "R",
            Piece::WhiteKnight => "N",
            Piece::WhiteBishop => "B",
            Piece::WhiteQueen => "Q",
            Piece::WhiteKing => "K",
            Piece::BlackPawn => "p",
            Piece::BlackRook => "r",
            Piece::BlackKnight => "n",
            Piece::BlackBishop => "b",
            Piece::BlackQueen => "q",
            Piece::BlackKing => "k",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn | Piece::WhiteRook | Piece::WhiteKnight |
            Piece::WhiteBishop | Piece::WhiteQueen | Piece::WhiteKing => Color::White,
            _ => Color::Black
        }
    }
}

// Define a Move struct using the Square enum.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub target: Square,
    pub promotion: Option<PieceType>, // Optional promotion piece type
}

// Example conversion of a move string into a Move struct.
impl TryFrom<&str> for Move {
    type Error = &'static str;
    fn try_from(mv: &str) -> Result<Self, Self::Error> {
        if mv.len() == 5 {
            // Promotion move, e.g., "e7e8q"
            let src = Square::try_from(&mv[0..2])?;
            let target = Square::try_from(&mv[2..4])?;
            let promotion_char = mv.chars().nth(4).unwrap();
            let promotion = match promotion_char {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'b' => Some(PieceType::Bishop),
                'n' => Some(PieceType::Knight),
                _ => return Err("Invalid promotion piece type"),
            };
            return Ok(Move { src, target, promotion })
        }
        else if mv.len() != 4 {
            return Err("Move string must be exactly 4 characters long unless it's a promotion move");
        }
        let src = Square::try_from(&mv[0..2])?;
        let target = Square::try_from(&mv[2..4])?;
        Ok(Move { src, target, promotion: None })
    }
}