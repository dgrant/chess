extern crate chesslib;
use chesslib::board::{bitboard_to_pawn_single_moves, bitboard_to_string, get_starting_board, is_bit_set};
use chesslib::move_generation::{b_pawns_able_to_double_push, b_pawns_able_to_push, w_pawns_able_to_double_push, w_pawns_able_to_push};
use chesslib::Square;
use chesslib::types::Color;

use chesslib::board::{Board, Move};

#[test]
fn test_initial_board_pawns() {
    let board = get_starting_board();

    // Test white pawns are in correct position (second rank)
    for file in 0..8 {
        let square = match file {
            0 => Square::A2,
            1 => Square::B2,
            2 => Square::C2,
            3 => Square::D2,
            4 => Square::E2,
            5 => Square::F2,
            6 => Square::G2,
            7 => Square::H2,
            _ => unreachable!()
        };
        assert!(is_bit_set(board.white_pawns, square.to_bit_index()), 
            "White pawn should be present at {}{}", ('a' as u8 + file) as char, 2);
    }

    // Test black pawns are in correct position (seventh rank)
    for file in 0..8 {
        let square = match file {
            0 => Square::A7,
            1 => Square::B7,
            2 => Square::C7,
            3 => Square::D7,
            4 => Square::E7,
            5 => Square::F7,
            6 => Square::G7,
            7 => Square::H7,
            _ => unreachable!()
        };
        assert!(is_bit_set(board.black_pawns, square.to_bit_index()),
            "Black pawn should be present at {}{}", ('a' as u8 + file) as char, 7);
    }
}



//
// #[test]
// fn test_w_single_push_targets() {
//     let board = get_starting_board();
//
//     // Test white single push targets
//     let white_single_push = w_single_push_targets(board.white_pawns, board.empty);
//     let expected_white_single_push = 0x0000000000FF0000;
//     let diff_white_single_push = white_single_push ^ expected_white_single_push;
//     assert!(diff_white_single_push == 0, "White single push targets mismatch: diff = {:064b}", diff_white_single_push);
// }
//
// #[test]
// fn test_w_double_push_targets() {
//     let board = get_starting_board();
//
//     // Test white double push targets
//     let white_double_push = w_double_push_targets(board.white_pawns, board.empty);
//     let expected_white_double_push = 0x00000000FF000000;
//     let diff_white_double_push = white_double_push ^ expected_white_double_push;
//     assert!(diff_white_double_push == 0, "White double push targets mismatch: diff = {:064b}", diff_white_double_push);
// }
//
// #[test]
// fn test_b_single_push_targets() {
//     let board = get_starting_board();
//
//     // Test black single push targets
//     let black_single_push = b_single_push_targets(board.black_pawns, board.empty);
//     let expected_black_single_push = 0x0000FF0000000000; // Adjusted expected value
//     let diff_black_single_push = black_single_push ^ expected_black_single_push;
//     assert!(diff_black_single_push == 0, "Black single push targets mismatch: diff = {:064b}", diff_black_single_push);
// }
//
// #[test]
// fn test_b_double_push_targets() {
//     let board = get_starting_board();
//
//     // Test black double push targets
//     let black_double_push = b_double_push_targets(board.black_pawns, board.empty);
//     let expected_black_double_push = 0x000000FF00000000; // Adjusted expected value
//     let diff_black_double_push = black_double_push ^ expected_black_double_push;
//     assert!(diff_black_double_push == 0, "Black double push targets mismatch: diff = {:064b}", diff_black_double_push);
// }

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
    use chesslib::move_generation::{w_pawns_able_to_push, b_pawns_able_to_push};

    let mut board = get_starting_board();
    let mut iteration_count = 0;
    const MAX_ITERATIONS: usize = 1000;

    loop {
        let white_pawns_push = w_pawns_able_to_push(board.white_pawns, board.empty);
        let black_pawns_push = b_pawns_able_to_push(board.black_pawns, board.empty);

        if (white_pawns_push == 0 && black_pawns_push == 0) || iteration_count >= MAX_ITERATIONS {
            break;
        }

        let possible_moves = if board.side_to_move == Color::White {
            bitboard_to_pawn_single_moves(white_pawns_push, false)
        } else {
            bitboard_to_pawn_single_moves(black_pawns_push, true)
        };

        if !possible_moves.is_empty() {
            use rand::seq::SliceRandom;
            if let Some(mv) = possible_moves.as_slice().choose(&mut rand::thread_rng()) {
                println!("Applying move: {}", mv);
                board.apply_moves_from_strings(std::iter::once(mv.to_string()));

                assert_eq!(board.white_pawns & board.black_pawns, 0, "White and black pawns overlap!");

                println!("Current board state after move {}:", mv);
                println!("{}", bitboard_to_string(board.white_pawns | board.black_pawns));
            }
        }

        iteration_count += 1;
    }

    println!("Final board state after {} iterations:", iteration_count);
    println!("White pawns:\n{}", bitboard_to_string(board.white_pawns));
    println!("Black pawns:\n{}", bitboard_to_string(board.black_pawns));
}

#[test]
fn test_invalid_black_move() {
    let mut board = get_starting_board();

    // Apply a move for white
    board.apply_moves_from_strings(std::iter::once("e2e4".to_string()));

    assert_eq!(board.side_to_move, Color::Black);

    let moveable_black_pawns = b_pawns_able_to_push(board.black_pawns, board.empty);
    let possible_moves = bitboard_to_pawn_single_moves(moveable_black_pawns, true);

    for mv in &possible_moves {
        let from_rank = mv.chars().nth(1).unwrap().to_digit(10).unwrap();
        let to_rank = mv.chars().nth(3).unwrap().to_digit(10).unwrap();
        assert!(to_rank < from_rank, "Black pawn moving in wrong direction: {} to {}", from_rank, to_rank);
        assert!(!mv.starts_with("e2"), "Invalid move generated for black: {}", mv);
    }

    assert!(!possible_moves.is_empty(), "No moves were generated for black");
}


#[test]
fn test_starting_board_side_to_move() {
    let board = get_starting_board();
    assert_eq!(board.side_to_move, Color::White);
}

#[test]
fn test_starting_board_piece_counts() {
    let board = get_starting_board();

    // Count bits set in each piece bitboard
    let white_pawn_count = board.white_pawns.count_ones();
    let black_pawn_count = board.black_pawns.count_ones();
    let white_knight_count = board.white_knights.count_ones();
    let black_knight_count = board.black_knights.count_ones();

    assert_eq!(white_pawn_count, 8, "Should be 8 white pawns");
    assert_eq!(black_pawn_count, 8, "Should be 8 black pawns");
    assert_eq!(white_knight_count, 2, "Should be 2 white knights");
    assert_eq!(black_knight_count, 2, "Should be 2 black knights");
}

#[test]
fn test_starting_board_white_piece_positions() {
    let board = get_starting_board();

    // White pieces
    // Pawns on rank 2
    assert_eq!(board.get_piece_at_coordinate_as_unicode("a2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("b2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("c2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("d2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("e2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("f2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("g2"), "♙");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("h2"), "♙");

    // Back rank pieces
    assert_eq!(board.get_piece_at_coordinate_as_unicode("a1"), "♖");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("h1"), "♖");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("b1"), "♘");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("g1"), "♘");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("c1"), "♗");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("f1"), "♗");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("d1"), "♕");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("e1"), "♔");
}

#[test]
fn test_starting_board_black_piece_positions() {
    let board = get_starting_board();

    // Black pieces
    // Pawns on rank 7
    assert_eq!(board.get_piece_at_coordinate_as_unicode("a7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("b7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("c7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("d7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("e7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("f7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("g7"), "♟");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("h7"), "♟");

    // Back rank pieces
    assert_eq!(board.get_piece_at_coordinate_as_unicode("a8"), "♜");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("h8"), "♜");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("b8"), "♞");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("g8"), "♞");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("c8"), "♝");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("f8"), "♝");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("d8"), "♛");
    assert_eq!(board.get_piece_at_coordinate_as_unicode("e8"), "♚");
}

#[test]
fn test_starting_board_bitboard_representation() {
    let board = get_starting_board();

    // Verify exact bitboard representations
    assert_eq!(board.white_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
    assert_eq!(board.black_pawns, 0b0000000011111111000000000000000000000000000000000000000000000000);
}

#[test]
fn test_coordinate_conversion() {
    assert_eq!(Square::A1.to_bit_index(), 0);
    assert_eq!(Square::H1.to_bit_index(), 7);
    assert_eq!(Square::A8.to_bit_index(), 56);
    assert_eq!(Square::H8.to_bit_index(), 63);
    assert_eq!(Square::E4.to_bit_index(), 28);

    // a file
    assert_eq!(Square::A1.to_bit_index(), 0);
    assert_eq!(Square::A2.to_bit_index(), 8);
    assert_eq!(Square::A3.to_bit_index(), 16);
    assert_eq!(Square::A4.to_bit_index(), 24);
    assert_eq!(Square::A5.to_bit_index(), 32);
    assert_eq!(Square::A6.to_bit_index(), 40);
    assert_eq!(Square::A7.to_bit_index(), 48);
    assert_eq!(Square::A8.to_bit_index(), 56);

    // Center squares
    assert_eq!(Square::D4.to_bit_index(), 27);
    assert_eq!(Square::E4.to_bit_index(), 28);
    assert_eq!(Square::D5.to_bit_index(), 35);
    assert_eq!(Square::E5.to_bit_index(), 36);

    // h file
    assert_eq!(Square::H8.to_bit_index(), 63);
}

#[test]
fn test_complete_bitboard_representation() {
    let board = get_starting_board();

    // White pieces bitboard patterns
    assert_eq!(board.white_pawns, 0b0000000000000000000000000000000000000000000000001111111100000000);
    assert_eq!(board.white_knights, 0b0000000000000000000000000000000000000000000000000000000001000010);
    assert_eq!(board.white_bishops, 0b0000000000000000000000000000000000000000000000000000000000100100);
    assert_eq!(board.white_rooks, 0b0000000000000000000000000000000000000000000000000000000010000001);
    assert_eq!(board.white_queen, 0b0000000000000000000000000000000000000000000000000000000000001000);
    assert_eq!(board.white_king, 0b0000000000000000000000000000000000000000000000000000000000010000);

    // Black pieces bitboard patterns
    assert_eq!(board.black_pawns, 0b0000000011111111000000000000000000000000000000000000000000000000);
    assert_eq!(board.black_knights, 0b0100001000000000000000000000000000000000000000000000000000000000);
    assert_eq!(board.black_bishops, 0b0010010000000000000000000000000000000000000000000000000000000000);
    assert_eq!(board.black_rooks, 0b1000000100000000000000000000000000000000000000000000000000000000);
    assert_eq!(board.black_queen, 0b0000100000000000000000000000000000000000000000000000000000000000);
    assert_eq!(board.black_king, 0b0001000000000000000000000000000000000000000000000000000000000000);

    // Composite bitboards
    assert_eq!(board.any_white,
               board.white_pawns | board.white_knights | board.white_bishops |
                   board.white_rooks | board.white_queen | board.white_king);

    assert_eq!(board.any_black,
               board.black_pawns | board.black_knights | board.black_bishops |
                   board.black_rooks | board.black_queen | board.black_king);

    assert_eq!(board.empty, !(board.any_white | board.any_black));

    // Verify the relationship between the bitboards
    assert_eq!(board.any_white & board.any_black, 0); // No overlap between white and black pieces
    assert_eq!(board.any_white | board.any_black | board.empty, !0u64); // All squares are accounted for
}

#[test]
fn test_bitboard_layout_matches_chess_coordinates() {
    // This test verifies that the bit layout matches the chess board layout
    // according to the Wisconsin CS page convention

    // Test rank-by-rank layout (8 bits per rank)
    assert_eq!(Square::A1.to_bit_index(), 0);
    assert_eq!(Square::H1.to_bit_index(), 7);
    assert_eq!(Square::A2.to_bit_index(), 8);
    assert_eq!(Square::H2.to_bit_index(), 15);
    assert_eq!(Square::A8.to_bit_index(), 56);
    assert_eq!(Square::H8.to_bit_index(), 63);

    // Verify specific squares from the Wisconsin CS page examples
    // These are the bit positions as shown in their diagrams
    let board = get_starting_board();

    // White king is at e1 (bit 4)
    assert!(is_bit_set(board.white_king, 4));

    // Black queen is at d8 (bit 59)
    assert!(is_bit_set(board.black_queen, 59));
}

#[test]
fn test_apply_move() {
    let mut board = get_starting_board();

    // Test moving a white pawn from e2 to e4
    board.apply_move(&Move { src: Square::E2, target: Square::E4 });
    assert!(is_bit_set(board.white_pawns, Square::E4.to_bit_index()));
    assert!(!is_bit_set(board.white_pawns, Square::E2.to_bit_index()));

    // Test moving a black pawn from d7 to d5
    board.apply_move(&Move { src: Square::D7, target: Square::D5 });
    assert!(is_bit_set(board.black_pawns, Square::D5.to_bit_index()));
    assert!(!is_bit_set(board.black_pawns, Square::D7.to_bit_index()));

    // Verify empty squares are updated correctly
    assert_eq!(board.empty, !(board.any_white | board.any_black));
}

#[test]
fn test_side_to_move_after_sequence() {
    let mut board = get_starting_board();
    assert_eq!(board.side_to_move, Color::White);

    // First move: White e2e4
    board.apply_move(&Move { src: Square::E2, target: Square::E4 });
    assert_eq!(board.side_to_move, Color::Black);

    // Second move: Black d7d6
    board.apply_move(&Move { src: Square::D7, target: Square::D6 });
    assert_eq!(board.side_to_move, Color::White);

    // Third move: White g2g4
    board.apply_move(&Move { src: Square::G2, target: Square::G4 });
    assert_eq!(board.side_to_move, Color::Black);

    // Get next move - should suggest a black move
    let next_move = board.get_next_move();
    // Check if move starts with a valid black piece position
    assert!(
        // Pawns
        next_move.starts_with("a7") || next_move.starts_with("b7") ||
            next_move.starts_with("c7") || next_move.starts_with("d6") ||
            next_move.starts_with("e7") || next_move.starts_with("f7") ||
            next_move.starts_with("g7") || next_move.starts_with("h7") ||
            // Knights
            next_move.starts_with("b8") || next_move.starts_with("g8") ||
            // Bishops
            next_move.starts_with("c8") || next_move.starts_with("f8") ||
            // Rooks
            next_move.starts_with("a8") || next_move.starts_with("h8") ||
            // Queen
            next_move.starts_with("d8") ||
            // King
            next_move.starts_with("e8"),
        "Move {} should be a valid black piece move", next_move
    );
}

#[test]
#[should_panic(expected = "Attempted to move a White piece during Black's turn")]
fn test_apply_move_same_side_twice_fails() {
    let mut board = get_starting_board();

    // Test moving a white pawn from e2 to e4
    board.apply_move(&Move { src: Square::E2, target: Square::E4 });
    assert!(is_bit_set(board.white_pawns, Square::E4.to_bit_index()));
    assert!(!is_bit_set(board.white_pawns, Square::E2.to_bit_index()));

    // Test moving another white pawn from d2 to d4 - should panic
    board.apply_move(&Move { src: Square::D2, target: Square::D4 });
}

#[test]
fn test_knight_moves() {
    let mut board = get_starting_board();

    // Move white knight from b1 to c3
    board.apply_move(&Move { src: Square::B1, target: Square::C3 });
    assert!(is_bit_set(board.white_knights, Square::C3.to_bit_index()));
    assert!(!is_bit_set(board.white_knights, Square::B1.to_bit_index()));
    assert_eq!(board.side_to_move, Color::Black);

    // Move black knight from g8 to f6
    board.apply_move(&Move { src: Square::G8, target: Square::F6 });
    assert!(is_bit_set(board.black_knights, Square::F6.to_bit_index()));
    assert!(!is_bit_set(board.black_knights, Square::G8.to_bit_index()));
    assert_eq!(board.side_to_move, Color::White);

    // Test a capture: white knight takes black pawn
    board.apply_move(&Move { src: Square::C3, target: Square::D5 });
    assert!(is_bit_set(board.white_knights, Square::D5.to_bit_index()));
    assert!(!is_bit_set(board.white_knights, Square::C3.to_bit_index()));
    assert!(!is_bit_set(board.black_pawns, Square::D5.to_bit_index()));
}

#[test]
fn test_bitboard_to_moves() {
    let board = get_starting_board();

    // Test with a single source and multiple targets
    let source = Square::E4.to_bitboard();  // Knight on e4
    let targets = Square::F6.to_bitboard() | Square::D6.to_bitboard() | Square::C5.to_bitboard();

    let moves = board.bitboard_to_moves(source, targets);

    // Verify the moves are generated correctly
    assert!(moves.contains(&"e4f6".to_string()));
    assert!(moves.contains(&"e4d6".to_string()));
    assert!(moves.contains(&"e4c5".to_string()));
    assert_eq!(moves.len(), 3);
}

#[test]
fn test_bitboard_to_moves2() {
    let board = get_starting_board();

    // Test with a different single source and single target
    let source = Square::G1.to_bitboard();  // Knight on g1
    let target = Square::F3.to_bitboard();  // Target square f3

    let moves = board.bitboard_to_moves(source, target);

    // Verify move is generated correctly
    assert!(moves.contains(&"g1f3".to_string()));
    assert_eq!(moves.len(), 1);
}

#[test]
fn test_bitboard_to_moves3() {
    let board = get_starting_board();

    // Test b1 knight separately
    let source = Square::B1.to_bitboard();  // Knight on b1
    let target = Square::C3.to_bitboard();  // Target square c3

    let moves = board.bitboard_to_moves(source, target);

    // Verify move is generated correctly
    assert!(moves.contains(&"b1c3".to_string()));
    assert_eq!(moves.len(), 1);
}

#[test]
fn test_bitboard_to_moves4() {
    let board = get_starting_board();
    let source = Square::E4.to_bitboard();  // Knight on e4

    // Test with no target squares (should produce empty move list)
    assert!(board.bitboard_to_moves(source, 0).is_empty());
}

#[test]
fn test_get_next_move() {
    let mut board = get_starting_board();
    assert_eq!(board.side_to_move, Color::White);

    // White's first move should be either a pawn move or knight move
    let first_move = board.get_next_move();
    assert!(first_move.starts_with("a2") || first_move.starts_with("b2") ||
                first_move.starts_with("c2") || first_move.starts_with("d2") ||
                first_move.starts_with("e2") || first_move.starts_with("f2") ||
                first_move.starts_with("g2") || first_move.starts_with("h2") ||
                first_move.starts_with("b1") || first_move.starts_with("g1"),
            "First move {} should be a white pawn or knight move", first_move);

    // Apply the first move and get a response from black
    board.apply_move_from_string(&first_move);
    assert_eq!(board.side_to_move, Color::Black);

    let black_move = board.get_next_move();
    assert!(black_move.starts_with("a7") || black_move.starts_with("b7") ||
                black_move.starts_with("c7") || black_move.starts_with("d7") ||
                black_move.starts_with("e7") || black_move.starts_with("f7") ||
                black_move.starts_with("g7") || black_move.starts_with("h7") ||
                black_move.starts_with("b8") || black_move.starts_with("g8"),
            "Move {} should be a black pawn or knight move", black_move);

    // Apply black's move and get another white move
    board.apply_move_from_string(&black_move);
    assert_eq!(board.side_to_move, Color::White);

    // Get another move - make sure it's still valid format
    let next_move = board.get_next_move();
    assert_eq!(next_move.len(), 4, "Move should be in format 'e2e4', got {}", next_move);
    assert!(next_move.chars().all(|c| c.is_ascii_alphanumeric()),
            "Move should only contain letters and numbers, got {}", next_move);
}

#[test]
fn test_get_next_moves() {
    let board = get_starting_board();

    // Test getting no moves
    let no_moves = board.get_next_moves(0);
    assert!(no_moves.is_empty(), "Should return empty vec when n=0");

    // Test getting single move (should be same as get_next_move)
    let one_move = board.get_next_moves(1);
    assert_eq!(one_move.len(), 1, "Should return exactly one move when n=1");

    // Test getting multiple moves
    let five_moves = board.get_next_moves(5);
    assert!(five_moves.len() <= 5, "Should not return more moves than requested");
    assert!(!five_moves.is_empty(), "Should return at least one move");

    // Verify all moves are valid white moves from starting position
    for mv in five_moves {
        assert!(mv.starts_with("a2") || mv.starts_with("b2") ||
                    mv.starts_with("c2") || mv.starts_with("d2") ||
                    mv.starts_with("e2") || mv.starts_with("f2") ||
                    mv.starts_with("g2") || mv.starts_with("h2") ||
                    mv.starts_with("b1") || mv.starts_with("g1"),
                "Move {} should be a white pawn or knight move", mv);
    }

    // Test getting all moves (n = -1)
    let all_moves = board.get_next_moves(-1);
    assert!(!all_moves.is_empty(), "Should return all possible moves");
    // In starting position, each pawn can move 1 or 2 squares (16 moves)
    // and each knight has 2 possible moves (4 moves total)
    // So we expect exactly 20 possible moves
    assert_eq!(all_moves.len(), 20, "Starting position should have exactly 20 possible moves");
}

#[test]
fn test_hello_world() {
    println!("hello world");
}

#[test]
fn test_is_square_attacked_pawns() {
    let mut board = get_starting_board();

    // In starting position, no center squares are attacked
    assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White));
    assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::Black));

    // After 1.e4, e4 is not attacked by any pieces
    board.apply_move(&Move { src: Square::E2, target: Square::E4 });
    assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // This is false because white pieces can't attack squares occupied by white pieces
    assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::Black)); // This is false because the pawn on e4 isn't attacked by any black pieces yet.

    // Black responds with pawn to d5
    board.apply_move(&Move { src: Square::D7, target: Square::D5 });
    // d5 is attacked by white pawn on e4
    assert!(board.is_square_attacked(Square::D5.to_bit_index(), Color::White));
    assert!(!board.is_square_attacked(Square::D5.to_bit_index(), Color::Black)); // Just double-check make sure that black can't attack black's own pawn
    // e4 is attacked by black pawn on d5
    assert!(board.is_square_attacked(Square::E4.to_bit_index(), Color::Black));
    assert!(!board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // Just double-check make sure that white can't attack white's own pawn
}

#[test]
fn test_is_square_attacked_knights() {
    let mut board = get_starting_board();
    board.apply_move(&Move { src: Square::E2, target: Square::E4 });
    board.apply_move(&Move { src: Square::D7, target: Square::D5 });

    // Test knight attacks
    board.apply_move(&Move { src: Square::B1, target: Square::C3 });
    // Just a dummy move for black
    board.apply_move(&Move { src: Square::H7, target: Square::H6 });
    // move the e pawn up so it's not being attacked, or attacking the D pawn anymore:
    board.apply_move(&Move { src: Square::E4, target: Square::E5 });
    assert!(board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // White knight attacks black pawn on d5
    assert!(board.is_square_attacked(Square::E4.to_bit_index(), Color::Black)); // E4 is empty but is being attacked by the black pawn on d5
}

#[test]
fn test_is_square_attacked_bishops() {
    // Test bishop attacks
    let mut attack_test_board = Board {
        white_bishops: Square::C4.to_bitboard(),
        black_queen: Square::F7.to_bitboard(), // Add a piece to block diagonal
        ..get_starting_board()
    };
    attack_test_board.update_composite_bitboards();
    assert!(attack_test_board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // Bishop attacks e6
    assert!(attack_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // Bishop attacks e6
    assert!(attack_test_board.is_square_attacked(Square::F7.to_bit_index(), Color::White)); // Bishop attacks e6
    assert!(!attack_test_board.is_square_attacked(Square::G8.to_bit_index(), Color::White)); // Bishop attack blocked by queen

    // TODO: Add more bishop tests for different positions
}

#[test]
fn test_is_square_attacked_rooks() {
    // Test rook attacks
    let mut rook_test_board = Board {
        white_rooks: Square::E4.to_bitboard(),
        black_pawns: Square::E6.to_bitboard(), // Add a piece to block file
        ..get_starting_board()
    };
    rook_test_board.update_composite_bitboards();
    assert!(rook_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // Rook attacks e5
    assert!(rook_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // Rook attacks e5
    assert!(!rook_test_board.is_square_attacked(Square::E7.to_bit_index(), Color::White)); // Rook attack blocked by pawn
    assert!(!rook_test_board.is_square_attacked(Square::E8.to_bit_index(), Color::White)); // Rook attack blocked by pawn
}

#[test]
fn test_is_square_attacked_queen() {
    // Test queen attacks
    let mut queen_test_board = Board {
        white_queen: Square::D4.to_bitboard(),
        ..get_starting_board()
    };
    queen_test_board.update_composite_bitboards();
    assert!(!queen_test_board.is_square_attacked(Square::D2.to_bit_index(), Color::White)); // Not an attack, it's our own pawn
    assert!(queen_test_board.is_square_attacked(Square::D3.to_bit_index(), Color::White)); // Queen attacks vertically down
    assert!(queen_test_board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // Queen attacks vertically up
    assert!(queen_test_board.is_square_attacked(Square::D6.to_bit_index(), Color::White)); // Queen attacks vertically up
    assert!(queen_test_board.is_square_attacked(Square::D7.to_bit_index(), Color::White)); // Queen attacks vertically up
    assert!(queen_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
    assert!(queen_test_board.is_square_attacked(Square::F6.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
    assert!(queen_test_board.is_square_attacked(Square::G7.to_bit_index(), Color::White)); // Queen attacks diagonally north-east
    assert!(!queen_test_board.is_square_attacked(Square::H8.to_bit_index(), Color::White)); // Queen attack blocked by G7
    assert!(queen_test_board.is_square_attacked(Square::A4.to_bit_index(), Color::White)); // Queen attacks horizontally left
    assert!(queen_test_board.is_square_attacked(Square::B4.to_bit_index(), Color::White)); // Queen attacks horizontally left
    assert!(queen_test_board.is_square_attacked(Square::C4.to_bit_index(), Color::White)); // Queen attacks horizontally left
    assert!(queen_test_board.is_square_attacked(Square::E4.to_bit_index(), Color::White)); // Queen attacks horizontally right
    assert!(queen_test_board.is_square_attacked(Square::F4.to_bit_index(), Color::White)); // Queen attacks horizontally right
    assert!(queen_test_board.is_square_attacked(Square::G4.to_bit_index(), Color::White)); // Queen attacks horizontally right
    assert!(queen_test_board.is_square_attacked(Square::H4.to_bit_index(), Color::White)); // Queen attacks horizontally right

}

#[test]
fn test_is_legal_move() {
    // Test 1: Starting position, any legal move is valid
    let board = get_starting_board();
    assert!(board.is_legal_move(&Move { src: Square::E2, target: Square::E4 }));
}

#[test]
fn test_is_legal_move_complex() {
    // Test 2: White king in check by rook on E8
    let mut check_board = Board {
        white_king: Square::E1.to_bitboard(),
        white_queen: Square::D2.to_bitboard(), // Queen in position to block check
        black_rooks: Square::E8.to_bitboard(),
        side_to_move: Color::White,
        white_pawns: 0,
        white_knights: 0,
        white_bishops: Square::D7.to_bitboard(), // Bishop in position to capture rook
        white_rooks: 0,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queen: 0,
        black_king: Square::G8.to_bitboard(), // Black king safely away
        any_white: 0,
        any_black: 0,
        empty: 0,
        white_king_in_check: true,
        black_king_in_check: false,
    };
    check_board.update_composite_bitboards();
    check_board.update_check_state();

    assert!(check_board.white_king_in_check);

    // Legal moves that escape check
    assert!(check_board.is_legal_move(&Move { src: Square::E1, target: Square::F1 })); // King escapes sideways
    assert!(check_board.is_legal_move(&Move { src: Square::E1, target: Square::D1 })); // King escapes other way
    assert!(check_board.is_legal_move(&Move { src: Square::D7, target: Square::E8 })); // Bishop captures rook
    assert!(check_board.is_legal_move(&Move { src: Square::D2, target: Square::E2 })); // Queen blocks check

    // Illegal moves that don't escape check
    assert!(!check_board.is_legal_move(&Move { src: Square::D2, target: Square::D3 })); // Queen moves away
    assert!(!check_board.is_legal_move(&Move { src: Square::E1, target: Square::E2 })); // King moves into check
}


#[test]
fn test_is_square_attacked_king() {
    // Test king attacks
    let mut king_test_board = Board {
        white_king: Square::E4.to_bitboard(),
        ..get_starting_board()
    };
    king_test_board.update_composite_bitboards();
    assert!(king_test_board.is_square_attacked(Square::D5.to_bit_index(), Color::White)); // King attacks above-left
    assert!(king_test_board.is_square_attacked(Square::E5.to_bit_index(), Color::White)); // King attacks above
    assert!(king_test_board.is_square_attacked(Square::F5.to_bit_index(), Color::White)); // King attacks above-right

    assert!(king_test_board.is_square_attacked(Square::D4.to_bit_index(), Color::White)); // King attacks left
    assert!(king_test_board.is_square_attacked(Square::F4.to_bit_index(), Color::White)); // King attacks right

    assert!(king_test_board.is_square_attacked(Square::D3.to_bit_index(), Color::White)); // King attacks below-left
    assert!(king_test_board.is_square_attacked(Square::E3.to_bit_index(), Color::White)); // King attacks below
    assert!(king_test_board.is_square_attacked(Square::F3.to_bit_index(), Color::White)); // King attacks below-right

    assert!(!king_test_board.is_square_attacked(Square::E6.to_bit_index(), Color::White)); // King can't attack 2 squares away
    assert!(!king_test_board.is_square_attacked(Square::C4.to_bit_index(), Color::White)); // King can't attack 2 squares away
    assert!(!king_test_board.is_square_attacked(Square::G4.to_bit_index(), Color::White)); // King can't attack 2 squares away
    assert!(!king_test_board.is_square_attacked(Square::E2.to_bit_index(), Color::White)); // King can't attack 2 squares away
}
