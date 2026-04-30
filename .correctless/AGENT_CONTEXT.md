# Agent Context — chess

> Last updated: 2026-04-29

## What This Project Does

A Rust UCI-compatible chess engine. The `chess` binary speaks the UCI protocol over stdin/stdout so it can plug into chess GUIs (or harnesses like fastchess). All engine logic — bitboard board representation, move generation, negamax search with alpha-beta + quiescence, evaluation, FEN — lives in the `chesslib` library crate. Single-author hobby project, no deployment target other than running locally.

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| Binary entry | `chess/src/main.rs` | UCI stdin/stdout loop. Calls `chesslib::handle_uci_command`. |
| UCI handler | `chesslib/src/uci.rs` | Parses `uci`, `isready`, `position`, `go`. Holds global `BOARD_STATE` behind a `Mutex`. |
| Board | `chesslib/src/board.rs` | Bitboard `Board` (12 `u64`s) + `BoardState` (castling, en passant, last move). `apply_move`, `undo_move`, check detection, `find_best_move`. |
| Move generation | `chesslib/src/move_generation.rs` | Bitboard pseudo-legal moves per piece. File masks prevent wrap-around. |
| Search | `chesslib/src/search.rs` + `Board::find_best_move` | Negamax + alpha-beta + iterative deepening + quiescence + MVV-LVA + killer/history ordering. |
| Evaluation | `chesslib/src/evaluation.rs` | Centipawn eval. White's POV. PSTs, material, mobility, king safety. |
| Types | `chesslib/src/types.rs` | `Square`, `Color`, `Piece`, `Move`, `CastlingRights`, material constants. |
| FEN | `chesslib/src/fen.rs` | Round-trip FEN serialization. |
| Logger | `chesslib/src/logger.rs` | File logger; per-run timestamped `chess/engine_<ts>.log`. |
| Tests | `chesslib/tests/*_tests.rs` | 17 integration test files. |

## Design Patterns

- **Bitboards everywhere**: 12 `u64`s on `Board`. Convert to/from `Square` (0..64) only at boundaries — see `chesslib/src/board.rs`.
- **Pseudo-legal then filter**: generators don't check legality; the search filters. See `chesslib/src/move_generation.rs`.
- **Material values centralized**: `PAWN_VALUE` etc. and `Piece::material_value()` in `chesslib/src/types.rs`. Never re-defined.
- **Score POV convention**: `evaluate()` is White's POV; UCI layer flips for side-to-move when reporting `score cp`.
- **Iterative deepening drives time control**: `go movetime/wtime/btime/depth` all funnel through the same loop.
- **Cheap-signal move ordering**: MVV-LVA for captures; killer table + history heuristic for quiets.
- **Determinism opt-in**: `CHESS_DETERMINISTIC=1` env var suppresses the random tie-break for reproducible benches.

## Common Pitfalls

- **Adding `println!` for engine diagnostics**: stdout is the UCI protocol. Use `chesslib::log_to_file` instead.
- **Re-defining piece values**: import from `types.rs` or call `Piece::material_value()`.
- **Introducing new `unsafe`**: only `Square::from_bit_index` transmutes — keep it that way.
- **Score sign confusion**: `evaluate()` is White's POV. The flip lives in the UCI layer. Don't sprinkle flips through search.
- **Co-locating tests with source**: tests live in `chesslib/tests/`, plural-named (`*_tests.rs`).
- **Bench non-reproducibility**: set `CHESS_DETERMINISTIC=1` when benchmarking against another engine, or random tie-breaks make runs differ.

## Quick Reference

Cargo workspace — run cargo from the repo root.

| Need to... | Do this |
|------------|---------|
| Run all tests | `cargo test --workspace` |
| Run library tests only | `cargo test -p chesslib` |
| Run with output | `cargo test --workspace -- --nocapture` |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` |
| Format | `cargo fmt --all` |
| Format check | `cargo fmt --all -- --check` |
| Build (release) | `cargo build --workspace --release` |
| Run engine (UCI) | `cargo run -p chess --release` |
| Reproducible bench | `CHESS_DETERMINISTIC=1 cargo run -p chess --release` |
| Coverage | (not configured) |
| Find a spec | `.correctless/specs/{feature}.md` |
| Check architecture | `.correctless/ARCHITECTURE.md` |
| See antipatterns | `.correctless/antipatterns.md` |
| See known bugs | `chesslib/TODO.md` |
