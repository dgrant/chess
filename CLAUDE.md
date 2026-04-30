## Project: chess engine (Rust)

Cargo workspace: `chess/` (binary, UCI loop) + `chesslib/` (engine library, where the tests live).

## Build / test / lint

Run cargo from the repo root. The workspace `Cargo.toml` covers both crates.

- Test: `cargo test --workspace`
- Verbose tests: `cargo test --workspace -- --nocapture`
- Library only: `cargo test -p chesslib`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`
- Format: `cargo fmt --all`
- Format check: `cargo fmt --all -- --check`
- Build (release): `cargo build --workspace --release`
- Run engine (UCI): `cargo run -p chess --release`
- Reproducible bench: set `CHESS_DETERMINISTIC=1` to suppress random tie-break

## Code conventions

- **Naming**: snake_case for files, modules, functions, variables. PascalCase for types/enums. UPPER_SNAKE_CASE for constants.
- **Test files**: live in `chesslib/tests/`, named `<feature>_tests.rs` (plural). Use built-in `#[test]` only — no external test frameworks.
- **Bug regressions**: add to `chesslib/tests/bug_tests.rs`, not a new file.
- **Modules**: each major area is its own file under `chesslib/src/` (`board.rs`, `move_generation.rs`, `search.rs`, etc.). Don't introduce subdirectories without a strong reason.
- **`unsafe`**: only one block exists in the codebase (`Square::from_bit_index` transmute). Don't add new `unsafe` without an invariant comment justifying it.
- **`println!`**: reserved for UCI protocol output (in `uci.rs`) and binary startup banner (in `main.rs`). Use `chesslib::log_to_file` for diagnostics — never `println!` from the library for non-UCI output.
- **Material values**: import from `types.rs` (`PAWN_VALUE`, `KNIGHT_VALUE`, …) or call `Piece::material_value()`. Never re-define locally.
- **Score POV**: `evaluate()` returns from White's POV. The UCI layer flips for side-to-move when reporting `score cp`. Don't introduce a third convention.

## Commit messages

Imperative-mood subject lines, no conventional-commit prefix. Two-part subjects with a colon are common when topic + detail is useful:

- `Killer moves + history heuristic for cheap quiet-move ordering`
- `Make bench reproducible: CHESS_DETERMINISTIC suppresses random tie-break`
- `Revert "Pawn structure eval: doubled and isolated pawn penalties"`

Reverts use git's standard `Revert "<original subject>"` form.

User preference: TDD-first — write failing tests first, then implement.

## GitHub

Remote: `dgrant/chess`. Use `gh` for GitHub operations (PRs, issues, checks). Default branch: `master`.

## Correctless

This project uses Correctless for structured development.
Read .correctless/AGENT_CONTEXT.md before starting any work.
Do NOT Read AGENT_CONTEXT.md from the project root — it may be stale or absent.
Available commands: /csetup, /cspec, /creview, /cmodel, /creview-spec, /ctdd, /cverify, /caudit, /cupdate-arch, /cdocs, /cpostmortem, /cdevadv, /credteam, /crefactor, /cpr-review, /ccontribute, /cmaintain, /cstatus, /csummary, /cmetrics, /cdebug, /chelp, /cwtf, /cquick, /crelease, /cexplain, /cauto, /carchitect, /cmodelupgrade

## Correctless Learnings
<!-- Auto-updated by Correctless workflow. Do not edit above this line. -->
