/// Material values in centipawns
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;

use crate::board::Board;

impl Board {
    /// Evaluates the current position from White's perspective.
    /// Returns the evaluation in centipawns (positive numbers favor White, negative numbers favor Black)
    /// Currently only considers material balance:
    /// - Pawn = 1 point (100 centipawns)
    /// - Knight = 3 points (300 centipawns)
    /// - Bishop = 3 points (300 centipawns)
    /// - Rook = 5 points (500 centipawns)
    /// - Queen = 9 points (900 centipawns)
    /// - King = not counted
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        
        // Count white pieces
        score += (self.white_pawns.count_ones() * PAWN_VALUE as u32) as i32;
        score += (self.white_knights.count_ones() * KNIGHT_VALUE as u32) as i32;
        score += (self.white_bishops.count_ones() * BISHOP_VALUE as u32) as i32;
        score += (self.white_rooks.count_ones() * ROOK_VALUE as u32) as i32;
        score += (self.white_queen.count_ones() * QUEEN_VALUE as u32) as i32;

        // Subtract black pieces
        score -= (self.black_pawns.count_ones() * PAWN_VALUE as u32) as i32;
        score -= (self.black_knights.count_ones() * KNIGHT_VALUE as u32) as i32;
        score -= (self.black_bishops.count_ones() * BISHOP_VALUE as u32) as i32;
        score -= (self.black_rooks.count_ones() * ROOK_VALUE as u32) as i32;
        score -= (self.black_queen.count_ones() * QUEEN_VALUE as u32) as i32;

        score
    }
}
