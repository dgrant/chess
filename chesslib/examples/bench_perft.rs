//! Move-generation benchmark.
//!
//! Perft (PERFormance Test) walks every legal move sequence from a given
//! position to a fixed depth, counting leaf nodes. It exercises move
//! generation and apply/undo without involving evaluation or search —
//! exactly what we want for measuring the speed of the bitboard
//! mutation core.
//!
//! Run with:
//!     cargo run --release --example bench_perft
//!
//! The starting-position perft node counts at each depth are well-known
//! (Stockfish, Pleco, every chess wiki) so they double as correctness
//! tests. We print expected vs actual; any mismatch indicates a
//! move-gen or apply/undo bug.

use chesslib::board_utils::get_starting_board;
use std::time::Instant;

/// (depth, expected node count, expected mate count) for the standard
/// starting position. Numbers from
/// https://www.chessprogramming.org/Perft_Results.
const STARTPOS_PERFT: &[(u32, u64, u64)] = &[
    (1, 20, 0),
    (2, 400, 0),
    (3, 8_902, 0),
    (4, 197_281, 8),
    (5, 4_865_609, 347),
    // Depth 6 is 119 060 324 nodes — uncomment if you have a few
    // minutes to spare. The current ray-casting move generator is
    // slow enough that this takes well over a minute.
    // (6, 119_060_324, 10_828),
];

fn main() {
    println!("Perft benchmark — starting position");
    println!(
        "{:<6} {:>13} {:>10} {:>10} {:>12}",
        "depth", "nodes", "mates", "secs", "Mnps"
    );
    println!("{}", "-".repeat(54));

    for &(depth, expected_nodes, expected_mates) in STARTPOS_PERFT {
        let mut board = get_starting_board();
        let start = Instant::now();
        let (nodes, mates) = board.perft(depth);
        let elapsed = start.elapsed().as_secs_f64();
        let mnps = (nodes as f64) / elapsed / 1_000_000.0;

        println!("{depth:<6} {nodes:>13} {mates:>10} {elapsed:>10.3} {mnps:>12.2}");

        assert_eq!(
            nodes, expected_nodes,
            "perft({depth}) node count diverged from the standard value — \
             a move-generation or apply/undo bug has been introduced"
        );
        assert_eq!(
            mates, expected_mates,
            "perft({depth}) mate count diverged from the standard value"
        );
    }
}
