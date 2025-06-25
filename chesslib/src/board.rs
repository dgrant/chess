use crate::board_utils;
use crate::types::{Color, Move, Piece, PieceType, Square, SPACE};
use crate::move_generation::{
    b_pawns_attack_targets, bishop_legal_moves,
    king_legal_moves, knight_legal_moves,
    rook_legal_moves,
    w_pawns_attack_targets,
};

#[derive(Clone, Debug)]
pub struct BoardState {
    pub white_kingside_castle_rights: bool,
    pub white_queenside_castle_rights: bool,
    pub black_kingside_castle_rights: bool,
    pub black_queenside_castle_rights: bool,
    pub en_passant_target: Option<Square>,
    pub last_move: Move,
    pub captured_piece: Option<Piece>,
    pub captured_piece_square: Option<Square>,
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

    /// Castling rights
    pub white_kingside_castle_rights: bool,
    pub white_queenside_castle_rights: bool,
    pub black_kingside_castle_rights: bool,
    pub black_queenside_castle_rights: bool,

    /// The square where an en-passant capture is possible (if any)
    /// This is set when a pawn makes a double move and cleared after each move
    pub en_passant_target: Option<Square>,

    /// Represents a snapshot of board state that can be restored when undoing a move
    pub move_history: Vec<BoardState>,
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
        if board_utils::is_bit_set(self.white_pawns, square_index) {
            Some(Piece::WhitePawn)
        } else if board_utils::is_bit_set(self.black_pawns, square_index) {
            Some(Piece::BlackPawn)
        } else if board_utils::is_bit_set(self.white_knights, square_index) {
            Some(Piece::WhiteKnight)
        } else if board_utils::is_bit_set(self.black_knights, square_index) {
            Some(Piece::BlackKnight)
        } else if board_utils::is_bit_set(self.white_bishops, square_index) {
            Some(Piece::WhiteBishop)
        } else if board_utils::is_bit_set(self.black_bishops, square_index) {
            Some(Piece::BlackBishop)
        } else if board_utils::is_bit_set(self.white_rooks, square_index) {
            Some(Piece::WhiteRook)
        } else if board_utils::is_bit_set(self.black_rooks, square_index) {
            Some(Piece::BlackRook)
        } else if board_utils::is_bit_set(self.white_queen, square_index) {
            Some(Piece::WhiteQueen)
        } else if board_utils::is_bit_set(self.black_queen, square_index) {
            Some(Piece::BlackQueen)
        } else if board_utils::is_bit_set(self.white_king, square_index) {
            Some(Piece::WhiteKing)
        } else if board_utils::is_bit_set(self.black_king, square_index) {
            Some(Piece::BlackKing)
        } else {
            None
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        let src_idx = mv.src.to_bit_index();
        let target_idx = mv.target.to_bit_index();

        // Get the piece at the source square
        let piece = self.get_piece_at_square(src_idx);
        // Check if there is a piece at the source square
        if piece.is_none() {
            panic!("No piece at source square: {}", mv.src);
        }
        // Check if the piece belongs to the side to move
        if piece.as_ref().unwrap().color() != self.side_to_move {
            panic!("Piece at source square {} does not match side to move: {:?}", mv.src, self.side_to_move);
        }

        let piece = piece.unwrap(); // Now safe to unwrap and move
        // Check for en-passant capture before storing state
        let is_en_passant = match piece {
            Piece::WhitePawn | Piece::BlackPawn => {
                if let Some(ep_square) = self.en_passant_target {
                    mv.target == ep_square
                } else {
                    false
                }
            },
            _ => false,
        };

        // For en-passant, we need to get the captured piece from its actual square
        let (captured_piece, captured_piece_square) = if is_en_passant {
            let captured_pawn_square = if piece == Piece::WhitePawn {
                target_idx - 8 // captured black pawn is one rank below
            } else {
                target_idx + 8 // captured white pawn is one rank above
            };
            (
                self.get_piece_at_square(captured_pawn_square),
                Some(Square::from_bit_index(captured_pawn_square))
            )
        } else {
            (
                self.get_piece_at_square(mv.target.to_bit_index()),
                Some(mv.target)
            )
        };

        // Now store the state with the correct captured piece
        let current_state = BoardState {
            white_kingside_castle_rights: self.white_kingside_castle_rights,
            white_queenside_castle_rights: self.white_queenside_castle_rights,
            black_kingside_castle_rights: self.black_kingside_castle_rights,
            black_queenside_castle_rights: self.black_queenside_castle_rights,
            en_passant_target: self.en_passant_target,
            last_move: mv.clone(),
            captured_piece,
            captured_piece_square,
        };

        // Store the state before making any changes
        self.move_history.push(current_state);

        let from_bit = mv.src.to_bitboard();
        let to_bit = mv.target.to_bitboard();
        let src_idx = mv.src.to_bit_index();
        let target_idx = mv.target.to_bit_index();

        // First, identify if this is a castling move
        // TODO: Use Square enum instead of target_idx for clarity
        let is_castle = match piece {
            Piece::WhiteKing if src_idx == 4 && (target_idx == 6 || target_idx == 2) => true,
            Piece::BlackKing if src_idx == 60 && (target_idx == 62 || target_idx == 58) => true,
            _ => false,
        };

        // Handle castling rook movements before we do any other piece movements
        if is_castle {
            match piece {
                Piece::WhiteKing => {
                    if target_idx == 6 {  // g1 - kingside castle
                        debug_assert!(self.white_kingside_castle_rights, "Attempting kingside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.white_rooks ^= (1u64 << 7) | (1u64 << 5);  // h1 to f1
                    } else if target_idx == 2 {  // c1 - queenside castle
                        debug_assert!(self.white_queenside_castle_rights, "Attempting queenside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.white_rooks ^= 1u64 | (1u64 << 3);  // a1 to d1
                    }
                },
                Piece::BlackKing => {
                    if target_idx == 62 {  // g8 - kingside castle
                        debug_assert!(self.black_kingside_castle_rights, "Attempting kingside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.black_rooks ^= (1u64 << 63) | (1u64 << 61);  // h8 to f8
                    } else if target_idx == 58 {  // c8 - queenside castle
                        debug_assert!(self.black_queenside_castle_rights, "Attempting queenside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.black_rooks ^= (1u64 << 56) | (1u64 << 59);  // a8 to d8
                    }
                },
                _ => {}
            }
        }

        // Remove captured piece if any (but not for castling which doesn't capture)
        if !is_castle {
            // Handle en-passant capture
            if is_en_passant {
                let captured_pawn_square = if piece == Piece::WhitePawn {
                    target_idx - 8 // captured black pawn is one rank below
                } else {
                    target_idx + 8 // captured white pawn is one rank above
                };
                let captured_pawn_bit = 1u64 << captured_pawn_square;
                if piece == Piece::WhitePawn {
                    self.black_pawns &= !captured_pawn_bit;
                } else {
                    self.white_pawns &= !captured_pawn_bit;
                }
            } else if let Some(captured_piece) = self.get_piece_at_square(target_idx) {
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
        }

        // Update castling rights based on the moving piece
        match piece {
            Piece::WhiteKing => {
                self.white_kingside_castle_rights = false;
                self.white_queenside_castle_rights = false;
            }
            Piece::BlackKing => {
                self.black_kingside_castle_rights = false;
                self.black_queenside_castle_rights = false;
            }
            Piece::WhiteRook => {
                // Check if it's the kingside or queenside rook
                // TODO: use Square enum instead of hardcoded indices
                if src_idx == 7 { // h1
                    self.white_kingside_castle_rights = false;
                // TODO: use Square enum instead of hardcoded indices
                } else if src_idx == 0 { // a1
                    self.white_queenside_castle_rights = false;
                }
            }
            Piece::BlackRook => {
                // Check if it's the kingside or queenside rook
                // TODO: use Square enum instead of hardcoded indices
                if src_idx == 63 { // h8
                    self.black_kingside_castle_rights = false;
                // TODO: use Square enum instead of hardcoded indices
                } else if src_idx == 56 { // a8
                    self.black_queenside_castle_rights = false;
                }
            }
            _ => {}
        }

        // Also remove castling rights if a rook is captured
        if let Some(captured_piece) = self.get_piece_at_square(target_idx) {
            match captured_piece {
                Piece::WhiteRook => {
                    // TODO: use Square enum instead of hardcoded indices
                    if target_idx == 7 { // h1
                        self.white_kingside_castle_rights = false;
                    // TODO: use Square enum instead of hardcoded indices
                    } else if target_idx == 0 { // a1
                        self.white_queenside_castle_rights = false;
                    }
                }
                Piece::BlackRook => {
                    // TODO: use Square enum instead of hardcoded indices
                    if target_idx == 63 { // h8
                        self.black_kingside_castle_rights = false;
                    // TODO: use Square enum instead of hardcoded indices
                    } else if target_idx == 56 { // a8
                        self.black_queenside_castle_rights = false;
                    }
                }
                _ => {}
            }
        }

        // Move the main piece (king or other)
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

        // Set en-passant target square for double pawn moves
        self.en_passant_target = match piece {
            Piece::WhitePawn if src_idx / 8 == 1 && target_idx / 8 == 3 => {
                // White pawn double push
                Some(Square::from_bit_index(src_idx + 8))
            },
            Piece::BlackPawn if src_idx / 8 == 6 && target_idx / 8 == 4 => {
                // Black pawn double push
                Some(Square::from_bit_index(src_idx - 8))
            },
            _ => None
        };

        // Handle promotions if any
        if mv.promotion.is_some() {
            assert!(piece == Piece::WhitePawn || piece == Piece::BlackPawn, "Promotion can only be applied to pawns");
            // Handle promotion
            let promotion_piece = match mv.promotion.unwrap() {
                PieceType::Queen => if piece == Piece::WhitePawn { Piece::WhiteQueen } else { Piece::BlackQueen },
                PieceType::Rook => if piece == Piece::WhitePawn { Piece::WhiteRook } else { Piece::BlackRook },
                PieceType::Bishop => if piece == Piece::WhitePawn { Piece::WhiteBishop } else { Piece::BlackBishop },
                PieceType::Knight => if piece == Piece::WhitePawn { Piece::WhiteKnight } else { Piece::BlackKnight },
                _ => panic!("Invalid promotion piece type: {:?}", mv.promotion),
            };
            // Remove the pawn from the board
            *piece_bitboard ^= to_bit;
            // Add the promoted piece to the target square
            match promotion_piece {
                Piece::WhiteQueen => self.white_queen |= to_bit,
                Piece::WhiteRook => self.white_rooks |= to_bit,
                Piece::WhiteBishop => self.white_bishops |= to_bit,
                Piece::WhiteKnight => self.white_knights |= to_bit,
                Piece::BlackQueen => self.black_queen |= to_bit,
                Piece::BlackRook => self.black_rooks |= to_bit,
                Piece::BlackBishop => self.black_bishops |= to_bit,
                Piece::BlackKnight => self.black_knights |= to_bit,
                _ => panic!("Invalid promotion piece"),
            }
        }

        self.update_composite_bitboards();
        self.update_check_state();
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
        } else {
            panic!("Invalid move string: {}", mv_str);
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
                                   w_pawns_en_passant_targets, b_pawns_en_passant_targets,
                                   knight_legal_moves, bishop_legal_moves, rook_legal_moves,
                                   queen_legal_moves, king_legal_moves};
        use rand::seq::IteratorRandom;
        use crate::board_utils;

        let mut possible_moves = Vec::new();

        if self.side_to_move == Color::Black {
            // Get all possible pawn moves
            let moveable_pawns = b_pawns_able_to_push(self.black_pawns, self.empty);
            let double_moveable_pawns = b_pawns_able_to_double_push(self.black_pawns, self.empty);
            let attacking_pawns = b_pawns_attack_targets(self.black_pawns, self.any_white);

            // Add en-passant moves if available
            if let Some(ep_square) = self.en_passant_target {
                let ep_targets = b_pawns_en_passant_targets(self.black_pawns, ep_square.to_bitboard());
                if ep_targets != 0 {
                    // Find the source pawns that can make the en-passant capture
                    let mut working_pawns = self.black_pawns;
                    while working_pawns != 0 {
                        let from_square = working_pawns.trailing_zeros() as u8;
                        working_pawns &= working_pawns - 1;  // Clear the processed bit
                        let pawn = 1u64 << from_square;
                        if b_pawns_en_passant_targets(pawn, ep_square.to_bitboard()) != 0 {
                            possible_moves.push(board_utils::bitboard_squares_to_move(pawn, ep_square.to_bitboard()));
                        }
                    }
                }
            }

            possible_moves.extend(board_utils::bitboard_to_pawn_single_moves(moveable_pawns, true));
            possible_moves.extend(board_utils::bitboard_to_pawn_double_moves(double_moveable_pawns, true));
            possible_moves.extend(board_utils::bitboard_to_pawn_capture_moves(self.black_pawns, attacking_pawns, true));

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

            // Process black king (only one) with normal moves and castling
            let moves = king_legal_moves(self.black_king, self.any_black);
            possible_moves.extend(self.bitboard_to_moves(self.black_king, moves));

            // Add castling moves if legal
            if self.is_castling_legal(true, false) {  // Black kingside castle
                possible_moves.push("e8g8".to_string());
            }
            if self.is_castling_legal(false, false) {  // Black queenside castle
                possible_moves.push("e8c8".to_string());
            }

        } else {
            // Get all possible pawn moves
            let moveable_pawns = w_pawns_able_to_push(self.white_pawns, self.empty);
            let double_moveable_pawns = w_pawns_able_to_double_push(self.white_pawns, self.empty);
            let attacking_pawns = w_pawns_attack_targets(self.white_pawns, self.any_black);

            // Add en-passant moves if available
            if let Some(ep_square) = self.en_passant_target {
                let ep_targets = w_pawns_en_passant_targets(self.white_pawns, ep_square.to_bitboard());
                if ep_targets != 0 {
                    // Find the source pawns that can make the en-passant capture
                    let mut working_pawns = self.white_pawns;
                    while working_pawns != 0 {
                        let from_square = working_pawns.trailing_zeros() as u8;
                        working_pawns &= working_pawns - 1;  // Clear the processed bit
                        let pawn = 1u64 << from_square;
                        if w_pawns_en_passant_targets(pawn, ep_square.to_bitboard()) != 0 {
                            possible_moves.push(board_utils::bitboard_squares_to_move(pawn, ep_square.to_bitboard()));
                        }
                    }
                }
            }

            possible_moves.extend(board_utils::bitboard_to_pawn_single_moves(moveable_pawns, false));
            possible_moves.extend(board_utils::bitboard_to_pawn_double_moves(double_moveable_pawns, false));
            possible_moves.extend(board_utils::bitboard_to_pawn_capture_moves(self.white_pawns, attacking_pawns, false));

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

            // Process white king (only one) with normal moves and castling
            let moves = king_legal_moves(self.white_king, self.any_white);
            possible_moves.extend(self.bitboard_to_moves(self.white_king, moves));

            // Add castling moves if legal
            if self.is_castling_legal(true, true) {  // White kingside castle
                possible_moves.push("e1g1".to_string());
            }
            if self.is_castling_legal(false, true) {  // White queenside castle
                possible_moves.push("e1c1".to_string());
            }
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
                let from_file = board_utils::int_file_to_string(from_square % 8);
                let from_rank = (from_square / 8 + 1).to_string();
                let to_file = board_utils::int_file_to_string(to_square % 8);
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

            // Check for king attacks - king can only be blocked by its own pieces
            let king_moves = king_legal_moves(square_bb, self.any_black);
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

            // Check for king attacks - king can only be blocked by its own pieces
            let king_moves = king_legal_moves(square_bb, self.any_white);
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

    /// Checks if squares between king and rook are empty for castling
    fn is_castling_path_empty(&self, is_kingside: bool, is_white: bool) -> bool {
        // TODO: pass in color
        let rank = if is_white { 0 } else { 7 * 8 };
        let path = if is_kingside {
            // f1 and g1 must be empty for white kingside, f8 and g8 for black
            (1u64 << (rank + 5)) | (1u64 << (rank + 6))
        } else {
            // b1, c1, and d1 must be empty for white queenside, b8, c8, d8 for black
            (1u64 << (rank + 1)) | (1u64 << (rank + 2)) | (1u64 << (rank + 3))
        };
        (path & self.empty) == path
    }

    /// Checks if any of the squares the king passes through (including start and end) are attacked
    fn is_castling_path_attacked(&self, is_kingside: bool, is_white: bool) -> bool {
        // TODO: pass in color
        let rank = if is_white { 0 } else { 7 * 8 };
        let king_path = if is_kingside {
            // e1, f1, g1 for white kingside, e8, f8, g8 for black
            (1u64 << (rank + 4)) | (1u64 << (rank + 5)) | (1u64 << (rank + 6))
        } else {
            // e1, d1, c1 for white queenside, e8, d8, c8 for black
            (1u64 << (rank + 4)) | (1u64 << (rank + 3)) | (1u64 << (rank + 2))
        };

        // Check each square in the path
        let mut path = king_path;
        while path != 0 {
            let square = path.trailing_zeros() as u8;
            path &= path - 1;  // Clear the processed bit
            if self.is_square_attacked(square, if is_white { Color::Black } else { Color::White }) {
                return true;
            }
        }
        false
    }

    /// Returns true if castling is legal (all conditions are met)
    fn is_castling_legal(&self, is_kingside: bool, is_white: bool) -> bool {
        // Check castling rights
        let has_rights = if is_white {
            if is_kingside { self.white_kingside_castle_rights } else { self.white_queenside_castle_rights }
        } else {
            if is_kingside { self.black_kingside_castle_rights } else { self.black_queenside_castle_rights }
        };

        // Early return if no castling rights
        if !has_rights {
            return false;
        }

        // Check if king is in check
        if (is_white && self.white_king_in_check) || (!is_white && self.black_king_in_check) {
            return false;
        }

        // Check if squares between king and rook are empty
        if !self.is_castling_path_empty(is_kingside, is_white) {
            return false;
        }

        // Check if any squares in the king's path are attacked
        if self.is_castling_path_attacked(is_kingside, is_white) {
            return false;
        }

        true
    }

    /// Performs a perft (performance test) to count all possible moves at a given depth.
    /// This is used to verify move generation correctness by comparing against known values.
    ///
    /// # Arguments
    /// * `depth` - The depth to search to, where 0 means just count the current position
    ///
    /// # Returns
    /// A tuple containing (number of leaf nodes at the specified depth, number of checkmates found)
    pub fn perft(&mut self, depth: u32) -> (u64, u64) {
        if depth == 0 {
            return (1, if self.is_checkmate() { 1 } else { 0 });
        }

        let mut nodes: u64 = 0;
        let mut checkmates: u64 = 0;
        let moves = self.generate_legal_moves();

        for mv in moves {
            self.apply_move(&mv);
            let (sub_nodes, sub_checkmates) = self.perft(depth - 1);
            nodes += sub_nodes;
            checkmates += sub_checkmates;
            self.undo_last_move();
        }
        (nodes, checkmates)
    }

    /// Generates all legal moves in the current position
    fn generate_legal_moves(&self) -> Vec<Move> {
        // Get all possible moves first
        let moves_str = self.get_next_moves(-1);

        // Convert string moves to Move structs
        moves_str.into_iter()
            .filter_map(|mv_str| Move::try_from(mv_str.as_str()).ok())
            .collect()
    }

    /// Undoes the last move made, restoring the board to its previous state
    pub fn undo_last_move(&mut self) {
        if let Some(state) = self.move_history.pop() {
            let mv = &state.last_move;
            
            let piece = self.get_piece_at_square(mv.target.to_bit_index())
                .expect("No piece at target square when undoing move");

            let src_bit = mv.src.to_bitboard();
            let target_bit = mv.target.to_bitboard();

            // Handle promotions first
            if mv.promotion.is_some() {
                // Remove the promoted piece
                match piece {
                    Piece::WhiteQueen => self.white_queen &= !target_bit,
                    Piece::WhiteRook => self.white_rooks &= !target_bit,
                    Piece::WhiteBishop => self.white_bishops &= !target_bit,
                    Piece::WhiteKnight => self.white_knights &= !target_bit,
                    Piece::BlackQueen => self.black_queen &= !target_bit,
                    Piece::BlackRook => self.black_rooks &= !target_bit,
                    Piece::BlackBishop => self.black_bishops &= !target_bit,
                    Piece::BlackKnight => self.black_knights &= !target_bit,
                    _ => panic!("Invalid promotion piece when undoing"),
                }
                // Restore the pawn
                if piece.color() == Color::White {
                    self.white_pawns |= src_bit;
                } else {
                    self.black_pawns |= src_bit;
                }
            } else {
                // Move the piece back to its source square
                let piece_bb = match piece {
                    Piece::WhitePawn => &mut self.white_pawns,
                    Piece::BlackPawn => &mut self.black_pawns,
                    Piece::WhiteKnight => &mut self.white_knights,
                    Piece::BlackKnight => &mut self.black_knights,
                    Piece::WhiteBishop => &mut self.white_bishops,
                    Piece::BlackBishop => &mut self.black_bishops,
                    Piece::WhiteRook => &mut self.white_rooks,
                    Piece::BlackRook => &mut self.black_rooks,
                    Piece::WhiteQueen => &mut self.white_queen,
                    Piece::BlackQueen => &mut self.black_queen,
                    Piece::WhiteKing => &mut self.white_king,
                    Piece::BlackKing => &mut self.black_king,
                };
                *piece_bb &= !target_bit;  // Remove from target
                *piece_bb |= src_bit;      // Add to source
            }

            // Restore captured piece if any
            if let Some(captured) = state.captured_piece {
                let captured_bb = match captured {
                    Piece::WhitePawn => &mut self.white_pawns,
                    Piece::BlackPawn => &mut self.black_pawns,
                    Piece::WhiteKnight => &mut self.white_knights,
                    Piece::BlackKnight => &mut self.black_knights,
                    Piece::WhiteBishop => &mut self.white_bishops,
                    Piece::BlackBishop => &mut self.black_bishops,
                    Piece::WhiteRook => &mut self.white_rooks,
                    Piece::BlackRook => &mut self.black_rooks,
                    Piece::WhiteQueen => &mut self.white_queen,
                    Piece::BlackQueen => &mut self.black_queen,
                    Piece::WhiteKing => &mut self.white_king,
                    Piece::BlackKing => &mut self.black_king,
                };
                *captured_bb |= &state.captured_piece_square.unwrap().to_bitboard();
            }

            // Restore castling state
            self.white_kingside_castle_rights = state.white_kingside_castle_rights;
            self.white_queenside_castle_rights = state.white_queenside_castle_rights;
            self.black_kingside_castle_rights = state.black_kingside_castle_rights;
            self.black_queenside_castle_rights = state.black_queenside_castle_rights;

            // Restore en-passant state
            self.en_passant_target = state.en_passant_target;

            // Update composite bitboards
            self.update_composite_bitboards();

            // Switch back to previous side
            self.side_to_move = match self.side_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };

            // Update check states
            self.update_check_state();
        }
    }

    /// Returns true if the current position is checkmate
    /// A position is checkmate if the side to move is in check and has no legal moves
    pub fn is_checkmate(&self) -> bool {
        // First check if the appropriate king is in check
        let in_check = match self.side_to_move {
            Color::White => self.white_king_in_check,
            Color::Black => self.black_king_in_check,
        };

        // If not in check, can't be checkmate
        if !in_check {
            return false;
        }

        // If in check, it's checkmate if there are no legal moves
        self.generate_legal_moves().is_empty()
    }

    /// Gets a complete debug state of the board including bitboards and move history
    pub fn get_debug_state(&self) -> String {
        let mut output = String::new();
        output.push_str("Board state dump:\n");
        output.push_str(&format!("White pawns:   {:064b}\n", self.white_pawns));
        output.push_str(&format!("White knights: {:064b}\n", self.white_knights));
        output.push_str(&format!("White bishops: {:064b}\n", self.white_bishops));
        output.push_str(&format!("White rooks:   {:064b}\n", self.white_rooks));
        output.push_str(&format!("White queen:   {:064b}\n", self.white_queen));
        output.push_str(&format!("White king:    {:064b}\n", self.white_king));
        output.push_str(&format!("Black pawns:   {:064b}\n", self.black_pawns));
        output.push_str(&format!("Black knights: {:064b}\n", self.black_knights));
        output.push_str(&format!("Black bishops: {:064b}\n", self.black_bishops));
        output.push_str(&format!("Black rooks:   {:064b}\n", self.black_rooks));
        output.push_str(&format!("Black queen:   {:064b}\n", self.black_queen));
        output.push_str(&format!("Black king:    {:064b}\n", self.black_king));
        output.push_str(&format!("Move history length: {}\n", self.move_history.len()));
        output.push_str(&format!("Side to move: {:?}\n", self.side_to_move));
        
        // Add complete move history in algebraic notation
        if !self.move_history.is_empty() {
            output.push_str("\nComplete move history:\n");
            for (i, state) in self.move_history.iter().enumerate() {
                output.push_str(&format!("{}. {} (captured: {:?})\n", 
                    i + 1, 
                    state.last_move,
                    state.captured_piece));
            }
        }
        output
    }

    /// Gets the complete move history in a format suitable for debugging
    pub fn get_move_history(&self) -> Vec<String> {
        self.move_history.iter()
            .enumerate()
            .map(|(i, state)| {
                format!("{}. {} (captured: {:?})", 
                    i + 1, 
                    state.last_move,
                    state.captured_piece)
            })
            .collect()
    }
}
