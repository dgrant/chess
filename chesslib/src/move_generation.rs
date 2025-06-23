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
        
        // Enemy pieces on c3 and d2
        let enemy_pieces = Square::C3.to_bitboard() | Square::D2.to_bitboard();
        
        let legal_moves = knight_legal_moves(knight, friendly_pieces);
        let attack_targets = knight_attack_targets(knight, enemy_pieces);
        
        // Legal moves should exclude f6 and g5
        let expected_legal = knight_moves(knight) & !friendly_pieces;
        assert_eq!(legal_moves, expected_legal);
        
        // Attack targets should only include c3 and d2
        assert_eq!(attack_targets, enemy_pieces);
    }
}
