//! Search state shared across the negamax recursion: killer moves and
//! history heuristic. Both are *cheap* signals used to order quiet (non-
//! capture) moves so alpha-beta beta-cutoffs land earlier and prune more
//! of the tree.
//!
//! Captures are already ordered by MVV-LVA in the search; killers and
//! history target the much larger pool of quiet moves where MVV-LVA has
//! nothing to say.

use crate::types::Move;

/// Maximum search ply we'll address with killers. Iterative deepening
/// caps at depth 20 in our engine, plus quiescence may extend a few
/// more plies for captures, so 64 is generous.
pub const MAX_SEARCH_PLY: usize = 64;

pub struct SearchState {
    /// Two killer slots per ply. A "killer" is a quiet move that caused
    /// a beta cutoff at this ply somewhere else in the tree — likely
    /// good in similar positions at the same ply.
    pub killers: [[Option<Move>; 2]; MAX_SEARCH_PLY],

    /// Cumulative cutoff score per (from, to) square pair. Quiet moves
    /// that frequently cause cutoffs anywhere in the tree get bumped up.
    /// Indexed by `Square::to_bit_index()` (0..64).
    pub history: [[i64; 64]; 64],
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            killers: [[None; 2]; MAX_SEARCH_PLY],
            history: [[0; 64]; 64],
        }
    }

    /// Called after a beta cutoff. Updates killers (only for quiet moves)
    /// and history (only for quiet moves, weighted by depth^2 so deep
    /// cutoffs carry more signal).
    pub fn record_cutoff(&mut self, ply: usize, mv: Move, depth: i32, is_capture: bool) {
        if is_capture {
            return;
        }
        if ply < MAX_SEARCH_PLY {
            // Don't waste both slots on the same move.
            if self.killers[ply][0] != Some(mv) {
                self.killers[ply][1] = self.killers[ply][0];
                self.killers[ply][0] = Some(mv);
            }
        }
        let from = mv.src.to_bit_index() as usize;
        let to = mv.target.to_bit_index() as usize;
        self.history[from][to] += (depth as i64).pow(2);
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
