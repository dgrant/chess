//! Search benchmark.
//!
//! Times `find_best_move(depth)` on a small set of positions
//! representative of different search regimes:
//!
//!   - opening    — many quiet moves, lots of branches
//!   - middlegame — tactics + structure, moderately busy move list
//!   - endgame    — sparse, deep search lines
//!
//! Run with:
//!     CHESS_DETERMINISTIC=1 cargo run --release --example bench_search
//!
//! `CHESS_DETERMINISTIC=1` is required for stable timing — without it
//! the engine breaks score ties randomly and the search tree explored
//! at any given depth varies between runs.

use chesslib::fen::load_fen;
use chesslib::search::Searcher;
use std::time::Instant;

/// Each fixture: (label, FEN, depth-to-search).
/// Depths are tuned so each search takes a fraction of a second on
/// modern hardware — small enough to iterate quickly, large enough
/// that timing noise is < 5 %.
const FIXTURES: &[(&str, &str, i32)] = &[
    (
        "starting position",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        5,
    ),
    (
        "italian game (after 1.e4 e5 2.Nf3 Nc6 3.Bc5 Bc5)",
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4",
        5,
    ),
    (
        "tactical middlegame (kiwipete)",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        4,
    ),
    ("endgame (KP vs K)", "8/8/8/4k3/4P3/4K3/8/8 w - - 0 1", 7),
];

fn main() {
    println!("Search benchmark — find_best_move(depth) per position");
    println!(
        "{:<48} {:>5} {:>10} {:>14}",
        "position", "depth", "secs", "best move"
    );
    println!("{}", "-".repeat(80));

    let mut total_secs = 0.0;
    for &(label, fen, depth) in FIXTURES {
        let mut board = load_fen(fen).expect("FEN should parse");
        let start = Instant::now();
        let (best, score) = Searcher::new().find_best_move(&mut board, depth);
        let elapsed = start.elapsed().as_secs_f64();
        total_secs += elapsed;

        let best_str = best
            .map(|m| format!("{m} ({score:+})"))
            .unwrap_or_else(|| "<none>".into());
        println!(
            "{:<48} {:>5} {:>10.3} {:>14}",
            // Truncate long labels so the table stays aligned.
            if label.len() > 47 {
                &label[..47]
            } else {
                label
            },
            depth,
            elapsed,
            best_str
        );
    }
    println!("{}", "-".repeat(80));
    println!("{:<48} {:>5} {:>10.3}", "total", "", total_secs);
}
