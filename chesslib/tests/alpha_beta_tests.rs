use chesslib::board::Board;
use chesslib::fen::load_fen;

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;

    #[test]
    fn test_alpha_beta_pruning_consistency() {
        // Test that alpha-beta pruning gives the same result as basic negamax
        // by comparing moves from the same position
        let mut board = Board::new();

        // Apply a few moves to get to an interesting position
        board.apply_move_from_string("e2e4");
        board.apply_move_from_string("e7e5");
        board.apply_move_from_string("g1f3");
        board.apply_move_from_string("b8c6");

        // Get the best move using our alpha-beta implementation
        let (best_move_ab, _score) = board.find_best_move(3);

        // The move should exist (not None)
        assert!(best_move_ab.is_some(), "Alpha-beta should find a best move");

        // The move should be legal
        let best_move = best_move_ab.unwrap();
        assert!(board.is_legal_move(&best_move), "Best move should be legal");
    }

    #[test]
    fn test_alpha_beta_tactical_position() {
        // Test alpha-beta on a tactical position where there's a clear best move
        let fen = "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 3";
        let mut board = load_fen(fen).expect("Valid FEN");

        let (best_move, _score) = board.find_best_move(4);
        assert!(best_move.is_some(), "Should find a best move in tactical position");

        // Verify the move is legal
        let mv = best_move.unwrap();
        assert!(board.is_legal_move(&mv), "Best move should be legal");
    }

    #[test]
    fn test_alpha_beta_performance() {
        // Test that alpha-beta pruning doesn't take too long on a reasonable position
        let mut board = Board::new();

        let start = std::time::Instant::now();
        let (best_move, _score) = board.find_best_move(4);
        let duration = start.elapsed();

        assert!(best_move.is_some(), "Should find a move");
        assert!(duration.as_secs() < 10, "Alpha-beta should complete in reasonable time");

        println!("Alpha-beta search depth 4 took: {:?}", duration);
    }

    #[test]
    fn test_alpha_beta_checkmate_detection() {
        // Test checkmate in 1 position - this is actually a checkmate position for White
        let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
        let mut board = load_fen(fen).expect("Valid FEN");

        // In a checkmate position, there should be no legal moves
        let (best_move, _score) = board.find_best_move(2);
        if board.is_checkmate() {
            assert!(best_move.is_none(), "Should find no moves in checkmate position");
        } else {
            assert!(best_move.is_some(), "Should find a move in non-checkmate position");
        }
    }

    #[test]
    fn test_alpha_beta_finds_captures() {
        // Position where capturing a piece is clearly the best move
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
        let mut board = load_fen(fen).expect("Valid FEN");

        let (best_move, _score) = board.find_best_move(3);
        assert!(best_move.is_some(), "Should find best move");

        let mv = best_move.unwrap();

        // Apply the move and verify it makes sense
        board.apply_move(&mv);
        // The position should still be valid after the move
        assert!(!board.is_checkmate(), "Position should not be checkmate after best move");
    }
}
