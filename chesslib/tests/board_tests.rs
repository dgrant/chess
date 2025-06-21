extern crate chesslib;
use chesslib::board::{get_starting_board, convert_coordinate_to_bitboard_index, is_bit_set, bitboard_to_string, Color, bitboard_to_pawn_single_moves};
use chesslib::move_generation::{w_single_push_targets, w_double_push_targets, b_single_push_targets, b_double_push_targets, w_pawns_able_to_push, w_pawns_able_to_double_push, b_pawns_able_to_push, b_pawns_able_to_double_push};

#[test]
fn test_initial_board_pawns() {
    let board = get_starting_board();

    // Test white pawns are in correct position (second rank)
    for file in 0..8 {
        let index = convert_coordinate_to_bitboard_index(&format!("{}{}", ('a' as u8 + file) as char, 2));
        assert!(is_bit_set(board.white_pawns, index), "White pawn should be present at {}{}", ('a' as u8 + file) as char, 2);
    }

    // Test black pawns are in correct position (seventh rank)
    for file in 0..8 {
        let index = convert_coordinate_to_bitboard_index(&format!("{}{}", ('a' as u8 + file) as char, 7));
        assert!(is_bit_set(board.black_pawns, index), "Black pawn should be present at {}{}", ('a' as u8 + file) as char, 7);
    }
}

#[test]
fn test_coordinate_conversion() {
    assert_eq!(convert_coordinate_to_bitboard_index("a1"), 0);
    assert_eq!(convert_coordinate_to_bitboard_index("h1"), 7);
    assert_eq!(convert_coordinate_to_bitboard_index("a8"), 56);
    assert_eq!(convert_coordinate_to_bitboard_index("h8"), 63);
    assert_eq!(convert_coordinate_to_bitboard_index("e4"), 28);
}



#[test]
fn test_w_single_push_targets() {
    let board = get_starting_board();

    // Test white single push targets
    let white_single_push = w_single_push_targets(board.white_pawns, board.empty);
    let expected_white_single_push = 0x0000000000FF0000;
    let diff_white_single_push = white_single_push ^ expected_white_single_push;
    assert!(diff_white_single_push == 0, "White single push targets mismatch: diff = {:064b}", diff_white_single_push);
}

#[test]
fn test_w_double_push_targets() {
    let board = get_starting_board();

    // Test white double push targets
    let white_double_push = w_double_push_targets(board.white_pawns, board.empty);
    let expected_white_double_push = 0x00000000FF000000;
    let diff_white_double_push = white_double_push ^ expected_white_double_push;
    assert!(diff_white_double_push == 0, "White double push targets mismatch: diff = {:064b}", diff_white_double_push);
}

#[test]
fn test_b_single_push_targets() {
    let board = get_starting_board();

    // Test black single push targets
    let black_single_push = b_single_push_targets(board.black_pawns, board.empty);
    let expected_black_single_push = 0x0000FF0000000000; // Adjusted expected value
    let diff_black_single_push = black_single_push ^ expected_black_single_push;
    assert!(diff_black_single_push == 0, "Black single push targets mismatch: diff = {:064b}", diff_black_single_push);
}

#[test]
fn test_b_double_push_targets() {
    let board = get_starting_board();

    // Test black double push targets
    let black_double_push = b_double_push_targets(board.black_pawns, board.empty);
    let expected_black_double_push = 0x000000FF00000000; // Adjusted expected value
    let diff_black_double_push = black_double_push ^ expected_black_double_push;
    assert!(diff_black_double_push == 0, "Black double push targets mismatch: diff = {:064b}", diff_black_double_push);
}

#[test]
fn test_w_pawns_able_to_push() {
    let board = get_starting_board();

    // Test white pawns able to push
    let white_pawns_push = w_pawns_able_to_push(board.white_pawns, board.empty);
    let expected_white_pawns_push = 0x000000000000FF00;
    let diff_white_pawns_push = white_pawns_push ^ expected_white_pawns_push;
    assert!(diff_white_pawns_push == 0, "White pawns able to push mismatch: diff = {:064b}", diff_white_pawns_push);
}

#[test]
fn test_w_pawns_able_to_double_push() {
    let board = get_starting_board();

    // Test white pawns able to double push
    let white_pawns_double_push = w_pawns_able_to_double_push(board.white_pawns, board.empty);
    let expected_white_pawns_double_push = 0x000000000000FF00; // Adjusted expected value
    let diff_white_pawns_double_push = white_pawns_double_push ^ expected_white_pawns_double_push;
    assert!(diff_white_pawns_double_push == 0, "White pawns able to double push mismatch: diff = {:064b}", diff_white_pawns_double_push);
}

#[test]
fn test_b_pawns_able_to_push() {
    let board = get_starting_board();

    // Test black pawns able to push
    let black_pawns_push = b_pawns_able_to_push(board.black_pawns, board.empty);
    let expected_black_pawns_push = 0x00FF000000000000;
    let diff_black_pawns_push = black_pawns_push ^ expected_black_pawns_push;
    assert!(diff_black_pawns_push == 0, "Black pawns able to push mismatch: diff = {:064b}", diff_black_pawns_push);
}

#[test]
fn test_b_pawns_able_to_double_push() {
    let board = get_starting_board();

    // Test black pawns able to double push
    let black_pawns_double_push = b_pawns_able_to_double_push(board.black_pawns, board.empty);
    let expected_black_pawns_double_push = 0x00FF000000000000; // Adjusted expected value
    let diff_black_pawns_double_push = black_pawns_double_push ^ expected_black_pawns_double_push;
    assert!(diff_black_pawns_double_push == 0, "Black pawns able to double push mismatch: diff = {:064b}", diff_black_pawns_double_push);
}

#[test]
fn test_random_pawn_moves_no_capture() {
    use rand::Rng;

    let mut board = get_starting_board();
    let mut rng = rand::thread_rng();

    loop { // Run until no more valid moves for white or black
        let white_pawns_push = w_pawns_able_to_push(board.white_pawns, board.empty);
        let black_pawns_push = b_pawns_able_to_push(board.black_pawns, board.empty);

        // Break the loop if no valid moves for both sides
        if white_pawns_push == 0 && black_pawns_push == 0 {
            break;
        }

        // Randomly select a pawn to move for white
        if white_pawns_push != 0 {
            let white_pawn_that_can_push = white_pawns_push & (1 << rng.gen_range(0..64));
            if white_pawn_that_can_push != 0 {
                println!("White pawn that will push:\n{}", bitboard_to_string(white_pawn_that_can_push));
                board.white_pawns ^= white_pawn_that_can_push; // Remove pawn from current position
                board.empty ^= white_pawn_that_can_push; // Update empty squares
                let new_position = white_pawn_that_can_push << 8;
                board.white_pawns |= new_position; // Add pawn to new position
                board.empty ^= new_position; // Update empty squares
                println!("White pawns:\n{}", bitboard_to_string(board.white_pawns));
                // println!("Empty squares:\n{}", bitboard_to_string(board.empty));
            }
        }

        // Randomly select a pawn to move for black
        if black_pawns_push != 0 {
            let black_move = black_pawns_push & (1 << rng.gen_range(0..64));
            if black_move != 0 {
                println!("Black pawn that will move:\n{}", bitboard_to_string(black_move));
                board.black_pawns ^= black_move; // Remove pawn from current position
                board.empty ^= black_move; // Update empty squares
                let new_position = black_move >> 8;
                board.black_pawns |= new_position; // Add pawn to new position
                board.empty ^= new_position; // Update empty squares
                println!("Black pawns:\n{}", bitboard_to_string(board.black_pawns));
                // println!("Empty squares:\n{}", bitboard_to_string(board.empty));
            }
        }

        // Ensure no overlap between white and black pawns
        assert_eq!(board.white_pawns & board.black_pawns, 0, "White and black pawns overlap!");
    }

    // Verify that white and black pawns reached each other
    assert!(board.white_pawns ^ (board.black_pawns >> 8) == 0, "White pawns and black pawns should have reached each other!");
    assert!((board.white_pawns << 8) ^ board.black_pawns == 0, "White pawns and black pawns should have reached each other!");
}

#[test]
fn test_invalid_black_move() {
    let mut board = get_starting_board();

    // Apply a move for white
    board.apply_pawn_move("e2e4");

    // Ensure the side to move is now black
    assert_eq!(board.side_to_move, Color::Black);

    // Get moveable black pawns (the source squares)
    let moveable_black_pawns = b_pawns_able_to_push(board.black_pawns, board.empty);
    let possible_moves: Vec<String> = bitboard_to_pawn_single_moves(moveable_black_pawns, true);

    // Verify black moves are going in the correct direction
    for mv in &possible_moves {
        let from_rank = mv.chars().nth(1).unwrap().to_digit(10).unwrap();
        let to_rank = mv.chars().nth(3).unwrap().to_digit(10).unwrap();
        assert!(to_rank < from_rank, "Black pawn moving in wrong direction: {} to {}", from_rank, to_rank);
        assert!(!mv.starts_with("e2"), "Invalid move generated for black: {}", mv);
    }

    // Also verify at least one move was generated
    assert!(!possible_moves.is_empty(), "No moves were generated for black");
}

#[test]
fn test_apply_pawn_move() {
    let mut board = get_starting_board();

    // Test moving a white pawn from e2 to e4
    board.apply_pawn_move("e2e4");
    assert!(is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e4")));
    assert!(!is_bit_set(board.white_pawns, convert_coordinate_to_bitboard_index("e2")));

    // Test moving a black pawn from d7 to d5
    board.apply_pawn_move("d7d5");
    assert!(is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d5")));
    assert!(!is_bit_set(board.black_pawns, convert_coordinate_to_bitboard_index("d7")));

    // Verify empty squares are updated correctly
    assert_eq!(board.empty, !(board.any_white | board.any_black));
}

#[test]
fn test_uci_black_move_generation() {
    use chesslib::handle_uci_command;

    // Simulate UCI commands
    assert_eq!(handle_uci_command("ucinewgame"), "");
    assert_eq!(handle_uci_command("position startpos moves e2e4"), "position set");

    // Generate a move for black
    let response = handle_uci_command("go wtime 300000 btime 300000 movestogo 40");

    // Ensure the move is valid for black and not "e2e4"
    assert!(response.starts_with("bestmove"), "Response should start with 'bestmove'");
    assert!(!response.contains("e2e4"), "Invalid move generated for black: {}", response);
}
