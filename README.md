# chess

A UCI-compatible chess engine written in Rust. Plug it into any UCI-speaking GUI (CuteChess, Arena, BanksiaGUI) or test harness (fastchess).

## Layout

Cargo workspace with two member crates:

- [`chesslib/`](chesslib/) — the engine: bitboard board representation, move generation, negamax + alpha-beta search with iterative deepening and quiescence, MVV-LVA capture ordering, killer moves and history heuristic, evaluation (material, piece-square tables, mobility, king safety), FEN parsing, UCI protocol, file logging.
- [`chess/`](chess/) — thin binary that runs the UCI stdin/stdout loop.

## Build and run

```bash
cargo build --workspace --release
cargo run -p chess --release
```

The engine then reads UCI commands from stdin. Common ones:

```
uci
isready
position startpos moves e2e4 e7e5
go depth 6
```

## Test

```bash
cargo test --workspace
```

About 60 integration tests live in `chesslib/tests/`, covering move generation, perft, FEN round-trips, castling, en passant, pawn promotion, alpha-beta correctness, UCI protocol, and bug regressions.

## Lint and format

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
```

`rustfmt` is enforced in CI. Clippy is currently advisory while the existing warning backlog is cleaned up.

## Reproducible benchmarking

By default the engine breaks ties between equal-scored moves randomly so self-play games show variety. Set `CHESS_DETERMINISTIC=1` to suppress the random tie-break and get the same game on every run with the same input:

```bash
CHESS_DETERMINISTIC=1 cargo run -p chess --release
```

## Engine features

- Bitboard board representation (twelve `u64`s, one per piece-type-and-color)
- Pseudo-legal move generation with file masks to prevent wrap-around
- Negamax with alpha-beta pruning
- Iterative deepening (caps at depth 20)
- Quiescence search at the depth-0 horizon
- MVV-LVA capture ordering
- Killer moves (two slots per ply) and history heuristic for quiet-move ordering
- Piece-square tables (PeSTO middlegame values)
- Material, mobility, king safety, bishop pair, check evaluation
- UCI protocol: `position`, `go movetime/wtime/btime/depth`, score reporting from side-to-move POV

## Status

Hobby project. Open bugs and roadmap in [`chesslib/TODO.md`](chesslib/TODO.md).

## License

MIT — see [LICENSE](LICENSE).
