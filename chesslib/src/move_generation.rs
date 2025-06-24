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

// Queen moves combine bishop and rook moves
pub fn queen_moves(queens: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    bishop_moves(queens, friendly_pieces, enemy_pieces) | rook_moves(queens, friendly_pieces, enemy_pieces)
}

// Legal moves are already computed in queen_moves
pub fn queen_legal_moves(queens: u64, friendly_pieces: u64, enemy_pieces: u64) -> u64 {
    queen_moves(queens, friendly_pieces, enemy_pieces)
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

