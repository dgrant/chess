// Material values (PAWN_VALUE etc.) live in types.rs alongside Piece so
// they have a single home shared with Piece::material_value().

/// Positional bonuses in centipawns
// CENTER_CONTROL_BONUS removed — central squares are now rewarded via PSTs.
const CHECK_BONUS: i64 = 50;             // Bonus for giving check
const BISHOP_PAIR_BONUS: i64 = 25;       // Bonus for having both bishops
const CHECKMATE_BONUS: i64 = 100000;     // Large bonus for checkmate
// const DRAW_SCORE: i64 = 0;               // Score for drawn positions
const CASTLED_BONUS: i64 = 75;           // Bonus for having castled (king safety)
const CASTLING_RIGHTS_BONUS: i64 = 20;   // Bonus for each available castling right
// Disabled (set to 0) while PSTs are providing the positional signal.
// PeSTO's PSTs are tuned standalone; double-counting with explicit mobility
// caused a -173 ELO regression on the bench. Try non-zero values via this
// constant and re-bench if you want to put mobility back.
const MOBILITY_BONUS: i64 = 0;

use crate::board::Board;
use crate::types::{Color, PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE};
use crate::move_generation::{knight_legal_moves, bishop_moves, rook_moves};
use crate::Square;

impl Board {
    /// Evaluates the current position from White's perspective.
    pub fn evaluate(&self) -> i64 {
        // Check for terminal positions first
        let mut board = self.clone();
        if board.side_to_move == Color::Black && board.is_checkmate() {
            return CHECKMATE_BONUS;
        } else if board.is_checkmate() {
            return -CHECKMATE_BONUS;
        }

        let mut score = 0;

        // Material evaluation
        score += self.evaluate_material();

        // Position evaluation (with reduced weights)
        score += self.evaluate_position();

        // Check bonus
        if self.black_king_in_check {
            score += CHECK_BONUS;
        }
        if self.white_king_in_check {
            score -= CHECK_BONUS;
        }

        score
    }

    /// Evaluates material balance
    fn evaluate_material(&self) -> i64 {
        let mut score = 0;

        // Count white pieces
        score += (self.white_pawns.count_ones() * PAWN_VALUE as u32) as i64;
        score += (self.white_knights.count_ones() * KNIGHT_VALUE as u32) as i64;
        score += (self.white_bishops.count_ones() * BISHOP_VALUE as u32) as i64;
        score += (self.white_rooks.count_ones() * ROOK_VALUE as u32) as i64;
        score += (self.white_queen.count_ones() * QUEEN_VALUE as u32) as i64;

        // Subtract black pieces
        score -= (self.black_pawns.count_ones() * PAWN_VALUE as u32) as i64;
        score -= (self.black_knights.count_ones() * KNIGHT_VALUE as u32) as i64;
        score -= (self.black_bishops.count_ones() * BISHOP_VALUE as u32) as i64;
        score -= (self.black_rooks.count_ones() * ROOK_VALUE as u32) as i64;
        score -= (self.black_queen.count_ones() * QUEEN_VALUE as u32) as i64;

        // Bishop pair bonus
        if self.white_bishops.count_ones() >= 2 {
            score += BISHOP_PAIR_BONUS;
        // Add a bonus for check (50 centipawns)
        }
        if self.black_bishops.count_ones() >= 2 {
            score -= BISHOP_PAIR_BONUS;
        }

        score
    }

    /// Evaluates positional factors
    fn evaluate_position(&self) -> i64 {
        let mut score = 0;

        // Piece-square tables: per-piece per-square positional bonus.
        // Subsumes center control (central squares have higher PST values).
        score += self.evaluate_pst();

        // Mobility evaluation for pieces (kept for now — will tune weight separately).
        score += self.evaluate_piece_mobility();

        // Castling evaluation (PSTs capture some king-position bonus, but
        // castled bonus is stricter — it requires actual castling, not a
        // king-walk to the same square).
        score += self.evaluate_castling();

        score
    }

    /// Sum of PST values for every piece on the board, from White's perspective.
    /// Black squares are mirrored vertically before lookup.
    fn evaluate_pst(&self) -> i64 {
        use crate::pst::{PAWN_PST, KNIGHT_PST, BISHOP_PST, ROOK_PST, QUEEN_PST, KING_PST, mirror};

        fn sum_white(mut bb: u64, table: &[i64; 64]) -> i64 {
            let mut s = 0;
            while bb != 0 {
                let idx = bb.trailing_zeros() as usize;
                s += table[idx];
                bb &= bb - 1;
            }
            s
        }
        fn sum_black(mut bb: u64, table: &[i64; 64]) -> i64 {
            let mut s = 0;
            while bb != 0 {
                let idx = bb.trailing_zeros() as u8;
                s += table[mirror(idx) as usize];
                bb &= bb - 1;
            }
            s
        }

        let mut score = 0;
        score += sum_white(self.white_pawns,   &PAWN_PST);
        score += sum_white(self.white_knights, &KNIGHT_PST);
        score += sum_white(self.white_bishops, &BISHOP_PST);
        score += sum_white(self.white_rooks,   &ROOK_PST);
        score += sum_white(self.white_queen,   &QUEEN_PST);
        score += sum_white(self.white_king,    &KING_PST);

        score -= sum_black(self.black_pawns,   &PAWN_PST);
        score -= sum_black(self.black_knights, &KNIGHT_PST);
        score -= sum_black(self.black_bishops, &BISHOP_PST);
        score -= sum_black(self.black_rooks,   &ROOK_PST);
        score -= sum_black(self.black_queen,   &QUEEN_PST);
        score -= sum_black(self.black_king,    &KING_PST);

        score
    }

    /// Evaluates castling-related factors including both castled position and available rights
    fn evaluate_castling(&self) -> i64 {
        let mut score = 0;

        // Evaluate castling rights
        if self.white_kingside_castle_rights {
            score += CASTLING_RIGHTS_BONUS;
        }
        if self.white_queenside_castle_rights {
            score += CASTLING_RIGHTS_BONUS;
        }
        if self.black_kingside_castle_rights {
            score -= CASTLING_RIGHTS_BONUS;
        }
        if self.black_queenside_castle_rights {
            score -= CASTLING_RIGHTS_BONUS;
        }

        // Check if kings have moved to typical castled positions
        let white_king_kingside = self.white_king & Square::G1.to_bitboard();  // g1
        let white_king_queenside = self.white_king & Square::C1.to_bitboard();  // c1
        let black_king_kingside = self.black_king & Square::G8.to_bitboard();  // g8
        let black_king_queenside = self.black_king & Square::C8.to_bitboard();  // c8

        // Evaluate actual castled positions
        // We check if the king is in a castled position AND we've lost the corresponding castling right
        // This ensures we're detecting actual castling rather than just a king walk
        if (white_king_kingside != 0 && !self.white_kingside_castle_rights) ||
           (white_king_queenside != 0 && !self.white_queenside_castle_rights) {
            score += CASTLED_BONUS;
        }
        if (black_king_kingside != 0 && !self.black_kingside_castle_rights) ||
           (black_king_queenside != 0 && !self.black_queenside_castle_rights) {
            score -= CASTLED_BONUS;
        }

        score
    }

    /// Evaluates mobility (number of legal moves) for pieces
    fn evaluate_piece_mobility(&self) -> i64 {
        let mut score = 0;

        // Knights
        let mut white_knights = self.white_knights;
        while white_knights != 0 {
            let pos = white_knights.trailing_zeros() as u8;
            let moves = knight_legal_moves(1u64 << pos, self.any_white);
            score += (moves.count_ones() as i64) * MOBILITY_BONUS;
            white_knights &= white_knights - 1;
        }

        let mut black_knights = self.black_knights;
        while black_knights != 0 {
            let pos = black_knights.trailing_zeros() as u8;
            let moves = knight_legal_moves(1u64 << pos, self.any_black);
            score -= (moves.count_ones() as i64) * MOBILITY_BONUS;
            black_knights &= black_knights - 1;
        }

        // Bishops
        let mut white_bishops = self.white_bishops;
        while white_bishops != 0 {
            let pos = white_bishops.trailing_zeros() as u8;
            let moves = bishop_moves(1u64 << pos, self.any_white, self.any_black);
            score += (moves.count_ones() as i64) * MOBILITY_BONUS;
            white_bishops &= white_bishops - 1;
        }

        let mut black_bishops = self.black_bishops;
        while black_bishops != 0 {
            let pos = black_bishops.trailing_zeros() as u8;
            let moves = bishop_moves(1u64 << pos, self.any_black, self.any_white);
            score -= (moves.count_ones() as i64) * MOBILITY_BONUS;
            black_bishops &= black_bishops - 1;
        }

        // Rooks
        let mut white_rooks = self.white_rooks;
        while white_rooks != 0 {
            let pos = white_rooks.trailing_zeros() as u8;
            let moves = rook_moves(1u64 << pos, self.any_white, self.any_black);
            score += (moves.count_ones() as i64) * MOBILITY_BONUS;
            white_rooks &= white_rooks - 1;
        }

        let mut black_rooks = self.black_rooks;
        while black_rooks != 0 {
            let pos = black_rooks.trailing_zeros() as u8;
            let moves = rook_moves(1u64 << pos, self.any_black, self.any_white);
            score -= (moves.count_ones() as i64) * MOBILITY_BONUS;
            black_rooks &= black_rooks - 1;
        }

        score
    }
}
