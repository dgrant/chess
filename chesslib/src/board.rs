use rand::prelude::IteratorRandom;
use crate::board_utils;
use crate::types::{Color, Move, Piece, PieceType, Square, SPACE};
use crate::move_generation::{
    b_pawns_attack_targets, bishop_moves,
    king_legal_moves, knight_legal_moves,
    rook_moves,
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
    pub rook_castle_move: Option<Move>,  // Stores the rook's move during castling
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

    pub piece_map: [Option<Piece>; 64],
}

impl Board {
    #[inline(always)]
    pub fn get_piece_at_square_fast(&self, sq: u8) -> Option<Piece> {
        self.piece_map[sq as usize]
    }

    fn move_piece_in_map(&mut self, from: u8, to: u8) {
        let p = self.piece_map[from as usize];
        self.piece_map[from as usize] = None;
        self.piece_map[to   as usize] = p;
    }

    fn remove_piece_in_map(&mut self, at: u8) {
        self.piece_map[at as usize] = None;
    }

    fn add_piece_in_map(&mut self, at: u8, p: Piece) {
        self.piece_map[at as usize] = Some(p);
    }

    /// Recomputes `piece_map` from the individual bitboards.
    /// Call this after you created a new position or when you bulk-rewrite bitboards.
    pub fn rebuild_piece_map(&mut self) {
        self.piece_map = [None; 64];

        macro_rules! fill {
            ($bb:expr, $piece:expr) => {{
                let mut bb = $bb;
                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    self.piece_map[sq] = Some($piece);
                    bb &= bb - 1; // clear LS1B
                }
            }};
        }

        fill!(self.white_pawns,   Piece::WhitePawn);
        fill!(self.black_pawns,   Piece::BlackPawn);
        fill!(self.white_knights, Piece::WhiteKnight);
        fill!(self.black_knights, Piece::BlackKnight);
        fill!(self.white_bishops, Piece::WhiteBishop);
        fill!(self.black_bishops, Piece::BlackBishop);
        fill!(self.white_rooks,   Piece::WhiteRook);
        fill!(self.black_rooks,   Piece::BlackRook);
        fill!(self.white_queen,   Piece::WhiteQueen);
        fill!(self.black_queen,   Piece::BlackQueen);
        fill!(self.white_king,    Piece::WhiteKing);
        fill!(self.black_king,    Piece::BlackKing);
    }

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
        match self.get_piece_at_square_fast(bitboard_index) {
            Some(piece) => piece.to_unicode(),
            None => SPACE
        }
    }

    pub fn get_piece_at_coordinate_as_fen(&self, coordinate: &str) -> &'static str {
        let bitboard_index = Square::try_from(coordinate).unwrap().to_bit_index();
        match self.get_piece_at_square_fast(bitboard_index) {
            Some(piece) => piece.to_fen(),
            None => SPACE
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        let src_idx = mv.src.to_bit_index();
        let target_idx = mv.target.to_bit_index();
        let target_piece = self.get_piece_at_square_fast(target_idx);
        let source_piece = self.get_piece_at_square_fast(src_idx).unwrap();
        let from_bit = mv.src.to_bitboard();
        let to_bit = mv.target.to_bitboard();

        // Check if there is a piece at the source square
        // Check if the piece belongs to the side to move
        if source_piece.color() != self.side_to_move {
            panic!("Piece at source square {} does not match side to move: {:?}", mv.src, self.side_to_move);
        }

        let piece = source_piece; // Now safe to unwrap and move
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
                self.get_piece_at_square_fast(captured_pawn_square),
                Some(Square::from_bit_index(captured_pawn_square))
            )
        } else {
            (
                target_piece.clone(),
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
            rook_castle_move: None, // Initialize as None, will be updated if castling
            captured_piece: captured_piece.clone(),
            captured_piece_square,
        };

        // Store the state before making any changes
        self.move_history.push(current_state);



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
                        self.move_piece_in_map(7, 5);
                        // Store the rook's move for undoing later
                        if let Some(state) = self.move_history.last_mut() {
                            state.rook_castle_move = Some(Move {
                                src: Square::H1,
                                target: Square::F1,
                                promotion: None,
                            });
                        }
                    } else if target_idx == 2 {  // c1 - queenside castle
                        debug_assert!(self.white_queenside_castle_rights, "Attempting queenside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.white_rooks ^= 1u64 | (1u64 << 3);  // a1 to d1
                        self.move_piece_in_map(0, 3);
                        // Store the rook's move for undoing later
                        if let Some(state) = self.move_history.last_mut() {
                            state.rook_castle_move = Some(Move {
                                src: Square::A1,
                                target: Square::D1,
                                promotion: None,
                            });
                        }
                    }
                },
                Piece::BlackKing => {
                    if target_idx == 62 {  // g8 - kingside castle
                        debug_assert!(self.black_kingside_castle_rights, "Attempting kingside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.black_rooks ^= (1u64 << 63) | (1u64 << 61);  // h8 to f8
                        self.move_piece_in_map(63, 61);
                        // Store the rook's move for undoing later
                        if let Some(state) = self.move_history.last_mut() {
                            state.rook_castle_move = Some(Move {
                                src: Square::H8,
                                target: Square::F8,
                                promotion: None,
                            });
                        }
                    } else if target_idx == 58 {  // c8 - queenside castle
                        debug_assert!(self.black_queenside_castle_rights, "Attempting queenside castle without rights");
                        // TODO: use Square enum instead of hardcoded indices
                        self.black_rooks ^= (1u64 << 56) | (1u64 << 59);  // a8 to d8
                        self.move_piece_in_map(56, 59);
                        // Store the rook's move for undoing later
                        if let Some(state) = self.move_history.last_mut() {
                            state.rook_castle_move = Some(Move {
                                src: Square::A8,
                                target: Square::D8,
                                promotion: None,
                            });
                        }
                    }
                },
                _ => {}
            }
        }

        // Remove captured piece if any (but not for castling which doesn't capture)
        if !is_castle {
            // Handle en-passant capture
            if is_en_passant {
                let captured_pawn_square_idx = if piece == Piece::WhitePawn {
                    target_idx - 8 // captured black pawn is one rank below
                } else {
                    target_idx + 8 // captured white pawn is one rank above
                };
                let captured_pawn_bit = 1u64 << captured_pawn_square_idx;
                if piece == Piece::WhitePawn {
                    self.black_pawns &= !captured_pawn_bit;
                    self.remove_piece_in_map(captured_pawn_square_idx);
                } else {
                    self.white_pawns &= !captured_pawn_bit;
                    self.remove_piece_in_map(captured_pawn_square_idx);
                }
            } else if target_piece.is_some() {
                let captured_bitboard = match target_piece.clone().unwrap() {
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
                self.remove_piece_in_map(target_idx);
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
        if target_piece.is_some() {
            match target_piece.unwrap() {
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

        {
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
        }
        self.move_piece_in_map(src_idx, target_idx);

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
            // Remove the pawn from the board
            *piece_bitboard ^= to_bit;
            self.remove_piece_in_map(target_idx);

            // Add the promoted piece to the target square
            let promotion_piece_bitboard = match promotion_piece {
                Piece::WhiteQueen => &mut self.white_queen,
                Piece::WhiteRook => &mut self.white_rooks,
                Piece::WhiteBishop => &mut self.white_bishops,
                Piece::WhiteKnight => &mut self.white_knights,
                Piece::BlackQueen => &mut self.black_queen,
                Piece::BlackRook => &mut self.black_rooks,
                Piece::BlackBishop => &mut self.black_bishops,
                Piece::BlackKnight => &mut self.black_knights,
                _ => panic!("Invalid promotion piece"),
            };
            *promotion_piece_bitboard |= to_bit;
            self.add_piece_in_map(target_idx, promotion_piece);
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

    pub fn get_raw_moves(&mut self, n: i32) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        self.get_all_raw_moves_append(&mut possible_moves);

        // Apply randomization if n >= 0
        if n >= 0 {
            let n = n as usize;
            if n == 0 {
                return Vec::new();
            } else {
                let mut rng = rand::thread_rng();
                return possible_moves
                    .iter()
                    .choose_multiple(&mut rng, n.min(possible_moves.len())).into_iter().cloned().collect();
            }
        }
        possible_moves
    }

    pub fn get_all_raw_moves_append(&mut self, possible_moves: &mut Vec<Move>) {
        use crate::move_generation::{w_pawns_able_to_push, b_pawns_able_to_push,
                                   w_pawns_able_to_double_push, b_pawns_able_to_double_push,
                                   w_pawns_attack_targets, b_pawns_attack_targets,
                                   w_pawns_en_passant_targets, b_pawns_en_passant_targets,
                                   knight_legal_moves, bishop_moves, rook_moves,
                                   queen_legal_moves, king_legal_moves};

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
                            let mv = board_utils::bitboard_squares_to_move(pawn, ep_square.to_bitboard());
                            if self.is_legal_move(&mv) {
                                possible_moves.push(mv);
                            }
                        }
                    }
                }
            }

            // Process regular pawn moves using existing board_utils functions
            self.bitboard_to_pawn_single_moves_append(possible_moves, moveable_pawns, true);
            self.bitboard_to_pawn_double_moves_append(possible_moves, double_moveable_pawns, true);
            self.bitboard_to_pawn_capture_moves_append(possible_moves, self.black_pawns, attacking_pawns, true);

            // Process each black knight separately
            let mut working_knights = self.black_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the processed bit

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_black);
                self.bitboard_to_moves_append(possible_moves, single_knight, moves);
            }

            // Process each black bishop separately
            let mut working_bishops = self.black_bishops;
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;
                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_moves(single_bishop, self.any_black, self.any_white);
                self.bitboard_to_moves_append(possible_moves, single_bishop, moves);
            }

            // Process each black rook separately
            let mut working_rooks = self.black_rooks;
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;
                let single_rook = 1u64 << rook_pos;
                let moves = rook_moves(single_rook, self.any_black, self.any_white);
                self.bitboard_to_moves_append(possible_moves, single_rook, moves);
            }

            // Process each black queen separately
            let mut working_queens = self.black_queen;
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;
                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_black, self.any_white);
                self.bitboard_to_moves_append(possible_moves, single_queen, moves);
            }

            // Process black king moves and castling
            let moves = king_legal_moves(self.black_king, self.any_black);
            self.bitboard_to_moves_append(possible_moves, self.black_king, moves);

            // Add castling moves if legal
            if self.is_castling_legal(true, false) {  // Black kingside castle
                let mv = Move {
                    src: Square::E8,
                    target: Square::G8,
                    promotion: None
                };
                if self.is_legal_move(&mv) {  // Check if the move is legal
                    possible_moves.push(mv);
                }
            }
            if self.is_castling_legal(false, false) {  // Black queenside castle
                let mv = Move {
                    src: Square::E8,
                    target: Square::C8,
                    promotion: None
                };
                if self.is_legal_move(&mv) {  // Check if the move is legal
                    possible_moves.push(mv);
                }
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
                            let mv = board_utils::bitboard_squares_to_move(pawn, ep_square.to_bitboard());
                            if self.is_legal_move(&mv) {
                                possible_moves.push(mv);
                            }
                        }
                    }
                }
            }

            self.bitboard_to_pawn_single_moves_append(possible_moves, moveable_pawns, false);
            self.bitboard_to_pawn_double_moves_append(possible_moves, double_moveable_pawns, false);
            self.bitboard_to_pawn_capture_moves_append(possible_moves, self.white_pawns, attacking_pawns, false);

            // Process each white knight separately
            let mut working_knights = self.white_knights;
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1;  // Clear the bit we are processing, the lowest significant bit that is set

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_white);
                self.bitboard_to_moves_append(possible_moves, single_knight, moves);
            }

            // Process each white bishop separately
            let mut working_bishops = self.white_bishops;
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;

                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_moves(single_bishop, self.any_white, self.any_black);
                self.bitboard_to_moves_append(possible_moves, single_bishop, moves);
            }

            // Process each white rook separately
            let mut working_rooks = self.white_rooks;
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;

                let single_rook = 1u64 << rook_pos;
                let moves = rook_moves(single_rook, self.any_white, self.any_black);
                self.bitboard_to_moves_append(possible_moves, single_rook, moves);
            }

            // Process each white queen separately (usually just one)
            let mut working_queens = self.white_queen;
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;

                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_white, self.any_black);
                self.bitboard_to_moves_append(possible_moves, single_queen, moves);
            }

            // Process white king (only one) with normal moves and castling
            let moves = king_legal_moves(self.white_king, self.any_white);
            self.bitboard_to_moves_append(possible_moves, self.white_king, moves);

            // Add castling moves if legal
            if self.is_castling_legal(true, true) {  // White kingside castle
                let mv = Move {
                    src: Square::E1,
                    target: Square::G1,
                    promotion: None
                };
                if self.is_legal_move(&mv) {  // Check if the move is legal
                    possible_moves.push(mv);
                }
            };
            if self.is_castling_legal(false, true) {  // White queenside castle
                let mv = Move {
                    src: Square::E1,
                    target: Square::C1,
                    promotion: None
                };
                if self.is_legal_move(&mv) {  // Check if the move is legal
                    possible_moves.push(mv);
                }
            };
        }

        // Filter the moves to only include legal ones (that get out of check if we're in check)
        // possible_moves.retain(|mv| self.is_legal_move(mv));
    }

    pub fn get_next_moves(&mut self, n: i32) -> Vec<String> {
        // Use get_raw_moves and convert to strings
        self.get_raw_moves(n).into_iter()
            .map(|mv| mv.to_string())
            .collect()
    }

    pub fn get_next_move(&mut self) -> String {
        // Default to getting one move
        self.get_next_moves(1)
            .into_iter()
            .next()
            .expect("No moves found, which should be impossible in current state")
    }

    pub fn bitboard_to_moves(&mut self, source_pieces: u64, target_squares: u64) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        self.bitboard_to_moves_append(&mut possible_moves, source_pieces, target_squares);
        possible_moves
    }

    // Generic helper function to convert a source bitboard and target bitboard into a list of moves
    pub fn bitboard_to_moves_append(&mut self, possible_moves: &mut Vec<Move>, source_pieces: u64, target_squares: u64) {
        // Assert that source_pieces contains exactly one piece (one bit set)
        debug_assert_eq!(source_pieces.count_ones(), 1,
            "bitboard_to_moves should be called with exactly one source piece, got {} pieces",
            source_pieces.count_ones());


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

                let mv = Move {
                    src: Square::from_bit_index(from_square),
                    target: Square::from_bit_index(to_square),
                    promotion: None,  // Promotions are handled separately
                };
                if self.is_legal_move(&mv) {
                    possible_moves.push(mv);
                }
            }
        }
    }

    pub fn bitboard_to_pawn_single_moves(&mut self, moveable_pawns: u64, is_black: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_single_moves_append(&mut moves, moveable_pawns, is_black);
        moves
    }

    /// Convert a bitboard of pawn single moves into a list of move strings
    pub fn bitboard_to_pawn_single_moves_append(&mut self, possible_moves: &mut Vec<Move>, moveable_pawns: u64, is_black: bool) {
        let mut working_pawns = moveable_pawns;

        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1;  // Clear the processed bit

            let to_square = if is_black {
                from_square - 8  // Black pawns move down
            } else {
                from_square + 8  // White pawns move up
            };

            let to_rank = to_square / 8;

            // Handle promotion
            if (is_black && to_rank == 0) || (!is_black && to_rank == 7) {
                let base_mv = Move {
                    src: Square::from_bit_index(from_square),
                    target: Square::from_bit_index(to_square),
                    promotion: None,
                };

                for promotion in [PieceType::Bishop, PieceType::Knight, PieceType::Rook, PieceType::Queen] {
                    let mv = Move {
                        promotion: Some(promotion),
                        ..base_mv.clone()
                    };
                    if self.is_legal_move(&mv) {
                        possible_moves.push(mv);
                    }
                }
            } else {
                let mv = Move {
                    src: Square::from_bit_index(from_square),
                    target: Square::from_bit_index(to_square),
                    promotion: None,
                };
                if self.is_legal_move(&mv) {
                    possible_moves.push(mv);
                }
            }
        }

    }

    /// Convert a bitboard of pawn double moves into a list of move strings
    pub fn bitboard_to_pawn_double_moves(&mut self, moveable_pawns: u64, is_black: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_double_moves_append(&mut moves, moveable_pawns, is_black);
        moves
    }

    pub fn bitboard_to_pawn_double_moves_append(&mut self, possible_moves: &mut Vec<Move>, moveable_pawns: u64, is_black: bool) {
        let mut working_pawns = moveable_pawns;

        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1;  // Clear the processed bit

            let to_square = if is_black {
                from_square - 16  // Black pawns move down two squares
            } else {
                from_square + 16  // White pawns move up two squares
            };

            let mv = Move {
                src: Square::from_bit_index(from_square),
                target: Square::from_bit_index(to_square),
                promotion: None,
            };
            // Check if the move is legal
            if self.is_legal_move(&mv) {
                possible_moves.push(mv);
            }
        }
    }

    pub fn bitboard_to_pawn_capture_moves(&mut self, source_pawns: u64, target_squares: u64, is_black: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_capture_moves_append(&mut moves, source_pawns, target_squares, is_black);
        moves
    }

    /// Convert a bitboard of pawn capture moves into a list of move strings
    pub fn bitboard_to_pawn_capture_moves_append(&mut self, possible_moves: &mut Vec<Move>, source_pawns: u64, target_squares: u64, is_black: bool) {
        let mut working_pawns = source_pawns;

        // For each pawn
        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1;  // Clear the processed bit

            // Get this pawn's possible captures
            let pawn = 1u64 << from_square;
            let mut captures = if is_black {
                b_pawns_attack_targets(pawn, target_squares)
            } else {
                w_pawns_attack_targets(pawn, target_squares)
            };

            // For each capture
            while captures != 0 {
                let to_square = captures.trailing_zeros() as u8;
                captures &= captures - 1;  // Clear the processed bit

                // Convert to algebraic notation
                let to_rank_int = to_square / 8 + 1;

                if (is_black && to_rank_int == 1) || (!is_black && to_rank_int == 8) {
                    // Promotion case
                    for promotion in [PieceType::Bishop, PieceType::Knight, PieceType::Rook, PieceType::Queen] {
                        let mv = Move {
                            src: Square::from_bit_index(from_square),
                            target: Square::from_bit_index(to_square),
                            promotion: Some(promotion),
                        };
                        if self.is_legal_move(&mv) {
                            // Only add the move if it's legal
                            possible_moves.push(mv);
                        }
                    }
                } else {
                    let mv = Move {
                        src: Square::from_bit_index(from_square),
                        target: Square::from_bit_index(to_square),
                        promotion: None,
                    };
                    if self.is_legal_move(&mv) {
                        possible_moves.push(mv);
                    }
                }
            }
        }
    }

    pub fn is_square_attacked(&self, square: u8, attacked_by_color: Color) -> bool {
        self.is_square_attacked_impl1(square, attacked_by_color)
    }

    pub fn is_square_attacked_impl1(&self, square: u8, attacked_by_color: Color) -> bool {
        let square_bb = 1u64 << square;

        if let Some(piece) = self.get_piece_at_square_fast(square) {
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
            if w_pawns_attack_targets(self.white_pawns, square_bb) != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            // Get all squares a knight could attack this square from
            if knight_legal_moves(square_bb, 0) & self.white_knights != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            if bishop_moves(square_bb, self.any_black, self.any_white) & (self.white_bishops | self.white_queen) != 0 {
                return true;
            }

            // Check for rook/straight queen attacks
            if rook_moves(square_bb, self.any_black, self.any_white) & (self.white_rooks | self.white_queen) != 0 {
                return true;
            }

            // Check for king attacks - king can only be blocked by its own pieces
            if king_legal_moves(square_bb, self.any_black) & self.white_king != 0 {
                return true;
            }
        } else {
            // At this point we know the square is empty or occupied by a white piece

            // Check for pawn attacks
            if b_pawns_attack_targets(self.black_pawns, square_bb) != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            if knight_legal_moves(square_bb, 0) & self.black_knights != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            if bishop_moves(square_bb, self.any_white, self.any_black) & (self.black_bishops | self.black_queen) != 0 {
                return true;
            }

            // Check for rook/straight queen attacks
            if rook_moves(square_bb, self.any_white, self.any_black) & (self.black_rooks | self.black_queen) != 0 {
                return true;
            }

            // Check for king attacks - king can only be blocked by its own pieces
            if king_legal_moves(square_bb, self.any_white) & self.black_king != 0 {
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

    pub fn is_legal_move(&mut self, mv: &Move) -> bool {
        // If we're not in check, all moves are legal (for now - we'll add more restrictions later)
        if !self.white_king_in_check && !self.black_king_in_check {
            if self.side_to_move == Color::White {
                assert!(!self.white_king_in_check);
                self.apply_move(mv);
                // If the move doesn't put the white king in check, it's legal
                let ret = !self.white_king_in_check;
                self.undo_last_move();
                return ret;
            } else {
                assert!(!self.black_king_in_check);
                self.apply_move(mv);
                // If the move doesn't put the black king in check, it's legal
                let ret = !self.black_king_in_check;
                self.undo_last_move();
                return  ret;
            }
        }

        // At this point we know we are in check.

        // We are in check, so make a copy of the board and try the move

        // The move is legal if it got us out of check
        if self.white_king_in_check {
            assert_eq!(self.side_to_move, Color::White, "White king is in check, but side to move is not White");
            self.apply_move(mv);
            let ret = !self.white_king_in_check;
            self.undo_last_move();
            ret
        } else {
            assert!(self.black_king_in_check);
            assert_eq!(self.side_to_move, Color::Black, "Black king is in check, but side to move is not Black");
            // Assumed we are in check and moving black
            self.apply_move(mv);
            let ret = !self.black_king_in_check;
            self.undo_last_move();
            ret
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
        let mut moves: Vec<Move> = Vec::with_capacity(218);
        self.generate_legal_moves_append(&mut moves);

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
    fn generate_legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        self.generate_legal_moves_append(&mut moves);
        moves
    }

    fn generate_legal_moves_append(&mut self, moves: &mut Vec<Move>) {
        self.get_all_raw_moves_append(moves);
    }

    /// Undoes the last move made, restoring the board to its previous state
    pub fn undo_last_move(&mut self) {
        if let Some(state) = self.move_history.pop() {
            let mv = &state.last_move;
            
            let piece = self.get_piece_at_square_fast(mv.target.to_bit_index())
                .expect("No piece at target square when undoing move");

            let src_bit = mv.src.to_bitboard();
            let src_index = mv.src.to_bit_index();
            let target_bit = mv.target.to_bitboard();
            let target_idx = mv.target.to_bit_index();

            // Handle promotions first
            if mv.promotion.is_some() {
                // Remove the promoted piece
                let promoted_piece_bb = match piece {
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
                *promoted_piece_bb &= !target_bit;  // Remove from target square
                self.remove_piece_in_map(target_idx);

                // Restore the pawn
                if piece.color() == Color::White {
                    self.white_pawns |= src_bit;
                    self.add_piece_in_map(src_index, Piece::WhitePawn);
                } else {
                    self.black_pawns |= src_bit;
                    self.add_piece_in_map(src_index, Piece::BlackPawn);
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
                self.move_piece_in_map(target_idx, src_index);
            }

            // If this was a castling move, undo the rook's move as well
            if let Some(rook_mv) = &state.rook_castle_move {
                let rook_src_bit = rook_mv.src.to_bitboard();
                let rook_target_bit = rook_mv.target.to_bitboard();
                let rook_src_index = rook_mv.src.to_bit_index();
                let rook_target_index = rook_mv.target.to_bit_index();

                // Determine if this was a white or black rook
                if piece == Piece::WhiteKing {
                    self.white_rooks &= !rook_target_bit; // Remove from target square
                    self.white_rooks |= rook_src_bit;     // Add to source square
                } else {
                    self.black_rooks &= !rook_target_bit; // Remove from target square
                    self.black_rooks |= rook_src_bit;     // Add to source square
                }
                self.move_piece_in_map(rook_target_index, rook_src_index);
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
                let square = state.captured_piece_square.unwrap();
                let square_idx = square.to_bit_index();
                *captured_bb |= &square.to_bitboard();
                self.add_piece_in_map(square_idx, state.captured_piece.unwrap());
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
    pub fn is_checkmate(&mut self) -> bool {
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
