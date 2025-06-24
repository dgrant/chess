#[cfg(test)]
mod tests {
    use chesslib::move_generation::{b_pawn_attacks, b_pawn_east_attacks, b_pawn_west_attacks, b_pawns_attack_targets, bishop_legal_moves, bishop_moves, king_legal_moves, king_moves, knight_legal_moves, knight_moves, queen_legal_moves, queen_moves, rook_legal_moves, rook_moves, w_pawn_attacks, w_pawn_east_attacks, w_pawn_west_attacks, w_pawns_attack_targets};
    use chesslib::Square;


    #[test]
    fn test_white_pawn_attacks() {
        // Test a white pawn in the center of the board (e4, bit 28)
        let wp = 1u64 << 28;  // e4
        let east_attacks = w_pawn_east_attacks(wp);
        let west_attacks = w_pawn_west_attacks(wp);
        let all_attacks = w_pawn_attacks(wp);

        // Should attack f5 (bit 37) and d5 (bit 35)
        assert_eq!(east_attacks, 1u64 << 37); // f5
        assert_eq!(west_attacks, 1u64 << 35); // d5
        assert_eq!(all_attacks, (1u64 << 37) | (1u64 << 35));
    }

    #[test]
    fn test_black_pawn_attacks() {
        // Test a black pawn in the center of the board (e5, bit 36)
        let bp = 1u64 << 36;  // e5
        let east_attacks = b_pawn_east_attacks(bp);
        let west_attacks = b_pawn_west_attacks(bp);
        let all_attacks = b_pawn_attacks(bp);

        // Should attack f4 (bit 29) and d4 (bit 27)
        assert_eq!(east_attacks, 1u64 << 29); // f4
        assert_eq!(west_attacks, 1u64 << 27); // d4
        assert_eq!(all_attacks, (1u64 << 29) | (1u64 << 27));
    }

    #[test]
    fn test_pawn_attacks_edge_cases() {
        // Test pawns on A and H files to ensure no wrapping occurs

        // White pawn on a2 (bit 8)
        let wp_a_file = 1u64 << 8;
        assert_eq!(w_pawn_east_attacks(wp_a_file), 1u64 << 17); // only b3
        assert_eq!(w_pawn_west_attacks(wp_a_file), 0); // no wrap to h-file

        // White pawn on h2 (bit 15)
        let wp_h_file = 1u64 << 15;
        assert_eq!(w_pawn_east_attacks(wp_h_file), 0); // no wrap to a-file
        assert_eq!(w_pawn_west_attacks(wp_h_file), 1u64 << 22); // only g3

        // Black pawn on a7 (bit 48)
        let bp_a_file = 1u64 << 48;
        assert_eq!(b_pawn_east_attacks(bp_a_file), 1u64 << 41); // only b6
        assert_eq!(b_pawn_west_attacks(bp_a_file), 0); // no wrap to h-file

        // Black pawn on h7 (bit 55)
        let bp_h_file = 1u64 << 55;
        assert_eq!(b_pawn_east_attacks(bp_h_file), 0); // no wrap to a-file
        assert_eq!(b_pawn_west_attacks(bp_h_file), 1u64 << 46); // only g6
    }

    #[test]
    fn test_pawn_attack_targets() {
        // Test white pawn attacking black pieces
        let wp = 1u64 << 28;  // white pawn on e4
        let black_pieces = (1u64 << 37) | (1u64 << 35);  // black pieces on f5 and d5
        let attack_targets = w_pawns_attack_targets(wp, black_pieces);
        assert_eq!(attack_targets, black_pieces); // can attack both pieces

        // Test black pawn attacking white pieces
        let bp = 1u64 << 36;  // black pawn on e5
        let white_pieces = (1u64 << 29) | (1u64 << 27);  // white pieces on f4 and d4
        let attack_targets = b_pawns_attack_targets(bp, white_pieces);
        assert_eq!(attack_targets, white_pieces); // can attack both pieces

        // Test when no pieces are available to capture
        let empty_board = 0u64;
        assert_eq!(w_pawns_attack_targets(wp, empty_board), 0);
        assert_eq!(b_pawns_attack_targets(bp, empty_board), 0);
    }

    #[test]
    fn test_multiple_pawn_attacks() {
        // Test multiple white pawns attacking
        let wp = (1u64 << 28) | (1u64 << 29);  // white pawns on e4 and f4
        let black_pieces = (1u64 << 37) | (1u64 << 38);  // black pieces on f5 and g5
        let attack_targets = w_pawns_attack_targets(wp, black_pieces);
        assert_eq!(attack_targets, black_pieces); // both pawns can attack

        // Test multiple black pawns attacking
        let bp = (1u64 << 36) | (1u64 << 37);  // black pawns on e5 and f5
        let white_pieces = (1u64 << 29) | (1u64 << 30);  // white pieces on f4 and g4
        let attack_targets = b_pawns_attack_targets(bp, white_pieces);
        assert_eq!(attack_targets, white_pieces); // both pawns can attack
    }

    #[test]
    fn test_knight_moves() {
        // Test knight moves from central position (e4)
        let knights = Square::E4.to_bitboard();
        let moves = knight_moves(knights);

        // Knight on e4 should move to:
        let expected_moves =
            Square::F6.to_bitboard() | Square::D6.to_bitboard() |  // up 2, left/right 1
                Square::F2.to_bitboard() | Square::D2.to_bitboard() |  // down 2, left/right 1
                Square::G5.to_bitboard() | Square::G3.to_bitboard() |  // right 2, up/down 1
                Square::C5.to_bitboard() | Square::C3.to_bitboard();   // left 2, up/down 1

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_knight_edge_cases() {
        // Test corner cases to ensure no wrapping occurs

        // Knight on a1
        let moves_a1 = knight_moves(Square::A1.to_bitboard());
        // Should only be able to move to b3 and c2
        assert_eq!(moves_a1, Square::B3.to_bitboard() | Square::C2.to_bitboard());

        // Knight on h8
        let moves_h8 = knight_moves(Square::H8.to_bitboard());
        // Should only be able to move to f7 and g6
        assert_eq!(moves_h8, Square::F7.to_bitboard() | Square::G6.to_bitboard());
    }

    #[test]
    fn test_knight_legal_moves_and_attacks() {
        // Knight on e4
        let knight = Square::E4.to_bitboard();

        // Friendly pieces on f6 and g5
        let friendly_pieces = Square::F6.to_bitboard() | Square::G5.to_bitboard();

        let legal_moves = knight_legal_moves(knight, friendly_pieces);

        // Legal moves should exclude f6 and g5
        let expected_legal = knight_moves(knight) & !friendly_pieces;
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_moves_blank_board() {
        // Test bishop moves from central position (d4)
        let bishops = Square::D4.to_bitboard();
        let moves = bishop_moves(bishops, 0, 0); // Use empty board instead of full board

        // Bishop on d4 should move to all squares along the diagonals:
        let expected_moves =
            // Northeast diagonal
            (Square::E5.to_bitboard() | Square::F6.to_bitboard() |
                Square::G7.to_bitboard() | Square::H8.to_bitboard()) |
                // Southeast diagonal
                (Square::E3.to_bitboard() | Square::F2.to_bitboard() |
                    Square::G1.to_bitboard()) |
                // Southwest diagonal
                (Square::C3.to_bitboard() | Square::B2.to_bitboard() |
                    Square::A1.to_bitboard()) |
                // Northwest diagonal
                (Square::C5.to_bitboard() | Square::B6.to_bitboard() |
                    Square::A7.to_bitboard());

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_bishop_edge_cases_blank_board() {
        // Test edge cases for bishops on the board edges

        // Bishop on a3
        let moves_a3 = bishop_moves(Square::A3.to_bitboard(), 0, 0);
        // Should be able to move along two diagonals:
        // Northeast: b4, c5, d6, e7, f8
        // Southeast: b2, c1
        let expected_moves_a3 =
            (Square::B4.to_bitboard() | Square::C5.to_bitboard() |
                Square::D6.to_bitboard() | Square::E7.to_bitboard() |
                Square::F8.to_bitboard()) |  // Northeast diagonal
                (Square::B2.to_bitboard() | Square::C1.to_bitboard());   // Southeast diagonal
        assert_eq!(moves_a3, expected_moves_a3);

        // Bishop on h6
        let moves_h6 = bishop_moves(Square::H6.to_bitboard(), 0, 0);
        // Should be able to move along two diagonals:
        // Southwest: g5, f4, e3, d2, c1
        // Northwest: g7, f8
        let expected_moves_h6 =
            (Square::G5.to_bitboard() | Square::F4.to_bitboard() |
                Square::E3.to_bitboard() | Square::D2.to_bitboard() |
                Square::C1.to_bitboard()) |  // Southwest diagonal
                (Square::G7.to_bitboard() | Square::F8.to_bitboard());   // Northwest diagonal
        assert_eq!(moves_h6, expected_moves_h6);
    }

    #[test]
    fn test_bishop_legal_moves_with_friendlies() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces on e5 and c5 and b2 and f2
        let friendly_pieces = Square::E5.to_bitboard() | Square::C5.to_bitboard() | Square::B2.to_bitboard() | Square::F2.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, 0);

        let expected_legal = Square::C3.to_bitboard() | Square::E3.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_enemies() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces on e5 and c5 and b2 and f2
        let friendly_pieces = Square::E5.to_bitboard() | Square::C5.to_bitboard() | Square::B2.to_bitboard() | Square::F2.to_bitboard();
        let enemy_pieces = Square::C3.to_bitboard() | Square::E3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        let expected_legal = Square::C3.to_bitboard() | Square::E3.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_friendlies_going_northwest() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::E5.to_bitboard() | Square::E3.to_bitboard() | Square::C3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, 0);

        let expected_legal = Square::C5.to_bitboard() | Square::B6.to_bitboard() | Square::A7.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_friendlies_going_northeast() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E3.to_bitboard() | Square::C3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, 0);

        let expected_legal = Square::E5.to_bitboard() | Square::F6.to_bitboard() | Square::G7.to_bitboard() | Square::H8.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_friendlies_going_southwest() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E5.to_bitboard() | Square::E3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, 0);

        let expected_legal = Square::C3.to_bitboard() | Square::B2.to_bitboard() | Square::A1.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_friendlies_going_southeast() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E5.to_bitboard() | Square::C3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, 0);

        let expected_legal = Square::E3.to_bitboard() | Square::F2.to_bitboard() | Square::G1.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_enemies_going_northwest() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::E5.to_bitboard() | Square::E3.to_bitboard() | Square::C3.to_bitboard();
        let enemy_pieces = Square::B6.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        let expected_legal = Square::C5.to_bitboard() | Square::B6.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_enemies_going_northeast() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E3.to_bitboard() | Square::C3.to_bitboard();
        let enemy_pieces = Square::F6.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        let expected_legal = Square::E5.to_bitboard() | Square::F6.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_enemies_going_southwest() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E5.to_bitboard() | Square::E3.to_bitboard();
        let enemy_pieces = Square::B2.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        let expected_legal = Square::C3.to_bitboard() | Square::B2.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_bishop_legal_moves_with_enemies_going_southeast() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces blocking other directions
        let friendly_pieces = Square::C5.to_bitboard() | Square::E5.to_bitboard() | Square::C3.to_bitboard();
        let enemy_pieces = Square::F2.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        let expected_legal = Square::E3.to_bitboard() | Square::F2.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_moves() {
        // Test rook moves from central position (d4)
        let rooks = Square::D4.to_bitboard();
        let moves = rook_moves(rooks, 0, 0);  // Use empty board

        // Rook on d4 should move to:
        let expected_moves =
            Square::D1.to_bitboard() | Square::D2.to_bitboard() | Square::D3.to_bitboard() |
                Square::D5.to_bitboard() | Square::D6.to_bitboard() | Square::D7.to_bitboard() | Square::D8.to_bitboard() |
                Square::A4.to_bitboard() | Square::B4.to_bitboard() |
                Square::C4.to_bitboard() | Square::E4.to_bitboard() | Square::F4.to_bitboard() | Square::G4.to_bitboard() | Square::H4.to_bitboard();

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_edge_cases() {
        // Test edge cases for rooks on central squares near the edges

        // Rook on a3
        let moves_a3 = rook_moves(Square::A3.to_bitboard(), 0, 0);
        // Should be able to move to all squares in the same rank and file
        assert_eq!(moves_a3, Square::A1.to_bitboard() | Square::A2.to_bitboard() | Square::A4.to_bitboard() |
            Square::A5.to_bitboard() | Square::A6.to_bitboard() | Square::A7.to_bitboard() | Square::A8.to_bitboard() |
            Square::B3.to_bitboard() | Square::C3.to_bitboard() |
            Square::D3.to_bitboard() | Square::E3.to_bitboard() | Square::F3.to_bitboard() | Square::G3.to_bitboard() | Square::H3.to_bitboard());

        // Rook on h6
        let moves_h6 = rook_moves(Square::H6.to_bitboard(), 0, 0);
        // Should be able to move to all squares in the same rank and file
        assert_eq!(moves_h6, Square::H1.to_bitboard() | Square::H2.to_bitboard() | Square::H3.to_bitboard() | Square::H4.to_bitboard() |
            Square::H5.to_bitboard() | Square::H7.to_bitboard() | Square::H8.to_bitboard() |
            Square::A6.to_bitboard() | Square::B6.to_bitboard() |
            Square::C6.to_bitboard() | Square::D6.to_bitboard() | Square::E6.to_bitboard() | Square::F6.to_bitboard() | Square::G6.to_bitboard());
    }

    #[test]
    fn test_rook_legal_moves_with_friendlies_going_left() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4 and c4
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard() | Square::D3.to_bitboard();

        let legal_moves = rook_legal_moves(rook, friendly_pieces, 0);

        let expected_legal = Square::C4.to_bitboard() | Square::B4.to_bitboard() | Square::A4.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_friendlies_going_right() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4 and c4
        let friendly_pieces = Square::C4.to_bitboard() | Square::D3.to_bitboard() | Square::D5.to_bitboard();

        let legal_moves = rook_legal_moves(rook, friendly_pieces, 0);

        let expected_legal = Square::E4.to_bitboard() | Square::F4.to_bitboard() | Square::G4.to_bitboard() | Square::H4.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_friendlies_going_up() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4 and c4
        let friendly_pieces = Square::C4.to_bitboard() | Square::D3.to_bitboard() | Square::E4.to_bitboard();

        let legal_moves = rook_legal_moves(rook, friendly_pieces, 0);

        let expected_legal = Square::D5.to_bitboard() | Square::D6.to_bitboard() | Square::D7.to_bitboard() | Square::D8.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_friendlies_going_down() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4 and c4
        let friendly_pieces = Square::C4.to_bitboard() | Square::D5.to_bitboard() | Square::E4.to_bitboard();

        let legal_moves = rook_legal_moves(rook, friendly_pieces, 0);

        let expected_legal = Square::D3.to_bitboard() | Square::D2.to_bitboard() | Square::D1.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_enemies_going_left() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4 and d3
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard() | Square::D3.to_bitboard();
        let enemy_pieces = Square::B4.to_bitboard();
        let legal_moves = rook_legal_moves(rook, friendly_pieces, enemy_pieces);

        let expected_legal = Square::C4.to_bitboard() | Square::B4.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_enemies_going_right() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and c4 and d3
        let friendly_pieces = Square::D5.to_bitboard() | Square::C4.to_bitboard() | Square::D3.to_bitboard();
        let enemy_pieces = Square::F4.to_bitboard();
        let legal_moves = rook_legal_moves(rook, friendly_pieces, enemy_pieces);

        let expected_legal = Square::E4.to_bitboard() | Square::F4.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_enemies_going_up() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and c4 and d3
        let friendly_pieces = Square::E4.to_bitboard() | Square::C4.to_bitboard() | Square::D3.to_bitboard();
        let enemy_pieces = Square::D6.to_bitboard();
        let legal_moves = rook_legal_moves(rook, friendly_pieces, enemy_pieces);

        let expected_legal = Square::D5.to_bitboard() | Square::D6.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_rook_legal_moves_with_enemies_going_down() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and c4 and d3
        let friendly_pieces = Square::D5.to_bitboard() | Square::C4.to_bitboard() | Square::E4.to_bitboard();
        let enemy_pieces = Square::D2.to_bitboard();
        let legal_moves = rook_legal_moves(rook, friendly_pieces, enemy_pieces);

        let expected_legal = Square::D3.to_bitboard() | Square::D2.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_queen_moves() {
        // Test queen moves from central position (d4)
        let queens = Square::D4.to_bitboard();
        let moves = queen_moves(queens, 0, 0);  // Use empty board

        // Queen on d4 should move to:
        let expected_moves =
            // Rook-like moves
            // Vertical moves (up and down from d4)
            (Square::D5.to_bitboard() | Square::D6.to_bitboard() |
                Square::D7.to_bitboard() | Square::D8.to_bitboard() |
                Square::D3.to_bitboard() | Square::D2.to_bitboard() |
                Square::D1.to_bitboard()) |
                // Horizontal moves (left and right from d4)
                (Square::E4.to_bitboard() | Square::F4.to_bitboard() |
                    Square::G4.to_bitboard() | Square::H4.to_bitboard() |
                    Square::C4.to_bitboard() | Square::B4.to_bitboard() |
                    Square::A4.to_bitboard()) |
                // Bishop-like moves
                // Northeast diagonal
                (Square::E5.to_bitboard() | Square::F6.to_bitboard() |
                    Square::G7.to_bitboard() | Square::H8.to_bitboard()) |
                // Southeast diagonal
                (Square::E3.to_bitboard() | Square::F2.to_bitboard() |
                    Square::G1.to_bitboard()) |
                // Southwest diagonal
                (Square::C3.to_bitboard() | Square::B2.to_bitboard() |
                    Square::A1.to_bitboard()) |
                // Northwest diagonal
                (Square::C5.to_bitboard() | Square::B6.to_bitboard() |
                    Square::A7.to_bitboard());

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_queen_edge_cases() {
        // Test edge cases for queens on the board edges

        // Queen on a3
        let moves_a3 = queen_moves(Square::A3.to_bitboard(), 0, 0);
        // Should be able to move:
        // Vertically: a4-a8, a2-a1
        // Horizontally: b3-h3
        // Diagonally: b4-h8, b2-h1
        let expected_moves_a3 =
            // Vertical moves
            (Square::A4.to_bitboard() | Square::A5.to_bitboard() |
                Square::A6.to_bitboard() | Square::A7.to_bitboard() |
                Square::A8.to_bitboard() | Square::A2.to_bitboard() |
                Square::A1.to_bitboard()) |
                // Horizontal moves
                (Square::B3.to_bitboard() | Square::C3.to_bitboard() |
                    Square::D3.to_bitboard() | Square::E3.to_bitboard() |
                    Square::F3.to_bitboard() | Square::G3.to_bitboard() |
                    Square::H3.to_bitboard()) |
                // Northeast diagonal
                (Square::B4.to_bitboard() | Square::C5.to_bitboard() |
                    Square::D6.to_bitboard() | Square::E7.to_bitboard() |
                    Square::F8.to_bitboard()) |
                // Southeast diagonal
                (Square::B2.to_bitboard() | Square::C1.to_bitboard());
        assert_eq!(moves_a3, expected_moves_a3);

        // Queen on h6
        let moves_h6 = queen_moves(Square::H6.to_bitboard(), 0, 0);
        // Should be able to move:
        // Vertically: h7-h8, h5-h1
        // Horizontally: a6-g6
        // Diagonally: g7-f8, g5-e3
        let expected_moves_h6 =
            // Vertical moves
            (Square::H7.to_bitboard() | Square::H8.to_bitboard() |
                Square::H5.to_bitboard() | Square::H4.to_bitboard() |
                Square::H3.to_bitboard() | Square::H2.to_bitboard() |
                Square::H1.to_bitboard()) |
                // Horizontal moves
                (Square::G6.to_bitboard() | Square::F6.to_bitboard() |
                    Square::E6.to_bitboard() | Square::D6.to_bitboard() |
                    Square::C6.to_bitboard() | Square::B6.to_bitboard() |
                    Square::A6.to_bitboard()) |
                // Northwest diagonal
                (Square::G7.to_bitboard() | Square::F8.to_bitboard()) |
                // Southwest diagonal
                (Square::G5.to_bitboard() | Square::F4.to_bitboard() |
                    Square::E3.to_bitboard() | Square::D2.to_bitboard() |
                    Square::C1.to_bitboard());
        assert_eq!(moves_h6, expected_moves_h6);
    }

    #[test]
    fn test_queen_legal_moves() {
        // Queen on d4
        let queen = Square::D4.to_bitboard();

        // Friendly pieces on d5, e4, and e5
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard() | Square::E5.to_bitboard();

        // Enemy pieces on d3, c4, and e3
        let enemy_pieces = Square::D3.to_bitboard() | Square::C4.to_bitboard() | Square::E3.to_bitboard();

        let legal_moves = queen_legal_moves(queen, friendly_pieces, enemy_pieces);

        // Legal moves should exclude squares with friendly pieces but include empty squares and captures
        let expected_legal = queen_moves(queen, friendly_pieces, enemy_pieces);
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_king_moves() {
        // Test king moves from central position (d4)
        let kings = Square::D4.to_bitboard();
        let moves = king_moves(kings);

        // King on d4 should move to:
        let expected_moves =
            Square::D5.to_bitboard() | Square::D3.to_bitboard() |  // up 1 and down 1
                Square::E4.to_bitboard() | Square::C4.to_bitboard() |  // right 1 and left 1
                Square::E5.to_bitboard() | Square::C5.to_bitboard() |  // up 1, right/left 1
                Square::E3.to_bitboard() | Square::C3.to_bitboard();   // down 1, right/left 1

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_king_edge_cases() {
        // Test edge cases for kings on central squares near the edges

        // King on a3
        let moves_a3 = king_moves(Square::A3.to_bitboard());
        // Should be able to move to a4, a2, b4, b3, b2
        assert_eq!(moves_a3, Square::A4.to_bitboard() | Square::A2.to_bitboard() |
            Square::B4.to_bitboard() | Square::B3.to_bitboard() |
            Square::B2.to_bitboard());

        // King on h6
        let moves_h6 = king_moves(Square::H6.to_bitboard());
        // Should be able to move to h7, h5, g7, g6, g5
        assert_eq!(moves_h6, Square::H7.to_bitboard() | Square::H5.to_bitboard() |
            Square::G7.to_bitboard() | Square::G6.to_bitboard() |
            Square::G5.to_bitboard());
    }

    #[test]
    fn test_king_legal_moves() {
        // King on d4
        let king = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard();

        let legal_moves = king_legal_moves(king, friendly_pieces);

        // Legal moves should exclude d5 and e4
        let expected_legal = Square::D3.to_bitboard() | Square::C3.to_bitboard() |
            Square::C4.to_bitboard() | Square::C5.to_bitboard() | Square::E3.to_bitboard() |
            Square::E5.to_bitboard();
        assert_eq!(legal_moves, expected_legal);
    }

    #[test]
    fn test_hello_world() {
        println!("hello world");
    }
}
