use chesslib::board::Board;

fn main() {
    let mut board = Board::new();
    
    // Set up the test position
    board.apply_move_from_string("f2f4");
    board.apply_move_from_string("e7e5");
    
    println!("Position after f2-f4, e7-e5:");
    println!("Side to move: {:?}", board.side_to_move);
    
    // Test the capture move specifically
    let capture_move = chesslib::types::Move::try_from("f4e5").unwrap();
    board.apply_move(&capture_move);
    let capture_eval = board.evaluate();
    board.undo_last_move();
    
    // Test the knight move that was returned
    let knight_move = chesslib::types::Move::try_from("g1h3").unwrap();
    board.apply_move(&knight_move);
    let knight_eval = board.evaluate();
    board.undo_last_move();
    
    println!("Evaluation after f4xe5: {}", capture_eval);
    println!("Evaluation after g1-h3: {}", knight_eval);
    
    // Get all possible moves and their evaluations from WHITE's perspective
    let mut moves = Vec::new();
    board.get_all_raw_moves_append(&mut moves);
    
    println!("\nAll possible moves and their evaluations (from White's perspective):");
    for mv in moves.iter() {
        board.apply_move(mv);
        let raw_eval = board.evaluate(); // This is always from White's perspective
        // Since we just moved as White, the evaluation should be from Black's perspective
        // But evaluate() always returns from White's perspective, so we need to negate it
        // to get what White's position looks like after the move
        let eval = -raw_eval; // Negate because after White moves, we evaluate from Black's perspective
        board.undo_last_move();
        
        if mv.to_string() == "f4e5" || mv.to_string() == "g1h3" || mv.to_string() == "b1c3" {
            println!("{}: {} (RAW: {})", mv, eval, raw_eval);
        }
    }
    
    // Let's manually implement what find_best_move should do and compare
    let mut best_score = i64::MIN;
    let mut best_move = None;
    
    println!("\nManual search through all moves:");
    for mv in moves.iter() {
        board.apply_move(mv);
        // After making the move, the side to move has changed
        // evaluate() returns from White's perspective
        // If it's now Black's turn, the evaluation is what White sees
        // If it's now White's turn, we need to negate to get what the previous player (who just moved) sees
        let score = if board.side_to_move == chesslib::types::Color::Black {
            // It's Black's turn, so White just moved
            // evaluate() returns from White's perspective, which is what we want
            board.evaluate()
        } else {
            // It's White's turn, so Black just moved  
            // evaluate() returns from White's perspective, so negate to get Black's perspective
            -board.evaluate()
        };
        board.undo_last_move();
        
        if score > best_score {
            best_score = score;
            best_move = Some(mv.clone());
        }
        
        if mv.to_string() == "f4e5" || mv.to_string() == "g1h3" || mv.to_string() == "b1c3" {
            println!("{}: {}", mv, score);
        }
    }
    
    println!("\nManual best move: {:?} with score: {}", best_move, best_score);
    
    // Test find_best_move
    let best_move = board.find_best_move(3);
    println!("find_best_move result: {:?}", best_move);
}
