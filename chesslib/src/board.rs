use crate::board_utils;
use crate::board_utils::get_starting_board;
use crate::move_generation::{
    b_pawns_attack_targets, bishop_moves, king_legal_moves, knight_legal_moves, rook_moves,
    w_pawns_attack_targets,
};
use crate::types::{Color, Move, Piece, PieceType, Square, SPACE};

#[derive(Clone, Debug, PartialEq)]
pub struct BoardState {
    pub white_kingside_castle_rights: bool,
    pub white_queenside_castle_rights: bool,
    pub black_kingside_castle_rights: bool,
    pub black_queenside_castle_rights: bool,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub last_move: Move,
    pub rook_castle_move: Option<Move>, // Stores the rook's move during castling
    pub captured_piece: Option<Piece>,
    pub captured_piece_square: Option<Square>,
}

#[derive(Debug, Clone)]
pub struct Board {
    /// Per-piece-type bitboards. Each entry is the set of squares
    /// occupied by that piece type, regardless of colour. Index with
    /// `PieceType::idx()`. To get e.g. only the white knights, AND
    /// `pieces[Knight.idx()]` with `colors[White.idx()]`. The
    /// readability accessors (`white_pawns()`, `black_king()`, …) below
    /// do exactly that and should be preferred at call sites.
    ///
    /// Storing pieces as `[u64; 6] + [u64; 2]` rather than as twelve
    /// named fields collapses what would otherwise be a twelve-arm
    /// match into a single array index. This is the layout used by
    /// every popular open-source Rust chess engine I checked (Pleco,
    /// Carp, Cozy-Chess).
    pub pieces: [u64; 6],

    /// Per-colour bitboards. `colors[White.idx()]` is the set of
    /// squares occupied by *any* white piece; same for Black. Index
    /// with `Color::idx()`.
    ///
    /// Kept in lock-step with `pieces`: a square is set in `colors[c]`
    /// iff it is set in some `pieces[pt]` AND that piece is colour `c`.
    /// `apply_move` and `undo_last_move` are the only places that may
    /// mutate these arrays, and both update `pieces` and `colors` in
    /// the same XOR pair to preserve the invariant.
    pub colors: [u64; 2],

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

    /// Plies since the last pawn move or capture. Resets to 0 on either.
    /// Drives the fifty-move rule (not yet enforced) and FEN round-trip.
    pub halfmove_clock: u32,

    /// Full move counter: starts at 1 and increments after each black move
    /// (FEN convention). Used only for FEN round-trip.
    pub fullmove_number: u32,

    /// Represents a snapshot of board state that can be restored when undoing a move
    pub move_history: Vec<BoardState>,

    pub piece_map: [Option<Piece>; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        get_starting_board()
    }

    // -------------------------------------------------------------------
    // Bitboard accessor methods.
    //
    // These compose the per-piece-type and per-colour arrays into the
    // twelve named bitboards that chess code naturally wants ("white's
    // pawns", "black's king"). Call sites SHOULD prefer these named
    // accessors over poking the arrays directly — they read like chess
    // and the compiler inlines them away. Reach into `pieces` /
    // `colors` directly only in generic loops or when mutating
    // (mutation must happen through both arrays in lock-step).
    //
    // Storage convention: `pieces[pt] & colors[c]` is the set of
    // squares occupied by colour `c` pieces of type `pt`. The two
    // arrays are independently informative: `pieces[Pawn]` is "all
    // pawns regardless of colour", `colors[White]` is "every white
    // piece regardless of type".
    // -------------------------------------------------------------------

    /// Squares occupied by a `(piece_type, colour)` pair.
    #[inline]
    pub fn piece_bb(&self, pt: PieceType, c: Color) -> u64 {
        self.pieces[pt.idx()] & self.colors[c.idx()]
    }

    #[inline]
    pub fn white_pawns(&self) -> u64 {
        self.piece_bb(PieceType::Pawn, Color::White)
    }
    #[inline]
    pub fn white_knights(&self) -> u64 {
        self.piece_bb(PieceType::Knight, Color::White)
    }
    #[inline]
    pub fn white_bishops(&self) -> u64 {
        self.piece_bb(PieceType::Bishop, Color::White)
    }
    #[inline]
    pub fn white_rooks(&self) -> u64 {
        self.piece_bb(PieceType::Rook, Color::White)
    }
    #[inline]
    pub fn white_queen(&self) -> u64 {
        self.piece_bb(PieceType::Queen, Color::White)
    }
    #[inline]
    pub fn white_king(&self) -> u64 {
        self.piece_bb(PieceType::King, Color::White)
    }

    #[inline]
    pub fn black_pawns(&self) -> u64 {
        self.piece_bb(PieceType::Pawn, Color::Black)
    }
    #[inline]
    pub fn black_knights(&self) -> u64 {
        self.piece_bb(PieceType::Knight, Color::Black)
    }
    #[inline]
    pub fn black_bishops(&self) -> u64 {
        self.piece_bb(PieceType::Bishop, Color::Black)
    }
    #[inline]
    pub fn black_rooks(&self) -> u64 {
        self.piece_bb(PieceType::Rook, Color::Black)
    }
    #[inline]
    pub fn black_queen(&self) -> u64 {
        self.piece_bb(PieceType::Queen, Color::Black)
    }
    #[inline]
    pub fn black_king(&self) -> u64 {
        self.piece_bb(PieceType::King, Color::Black)
    }

    /// Every white piece, regardless of type. Equivalent to ORing all
    /// six white per-type bitboards together, but stored directly so
    /// no recomputation is needed.
    #[inline]
    pub fn any_white(&self) -> u64 {
        self.colors[Color::White.idx()]
    }

    /// Every black piece, regardless of type. Same convention.
    #[inline]
    pub fn any_black(&self) -> u64 {
        self.colors[Color::Black.idx()]
    }

    /// Every empty square — the complement of the union of both
    /// colours. Computed on demand (one OR, one NOT).
    #[inline]
    pub fn empty(&self) -> u64 {
        !(self.colors[0] | self.colors[1])
    }

    // -------------------------------------------------------------------
    // Bitboard mutation helpers.
    //
    // Every bitboard mutation in the codebase MUST go through one of
    // these three helpers (or update both `pieces` and `colors`
    // explicitly in lock-step). They are the only API for changing
    // piece occupancy and they exist to make the dual-array invariant
    // un-skippable: a square is set in `colors[c]` iff it is set in
    // some `pieces[pt]` for a piece of colour `c`.
    //
    // Call site preference: `xor_piece` for normal moves (toggles both
    // squares at once), `set_piece` for adding a piece, `clear_piece`
    // for removing one. All three take a `Piece` (which encodes both
    // type and colour) so the caller can't accidentally update only one
    // of the two arrays.
    // -------------------------------------------------------------------

    /// Toggle (XOR) the given bit-set in both `pieces[piece.piece_type()]`
    /// and `colors[piece.color()]`. The natural shape of a normal move:
    /// XOR `from_bit | to_bit` and the piece moves from one square to
    /// the other in a single step.
    #[inline]
    fn xor_piece(&mut self, piece: Piece, bits: u64) {
        self.pieces[piece.piece_type().idx()] ^= bits;
        self.colors[piece.color().idx()] ^= bits;
    }

    /// Set (OR) the given bits for this piece. Use when adding a piece
    /// to currently-unoccupied squares (e.g. promotion: a queen appears
    /// on the promotion square).
    #[inline]
    fn set_piece(&mut self, piece: Piece, bits: u64) {
        self.pieces[piece.piece_type().idx()] |= bits;
        self.colors[piece.color().idx()] |= bits;
    }

    /// Clear (AND-NOT) the given bits for this piece. Use when removing
    /// a piece (capture, or pawn vanishing during promotion).
    #[inline]
    fn clear_piece(&mut self, piece: Piece, bits: u64) {
        self.pieces[piece.piece_type().idx()] &= !bits;
        self.colors[piece.color().idx()] &= !bits;
    }

    /// Replace the entire bitboard for a `(PieceType, Color)` kind with
    /// a new value: every square currently occupied by that kind is
    /// cleared first, then `new_bb` is set. Both `pieces[]` and
    /// `colors[]` are updated in lock-step.
    ///
    /// Intended for hand-built test positions, where `Board { foo: bb,
    /// ..get_starting_board() }` struct-update syntax used to do the
    /// job. The new layout doesn't allow that pattern, so this helper
    /// takes its place. Production code paths (apply_move, load_fen,
    /// etc.) mutate bitboards through the per-move helpers and should
    /// not call this.
    ///
    /// **Caller must call `rebuild_piece_map()` afterwards** (or chain
    /// several `replace_pieces_of_kind` calls and rebuild once at the
    /// end) so the mailbox stays in sync with the bitboards.
    pub fn replace_pieces_of_kind(&mut self, pt: PieceType, color: Color, new_bb: u64) {
        let current = self.piece_bb(pt, color);
        self.pieces[pt.idx()] &= !current;
        self.colors[color.idx()] &= !current;
        self.pieces[pt.idx()] |= new_bb;
        self.colors[color.idx()] |= new_bb;
    }

    #[inline(always)]
    pub fn get_piece_at_square_fast(&self, sq: u8) -> Option<Piece> {
        self.piece_map[sq as usize]
    }

    fn move_piece_in_map(&mut self, from: u8, to: u8) {
        let p = self.piece_map[from as usize];
        self.piece_map[from as usize] = None;
        self.piece_map[to as usize] = p;
    }

    fn remove_piece_in_map(&mut self, at: u8) {
        self.piece_map[at as usize] = None;
    }

    fn add_piece_in_map(&mut self, at: u8, p: Piece) {
        self.piece_map[at as usize] = Some(p);
    }

    /// Recomputes the `piece_map` mailbox from the bitboards.
    /// Call after you bulk-rewrite the bitboards (e.g. just after
    /// `load_fen`) so the square-to-piece lookup table agrees with
    /// the bitboard truth.
    ///
    /// This used to be a twelve-arm `fill!` macro, one per named
    /// bitboard field. With pieces stored as `[u64; 6] + [u64; 2]`
    /// the same work is just two nested loops.
    pub fn rebuild_piece_map(&mut self) {
        self.piece_map = [None; 64];
        for pt in PieceType::ALL {
            for &color in &[Color::White, Color::Black] {
                let mut bb = self.piece_bb(pt, color);
                let piece = Piece::from_type_and_color(pt, color);
                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    self.piece_map[sq] = Some(piece);
                    bb &= bb - 1; // clear the lowest set bit
                }
            }
        }
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
            None => SPACE,
        }
    }

    pub fn get_piece_at_coordinate_as_fen(&self, coordinate: &str) -> &'static str {
        let bitboard_index = Square::try_from(coordinate).unwrap().to_bit_index();
        match self.get_piece_at_square_fast(bitboard_index) {
            Some(piece) => piece.to_fen(),
            None => SPACE,
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
            panic!(
                "Piece at source square {} does not match side to move: {:?}",
                mv.src, self.side_to_move
            );
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
            }
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
                Some(Square::from_bit_index(captured_pawn_square)),
            )
        } else {
            (target_piece, Some(mv.target))
        };

        // Now store the state with the correct captured piece
        let current_state = BoardState {
            white_kingside_castle_rights: self.white_kingside_castle_rights,
            white_queenside_castle_rights: self.white_queenside_castle_rights,
            black_kingside_castle_rights: self.black_kingside_castle_rights,
            black_queenside_castle_rights: self.black_queenside_castle_rights,
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
            last_move: *mv,
            rook_castle_move: None, // Initialize as None, will be updated if castling
            captured_piece,
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

        // Handle castling rook movements before we do any other piece movements.
        // The king's own move (e1→g1, e8→c8, etc.) is applied later as a
        // normal piece move; here we only deal with the *rook* leg of
        // castling, which is the second half of the castling primitive
        // and isn't expressible as a regular UCI Move.
        if is_castle {
            // (king square index, rook src square, rook target square,
            //  Piece::{Color}Rook to toggle)
            let (king_target, rook_src, rook_tgt, rook_piece) = match (piece, target_idx) {
                // White kingside: king e1→g1, rook h1→f1
                (Piece::WhiteKing, 6) => (Square::G1, Square::H1, Square::F1, Piece::WhiteRook),
                // White queenside: king e1→c1, rook a1→d1
                (Piece::WhiteKing, 2) => (Square::C1, Square::A1, Square::D1, Piece::WhiteRook),
                // Black kingside: king e8→g8, rook h8→f8
                (Piece::BlackKing, 62) => (Square::G8, Square::H8, Square::F8, Piece::BlackRook),
                // Black queenside: king e8→c8, rook a8→d8
                (Piece::BlackKing, 58) => (Square::C8, Square::A8, Square::D8, Piece::BlackRook),
                _ => unreachable!("is_castle was true but target_idx didn't match any castle"),
            };
            let _ = king_target; // silence unused — the king itself is moved below as a normal piece.

            // XOR (rook_src | rook_tgt) flips both squares in one step:
            // the rook leaves rook_src and appears on rook_tgt.
            self.xor_piece(rook_piece, rook_src.to_bitboard() | rook_tgt.to_bitboard());
            self.move_piece_in_map(rook_src.to_bit_index(), rook_tgt.to_bit_index());

            // Save the rook leg into BoardState so undo can reverse it.
            if let Some(state) = self.move_history.last_mut() {
                state.rook_castle_move = Some(Move {
                    src: rook_src,
                    target: rook_tgt,
                    promotion: None,
                });
            }
        }

        // Remove captured piece if any (castling doesn't capture, so skip).
        if !is_castle {
            if is_en_passant {
                // En passant: the captured pawn is NOT on the move's
                // target square — it's one rank "behind" the target
                // from the capturing pawn's POV. White captures by
                // moving up, so the black pawn is one rank below
                // target. Black captures by moving down, so the white
                // pawn is one rank above target.
                let captured_pawn_square_idx = if piece == Piece::WhitePawn {
                    target_idx - 8
                } else {
                    target_idx + 8
                };
                let captured_pawn_bit = 1u64 << captured_pawn_square_idx;
                let victim = if piece == Piece::WhitePawn {
                    Piece::BlackPawn
                } else {
                    Piece::WhitePawn
                };
                self.clear_piece(victim, captured_pawn_bit);
                self.remove_piece_in_map(captured_pawn_square_idx);
            } else if let Some(victim) = target_piece {
                // Normal capture: the victim sits on `target_idx` and
                // is replaced by the moving piece below.
                self.clear_piece(victim, to_bit);
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
                if src_idx == 7 {
                    // h1
                    self.white_kingside_castle_rights = false;
                // TODO: use Square enum instead of hardcoded indices
                } else if src_idx == 0 {
                    // a1
                    self.white_queenside_castle_rights = false;
                }
            }
            Piece::BlackRook => {
                // Check if it's the kingside or queenside rook
                // TODO: use Square enum instead of hardcoded indices
                if src_idx == 63 {
                    // h8
                    self.black_kingside_castle_rights = false;
                // TODO: use Square enum instead of hardcoded indices
                } else if src_idx == 56 {
                    // a8
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
                    if target_idx == 7 {
                        // h1
                        self.white_kingside_castle_rights = false;
                    // TODO: use Square enum instead of hardcoded indices
                    } else if target_idx == 0 {
                        // a1
                        self.white_queenside_castle_rights = false;
                    }
                }
                Piece::BlackRook => {
                    // TODO: use Square enum instead of hardcoded indices
                    if target_idx == 63 {
                        // h8
                        self.black_kingside_castle_rights = false;
                    // TODO: use Square enum instead of hardcoded indices
                    } else if target_idx == 56 {
                        // a8
                        self.black_queenside_castle_rights = false;
                    }
                }
                _ => {}
            }
        }

        // Move the moving piece itself: XOR (from | to) flips both
        // squares so the piece leaves `from` and appears on `to` in a
        // single bitboard mutation.
        self.xor_piece(piece, from_bit | to_bit);
        self.move_piece_in_map(src_idx, target_idx);

        // Set en-passant target square for double pawn moves
        self.en_passant_target = match piece {
            Piece::WhitePawn if src_idx / 8 == 1 && target_idx / 8 == 3 => {
                // White pawn double push
                Some(Square::from_bit_index(src_idx + 8))
            }
            Piece::BlackPawn if src_idx / 8 == 6 && target_idx / 8 == 4 => {
                // Black pawn double push
                Some(Square::from_bit_index(src_idx - 8))
            }
            _ => None,
        };

        // Handle promotions: at this point the pawn has already been
        // moved to `to_bit` (the rank-8/rank-1 promotion square). We
        // remove that pawn and put the promotion piece on the same
        // square. Promotions are always to Q / R / B / N — pawn and
        // king are not legal targets.
        if let Some(promotion_kind) = mv.promotion {
            assert!(
                piece == Piece::WhitePawn || piece == Piece::BlackPawn,
                "Promotion can only be applied to pawns"
            );
            assert!(
                matches!(
                    promotion_kind,
                    PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight
                ),
                "Invalid promotion piece type: {promotion_kind:?}"
            );
            let promotion_piece = Piece::from_type_and_color(promotion_kind, piece.color());

            // Remove the pawn from the promotion square and stamp the
            // promoted piece in its place.
            self.clear_piece(piece, to_bit);
            self.remove_piece_in_map(target_idx);
            self.set_piece(promotion_piece, to_bit);
            self.add_piece_in_map(target_idx, promotion_piece);
        }

        // No more bitboard mutations from here on. `pieces[]` and
        // `colors[]` are the source of truth — the per-array helpers
        // kept them in sync, so there is no separate "composite" cache
        // to refresh.
        self.update_check_state();

        // Update FEN counters before flipping side to move so we can use
        // self.side_to_move (the side that just moved) directly.
        let is_pawn_move = matches!(piece, Piece::WhitePawn | Piece::BlackPawn);
        let is_capture = captured_piece.is_some();
        if is_pawn_move || is_capture {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock = self.halfmove_clock.saturating_add(1);
        }
        if self.side_to_move == Color::Black {
            // Black just moved → increment fullmove number per FEN convention.
            self.fullmove_number = self.fullmove_number.saturating_add(1);
        }

        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    /// Recompute `colors[]` from the `pieces[]` bitboards using the
    /// `piece_map` mailbox to resolve which colour each occupied square
    /// belongs to.
    ///
    /// This is only useful in two situations:
    ///
    ///   1. `load_fen`, which sets `pieces[]` directly while building
    ///      a position from scratch and then needs `colors[]` filled in.
    ///   2. Any future bulk-rewriter of the bitboards (analysis tools,
    ///      tests).
    ///
    /// **Do not call this from `apply_move` or `undo_last_move`** —
    /// they maintain `colors[]` in lock-step with `pieces[]` via the
    /// mutation helpers, so this would be redundant work.
    ///
    /// Precondition: `piece_map` must already be in sync with
    /// `pieces[]`. Callers who set `pieces[]` directly should call
    /// `rebuild_piece_map()` first, then this.
    pub fn update_composite_bitboards(&mut self) {
        let mut white = 0u64;
        let mut black = 0u64;
        for sq in 0..64u8 {
            if let Some(p) = self.piece_map[sq as usize] {
                let bit = 1u64 << sq;
                match p.color() {
                    Color::White => white |= bit,
                    Color::Black => black |= bit,
                }
            }
        }
        self.colors[Color::White.idx()] = white;
        self.colors[Color::Black.idx()] = black;
    }

    pub fn apply_move_from_string(&mut self, mv_str: &str) {
        if let Ok(mv) = Move::try_from(mv_str) {
            self.apply_move(&mv);
        } else {
            panic!("Invalid move string: {mv_str}");
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

    pub fn convert_moves(
        moves: impl Iterator<Item = String>,
    ) -> impl Iterator<Item = Result<Move, &'static str>> {
        moves.map(|mv| Move::try_from(mv.as_str()))
    }

    pub fn get_raw_moves(&mut self, n: i32) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        self.get_all_raw_moves_append(&mut possible_moves);

        let n = n as usize;
        if n == 0 {
            Vec::new()
        } else {
            // Apply move ordering by evaluation score
            let side_to_move = self.side_to_move; // Color is Copy; no clone needed
            possible_moves.sort_by(|a, b| {
                // Apply each move, get evaluation, then undo
                self.apply_move(a);
                let score_a = self.evaluate();
                self.undo_last_move();

                self.apply_move(b);
                let score_b = self.evaluate();
                self.undo_last_move();

                // For white, higher scores are better (descending)
                // For black, lower scores are better (ascending)
                match side_to_move {
                    Color::White => score_b.cmp(&score_a), // descending
                    Color::Black => score_a.cmp(&score_b), // ascending
                }
            });
            if n >= 1 {
                // Return first n moves
                possible_moves.into_iter().take(n).collect()
            } else {
                possible_moves
            }
        }
    }

    pub fn get_all_raw_moves_append(&mut self, possible_moves: &mut Vec<Move>) {
        use crate::move_generation::{
            b_pawns_able_to_double_push, b_pawns_able_to_push, b_pawns_attack_targets,
            b_pawns_en_passant_targets, bishop_moves, king_legal_moves, knight_legal_moves,
            queen_legal_moves, rook_moves, w_pawns_able_to_double_push, w_pawns_able_to_push,
            w_pawns_attack_targets, w_pawns_en_passant_targets,
        };

        if self.side_to_move == Color::Black {
            // Get all possible pawn moves
            let moveable_pawns = b_pawns_able_to_push(self.black_pawns(), self.empty());
            let double_moveable_pawns =
                b_pawns_able_to_double_push(self.black_pawns(), self.empty());
            let attacking_pawns = b_pawns_attack_targets(self.black_pawns(), self.any_white());

            // Add en-passant moves if available
            if let Some(ep_square) = self.en_passant_target {
                let ep_targets =
                    b_pawns_en_passant_targets(self.black_pawns(), ep_square.to_bitboard());
                if ep_targets != 0 {
                    // Find the source pawns that can make the en-passant capture
                    let mut working_pawns = self.black_pawns();
                    while working_pawns != 0 {
                        let from_square = working_pawns.trailing_zeros() as u8;
                        working_pawns &= working_pawns - 1; // Clear the processed bit
                        let pawn = 1u64 << from_square;
                        if b_pawns_en_passant_targets(pawn, ep_square.to_bitboard()) != 0 {
                            let mv = board_utils::bitboard_squares_to_move(
                                pawn,
                                ep_square.to_bitboard(),
                            );
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
            self.bitboard_to_pawn_capture_moves_append(
                possible_moves,
                self.black_pawns(),
                attacking_pawns,
                true,
            );

            // Process each black knight separately
            let mut working_knights = self.black_knights();
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1; // Clear the processed bit

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_black());
                self.bitboard_to_moves_append(possible_moves, single_knight, moves);
            }

            // Process each black bishop separately
            let mut working_bishops = self.black_bishops();
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;
                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_moves(single_bishop, self.any_black(), self.any_white());
                self.bitboard_to_moves_append(possible_moves, single_bishop, moves);
            }

            // Process each black rook separately
            let mut working_rooks = self.black_rooks();
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;
                let single_rook = 1u64 << rook_pos;
                let moves = rook_moves(single_rook, self.any_black(), self.any_white());
                self.bitboard_to_moves_append(possible_moves, single_rook, moves);
            }

            // Process each black queen separately
            let mut working_queens = self.black_queen();
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;
                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_black(), self.any_white());
                self.bitboard_to_moves_append(possible_moves, single_queen, moves);
            }

            // Process black king moves and castling
            let moves = king_legal_moves(self.black_king(), self.any_black());
            self.bitboard_to_moves_append(possible_moves, self.black_king(), moves);

            // Add castling moves if legal
            if self.is_castling_legal(true, false) {
                // Black kingside castle
                let mv = Move {
                    src: Square::E8,
                    target: Square::G8,
                    promotion: None,
                };
                if self.is_legal_move(&mv) {
                    // Check if the move is legal
                    possible_moves.push(mv);
                }
            }
            if self.is_castling_legal(false, false) {
                // Black queenside castle
                let mv = Move {
                    src: Square::E8,
                    target: Square::C8,
                    promotion: None,
                };
                if self.is_legal_move(&mv) {
                    // Check if the move is legal
                    possible_moves.push(mv);
                }
            }
        } else {
            // Get all possible pawn moves
            let moveable_pawns = w_pawns_able_to_push(self.white_pawns(), self.empty());
            let double_moveable_pawns =
                w_pawns_able_to_double_push(self.white_pawns(), self.empty());
            let attacking_pawns = w_pawns_attack_targets(self.white_pawns(), self.any_black());

            // Add en-passant moves if available
            if let Some(ep_square) = self.en_passant_target {
                let ep_targets =
                    w_pawns_en_passant_targets(self.white_pawns(), ep_square.to_bitboard());
                if ep_targets != 0 {
                    // Find the source pawns that can make the en-passant capture
                    let mut working_pawns = self.white_pawns();
                    while working_pawns != 0 {
                        let from_square = working_pawns.trailing_zeros() as u8;
                        working_pawns &= working_pawns - 1; // Clear the processed bit
                        let pawn = 1u64 << from_square;
                        if w_pawns_en_passant_targets(pawn, ep_square.to_bitboard()) != 0 {
                            let mv = board_utils::bitboard_squares_to_move(
                                pawn,
                                ep_square.to_bitboard(),
                            );
                            if self.is_legal_move(&mv) {
                                possible_moves.push(mv);
                            }
                        }
                    }
                }
            }

            self.bitboard_to_pawn_single_moves_append(possible_moves, moveable_pawns, false);
            self.bitboard_to_pawn_double_moves_append(possible_moves, double_moveable_pawns, false);
            self.bitboard_to_pawn_capture_moves_append(
                possible_moves,
                self.white_pawns(),
                attacking_pawns,
                false,
            );

            // Process each white knight separately
            let mut working_knights = self.white_knights();
            while working_knights != 0 {
                let knight_pos = working_knights.trailing_zeros() as u8;
                working_knights &= working_knights - 1; // Clear the bit we are processing, the lowest significant bit that is set

                let single_knight = 1u64 << knight_pos;
                // Get all legal moves for this knight (including both empty squares and captures)
                let moves = knight_legal_moves(single_knight, self.any_white());
                self.bitboard_to_moves_append(possible_moves, single_knight, moves);
            }

            // Process each white bishop separately
            let mut working_bishops = self.white_bishops();
            while working_bishops != 0 {
                let bishop_pos = working_bishops.trailing_zeros() as u8;
                working_bishops &= working_bishops - 1;

                let single_bishop = 1u64 << bishop_pos;
                let moves = bishop_moves(single_bishop, self.any_white(), self.any_black());
                self.bitboard_to_moves_append(possible_moves, single_bishop, moves);
            }

            // Process each white rook separately
            let mut working_rooks = self.white_rooks();
            while working_rooks != 0 {
                let rook_pos = working_rooks.trailing_zeros() as u8;
                working_rooks &= working_rooks - 1;

                let single_rook = 1u64 << rook_pos;
                let moves = rook_moves(single_rook, self.any_white(), self.any_black());
                self.bitboard_to_moves_append(possible_moves, single_rook, moves);
            }

            // Process each white queen separately (usually just one)
            let mut working_queens = self.white_queen();
            while working_queens != 0 {
                let queen_pos = working_queens.trailing_zeros() as u8;
                working_queens &= working_queens - 1;

                let single_queen = 1u64 << queen_pos;
                let moves = queen_legal_moves(single_queen, self.any_white(), self.any_black());
                self.bitboard_to_moves_append(possible_moves, single_queen, moves);
            }

            // Process white king (only one) with normal moves and castling
            let moves = king_legal_moves(self.white_king(), self.any_white());
            self.bitboard_to_moves_append(possible_moves, self.white_king(), moves);

            // Add castling moves if legal
            if self.is_castling_legal(true, true) {
                // White kingside castle
                let mv = Move {
                    src: Square::E1,
                    target: Square::G1,
                    promotion: None,
                };
                if self.is_legal_move(&mv) {
                    // Check if the move is legal
                    possible_moves.push(mv);
                }
            };
            if self.is_castling_legal(false, true) {
                // White queenside castle
                let mv = Move {
                    src: Square::E1,
                    target: Square::C1,
                    promotion: None,
                };
                if self.is_legal_move(&mv) {
                    // Check if the move is legal
                    possible_moves.push(mv);
                }
            };
        }

        // Filter the moves to only include legal ones (that get out of check if we're in check)
        // possible_moves.retain(|mv| self.is_legal_move(mv));
    }

    pub fn get_next_moves(&mut self, n: i32) -> Vec<String> {
        // Use get_raw_moves and convert to strings
        self.get_raw_moves(n)
            .into_iter()
            .map(|mv| mv.to_string())
            .collect()
    }

    /// Returns one legal move from the current position as a string in
    /// long algebraic notation (`"e2e4"`, `"e7e8q"` for promotions).
    /// Useful for "play any legal move" scenarios in tests and toy
    /// drivers — no search, no evaluation, just move generation.
    /// Panics if the position has no legal moves (stalemate or
    /// checkmate); call `is_checkmate` / `is_stalemate` first if you
    /// need to handle that.
    pub fn get_next_move_random(&mut self) -> String {
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
    pub fn bitboard_to_moves_append(
        &mut self,
        possible_moves: &mut Vec<Move>,
        source_pieces: u64,
        target_squares: u64,
    ) {
        // 0 source pieces is fine — the loop simply does nothing. The original
        // assertion required exactly 1, but the implementation iterates over
        // any count, and search legitimately reaches positions with 0 pawns
        // (e.g. K+P endgames after the pawn is captured).
        let mut working_source = source_pieces;

        // For each source piece
        while working_source != 0 {
            let from_square = working_source.trailing_zeros() as u8;
            working_source &= working_source - 1; // Clear the processed bit

            // For each target square
            let mut current_targets = target_squares;
            while current_targets != 0 {
                let to_square = current_targets.trailing_zeros() as u8;
                current_targets &= current_targets - 1; // Clear the processed bit

                let mv = Move {
                    src: Square::from_bit_index(from_square),
                    target: Square::from_bit_index(to_square),
                    promotion: None, // Promotions are handled separately
                };
                if self.is_legal_move(&mv) {
                    possible_moves.push(mv);
                }
            }
        }
    }

    pub fn bitboard_to_pawn_single_moves(
        &mut self,
        moveable_pawns: u64,
        is_black: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_single_moves_append(&mut moves, moveable_pawns, is_black);
        moves
    }

    /// Convert a bitboard of pawn single moves into a list of move strings
    pub fn bitboard_to_pawn_single_moves_append(
        &mut self,
        possible_moves: &mut Vec<Move>,
        moveable_pawns: u64,
        is_black: bool,
    ) {
        let mut working_pawns = moveable_pawns;

        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1; // Clear the processed bit

            let to_square = if is_black {
                from_square - 8 // Black pawns move down
            } else {
                from_square + 8 // White pawns move up
            };

            let to_rank = to_square / 8;

            // Handle promotion
            if (is_black && to_rank == 0) || (!is_black && to_rank == 7) {
                let base_mv = Move {
                    src: Square::from_bit_index(from_square),
                    target: Square::from_bit_index(to_square),
                    promotion: None,
                };

                for promotion in [
                    PieceType::Bishop,
                    PieceType::Knight,
                    PieceType::Rook,
                    PieceType::Queen,
                ] {
                    let mv = Move {
                        promotion: Some(promotion),
                        ..base_mv
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
    pub fn bitboard_to_pawn_double_moves(
        &mut self,
        moveable_pawns: u64,
        is_black: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_double_moves_append(&mut moves, moveable_pawns, is_black);
        moves
    }

    pub fn bitboard_to_pawn_double_moves_append(
        &mut self,
        possible_moves: &mut Vec<Move>,
        moveable_pawns: u64,
        is_black: bool,
    ) {
        let mut working_pawns = moveable_pawns;

        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1; // Clear the processed bit

            let to_square = if is_black {
                from_square - 16 // Black pawns move down two squares
            } else {
                from_square + 16 // White pawns move up two squares
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

    pub fn bitboard_to_pawn_capture_moves(
        &mut self,
        source_pawns: u64,
        target_squares: u64,
        is_black: bool,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        self.bitboard_to_pawn_capture_moves_append(
            &mut moves,
            source_pawns,
            target_squares,
            is_black,
        );
        moves
    }

    /// Convert a bitboard of pawn capture moves into a list of move strings
    pub fn bitboard_to_pawn_capture_moves_append(
        &mut self,
        possible_moves: &mut Vec<Move>,
        source_pawns: u64,
        target_squares: u64,
        is_black: bool,
    ) {
        let mut working_pawns = source_pawns;

        // For each pawn
        while working_pawns != 0 {
            let from_square = working_pawns.trailing_zeros() as u8;
            working_pawns &= working_pawns - 1; // Clear the processed bit

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
                captures &= captures - 1; // Clear the processed bit

                // Convert to algebraic notation
                let to_rank_int = to_square / 8 + 1;

                if (is_black && to_rank_int == 1) || (!is_black && to_rank_int == 8) {
                    // Promotion case
                    for promotion in [
                        PieceType::Bishop,
                        PieceType::Knight,
                        PieceType::Rook,
                        PieceType::Queen,
                    ] {
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
        // Invalid square (off-board sentinel) can't be attacked. Without this
        // guard, `1u64 << square` shifts past width and panics in debug builds.
        if square >= 64 {
            return false;
        }
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
            if w_pawns_attack_targets(self.white_pawns(), square_bb) != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            // Get all squares a knight could attack this square from
            if knight_legal_moves(square_bb, 0) & self.white_knights() != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            if bishop_moves(square_bb, self.any_black(), self.any_white())
                & (self.white_bishops() | self.white_queen())
                != 0
            {
                return true;
            }

            // Check for rook/straight queen attacks
            if rook_moves(square_bb, self.any_black(), self.any_white())
                & (self.white_rooks() | self.white_queen())
                != 0
            {
                return true;
            }

            // Check for king attacks - king can only be blocked by its own pieces
            if king_legal_moves(square_bb, self.any_black()) & self.white_king() != 0 {
                return true;
            }
        } else {
            // At this point we know the square is empty or occupied by a white piece

            // Check for pawn attacks
            if b_pawns_attack_targets(self.black_pawns(), square_bb) != 0 {
                return true;
            }

            // For knight attacks, we need to check if any knights can move to this square
            if knight_legal_moves(square_bb, 0) & self.black_knights() != 0 {
                return true;
            }

            // Check for bishop/diagonal queen attacks
            if bishop_moves(square_bb, self.any_white(), self.any_black())
                & (self.black_bishops() | self.black_queen())
                != 0
            {
                return true;
            }

            // Check for rook/straight queen attacks
            if rook_moves(square_bb, self.any_white(), self.any_black())
                & (self.black_rooks() | self.black_queen())
                != 0
            {
                return true;
            }

            // Check for king attacks - king can only be blocked by its own pieces
            if king_legal_moves(square_bb, self.any_white()) & self.black_king() != 0 {
                return true;
            }
        }

        false
    }

    pub fn update_check_state(&mut self) {
        // Find white king square
        let white_king_square = self.white_king().trailing_zeros() as u8;
        // Find black king square
        let black_king_square = self.black_king().trailing_zeros() as u8;

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
                return ret;
            }
        }

        // At this point we know we are in check.

        // We are in check, so make a copy of the board and try the move

        // The move is legal if it got us out of check
        if self.white_king_in_check {
            assert_eq!(
                self.side_to_move,
                Color::White,
                "White king is in check, but side to move is not White"
            );
            self.apply_move(mv);
            let ret = !self.white_king_in_check;
            self.undo_last_move();
            ret
        } else {
            assert!(self.black_king_in_check);
            assert_eq!(
                self.side_to_move,
                Color::Black,
                "Black king is in check, but side to move is not Black"
            );
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
        (path & self.empty()) == path
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
            path &= path - 1; // Clear the processed bit
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
            if is_kingside {
                self.white_kingside_castle_rights
            } else {
                self.white_queenside_castle_rights
            }
        } else if is_kingside {
            self.black_kingside_castle_rights
        } else {
            self.black_queenside_castle_rights
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
        self.get_all_raw_moves_append(&mut moves);

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
        self.get_all_raw_moves_append(&mut moves);
        moves
    }

    /// Undoes the last move made, restoring the board to its previous state
    pub fn undo_last_move(&mut self) {
        if let Some(state) = self.move_history.pop() {
            let mv = &state.last_move;

            let piece = self
                .get_piece_at_square_fast(mv.target.to_bit_index())
                .expect("No piece at target square when undoing move");

            let src_bit = mv.src.to_bitboard();
            let src_index = mv.src.to_bit_index();
            let target_bit = mv.target.to_bitboard();
            let target_idx = mv.target.to_bit_index();

            // Promotion case: the piece currently on `target_bit` is
            // the promoted piece (Q/R/B/N), not the original pawn. We
            // need to wipe the promoted piece, restore the pawn on
            // `src_bit`, and skip the regular "move-piece-back" logic.
            if mv.promotion.is_some() {
                self.clear_piece(piece, target_bit);
                self.remove_piece_in_map(target_idx);

                // Restore the pawn on the square it left.
                let pawn = if piece.color() == Color::White {
                    Piece::WhitePawn
                } else {
                    Piece::BlackPawn
                };
                self.set_piece(pawn, src_bit);
                self.add_piece_in_map(src_index, pawn);
            } else {
                // Normal move: XOR (target | src) puts the piece back
                // on its source square in one bitboard mutation.
                self.xor_piece(piece, src_bit | target_bit);
                self.move_piece_in_map(target_idx, src_index);
            }

            // If this was a castling move, also undo the rook leg by
            // XORing the rook back from its post-castle square to its
            // pre-castle square.
            if let Some(rook_mv) = &state.rook_castle_move {
                let rook_src_bit = rook_mv.src.to_bitboard();
                let rook_target_bit = rook_mv.target.to_bitboard();
                let rook_piece = if piece == Piece::WhiteKing {
                    Piece::WhiteRook
                } else {
                    Piece::BlackRook
                };
                self.xor_piece(rook_piece, rook_src_bit | rook_target_bit);
                self.move_piece_in_map(rook_mv.target.to_bit_index(), rook_mv.src.to_bit_index());
            }

            // Put any captured piece back on the square it was taken
            // from. For a regular capture that's the move's target
            // square; for en passant it's a different square (one rank
            // behind the target), captured in BoardState.
            if let Some(captured) = state.captured_piece {
                let square = state.captured_piece_square.unwrap();
                let square_idx = square.to_bit_index();
                self.set_piece(captured, square.to_bitboard());
                self.add_piece_in_map(square_idx, captured);
            }

            // Restore the per-state metadata captured pre-move.
            self.white_kingside_castle_rights = state.white_kingside_castle_rights;
            self.white_queenside_castle_rights = state.white_queenside_castle_rights;
            self.black_kingside_castle_rights = state.black_kingside_castle_rights;
            self.black_queenside_castle_rights = state.black_queenside_castle_rights;
            self.en_passant_target = state.en_passant_target;
            self.halfmove_clock = state.halfmove_clock;
            self.fullmove_number = state.fullmove_number;

            // Side-to-move flips back. (No composite-bitboard cache to
            // refresh in the new layout — `pieces`/`colors` are the
            // source of truth and were updated in lock-step above.)
            self.side_to_move = match self.side_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
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

    pub fn is_stalemate(&mut self) -> bool {
        // If the side to move is not in check and has no legal moves, it's a stalemate
        let legal_moves = self.generate_legal_moves();
        legal_moves.is_empty() && !self.black_king_in_check && !self.white_king_in_check
    }

    /// Gets a complete debug state of the board including bitboards and move history
    pub fn get_debug_state(&self) -> String {
        let mut output = String::new();
        output.push_str("Board state dump:\n");
        output.push_str(&format!("White pawns:   {:064b}\n", self.white_pawns()));
        output.push_str(&format!("White knights: {:064b}\n", self.white_knights()));
        output.push_str(&format!("White bishops: {:064b}\n", self.white_bishops()));
        output.push_str(&format!("White rooks:   {:064b}\n", self.white_rooks()));
        output.push_str(&format!("White queen:   {:064b}\n", self.white_queen()));
        output.push_str(&format!("White king:    {:064b}\n", self.white_king()));
        output.push_str(&format!("Black pawns:   {:064b}\n", self.black_pawns()));
        output.push_str(&format!("Black knights: {:064b}\n", self.black_knights()));
        output.push_str(&format!("Black bishops: {:064b}\n", self.black_bishops()));
        output.push_str(&format!("Black rooks:   {:064b}\n", self.black_rooks()));
        output.push_str(&format!("Black queen:   {:064b}\n", self.black_queen()));
        output.push_str(&format!("Black king:    {:064b}\n", self.black_king()));
        output.push_str(&format!(
            "Move history length: {}\n",
            self.move_history.len()
        ));
        output.push_str(&format!("Side to move: {:?}\n", self.side_to_move));

        // Add complete move history in algebraic notation
        if !self.move_history.is_empty() {
            output.push_str("\nComplete move history:\n");
            for (i, state) in self.move_history.iter().enumerate() {
                output.push_str(&format!(
                    "{}. {} (captured: {:?})\n",
                    i + 1,
                    state.last_move,
                    state.captured_piece
                ));
            }
        }
        output
    }

    /// Gets the complete move history in a format suitable for debugging
    pub fn get_move_history(&self) -> Vec<String> {
        self.move_history
            .iter()
            .enumerate()
            .map(|(i, state)| {
                format!(
                    "{}. {} (captured: {:?})",
                    i + 1,
                    state.last_move,
                    state.captured_piece
                )
            })
            .collect()
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        // Compare playable position only. move_history, halfmove_clock,
        // and fullmove_number are game-history metadata and intentionally
        // excluded so transpositions compare equal.
        self.white_pawns() == other.white_pawns()
            && self.white_knights() == other.white_knights()
            && self.white_bishops() == other.white_bishops()
            && self.white_rooks() == other.white_rooks()
            && self.white_queen() == other.white_queen()
            && self.white_king() == other.white_king()
            && self.black_pawns() == other.black_pawns()
            && self.black_knights() == other.black_knights()
            && self.black_bishops() == other.black_bishops()
            && self.black_rooks() == other.black_rooks()
            && self.black_queen() == other.black_queen()
            && self.black_king() == other.black_king()
            && self.any_white() == other.any_white()
            && self.any_black() == other.any_black()
            && self.empty() == other.empty()
            && self.side_to_move == other.side_to_move
            && self.white_king_in_check == other.white_king_in_check
            && self.black_king_in_check == other.black_king_in_check
            && self.white_kingside_castle_rights == other.white_kingside_castle_rights
            && self.white_queenside_castle_rights == other.white_queenside_castle_rights
            && self.black_kingside_castle_rights == other.black_kingside_castle_rights
            && self.black_queenside_castle_rights == other.black_queenside_castle_rights
            && self.en_passant_target == other.en_passant_target
            && self.piece_map == other.piece_map
    }
}
