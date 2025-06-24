pub static SPACE: &'static str = " ";
pub static A: &'static str = "a";
pub static B: &'static str = "b";
pub static C: &'static str = "c";
pub static D: &'static str = "d";
pub static E: &'static str = "e";
pub static F: &'static str = "f";
pub static G: &'static str = "g";
pub static H: &'static str = "h";

use crate::types::{Color, Square};
use crate::move_generation::{
    b_pawns_attack_targets, bishop_legal_moves,
    king_legal_moves, knight_legal_moves,
    rook_legal_moves,
    w_pawns_attack_targets,
};

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


#[derive(Debug, Clone)]
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
    pub side_to_move: Color,
    pub white_king_in_check: bool,
    pub black_king_in_check: bool,
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
    pub fn get_piece_at_coordinate_as_unicode(&self, coordinate: &str) -> &'static str {
        let bitboard_index = Square::try_from(coordinate).unwrap().to_bit_index();
        match self.get_piece_at_square(bitboard_index) {
            Some(piece) => piece.to_unicode(),
            None => SPACE
        }
    }

    pub fn get_piece_at_coordinate_as_fen(&self, coordinate: &str) -> &'static str {
        let bitboard_index = Square::try_from(coordinate).unwrap().to_bit_index();
        match self.get_piece_at_square(bitboard_index) {
            Some(piece) => piece.to_fen(),
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
            // Validate that the piece being moved belongs to the side that has the turn
            if piece.color() != self.side_to_move {
                panic!("Attempted to move a {:?} piece during {:?}'s turn",
                    piece.color(), self.side_to_move);
            }

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
            self.update_check_state();
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
    pub fn update_composite_bitboards(&mut self) {
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

    pub fn get_next_moves(&self, n: i32) -> Vec<String> {
        use crate::move_generation::{w_pawns_able_to_push, b_pawns_able_to_push,
                                   w_pawns_able_to_double_push, b_pawns_able_to_double_push,
                                   w_pawns_attack_targets, b_pawns_attack_targets,
                                   knight_legal_moves, bishop_legal_moves, rook_legal_moves,
                                   queen_legal_moves, king_legal_moves};
        use rand::seq::IteratorRandom;

        let mut possible_moves = Vec::new();

        if self.side_to_move == Color::Black {
            // Get all possible pawn moves
            let moveable_pawns = b_pawns_able_to_push(self.black_pawns, self.empty);
            let double_moveable_pawns = b_pawns_able_to_double_push(self.black_pawns, self.empty);
            let attacking_pawns = b_pawns_attack_targets(self.black_pawns, self.any_white);

            possible_moves.extend(bitboard_to_pawn_single_moves(moveable_pawns, true));
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, true));
            possible_moves.extend(bitboard_to_pawn_capture_moves(self.black_pawns, attacking_pawns, true));

            // Process each black knight separately
            let mut working_knights = self.black_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the processed bit

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_black);
                possible_moves.extend(self.bitboard_to_moves(single_knight, moves));
            }

            // Process each black bishop separately
            let mut working_bishops = self.black_bishops;
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;

                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_legal_moves(single_bishop, self.any_black, self.any_white);
                possible_moves.extend(self.bitboard_to_moves(single_bishop, moves));
            }

            // Process each black rook separately
            let mut working_rooks = self.black_rooks;
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;

                let single_rook = 1u64 << rook_pos;
                let moves = rook_legal_moves(single_rook, self.any_black, self.any_white);
                possible_moves.extend(self.bitboard_to_moves(single_rook, moves));
            }

            // Process each black queen separately (usually just one)
            let mut working_queens = self.black_queen;
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;

                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_black, self.any_white);
                possible_moves.extend(self.bitboard_to_moves(single_queen, moves));
            }

            // Process black king (only one)
            let moves = king_legal_moves(self.black_king, self.any_black);
            possible_moves.extend(self.bitboard_to_moves(self.black_king, moves));

        } else {
            // Get all possible pawn moves
            let moveable_pawns = w_pawns_able_to_push(self.white_pawns, self.empty);
            let double_moveable_pawns = w_pawns_able_to_double_push(self.white_pawns, self.empty);
            let attacking_pawns = w_pawns_attack_targets(self.white_pawns, self.any_black);

            possible_moves.extend(bitboard_to_pawn_single_moves(moveable_pawns, false));
            possible_moves.extend(bitboard_to_pawn_double_moves(double_moveable_pawns, false));
            possible_moves.extend(bitboard_to_pawn_capture_moves(self.white_pawns, attacking_pawns, false));

            // Process each white knight separately
            let mut working_knights = self.white_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the bit we are processing, the lowest significant bit that is set

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_white);
                possible_moves.extend(self.bitboard_to_moves(single_knight, moves));
            }

            // Process each white bishop separately
            let mut working_bishops = self.white_bishops;
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;

                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_legal_moves(single_bishop, self.any_white, self.any_black);
                possible_moves.extend(self.bitboard_to_moves(single_bishop, moves));
            }

            // Process each white rook separately
            let mut working_rooks = self.white_rooks;
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;

                let single_rook = 1u64 << rook_pos;
                let moves = rook_legal_moves(single_rook, self.any_white, self.any_black);
                possible_moves.extend(self.bitboard_to_moves(single_rook, moves));
            }

            // Process each white queen separately (usually just one)
            let mut working_queens = self.white_queen;
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;

                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_white, self.any_black);
                possible_moves.extend(self.bitboard_to_moves(single_queen, moves));
            }

            // Process white king (only one)
            let moves = king_legal_moves(self.white_king, self.any_white);
            possible_moves.extend(self.bitboard_to_moves(self.white_king, moves));
        }

        // Filter the moves to only include legal ones (that get out of check if we're in check)
        let legal_moves: Vec<String> = possible_moves.into_iter()
            .filter(|mv_str| {
                if let Ok(mv) = Move::try_from(mv_str.as_str()) {
                    self.is_legal_move(&mv)
                } else {
                    false
                }
            })
            .collect();

        if n == -1 {
            legal_moves
        } else {
            let n = n as usize;
            if n == 0 {
                Vec::new()
            } else {
                let mut rng = rand::thread_rng();
                legal_moves.iter()
                    .choose_multiple(&mut rng, n.min(legal_moves.len()))
                    .into_iter()
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn get_next_move(&self) -> String {
        // Default to getting one move
        self.get_next_moves(1)
            .into_iter()
            .next()
            .expect("No moves found, which should be impossible in current state")
    }

    // Generic helper function to convert a source bitboard and target bitboard into a list of moves
    pub fn bitboard_to_moves(&self, source_pieces: u64, target_squares: u64) -> Vec<String> {
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

    pub fn is_square_attacked(&self, square: u8, attacked_by_color: Color) -> bool {
        let square_bb = 1u64 << square;

        if let Some(piece) = self.get_piece_at_square(square) {
            if piece.color() == attacked_by_color {
                // If the square is occupied by a piece of the same color we are looking for
                // attacks BY, then it cannot be attacked
                return false;
            }
        }

        // At this point, we know the square is empty or occupied by a piece of the opposite color

        if attacked_by_color == Color::White {
            // At this point we know the square is empty or occupied by a black piece
            // Check for pawn attacks
            let pawn_attacks = w_pawns_attack_targets(self.white_pawns, square_bb);
            if pawn_attacks != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            // Get all squares a knight could attack this square from
            let attacking_squares = knight_legal_moves(square_bb, 0);
            if attacking_squares & self.white_knights != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            let bishop_moves = bishop_legal_moves(square_bb, self.any_black, self.any_white);
            if bishop_moves & (self.white_bishops | self.white_queen) != 0 {
                return true;
            }

            // Check for rook/straight queen attacks
            let rook_moves = rook_legal_moves(square_bb, self.any_black, self.any_white);
            if rook_moves & (self.white_rooks | self.white_queen) != 0 {
                return true;
            }

            // Check for king attacks
            let king_moves = king_legal_moves(square_bb, self.any_white);
            if king_moves & self.white_king != 0 {
                return true;
            }
        } else {
            // At this point we know the square is empty or occupied by a white piece

            // Check for pawn attacks
            let pawn_attacks = b_pawns_attack_targets(self.black_pawns, square_bb);
            if pawn_attacks != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            let attacking_squares = knight_legal_moves(square_bb, 0);
            if attacking_squares & self.black_knights != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            let bishop_moves = bishop_legal_moves(square_bb, self.any_white, self.any_black);
            if bishop_moves & (self.black_bishops | self.black_queen) != 0 {
                return true;
            }

            // Check for rook/straight queen attacks
            let rook_moves = rook_legal_moves(square_bb, self.any_white, self.any_black);
            if rook_moves & (self.black_rooks | self.black_queen) != 0 {
                return true;
            }

            // Check for king attacks
            let king_moves = king_legal_moves(square_bb, self.any_black);
            if king_moves & self.black_king != 0 {
                return true;
            }
        }

        false
    }

    pub fn update_check_state(&mut self) {
        // Find white king square
        let white_king_square = self.white_king.trailing_zeros() as u8;
        // Find black king square
        let black_king_square = self.black_king.trailing_zeros() as u8;

        // Check if either king is under attack
        self.white_king_in_check = self.is_square_attacked(white_king_square, Color::Black);
        self.black_king_in_check = self.is_square_attacked(black_king_square, Color::White);
    }

    pub fn is_legal_move(&self, mv: &Move) -> bool {
        // If we're not in check, all moves are legal (for now - we'll add more restrictions later)
        if !self.white_king_in_check && !self.black_king_in_check {
            let mut test_board = self.clone();
            test_board.apply_move(mv);
            if self.side_to_move == Color::White {
                assert!(!self.white_king_in_check);
                // If the move doesn't put the white king in check, it's legal
                return !test_board.white_king_in_check;
            } else {
                assert!(!self.black_king_in_check);
                // If the move doesn't put the black king in check, it's legal
                return !test_board.black_king_in_check;
            }
        }

        // At this point we know we are in check.

        // We are in check, so make a copy of the board and try the move
        let mut test_board = self.clone();
        test_board.apply_move(mv);

        // The move is legal if it got us out of check
        if self.white_king_in_check {
            assert_eq!(self.side_to_move, Color::White, "White king is in check, but side to move is not White");
            !test_board.white_king_in_check
        } else {
            assert!(self.black_king_in_check);
            assert_eq!(self.side_to_move, Color::Black, "Black king is in check, but side to move is not Black");
            // Assumed we are in check and moving black
            !test_board.black_king_in_check
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
        side_to_move: Color::White,
        white_king_in_check: false,
        black_king_in_check: false,
    };
    board.update_composite_bitboards();
    board
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

