// Material values (PAWN_VALUE etc.) live in types.rs alongside Piece so
// they have a single home shared with Piece::material_value().

/// Positional bonuses in centipawns
const CENTER_CONTROL_BONUS: i64 = 10;    // Bonus for controlling center squares
const CHECK_BONUS: i64 = 50;             // Bonus for giving check
const BISHOP_PAIR_BONUS: i64 = 25;       // Bonus for having both bishops
const CHECKMATE_BONUS: i64 = 100000;     // Large bonus for checkmate
// const DRAW_SCORE: i64 = 0;               // Score for drawn positions
const CASTLED_BONUS: i64 = 75;           // Bonus for having castled (king safety)
const CASTLING_RIGHTS_BONUS: i64 = 20;   // Bonus for each available castling right
const MOBILITY_BONUS: i64 = 5;           // Bonus per available move for piece mobility
const DOUBLED_PAWN_PENALTY: i64 = -15;   // Per "extra" pawn on the same file (a + b doubled = -15, a + b + c tripled = -30)
const ISOLATED_PAWN_PENALTY: i64 = -20;  // Per pawn with no friendly pawn on either neighbouring file

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

        // Center control (e4, e5, d4, d5)
        let center_squares = 0x0000001818000000u64;
        score += ((self.any_white & center_squares).count_ones() as i64) * CENTER_CONTROL_BONUS;
        score -= ((self.any_black & center_squares).count_ones() as i64) * CENTER_CONTROL_BONUS;

        // Extended center control (e3, e6, d3, d6, c4, c5, f4, f5)
        let extended_center = 0x00003C3C3C3C0000u64;
        score += ((self.any_white & extended_center).count_ones() as i64) * (CENTER_CONTROL_BONUS / 2);
        score -= ((self.any_black & extended_center).count_ones() as i64) * (CENTER_CONTROL_BONUS / 2);

        // Mobility evaluation for pieces
        score += self.evaluate_piece_mobility();

        // Castling evaluation
        score += self.evaluate_castling();

        // Pawn structure (doubled, isolated)
        score += self.evaluate_pawn_structure();

        score
    }

    /// Penalises poor pawn structure: doubled (multiple pawns on the same
    /// file) and isolated (no friendly pawn on either adjacent file).
    /// Returns score from White's perspective.
    fn evaluate_pawn_structure(&self) -> i64 {
        let mut score = 0;

        // Doubled pawns: per file, count pawns; each pawn beyond the first
        // contributes one DOUBLED_PAWN_PENALTY.
        for file in 0..8u32 {
            let file_mask = 0x0101010101010101u64 << file;
            let w = (self.white_pawns & file_mask).count_ones();
            let b = (self.black_pawns & file_mask).count_ones();
            if w > 1 { score += (w as i64 - 1) * DOUBLED_PAWN_PENALTY; }
            if b > 1 { score -= (b as i64 - 1) * DOUBLED_PAWN_PENALTY; }
        }

        // Isolated pawns: a pawn whose neighbour-file mask has no friendly pawn.
        for file in 0..8u32 {
            let file_mask = 0x0101010101010101u64 << file;
            let neighbour_mask = {
                let mut m = 0u64;
                if file > 0 { m |= 0x0101010101010101u64 << (file - 1); }
                if file < 7 { m |= 0x0101010101010101u64 << (file + 1); }
                m
            };

            let w_on_file = self.white_pawns & file_mask;
            if w_on_file != 0 && (self.white_pawns & neighbour_mask) == 0 {
                score += (w_on_file.count_ones() as i64) * ISOLATED_PAWN_PENALTY;
            }
            let b_on_file = self.black_pawns & file_mask;
            if b_on_file != 0 && (self.black_pawns & neighbour_mask) == 0 {
                score -= (b_on_file.count_ones() as i64) * ISOLATED_PAWN_PENALTY;
            }
        }

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
