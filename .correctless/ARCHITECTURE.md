# Architecture — chess

A Rust UCI-compatible chess engine. Cargo workspace at the repo root with two member crates:

- `chess/` — thin binary wrapper that reads UCI commands from stdin and dispatches. Depends on `chesslib` via path.
- `chesslib/` — engine library: board representation, move generation, search, evaluation, FEN, UCI, logging. The full test suite lives here.

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| Binary entry point | `chess/src/main.rs` | Reads UCI lines from stdin, calls `chesslib::handle_uci_command`, writes responses. |
| UCI protocol handler | `chesslib/src/uci.rs` | Parses UCI commands (`uci`, `isready`, `position`, `go`, `quit`). Holds global `BOARD_STATE` behind a `Mutex`. Reports `score cp` from side-to-move POV. |
| Board representation | `chesslib/src/board.rs` | Bitboard-based `Board` (one `u64` per piece-type-and-color), plus `BoardState` for castling rights / en passant / last move. Owns `apply_move`, `undo_move`, check detection. |
| Move generation | `chesslib/src/move_generation.rs` | Pseudo-legal bitboard move generation for each piece type. File masks (`NOT_A_FILE`, `NOT_H_FILE`) prevent wrap-around. |
| Search | `chesslib/src/search.rs` + `Board::find_best_move` in `board.rs` | Negamax with alpha-beta, iterative deepening (capped at depth 20), quiescence search at depth-0 horizon, MVV-LVA capture ordering, killer moves + history heuristic for quiet-move ordering. |
| Evaluation | `chesslib/src/evaluation.rs` | Centipawn evaluation: material (values in `types.rs`), piece-square tables, mobility, king safety / castling, check bonus, bishop pair, checkmate score `100000`. Returned from White's POV; UCI layer flips for Black. |
| FEN | `chesslib/src/fen.rs` | `Board::to_fen()` and `load_fen()` — round-trip serialization. |
| Types | `chesslib/src/types.rs` | `Square` (enum 0..64 with `from_bit_index` via `transmute`), `Color`, `Piece`, `PieceType`, `Move`, `CastlingRights`, material constants (`PAWN_VALUE` etc.), `Piece::material_value()`. |
| Board utilities | `chesslib/src/board_utils.rs` | `get_starting_board`, `get_empty_board`, bitboard↔Move conversions, file-letter helpers. |
| Logger | `chesslib/src/logger.rs` | File-based logger; engine writes per-run log to `chess/engine_<timestamp>.log`. Path overridable via `set_log_path`. |
| Test suite | `chesslib/tests/*.rs` | 17 integration test files covering move-gen, perft, FEN, castling, en passant, alpha-beta, negamax, UCI, evaluation, search-state, ordering, bug regressions. |

## Design Patterns

### PAT-001: Bitboard representation
- The board is twelve `u64` bitboards on `Board` (one per piece-type-and-color: `white_pawns`, `white_knights`, …, `black_king`).
- All move generation operates on bitboards; squares are decoded via `Square::from_bit_index` (0..64).
- Never store the board as a `[Piece; 64]` array in hot paths — bitboard operations are the performance contract.

### PAT-002: Pseudo-legal generation, then legality filter
- `move_generation` produces pseudo-legal moves (correct geometry, ignoring whether the move leaves the king in check).
- The search layer applies the move on a clone, asks `is_in_check` for the side that just moved, and discards illegal moves.
- Don't bake legality into the bitboard generator — keep the generator side-effect-free and fast.

### PAT-003: Material values centralized in `types.rs`
- `PAWN_VALUE`, `KNIGHT_VALUE`, `BISHOP_VALUE`, `ROOK_VALUE`, `QUEEN_VALUE` and `Piece::material_value()` live alongside `Piece`.
- Evaluation, MVV-LVA capture ordering, and any other consumer must import from `types.rs` — never re-define a piece value locally.

### PAT-004: UCI score reporting from side-to-move POV
- `evaluate()` returns score from White's POV (positive = good for White).
- The UCI handler flips the sign when reporting to GUIs so `score cp` is always from the side-to-move's POV (UCI standard).
- Keep this flip in the UCI layer — never push the convention into `evaluate()`.

### PAT-005: Iterative deepening drives time control
- Search is depth-limited but iteratively deepens 1, 2, 3, … until time runs out.
- The current best move from the previous full iteration is what gets reported on timeout — never report a partially-completed deeper search.
- `go movetime`, `go wtime/btime`, and `go depth` all funnel through iterative deepening.

### PAT-006: Cheap-signal move ordering (killers + history)
- Captures are ordered first by MVV-LVA.
- Quiet moves are ordered by the killer table (per-ply, two slots) and the history heuristic (cumulative cutoff score per `(from, to)`).
- New ordering signals must be cheap (per-node O(1)). Anything heavier belongs in evaluation, not ordering.

### PAT-007: Determinism opt-in via `CHESS_DETERMINISTIC`
- Random tie-breaking among equal-score moves is enabled by default (it gives the engine variety in self-play).
- Setting `CHESS_DETERMINISTIC=1` suppresses the random tie-break so benchmarks are reproducible (same input → same output).
- The flag is read once at startup via `OnceLock`, never per-move.

## Conventions

- **Public API discipline**: `chesslib/src/lib.rs` re-exports the small surface intended for binary consumers (`handle_uci_command`, `Square`, `log_to_file`). Internals stay behind module boundaries even though crate-internal `pub` is wide.
- **No `unsafe` outside `Square::from_bit_index`**: the only `unsafe` block in the codebase is the `transmute` from `u8` (0..63) to `Square`. Don't introduce new unsafe code.
- **Tests live in `chesslib/tests/`**, not co-located with source. Each functional area has its own file (`castling_tests.rs`, `en_passant_tests.rs`, …). Bug regressions go in `bug_tests.rs`.
- **No external test frameworks** — built-in `#[test]` only.
- **Logging via `logger.rs`**: never use `println!` for engine output that isn't UCI. UCI replies go to stdout; everything else goes to the log file.
- **Material constants are `i64`**: scoring is in centipawns as signed 64-bit integers throughout. Don't introduce `f64` evaluation.

## Prohibitions

- **PROHIBIT: Raw piece arrays in hot paths.** Move generation and evaluation must use bitboards (`u64`). A `[Piece; 64]` representation is acceptable only for FEN parsing/printing.
- **PROHIBIT: `println!` for engine diagnostics.** Stdout is reserved for UCI protocol; all other output goes to the log file via `chesslib::log_to_file`.
- **PROHIBIT: Local re-definitions of material values.** Import from `types.rs` (`PAWN_VALUE` etc.) or call `Piece::material_value()`.
- **PROHIBIT: New `unsafe` blocks** without an explicit invariant comment justifying them. The single existing `unsafe` (in `Square::from_bit_index`) is grandfathered.
- **PROHIBIT: Score sign conventions other than "evaluate returns from White's POV; UCI flips for side-to-move."** Don't introduce a third convention.

## Known Limitations

- No transposition table — every position is re-searched. This is the next likely performance step after move ordering.
- No null-move pruning, no late-move reductions.
- En passant: implemented in core; see `bug_tests.rs` and `en_passant_tests.rs` for known edge cases.
- Two open bugs documented in `chesslib/TODO.md` (positions where best-move selection is wrong or slow).
- Mobility evaluation currently disabled (`MOBILITY_BONUS=0`) because piece-square tables subsume it; see commit history.
- No CI pipeline yet.
