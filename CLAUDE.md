## Project: chess engine (Rust)

Cargo workspace: `chess/` (binary, UCI loop) + `chesslib/` (engine library, where the tests live).

## Constitutional goal: human readability

**Readability is the top-priority quality of this codebase, treated as a hard
constraint, second only to correctness and performance.** A reader who has
never seen the file before should be able to understand what the code does,
why it does it that way, and what the surrounding constraints are — without
having to dig through git history or other files.

This OVERRIDES any default tendency to write terse code or avoid comments.
For this project specifically:

- **Comment generously.** Explain the *why*, the chess-engine-specific
  invariants, the bitboard tricks, the score-sign conventions, the
  "we tried X and reverted because Y" history, and anything a non-expert
  reader would have to puzzle over. Multi-line `///` doc comments on
  non-trivial functions are encouraged. Inline comments explaining
  non-obvious bitboard math, sign flips, or chess rules are encouraged.
- **Name things fully.** Prefer `score_from_white_pov` over `s` even at
  cost of a few extra characters. Cryptic abbreviations (`mv`, `bb`, `sq`)
  are fine when ubiquitous in the chess-engine domain, but spell out
  anything domain-specific (e.g., `mvv_lva` should be expanded somewhere
  in a comment if it appears).
- **Show the chess intent.** When a piece of code maps to a chess concept
  (e.g., "this resets the halfmove clock per the 50-move rule," "this
  checks for the king-passes-through-attacked-square castling rule"),
  say so in a comment. Future maintainers may not know the rule by name.
- **Performance still wins when forced to choose.** If the readable form
  is genuinely slower in a hot path, write the fast form AND comment it
  — never silently sacrifice clarity for performance without explaining
  what was traded. The default should be readable; performance hacks
  should look like deliberate exceptions with a `// PERF:` comment.
- **No half-explained magic numbers.** Constants like `0xfefefefefefefefe`
  must have a one-line comment explaining what they are (`NOT_A_FILE`).

This is a hard constitutional goal. When in doubt, err on the side of more
explanation, longer names, and more comments — not less.

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
