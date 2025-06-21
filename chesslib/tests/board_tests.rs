extern crate chesslib;
use chesslib::board::{get_starting_board, convert_coordinate_to_bitboard_index, is_bit_set, bitboard_to_string, Color, bitboard_to_pawn_single_moves};
use chesslib::move_generation::{w_pawns_able_to_push, w_pawns_able_to_double_push, b_pawns_able_to_push, b_pawns_able_to_double_push};

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

