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
        let bitboard_index = Square::try_from(coordinate).unwrap().to_bit_index();
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

    fn is_square_attacked(&self, square: u8, attacked_by_color: Color) -> bool {
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

    fn update_check_state(&mut self) {
        // Find white king square
        let white_king_square = self.white_king.trailing_zeros() as u8;
        // Find black king square
        let black_king_square = self.black_king.trailing_zeros() as u8;

        // Check if either king is under attack
        self.white_king_in_check = self.is_square_attacked(white_king_square, Color::Black);
        self.black_king_in_check = self.is_square_attacked(black_king_square, Color::White);
    }

    fn is_legal_move(&self, mv: &Move) -> bool {
        // If we're not in check, all moves are legal (for now - we'll add more restrictions later)
        if !self.white_king_in_check && !self.black_king_in_check {
            return true;
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
        assert_eq!(Square::A1.to_bit_index(), 0);
        assert_eq!(Square::A2.to_bit_index(), 8);
        assert_eq!(Square::A3.to_bit_index(), 16);
        assert_eq!(Square::A4.to_bit_index(), 24);
        assert_eq!(Square::A5.to_bit_index(), 32);
        assert_eq!(Square::A6.to_bit_index(), 40);
        assert_eq!(Square::A7.to_bit_index(), 48);
        assert_eq!(Square::A8.to_bit_index(), 56);

        // Center squares
        assert_eq!(Square::D4.to_bit_index(), 27);
        assert_eq!(Square::E4.to_bit_index(), 28);
        assert_eq!(Square::D5.to_bit_index(), 35);
        assert_eq!(Square::E5.to_bit_index(), 36);

        // h file
        assert_eq!(Square::H8.to_bit_index(), 63);
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
        assert_eq!(Square::A1.to_bit_index(), 0);
        assert_eq!(Square::H1.to_bit_index(), 7);
        assert_eq!(Square::A2.to_bit_index(), 8);
        assert_eq!(Square::H2.to_bit_index(), 15);
        assert_eq!(Square::A8.to_bit_index(), 56);
        assert_eq!(Square::H8.to_bit_index(), 63);

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
        assert!(is_bit_set(board.white_pawns, Square::E4.to_bit_index()));
        assert!(!is_bit_set(board.white_pawns, Square::E2.to_bit_index()));

        // Test moving a black pawn from d7 to d5
        board.apply_move(&Move { src: Square::D7, target: Square::D5 });
        assert!(is_bit_set(board.black_pawns, Square::D5.to_bit_index()));
        assert!(!is_bit_set(board.black_pawns, Square::D7.to_bit_index()));

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
        // Check if move starts with a valid black piece position
        assert!(
            // Pawns
            next_move.starts_with("a7") || next_move.starts_with("b7") ||
            next_move.starts_with("c7") || next_move.starts_with("d6") ||
            next_move.starts_with("e7") || next_move.starts_with("f7") ||
            next_move.starts_with("g7") || next_move.starts_with("h7") ||
            // Knights
            next_move.starts_with("b8") || next_move.starts_with("g8") ||
            // Bishops
            next_move.starts_with("c8") || next_move.starts_with("f8") ||
            // Rooks
            next_move.starts_with("a8") || next_move.starts_with("h8") ||
            // Queen
            next_move.starts_with("d8") ||
            // King
            next_move.starts_with("e8"),
            "Move {} should be a valid black piece move", next_move
        );
    }

    #[test]
    #[should_panic(expected = "Attempted to move a White piece during Black's turn")]
    fn test_apply_move_same_side_twice_fails() {
        let mut board = get_starting_board();

        // Test moving a white pawn from e2 to e4
        board.apply_move(&Move { src: Square::E2, target: Square::E4 });
        assert!(is_bit_set(board.white_pawns, Square::E4.to_bit_index()));
        assert!(!is_bit_set(board.white_pawns, Square::E2.to_bit_index()));

        // Test moving another white pawn from d2 to d4 - should panic
        board.apply_move(&Move { src: Square::D2, target: Square::D4 });
    }

    #[test]
    fn test_knight_moves() {
        let mut board = get_starting_board();

        // Move white knight from b1 to c3
        board.apply_move(&Move { src: Square::B1, target: Square::C3 });
        assert!(is_bit_set(board.white_knights, Square::C3.to_bit_index()));
        assert!(!is_bit_set(board.white_knights, Square::B1.to_bit_index()));
        assert_eq!(board.side_to_move, Color::Black);

        // Move black knight from g8 to f6
        board.apply_move(&Move { src: Square::G8, target: Square::F6 });
        assert!(is_bit_set(board.black_knights, Square::F6.to_bit_index()));
        assert!(!is_bit_set(board.black_knights, Square::G8.to_bit_index()));
        assert_eq!(board.side_to_move, Color::White);

        // Test a capture: white knight takes black pawn
        board.apply_move(&Move { src: Square::C3, target: Square::D5 });
        assert!(is_bit_set(board.white_knights, Square::D5.to_bit_index()));
        assert!(!is_bit_set(board.white_knights, Square::C3.to_bit_index()));
        assert!(!is_bit_set(board.black_pawns, Square::D5.to_bit_index()));
    }

    #[test]
    fn test_bitboard_to_moves() {
        let board = get_starting_board();

        // Test with a single source and multiple targets
        let source = Square::E4.to_bitboard();  // Knight on e4
        let targets = Square::F6.to_bitboard() | Square::D6.to_bitboard() | Square::C5.to_bitboard();

        let moves = board.bitboard_to_moves(source, targets);

        // Verify the moves are generated correctly
        assert!(moves.contains(&"e4f6".to_string()));
        assert!(moves.contains(&"e4d6".to_string()));
        assert!(moves.contains(&"e4c5".to_string()));
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_bitboard_to_moves2() {
        let board = get_starting_board();

        // Test with a different single source and single target
        let source = Square::G1.to_bitboard();  // Knight on g1
        let target = Square::F3.to_bitboard();  // Target square f3

        let moves = board.bitboard_to_moves(source, target);

        // Verify move is generated correctly
        assert!(moves.contains(&"g1f3".to_string()));
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_bitboard_to_moves3() {
        let board = get_starting_board();

        // Test b1 knight separately
        let source = Square::B1.to_bitboard();  // Knight on b1
        let target = Square::C3.to_bitboard();  // Target square c3

        let moves = board.bitboard_to_moves(source, target);

        // Verify move is generated correctly
        assert!(moves.contains(&"b1c3".to_string()));
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_bitboard_to_moves4() {
        let board = get_starting_board();
        let source = Square::E4.to_bitboard();  // Knight on e4

        // Test with no target squares (should produce empty move list)
        assert!(board.bitboard_to_moves(source, 0).is_empty());
    }

    #[test]
    fn test_get_next_move() {
        let mut board = get_starting_board();
        assert_eq!(board.side_to_move, Color::White);

        // White's first move should be either a pawn move or knight move
        let first_move = board.get_next_move();
        assert!(first_move.starts_with("a2") || first_move.starts_with("b2") ||
               first_move.starts_with("c2") || first_move.starts_with("d2") ||
               first_move.starts_with("e2") || first_move.starts_with("f2") ||
               first_move.starts_with("g2") || first_move.starts_with("h2") ||
               first_move.starts_with("b1") || first_move.starts_with("g1"),
               "First move {} should be a white pawn or knight move", first_move);

        // Apply the first move and get a response from black
        board.apply_move_from_string(&first_move);
        assert_eq!(board.side_to_move, Color::Black);

        let black_move = board.get_next_move();
        assert!(black_move.starts_with("a7") || black_move.starts_with("b7") ||
               black_move.starts_with("c7") || black_move.starts_with("d7") ||
               black_move.starts_with("e7") || black_move.starts_with("f7") ||
               black_move.starts_with("g7") || black_move.starts_with("h7") ||
               black_move.starts_with("b8") || black_move.starts_with("g8"),
               "Move {} should be a black pawn or knight move", black_move);

        // Apply black's move and get another white move
        board.apply_move_from_string(&black_move);
        assert_eq!(board.side_to_move, Color::White);

        // Get another move - make sure it's still valid format
        let next_move = board.get_next_move();
        assert_eq!(next_move.len(), 4, "Move should be in format 'e2e4', got {}", next_move);
        assert!(next_move.chars().all(|c| c.is_ascii_alphanumeric()),
               "Move should only contain letters and numbers, got {}", next_move);
    }

    #[test]
    fn test_get_next_moves() {
        let board = get_starting_board();

        // Test getting no moves
        let no_moves = board.get_next_moves(0);
        assert!(no_moves.is_empty(), "Should return empty vec when n=0");

        // Test getting single move (should be same as get_next_move)
        let one_move = board.get_next_moves(1);
        assert_eq!(one_move.len(), 1, "Should return exactly one move when n=1");

        // Test getting multiple moves
        let five_moves = board.get_next_moves(5);
        assert!(five_moves.len() <= 5, "Should not return more moves than requested");
        assert!(!five_moves.is_empty(), "Should return at least one move");

        // Verify all moves are valid white moves from starting position
        for mv in five_moves {
            assert!(mv.starts_with("a2") || mv.starts_with("b2") ||
                   mv.starts_with("c2") || mv.starts_with("d2") ||
                   mv.starts_with("e2") || mv.starts_with("f2") ||
                   mv.starts_with("g2") || mv.starts_with("h2") ||
                   mv.starts_with("b1") || mv.starts_with("g1"),
                   "Move {} should be a white pawn or knight move", mv);
        }

        // Test getting all moves (n = -1)
        let all_moves = board.get_next_moves(-1);
        assert!(!all_moves.is_empty(), "Should return all possible moves");
        // In starting position, each pawn can move 1 or 2 squares (16 moves)
        // and each knight has 2 possible moves (4 moves total)
        // So we expect exactly 20 possible moves
        assert_eq!(all_moves.len(), 20, "Starting position should have exactly 20 possible moves");
    }

    #[test]
    fn test_hello_world() {
        println!("hello world");
    }

    #[test]
    fn test_is_square_attacked_pawns() {
        let mut board = get_starting_board();

        // In starting position, no center squares are attacked
        assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White));
        assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::Black));

        // After 1.e4, e4 is not attacked by any pieces
        board.apply_move(&Move { src: Square::E2, target: Square::E4 });
        assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // This is false because white pieces can't attack squares occupied by white pieces
        assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::Black)); // This is false because the pawn on e4 isn't attacked by any black pieces yet.

        // Black responds with pawn to d5
        board.apply_move(&Move { src: Square::D7, target: Square::D5 });
        // d5 is attacked by white pawn on e4
        assert!(board.is_square_attacked(Square::D5.to_bit_index(), Color::White));
        assert!(!board.is_square_attacked(Square::D5.to_bit_index(), Color::Black)); // Just double-check make sure that black can't attack black's own pawn
        // e4 is attacked by black pawn on d5
        assert!(board.is_square_attacked(Square::E4.to_bit_index(), Color::Black));
        assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // Just double-check make sure that white can't attack white's own pawn
    }

    #[test]
    fn test_is_square_attacked_knights() {
        let mut board = get_starting_board();
        board.apply_move(&Move { src: Square::E2, target: Square::E4 });
        board.apply_move(&Move { src: Square::D7, target: Square::D5 });

        // Test knight attacks
        board.apply_move(&Move { src: Square::B1, target: Square::C3 });
        // Just a dummy move for black
        board.apply_move(&Move { src: Square::H7, target: Square::H6 });
        // move the e pawn up so it's not being attacked, or attacking the D pawn anymore:
        board.apply_move(&Move { src: Square::E4, target: Square::E5 });
        assert!(board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // White knight attacks black pawn on d5
        assert!(board.is_square_attacked(Square::E4.to_bit_index(), Color::Black)); // E4 is empty but is being attacked by the black pawn on d5
    }

    #[test]
    fn test_is_square_attacked_bishops() {
        // Test bishop attacks
        let mut attack_test_board = Board {
            white_bishops: Square::C4.to_bitboard(),
            black_queen: Square::F7.to_bitboard(), // Add a piece to block diagonal
            ..get_starting_board()
        };
        attack_test_board.update_composite_bitboards();
        assert!(attack_test_board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // Bishop attacks e6
        assert!(attack_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // Bishop attacks e6
        assert!(attack_test_board.is_square_attacked(Square::F7.to_bit_index(), Color::White)); // Bishop attacks e6
        assert!(!attack_test_board.is_square_attacked(Square::G8.to_bit_index(), Color::White)); // Bishop attack blocked by queen

        // TODO: Add more bishop tests for different positions
    }

    #[test]
    fn test_is_square_attacked_rooks() {
        // Test rook attacks
        let mut rook_test_board = Board {
            white_rooks: Square::E4.to_bitboard(),
            black_pawns: Square::E6.to_bitboard(), // Add a piece to block file
            ..get_starting_board()
        };
        rook_test_board.update_composite_bitboards();
        assert!(rook_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // Rook attacks e5
        assert!(rook_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // Rook attacks e5
        assert!(!rook_test_board.is_square_attacked(Square::E7.to_bit_index(), Color::White)); // Rook attack blocked by pawn
        assert!(!rook_test_board.is_square_attacked(Square::E8.to_bit_index(), Color::White)); // Rook attack blocked by pawn
    }

    #[test]
    fn test_is_square_attacked_queen() {
        // Test queen attacks
        let mut queen_test_board = Board {
            white_queen: Square::D4.to_bitboard(),
            ..get_starting_board()
        };
        queen_test_board.update_composite_bitboards();
        assert!(!queen_test_board.is_square_attacked(Square::D2.to_bit_index(), Color::White)); // Not an attack, it's our own pawn
        assert!(queen_test_board.is_square_attacked(Square::D3.to_bit_index(), Color::White)); // Queen attacks vertically down
        assert!(queen_test_board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // Queen attacks vertically up
        assert!(queen_test_board.is_square_attacked(Square::D6.to_bit_index(), Color::White)); // Queen attacks vertically up
        assert!(queen_test_board.is_square_attacked(Square::D7.to_bit_index(), Color::White)); // Queen attacks vertically up
        assert!(queen_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
        assert!(queen_test_board.is_square_attacked(Square::F6.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
        assert!(queen_test_board.is_square_attacked(Square::G7.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
        assert!(!queen_test_board.is_square_attacked(Square::H8.to_bit_index(), Color::White)); // Queen attack blocked by G7
        assert!(queen_test_board.is_square_attacked(Square::A4.to_bit_index(), Color::White)); // Queen attacks horizontally left
        assert!(queen_test_board.is_square_attacked(Square::B4.to_bit_index(), Color::White)); // Queen attacks horizontally left
        assert!(queen_test_board.is_square_attacked(Square::C4.to_bit_index(), Color::White)); // Queen attacks horizontally left
        assert!(queen_test_board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // Queen attacks horizontally right
        assert!(queen_test_board.is_square_attacked(Square::F4.to_bit_index(), Color::White)); // Queen attacks horizontally right
        assert!(queen_test_board.is_square_attacked(Square::G4.to_bit_index(), Color::White)); // Queen attacks horizontally right
        assert!(queen_test_board.is_square_attacked(Square::H4.to_bit_index(), Color::White)); // Queen attacks horizontally right

    }

    #[test]
    fn test_is_legal_move() {
        // Test 1: Starting position, any legal move is valid
        let board = get_starting_board();
        assert!(board.is_legal_move(&Move { src: Square::E2, target: Square::E4 }));
    }

    #[test]
    fn test_is_legal_move_complex() {
        // Test 2: White king in check by rook on E8
        let mut check_board = Board {
            white_king: Square::E1.to_bitboard(),
            white_queen: Square::D2.to_bitboard(), // Queen in position to block check
            black_rooks: Square::E8.to_bitboard(),
            side_to_move: Color::White,
            white_pawns: 0,
            white_knights: 0,
            white_bishops: Square::D7.to_bitboard(), // Bishop in position to capture rook
            white_rooks: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queen: 0,
            black_king: Square::G8.to_bitboard(), // Black king safely away
            any_white: 0,
            any_black: 0,
            empty: 0,
            white_king_in_check: true,
            black_king_in_check: false,
        };
        check_board.update_composite_bitboards();
        check_board.update_check_state();

        assert!(check_board.white_king_in_check);

        // Legal moves that escape check
        assert!(check_board.is_legal_move(&Move { src: Square::E1, target: Square::F1 })); // King escapes sideways
        assert!(check_board.is_legal_move(&Move { src: Square::E1, target: Square::D1 })); // King escapes other way
        assert!(check_board.is_legal_move(&Move { src: Square::D7, target: Square::E8 })); // Bishop captures rook
        assert!(check_board.is_legal_move(&Move { src: Square::D2, target: Square::E2 })); // Queen blocks check

        // Illegal moves that don't escape check
        assert!(!check_board.is_legal_move(&Move { src: Square::D2, target: Square::D3 })); // Queen moves away
        assert!(!check_board.is_legal_move(&Move { src: Square::E1, target: Square::E2 })); // King moves into check
    }

    //
    // #[test]
    // fn test_is_square_attacked_king() {
    //     // Test king attacks
    //     let mut king_test_board = Board {
    //         white_king: Square::E4.to_bitboard(),
    //         ..get_starting_board()
    //     };
    //     king_test_board.update_composite_bitboards();
    //     assert!(king_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // King attacks adjacent
    //     assert!(king_test_board.is_square_attacked(Square::F4.to_bit_index(), Color::White)); // King attacks adjacent
    //     assert!(!king_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // King can't attack 2 squares away
    // }
}
