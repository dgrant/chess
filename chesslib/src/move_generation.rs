// File masks to prevent wrapping around the board edges
const NOT_A_FILE: u64 = 0xfefefefefefefefe;  // ~(0x0101010101010101)
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;  // ~(0x8080808080808080)

// Additional file masks for knight moves
const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc;  // ~(0x0303030303030303)
const NOT_GH_FILE: u64 = 0x3f3f3f3f3f3f3f3f;  // ~(0xc0c0c0c0c0c0c0c0)

// White pawn capture moves
pub fn w_pawn_east_attacks(wp: u64) -> u64 {
    (wp & NOT_H_FILE) << 9  // Must mask BEFORE shifting to prevent wrapping
}

pub fn w_pawn_west_attacks(wp: u64) -> u64 {
    (wp & NOT_A_FILE) << 7  // Must mask BEFORE shifting to prevent wrapping
}

// Black pawn capture moves
pub fn b_pawn_east_attacks(bp: u64) -> u64 {
    (bp & NOT_H_FILE) >> 7  // Must mask BEFORE shifting to prevent wrapping
}

pub fn b_pawn_west_attacks(bp: u64) -> u64 {
    (bp & NOT_A_FILE) >> 9  // Must mask BEFORE shifting to prevent wrapping
}

// Combine all pawn attacks for a side
pub fn w_pawn_attacks(wp: u64) -> u64 {
    w_pawn_east_attacks(wp) | w_pawn_west_attacks(wp)
}

pub fn b_pawn_attacks(bp: u64) -> u64 {
    b_pawn_east_attacks(bp) | b_pawn_west_attacks(bp)
}

// Get actual legal pawn captures by masking with enemy pieces
pub fn w_pawns_attack_targets(wp: u64, black_pieces: u64) -> u64 {
    w_pawn_attacks(wp) & black_pieces
}

pub fn b_pawns_attack_targets(bp: u64, white_pieces: u64) -> u64 {
    b_pawn_attacks(bp) & white_pieces
}

pub fn w_pawns_able_to_push(wpawns: u64, empty: u64) -> u64 {
    (empty >> 8) & wpawns
}

pub fn w_pawns_able_to_double_push(wpawns: u64, empty: u64) -> u64 {
    const RANK4: u64 = 0x00000000FF000000;
    let empty_rank3 = (empty & RANK4) >> 8 & empty;
    w_pawns_able_to_push(wpawns, empty_rank3)
}

pub fn b_pawns_able_to_push(bpawns: u64, empty: u64) -> u64 {
    (empty << 8) & bpawns  // Shift empty squares UP to check squares BELOW the pawns
}

pub fn b_pawns_able_to_double_push(bpawns: u64, empty: u64) -> u64 {
    const RANK5: u64 = 0x000000FF00000000;
    let empty_rank6 = (empty & RANK5) << 8 & empty;
    b_pawns_able_to_push(bpawns, empty_rank6)
}

// Knight moves - handling all 8 possible L-shaped movements
pub fn knight_moves(knights: u64) -> u64 {
    let mut moves = 0u64;

    // North movements (up 2, left/right 1)
    moves |= (knights & NOT_A_FILE) << 15;  // Up 2, left 1
    moves |= (knights & NOT_H_FILE) << 17;  // Up 2, right 1

    // South movements (down 2, left/right 1)
    moves |= (knights & NOT_A_FILE) >> 17;  // Down 2, left 1
    moves |= (knights & NOT_H_FILE) >> 15;  // Down 2, right 1

    // East movements (right 2, up/down 1)
    moves |= (knights & NOT_GH_FILE) << 10;  // Right 2, up 1
    moves |= (knights & NOT_GH_FILE) >> 6;   // Right 2, down 1

    // West movements (left 2, up/down 1)
    moves |= (knights & NOT_AB_FILE) << 6;   // Left 2, up 1
    moves |= (knights & NOT_AB_FILE) >> 10;  // Left 2, down 1

    moves
}

// Get legal knight moves by excluding squares occupied by friendly pieces
pub fn knight_legal_moves(knights: u64, friendly_pieces: u64) -> u64 {
    knight_moves(knights) & !friendly_pieces
}

// Get knight attack targets (squares with enemy pieces that can be captured)
pub fn knight_attack_targets(knights: u64, enemy_pieces: u64) -> u64 {
    knight_moves(knights) & enemy_pieces
}

// Get bishop moves - handling all 4 diagonal directions
pub fn bishop_moves(bishops: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    let mut moves = 0u64;
    let mut working_bishops = bishops;

    while working_bishops != 0 {
        let bishop_pos = working_bishops.trailing_zeros() as u8;
        working_bishops &= working_bishops - 1;  // Clear the processed bit

        // Northeast diagonal
        let mut pos = bishop_pos;
        while pos % 8 != 7 && pos < 56 { // While not on h-file and not on rank 8
            pos += 9;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // Southeast diagonal
        pos = bishop_pos;
        while pos % 8 != 7 && pos >= 8 { // While not on h-file and not on rank 1
            pos -= 7;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // Southwest diagonal
        pos = bishop_pos;
        while pos % 8 != 0 && pos >= 8 { // While not on a-file and not on rank 1
            pos -= 9;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // Northwest diagonal
        pos = bishop_pos;
        while pos % 8 != 0 && pos < 56 { // While not on a-file and not on rank 8
            pos += 7;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }
    }
    moves
}

// Legal moves are already computed in bishop_moves
pub fn bishop_legal_moves(bishops: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    bishop_moves(bishops, friendly_pieces, enemy_pieces)
}

// Get rook moves - handling all 4 orthogonal directions
pub fn rook_moves(rooks: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    let mut moves = 0u64;
    let mut working_rooks = rooks;

    while working_rooks != 0 {
        let rook_pos = working_rooks.trailing_zeros() as u8;
        working_rooks &= working_rooks - 1;  // Clear the processed bit

        // North (up)
        let mut pos = rook_pos;
        while pos < 56 { // While not on rank 8
            pos += 8;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // South (down)
        pos = rook_pos;
        while pos >= 8 { // While not on rank 1
            pos -= 8;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // East (right)
        pos = rook_pos;
        while pos % 8 != 7 { // While not on h-file
            pos += 1;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }

        // West (left)
        pos = rook_pos;
        while pos % 8 != 0 { // While not on a-file
            pos -= 1;
            let target = 1u64 << pos;
            if friendly_pieces & target != 0 {
                break; // Stop at friendly piece
            }
            moves |= target;  // Add this square as a valid move
            if enemy_pieces & target != 0 {
                break; // Stop after capturing enemy piece
            }
        }
    }
    moves
}

// Legal moves are already computed in rook_moves
pub fn rook_legal_moves(rooks: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    rook_moves(rooks, friendly_pieces, enemy_pieces)
}

// Attack targets are just the enemy pieces that can be captured
pub fn rook_attack_targets(rooks: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    rook_moves(rooks, friendly_pieces, enemy_pieces) & enemy_pieces
}

// Queen moves combine bishop and rook moves
pub fn queen_moves(queens: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    bishop_moves(queens, friendly_pieces, enemy_pieces) | rook_moves(queens, friendly_pieces, enemy_pieces)
}

// Legal moves are already computed in queen_moves
pub fn queen_legal_moves(queens: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    queen_moves(queens, friendly_pieces, enemy_pieces)
}

// Attack targets are just the enemy pieces that can be captured
pub fn queen_attack_targets(queens: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    queen_moves(queens, friendly_pieces, enemy_pieces) & enemy_pieces
}

// King moves - handling all 8 adjacent squares
pub fn king_moves(kings: u64) -> u64 {
    let mut moves = 0u64;

    // Orthogonal moves
    moves |= (kings & NOT_H_FILE) << 1;     // Right
    moves |= (kings & NOT_A_FILE) >> 1;     // Left
    moves |= kings << 8;                    // Up
    moves |= kings >> 8;                    // Down

    // Diagonal moves
    moves |= (kings & NOT_H_FILE) << 9;     // Up-right
    moves |= (kings & NOT_A_FILE) << 7;     // Up-left
    moves |= (kings & NOT_H_FILE) >> 7;     // Down-right
    moves |= (kings & NOT_A_FILE) >> 9;     // Down-left

    moves
}

// Get legal king moves by excluding squares occupied by friendly pieces
pub fn king_legal_moves(kings: u64, friendly_pieces: u64) -> u64 {
    king_moves(kings) & !friendly_pieces
}

// Get king attack targets (squares with enemy pieces that can be captured)
pub fn king_attack_targets(kings: u64, enemy_pieces: u64) -> u64 {
    king_moves(kings) & enemy_pieces
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Square;

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
    fn test_bishop_legal_moves_and_attacks() {
        // Bishop on d4
        let bishop = Square::D4.to_bitboard();

        // Friendly pieces on e5 and f3
        let friendly_pieces = Square::E5.to_bitboard() | Square::F3.to_bitboard();

        // Enemy pieces on c5 and e3
        let enemy_pieces = Square::C5.to_bitboard() | Square::E3.to_bitboard();

        let legal_moves = bishop_legal_moves(bishop, friendly_pieces, enemy_pieces);

        // Legal moves should exclude e5 and f3, but include empty squares and captures
        let expected_legal = bishop_moves(bishop, friendly_pieces, enemy_pieces);
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
    fn test_rook_legal_moves_and_attacks() {
        // Rook on d4
        let rook = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard();

        // Enemy pieces on d3 and c4
        let enemy_pieces = Square::D3.to_bitboard() | Square::C4.to_bitboard();

        let legal_moves = rook_legal_moves(rook, friendly_pieces, enemy_pieces);
        let attack_targets = rook_attack_targets(rook, friendly_pieces, enemy_pieces);

        // Legal moves should exclude d5 and e4
        let expected_legal = rook_moves(rook, friendly_pieces, enemy_pieces);
        assert_eq!(legal_moves, expected_legal);

        // Attack targets should only include d3 and c4
        assert_eq!(attack_targets, enemy_pieces);
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
    fn test_queen_legal_moves_and_attacks() {
        // Queen on d4
        let queen = Square::D4.to_bitboard();

        // Friendly pieces on d5, e4, and e5
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard() | Square::E5.to_bitboard();

        // Enemy pieces on d3, c4, and e3
        let enemy_pieces = Square::D3.to_bitboard() | Square::C4.to_bitboard() | Square::E3.to_bitboard();

        let legal_moves = queen_legal_moves(queen, friendly_pieces, enemy_pieces);
        let attack_targets = queen_attack_targets(queen, friendly_pieces, enemy_pieces);

        // Legal moves should exclude squares with friendly pieces but include empty squares and captures
        let expected_legal = queen_moves(queen, friendly_pieces, enemy_pieces);
        assert_eq!(legal_moves, expected_legal);

        // Attack targets should only include d3, c4, and e3
        assert_eq!(attack_targets, enemy_pieces);
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
    fn test_king_legal_moves_and_attacks() {
        // King on d4
        let king = Square::D4.to_bitboard();

        // Friendly pieces on d5 and e4
        let friendly_pieces = Square::D5.to_bitboard() | Square::E4.to_bitboard();

        // Enemy pieces on d3 and c4
        let enemy_pieces = Square::D3.to_bitboard() | Square::C4.to_bitboard();

        let legal_moves = king_legal_moves(king, friendly_pieces);
        let attack_targets = king_attack_targets(king, enemy_pieces);

        // Legal moves should exclude d5 and e4
        let expected_legal = king_moves(king) & !friendly_pieces;
        assert_eq!(legal_moves, expected_legal);

        // Attack targets should only include d3 and c4
        assert_eq!(attack_targets, enemy_pieces);
    }
}
