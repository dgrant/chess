#[cfg(test)]
mod perft_tests {
    use chesslib::board_utils::get_starting_board;
    use std::time::Instant;

    #[test]
    fn test_checkmate_detection() {
        // Create a board in checkmate position (fool's mate)
        let mut board = get_starting_board();
        eprintln!("Starting fool's mate sequence...");
        
        // Fool's mate moves: 1. f3 e5 2. g4 Qh4#
        let moves = ["f2f3", "e7e5", "g2g4", "d8h4"];
        for mv in moves {
            eprintln!("Applying move: {}", mv);
            board.apply_move_from_string(mv);
        }

        eprintln!("Checking if white king is in check...");
        assert!(board.white_king_in_check);
        
        eprintln!("Checking if position is checkmate...");
        assert!(board.is_checkmate());

        eprintln!("Running perft at depth 0...");
        let (nodes, checkmates) = board.perft(0);
        eprintln!("Depth 0 results: {} nodes, {} checkmates", nodes, checkmates);
        assert_eq!(nodes, 1, "Should be exactly 1 node at depth 0");
        assert_eq!(checkmates, 1, "Should be exactly 1 checkmate at depth 0");

        eprintln!("Running perft at depth 1...");
        let (nodes, checkmates) = board.perft(1);
        eprintln!("Depth 1 results: {} nodes, {} checkmates", nodes, checkmates);
        assert_eq!(nodes, 0, "Should be 0 nodes at depth 1 in a checkmate position");
        assert_eq!(checkmates, 0, "Should be 0 checkmates at depth 1 in a checkmate position");
    }

    #[test]
    fn test_perft_1() {
        eprintln!("\nStarting perft test at depth 1...");
        let mut board = get_starting_board();
        let start = Instant::now();
        let (nodes, checkmates) = board.perft(1);
        let duration = start.elapsed();
        eprintln!("Depth 1: {} nodes ({} checkmates) in {:.3}s",
                nodes, checkmates, duration.as_secs_f64());
        assert_eq!(nodes, 20);
        assert_eq!(checkmates, 0);
    }

    #[test]
    fn test_perft_progressive() {
        let mut board = get_starting_board();
        
        // Test depths 2 through 5, with detailed output
        let depths = [(2, 400), (3, 8_902), (4, 197_281), (5, 4_865_609)];
        
        for (depth, expected_nodes) in depths {
            eprintln!("\nRunning perft at depth {}...", depth);
            let start = Instant::now();
            let (nodes, checkmates) = board.perft(depth);
            let duration = start.elapsed();
            eprintln!("Depth {}: {} nodes ({} checkmates) in {:.3}s",
                    depth, nodes, checkmates, duration.as_secs_f64());
            assert_eq!(nodes, expected_nodes, 
                "Incorrect node count at depth {}", depth);
            
            // Print stats about checkmates
            if checkmates > 0 {
                eprintln!("Found {} checkmates at depth {}", checkmates, depth);
            }
        }
    }

    #[test]
    fn test_perft_failing_case() {
        let mut board = get_starting_board();
        eprintln!("Starting perft depth 5 test...");

        // Print initial board state
        eprintln!("\nInitial board state:");
        eprintln!("{}", board.get_debug_state());

        // Try depth 1 first to see move generation working
        eprintln!("\nTesting depth 1...");
        let (nodes1, checkmates1) = board.perft(1);
        eprintln!("Depth 1 completed successfully: {} nodes, {} checkmates", nodes1, checkmates1);
        eprintln!("Move history after depth 1: {:?}", board.get_move_history());
        assert_eq!(nodes1, 20);

        // Then try depth 2
        eprintln!("\nTesting depth 2...");
        let (nodes2, checkmates2) = board.perft(2);
        eprintln!("Depth 2 completed successfully: {} nodes, {} checkmates", nodes2, checkmates2);
        eprintln!("Move history after depth 2: {:?}", board.get_move_history());
        assert_eq!(nodes2, 400);

        // Now try depth 3 which should still work
        eprintln!("\nTesting depth 3...");
        let (nodes3, checkmates3) = board.perft(3);
        eprintln!("Depth 3 completed successfully: {} nodes, {} checkmates", nodes3, checkmates3);
        eprintln!("Move history after depth 3: {:?}", board.get_move_history());
        assert_eq!(nodes3, 8_902);

        // Try depth 4
        eprintln!("\nTesting depth 4...");
        let (nodes4, checkmates4) = board.perft(4);
        eprintln!("Depth 4 completed successfully: {} nodes, {} checkmates", nodes4, checkmates4);
        eprintln!("Move history after depth 4: {:?}", board.get_move_history());
        assert_eq!(nodes4, 197_281);
        assert_eq!(checkmates4, 8);
    }
    
    #[test]
    fn test_perft_depth_5() {
        eprintln!("\nNow attempting depth 5...");
        let mut board = get_starting_board();
    
        eprintln!("\nNow attempting depth 5...");
        let (nodes5, checkmates5) = board.perft(5);
        eprintln!("Depth 5 results: {} nodes, {} checkmates", nodes5, checkmates5);
        eprintln!("Final move history: {:?}", board.get_move_history());
        assert_eq!(nodes5, 4_865_609);
        assert_eq!(checkmates5, 347);
    }

    #[test]
    fn test_perft_depth_6() {
        eprintln!("\nNow attempting depth 6...");
        let mut board = get_starting_board();
        eprintln!("Starting perft depth 6 test...");

        let (nodes6, checkmates6) = board.perft(6);
        eprintln!("Depth 6 results: {} nodes, {} checkmates", nodes6, checkmates6);
        eprintln!("Final move history: {:?}", board.get_move_history());
        assert_eq!(nodes6, 119_060_324);
        assert_eq!(checkmates6, 10_828);
    }

    // #[test]
    // fn test_perft_depth_7() {
    //     // First castles occur at this depth.
    //     eprintln!("\nNow attempting depth 7...");
    //     let mut board = get_starting_board();
    //     eprintln!("Starting perft depth 7 test...");
    //
    //     let (nodes7, checkmates7) = board.perft(7);
    //     eprintln!("Depth 7 results: {} nodes, {} checkmates", nodes7, checkmates7);
    //     eprintln!("Final move history: {:?}", board.get_move_history());
    //     assert_eq!(nodes7, 3_195_901_860);
    //     assert_eq!(checkmates7, 435_767);
    // }
    //
    // #[test]
    // fn test_perft_depth_8() {
    //     eprintln!("\nNow attempting depth 8...");
    //     let mut board = get_starting_board();
    //     eprintln!("Starting perft depth 8 test...");
    //
    //     let (nodes, checkmates) = board.perft(8);
    //     eprintln!("Depth 8 results: {} nodes, {} checkmates", nodes, checkmates);
    //     eprintln!("Final move history: {:?}", board.get_move_history());
    //     assert_eq!(nodes, 84_998_978_956);
    //     assert_eq!(checkmates, 9_852_036);
    // }
    //
    // #[test]
    // fn test_perft_depth_9() {
    //     // First promotions occur at this depth.
    //     eprintln!("\nNow attempting depth 9...");
    //     let mut board = get_starting_board();
    //     eprintln!("Starting perft depth 9 test...");
    //
    //     let (nodes, checkmates) = board.perft(9);
    //     eprintln!("Depth 9 results: {} nodes, {} checkmates", nodes, checkmates);
    //     eprintln!("Final move history: {:?}", board.get_move_history());
    //     assert_eq!(nodes, 2_439_530_234_167);
    //     assert_eq!(checkmates, 400_191_963); // No checkmates at this depth
    // }

    #[test]
    fn test_move_history_tracking() {
        let mut board = get_starting_board();
        
        // Make a series of moves and verify history is tracked correctly
        let test_moves = ["e2e4", "e7e5", "b1c3", "b8c6", "f2f4"];
        
        eprintln!("\nStarting move history test...");
        eprintln!("Initial position");
        eprintln!("{}", board.get_debug_state());
        
        for (i, mv) in test_moves.iter().enumerate() {
            eprintln!("\nMove {}: {}", i + 1, mv);
            board.apply_move_from_string(mv);
            eprintln!("After move {}:", mv);
            eprintln!("Complete move history: {:?}", board.get_move_history());
            eprintln!("{}", board.get_debug_state());
        }
        
        // Now try to undo moves and verify the history updates correctly
        for _ in 0..test_moves.len() {
            eprintln!("\nUndoing last move...");
            board.undo_last_move();
            eprintln!("Move history after undo: {:?}", board.get_move_history());
            eprintln!("{}", board.get_debug_state());
        }

        // Add assertions to verify the history is correct
        assert_eq!(board.get_move_history().len(), 0, "Move history should be empty after all undos");
    }
}
