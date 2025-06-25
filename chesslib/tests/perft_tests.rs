#[cfg(test)]
mod perft_tests {
    use chesslib::board_utils::get_starting_board;
    use std::time::Instant;

    #[test]
    fn test_perft_initial_position() {
        let mut board = get_starting_board();

        // Helper function to run perft with timing
        let mut run_perft = |depth: u32| {
            let start = Instant::now();
            let nodes = board.perft(depth);
            let duration = start.elapsed();
            let nps = (nodes as f64) / duration.as_secs_f64();
            println!("Depth {}: {} nodes in {:.3}s ({:.0} nodes/second)",
                    depth, nodes, duration.as_secs_f64(), nps);
            nodes
        };

        // Run perft for depths 0 through 4
        assert_eq!(run_perft(0), 1);
        assert_eq!(run_perft(1), 20);
        assert_eq!(run_perft(2), 400);
        assert_eq!(run_perft(3), 8_902);
        assert_eq!(run_perft(4), 197_281);
        assert_eq!(run_perft(5), 4_865_609);
    }
}
