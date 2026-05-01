//! Search module — find the best move from a given position.
//!
//! Public surface is [`Searcher`] plus the [`MATE_SCORE`] constant. The
//! [`Searcher`] owns the internal state that needs to live across calls
//! (today: killer-move table, history heuristic, deterministic-mode
//! flag; tomorrow: transposition table, repetition history, time
//! controller, statistics).
//!
//! The search algorithm itself is a fairly conventional negamax with
//! alpha-beta pruning, iterative deepening, quiescence at the
//! depth-zero horizon, and cheap-signal move ordering (MVV-LVA for
//! captures, killer + history for quiets). All of it lives in this
//! module — Board owns the position primitive (apply / undo /
//! attack queries), Search owns the algorithm.
//!
//! ## Score conventions
//!
//! Every score returned from a `Searcher` method is from **White's
//! perspective**: positive = good for white, negative = good for
//! black. Internally during the recursion scores live in side-to-move's
//! POV (negamax convention) and are flipped at the root.
//!
//! ## Mate scoring
//!
//! When the side-to-move has no legal moves and is in check, the
//! search returns `-MATE_SCORE + ply` so that closer mates score
//! larger in magnitude — the engine prefers shorter forced mates
//! over delayed ones. Stalemate scores 0. See the comment in
//! `negamax_ab` for the full reasoning.
//!
//! ## Why we have a stateful `Searcher`
//!
//! Killer and history tables persist across iterative-deepening
//! iterations: a quiet move that caused a beta cutoff at depth 4 is
//! tried first at depth 5. With free functions we'd thread that state
//! through every signature. With a `Searcher` it's just `&mut self`.
//! When the transposition table lands the same logic applies — it'll
//! be a private field on `Searcher`, persisting across calls within a
//! game so that "go depth N+1" reuses subtree results from "go depth
//! N".

use crate::board::Board;
use crate::types::{Color, Move};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------
// Public constants
// ---------------------------------------------------------------------

/// Maximum search ply we'll address with killers. Iterative deepening
/// caps at depth 20 in `find_best_move_within`, plus quiescence may
/// extend a few more plies for captures, so 64 is generous.
pub const MAX_SEARCH_PLY: usize = 64;

/// Mate score returned by the search when the side-to-move is checkmated.
///
/// Used as `-MATE_SCORE + ply` so closer mates score larger in
/// magnitude (the engine prefers shorter mates). Anything ≥
/// `MATE_SCORE - MAX_SEARCH_PLY` in absolute value is "a mate score."
/// Sized well above any plausible material/positional eval (a queen is
/// 900 cp, the whole board ~5000 cp).
pub const MATE_SCORE: i64 = 30000;

// ---------------------------------------------------------------------
// SearchState — private to the search module.
//
// Holds the cheap move-ordering tables. They live as a struct (rather
// than inline fields on Searcher) so we can pass `&mut SearchState`
// around if we ever want to parallelise. Today's single-threaded
// search just goes through `&mut self.state`.
// ---------------------------------------------------------------------

struct SearchState {
    /// Two killer slots per ply. A "killer" is a quiet move that caused
    /// a beta cutoff at this ply somewhere else in the tree — likely
    /// good in similar positions at the same ply.
    killers: [[Option<Move>; 2]; MAX_SEARCH_PLY],

    /// Cumulative cutoff score per (from, to) square pair. Quiet moves
    /// that frequently cause cutoffs anywhere in the tree get bumped
    /// up. Indexed by `Square::to_bit_index()` (0..64).
    history: [[i64; 64]; 64],
}

impl SearchState {
    fn new() -> Self {
        Self {
            killers: [[None; 2]; MAX_SEARCH_PLY],
            history: [[0; 64]; 64],
        }
    }

    /// Called after a beta cutoff. Updates killers (only for quiet
    /// moves) and history (only for quiet moves, weighted by depth^2
    /// so deep cutoffs carry more signal).
    fn record_cutoff(&mut self, ply: usize, mv: Move, depth: i32, is_capture: bool) {
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

// ---------------------------------------------------------------------
// Searcher — the public search engine.
// ---------------------------------------------------------------------

/// The search engine. Owns search-internal state (killer/history
/// tables, deterministic-mode flag) and exposes a small interface for
/// "find me the best move from this position."
///
/// One Searcher per UCI session is the typical lifetime: state
/// persists across moves within a game (killer/history mature, future
/// TT accumulates), and a fresh `Searcher::new()` starts the next
/// game from a clean slate. Tests typically construct a fresh
/// Searcher per case.
pub struct Searcher {
    state: SearchState,

    /// When `true`, suppresses random tie-breaking among equal-scoring
    /// root moves so benchmarks and tests are reproducible. Read from
    /// the `CHESS_DETERMINISTIC` environment variable at construction
    /// time; tests can override via [`Searcher::new_deterministic`].
    deterministic: bool,
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Searcher {
    /// Construct a fresh Searcher. Reads `CHESS_DETERMINISTIC` from
    /// the environment to decide whether random tie-breaking among
    /// equal-scoring root moves is enabled.
    pub fn new() -> Self {
        Self {
            state: SearchState::new(),
            deterministic: std::env::var("CHESS_DETERMINISTIC").is_ok(),
        }
    }

    /// Construct a Searcher with deterministic root-move tie-breaking
    /// forced on, regardless of the environment variable. Useful when
    /// a test wants stable output without having to set
    /// `CHESS_DETERMINISTIC=1` in the test runner.
    pub fn new_deterministic() -> Self {
        Self {
            state: SearchState::new(),
            deterministic: true,
        }
    }

    // ------------------------------------------------------------
    // Public entry points: find_best_move, find_best_move_within
    // ------------------------------------------------------------

    /// Search the position to a fixed depth and return the best move
    /// found, plus its score from White's POV (positive = good for
    /// White).
    ///
    /// If the position has no legal moves, returns
    /// `(None, ±MATE_SCORE)` for checkmate or `(None, 0)` for
    /// stalemate. Callers do NOT need to handle the None case
    /// specially; an `Option<Move>` is the only honest return type for
    /// "what's the best move" given that there might not be one.
    ///
    /// Borrows `board` mutably because the underlying make/unmake
    /// algorithm temporarily mutates the board during recursion. The
    /// board is restored to its original state before this method
    /// returns.
    pub fn find_best_move(&mut self, board: &mut Board, depth: i32) -> (Option<Move>, i64) {
        // Generate root moves once, then dispatch into the recursive
        // search. Most of the bookkeeping that used to live in
        // `find_best_move_with_state` has migrated here.
        let mut moves = Vec::new();
        board.get_all_raw_moves_append(&mut moves);

        // No legal moves at the root: mate (in check) or stalemate.
        // Return a ply-zero mate score (or 0) so callers don't have to
        // special-case the empty result.
        if moves.is_empty() {
            let in_check = side_in_check(board);
            let score_pov = if in_check { -MATE_SCORE } else { 0 };
            // Convert side-to-move POV → White POV for the public API.
            let white_pov = if board.side_to_move == Color::Black {
                -score_pov
            } else {
                score_pov
            };
            return (None, white_pov);
        }

        // Order moves with the same heuristics negamax uses internally
        // — captures via MVV-LVA, then killers, then history. Highest
        // score first (`Reverse` flips std's ascending sort).
        moves.sort_unstable_by_key(|m| std::cmp::Reverse(self.order_score(board, m, 0)));

        // best_score lives in the side-to-move's POV. Initialise to
        // i64::MIN + 1 — using MIN itself would overflow when we
        // negate at the end for Black-to-move.
        let mut best_score = i64::MIN + 1;
        let mut best_move: Option<Move> = None;

        for mv in moves {
            board.apply_move(&mv);
            let score = -self.negamax_ab(board, depth - 1, 1, i64::MIN + 1, i64::MAX - 1);
            board.undo_last_move();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            } else if score == best_score && !self.deterministic {
                // Random tie-break adds opening variety during real
                // play. Disabled in deterministic mode for stable
                // benches and test output.
                if rand::random::<bool>() {
                    best_move = Some(mv);
                }
            }
        }

        // Convert side-to-move POV → White POV at the seam.
        if board.side_to_move == Color::Black {
            (best_move, -best_score)
        } else {
            (best_move, best_score)
        }
    }

    /// Iterative deepening with a wall-clock budget. Always completes
    /// at least depth 1 (so we always return *something* legal if any
    /// move exists), then deepens until the next iteration would
    /// likely overshoot the deadline.
    ///
    /// Killer and history tables persist across iterations: a move
    /// that caused a cutoff at depth 4 will be tried first at depth 5.
    ///
    /// Returns `(best_move, score, completed_depth)` where score is in
    /// White's POV and `completed_depth` is the largest depth fully
    /// searched within the budget (useful for UCI `info depth N`).
    pub fn find_best_move_within(
        &mut self,
        board: &mut Board,
        time_budget: Duration,
    ) -> (Option<Move>, i64, i32) {
        let deadline = Instant::now() + time_budget;
        let mut best_move = None;
        let mut best_score = 0_i64;
        let mut completed_depth = 0;

        // Cap iterative deepening at 20 to keep the killer/history
        // arrays in their MAX_SEARCH_PLY=64 budget even after a few
        // quiescence extensions. If we ever push past depth 20, raise
        // MAX_SEARCH_PLY first.
        for depth in 1..=20 {
            let iter_start = Instant::now();
            let (mv, score) = self.find_best_move(board, depth);
            best_move = mv;
            best_score = score;
            completed_depth = depth;

            // Heuristic: chess search nodes per iteration grow ~3-4×
            // per ply. If the *next* iteration would likely overshoot
            // the deadline by 4× the time we just spent, stop now and
            // return what we have rather than start a search we'll
            // abort partway through (which would leave best_move
            // pointing at something potentially worse than the
            // depth-N result).
            let elapsed = iter_start.elapsed();
            if Instant::now() + elapsed * 4 >= deadline {
                break;
            }
        }
        (best_move, best_score, completed_depth)
    }

    // ------------------------------------------------------------
    // Recursive core: negamax_ab + quiesce
    // ------------------------------------------------------------

    /// Recursive negamax with alpha-beta + quiescence + killer/history
    /// move ordering. `ply` is the distance from the root (0 at root);
    /// `depth` is remaining depth.
    ///
    /// Returns score in the **current side-to-move's POV** (negamax
    /// convention). The public `find_best_move` flips to White's POV
    /// at the root.
    fn negamax_ab(
        &mut self,
        board: &mut Board,
        depth: i32,
        ply: usize,
        mut alpha: i64,
        beta: i64,
    ) -> i64 {
        // Generate moves up-front so we can detect mate/stalemate
        // before deciding whether to drop into quiescence. If we did
        // the depth==0 check first, mates discovered exactly at the
        // horizon would be missed (quiescence's stand-pat doesn't
        // recognise them) and the engine would happily delay a forced
        // mate by extra plies.
        let mut moves = Vec::new();
        board.get_all_raw_moves_append(&mut moves);

        if moves.is_empty() {
            let in_check = side_in_check(board);
            // Ply-aware mate score: closer mates score larger in
            // magnitude so the search prefers them. Stalemate scores 0.
            return if in_check {
                -MATE_SCORE + ply as i64
            } else {
                0
            };
        }

        if depth == 0 {
            return self.quiesce(board, alpha, beta);
        }

        // Order moves: MVV-LVA captures > killers > history > rest.
        moves.sort_unstable_by(|a, b| {
            self.order_score(board, b, ply)
                .cmp(&self.order_score(board, a, ply))
        });

        // best_score in side-to-move POV; initialise just above MIN
        // so a future negation doesn't overflow.
        let mut best_score = i64::MIN + 1;

        for mv in moves {
            let is_cap = move_is_capture(board, &mv);
            board.apply_move(&mv);
            let score = -self.negamax_ab(board, depth - 1, ply + 1, -beta, -alpha);
            board.undo_last_move();

            if score > best_score {
                best_score = score;
            }

            if score >= beta {
                // Beta cutoff. Record this move so the same position
                // (or similar at the same ply) tries it first next
                // time.
                self.state.record_cutoff(ply, mv, depth, is_cap);
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        best_score
    }

    /// Quiescence search: at the main-search horizon, keep extending
    /// on captures only until the position is "quiet" (no profitable
    /// captures left). Resolves pending exchanges so the static eval
    /// at the real leaf is honest. Without this, depth-N negamax
    /// happily evaluates positions with hanging pieces.
    ///
    /// **Stand-pat**: the side to move can decline to capture
    /// (= keep the static eval). So `evaluate_pov` is the floor;
    /// captures only get explored if they might beat it.
    ///
    /// Captures are ordered MVV-LVA (most valuable victim, least
    /// valuable attacker) so promising captures (PxQ) are tried before
    /// bad ones (QxP). With ordering, beta cutoffs land on the first
    /// or second move much more often, and the recursion terminates
    /// fast even in tactical positions.
    ///
    /// Limitations of this first cut:
    ///  - Doesn't generate check evasions when in check (should search
    ///    all moves if in check, not just captures).
    ///  - Misses en-passant captures (the `is_capture` test only
    ///    checks whether the target square is occupied; en passant
    ///    moves to an empty square).
    ///
    // `&mut self` is currently used only for the recursive call —
    // quiesce reads no Searcher state today. We keep it because the
    // transposition-table-aware quiesce we're about to add WILL read
    // and write `self.tt`, and changing the signature later costs
    // every call site. The alternative (free function now, method
    // later) would mean a churn-y rename.
    #[allow(clippy::only_used_in_recursion)]
    fn quiesce(&mut self, board: &mut Board, mut alpha: i64, beta: i64) -> i64 {
        let stand_pat = evaluate_pov(board);
        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut moves = Vec::new();
        board.get_all_raw_moves_append(&mut moves);

        // Filter to captures and score by MVV-LVA. Higher score first.
        // Score = victim_value * 10 - attacker_value, so PxQ (8990)
        // ranks far above QxP (-700) and beats every quiet move
        // (which is filtered out by the `victim?` ?-operator).
        let mut scored: Vec<(i64, Move)> = moves
            .into_iter()
            .filter_map(|mv| {
                let victim = board.get_piece_at_square_fast(mv.target.to_bit_index())?;
                let attacker = board.get_piece_at_square_fast(mv.src.to_bit_index())?;
                let score = victim.material_value() * 10 - attacker.material_value();
                Some((score, mv))
            })
            .collect();
        scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        for (_score, mv) in scored {
            board.apply_move(&mv);
            let score = -self.quiesce(board, -beta, -alpha);
            board.undo_last_move();

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    // ------------------------------------------------------------
    // Move ordering — see record_cutoff above for what feeds it.
    // ------------------------------------------------------------

    /// Score a move for ordering inside the main search. Higher = try
    /// first.
    ///
    /// Tier 1: captures, ordered MVV-LVA.
    /// Tier 2: killer-move slot 0 from this ply.
    /// Tier 3: killer-move slot 1 from this ply.
    /// Tier 4: history-heuristic score (any quiet move).
    fn order_score(&self, board: &Board, mv: &Move, ply: usize) -> i64 {
        if let (Some(victim), Some(attacker)) = (
            board.get_piece_at_square_fast(mv.target.to_bit_index()),
            board.get_piece_at_square_fast(mv.src.to_bit_index()),
        ) {
            // Capture: MVV-LVA, big offset to put captures above any
            // killer/history score.
            return 1_000_000 + victim.material_value() * 10 - attacker.material_value();
        }
        if ply < MAX_SEARCH_PLY {
            if self.state.killers[ply][0] == Some(*mv) {
                return 500_000;
            }
            if self.state.killers[ply][1] == Some(*mv) {
                return 400_000;
            }
        }
        let from = mv.src.to_bit_index() as usize;
        let to = mv.target.to_bit_index() as usize;
        self.state.history[from][to]
    }
}

// ---------------------------------------------------------------------
// Free-function helpers used by Searcher methods.
//
// They take `&Board` and don't touch any Searcher state, so making
// them free functions keeps the Searcher methods tidier.
// ---------------------------------------------------------------------

/// Is the side that's currently to move in check?
#[inline]
fn side_in_check(board: &Board) -> bool {
    match board.side_to_move {
        Color::White => board.white_king_in_check,
        Color::Black => board.black_king_in_check,
    }
}

/// Static evaluation from the side-to-move's perspective. Wraps
/// `Board::evaluate()` (which returns from White's POV) and flips for
/// Black.
#[inline]
fn evaluate_pov(board: &Board) -> i64 {
    if board.side_to_move == Color::White {
        board.evaluate()
    } else {
        -board.evaluate()
    }
}

/// Returns true if applying `mv` to `board` is a capture (target
/// square occupied). Doesn't catch en passant; that's a known
/// limitation matching the quiescence implementation.
#[inline]
fn move_is_capture(board: &Board, mv: &Move) -> bool {
    board
        .get_piece_at_square_fast(mv.target.to_bit_index())
        .is_some()
}

// ---------------------------------------------------------------------
// Unit tests for `SearchState`'s killer/history bookkeeping.
//
// `SearchState` is private to the search module, so these tests live
// here (rather than under `chesslib/tests/`) to access internal
// fields directly. Tests against the public `Searcher` interface
// belong with the rest of the integration tests.
// ---------------------------------------------------------------------
#[cfg(test)]
mod search_state_tests {
    use super::*;
    use crate::types::Square;

    fn mv(src: Square, target: Square) -> Move {
        Move {
            src,
            target,
            promotion: None,
        }
    }

    #[test]
    fn new_state_has_no_killers() {
        let ss = SearchState::new();
        assert_eq!(ss.killers[0][0], None);
        assert_eq!(ss.killers[0][1], None);
        assert_eq!(ss.killers[5][0], None);
    }

    #[test]
    fn record_cutoff_quiet_move_stores_as_killer_0() {
        let mut ss = SearchState::new();
        let m = mv(Square::E2, Square::E4);
        ss.record_cutoff(3, m, 5, false);
        assert_eq!(ss.killers[3][0], Some(m));
        assert_eq!(ss.killers[3][1], None);
    }

    #[test]
    fn record_cutoff_capture_does_not_store_killer() {
        // Captures already get priority through MVV-LVA; storing them
        // as killers would be redundant and could push real quiet
        // killers out of the slots.
        let mut ss = SearchState::new();
        let m = mv(Square::E4, Square::D5);
        ss.record_cutoff(3, m, 5, true);
        assert_eq!(ss.killers[3][0], None);
    }

    #[test]
    fn second_distinct_cutoff_shifts_old_killer_to_slot_1() {
        let mut ss = SearchState::new();
        let m1 = mv(Square::E2, Square::E4);
        let m2 = mv(Square::G1, Square::F3);
        ss.record_cutoff(3, m1, 5, false);
        ss.record_cutoff(3, m2, 5, false);
        assert_eq!(ss.killers[3][0], Some(m2));
        assert_eq!(ss.killers[3][1], Some(m1));
    }

    #[test]
    fn duplicate_cutoff_does_not_shift_killers() {
        // If the same move keeps causing cutoffs, don't waste both
        // slots on it.
        let mut ss = SearchState::new();
        let m1 = mv(Square::E2, Square::E4);
        ss.record_cutoff(3, m1, 5, false);
        ss.record_cutoff(3, m1, 5, false);
        assert_eq!(ss.killers[3][0], Some(m1));
        assert_eq!(ss.killers[3][1], None);
    }

    #[test]
    fn record_cutoff_accumulates_history_score_by_depth_squared() {
        // History scores quiet moves by depth^2, so cutoffs deep in
        // the tree (which prune more) carry more weight.
        let mut ss = SearchState::new();
        let m = mv(Square::E2, Square::E4);
        let from = Square::E2 as usize;
        let to = Square::E4 as usize;
        ss.record_cutoff(0, m, 5, false);
        assert_eq!(ss.history[from][to], 25);
        ss.record_cutoff(0, m, 3, false);
        assert_eq!(ss.history[from][to], 25 + 9);
    }

    #[test]
    fn capture_cutoff_does_not_increment_history() {
        let mut ss = SearchState::new();
        let m = mv(Square::E4, Square::D5);
        let from = Square::E4 as usize;
        let to = Square::D5 as usize;
        ss.record_cutoff(0, m, 5, true);
        assert_eq!(ss.history[from][to], 0);
    }

    #[test]
    fn ply_out_of_range_is_a_no_op_not_a_panic() {
        // Defensive: search depth might exceed our killer table by
        // accident.
        let mut ss = SearchState::new();
        let m = mv(Square::E2, Square::E4);
        ss.record_cutoff(1000, m, 5, false); // way past MAX_SEARCH_PLY
                                             // History is indexed by squares (always 0-63), so it should
                                             // still update.
        let from = Square::E2 as usize;
        let to = Square::E4 as usize;
        assert_eq!(ss.history[from][to], 25);
    }
}
