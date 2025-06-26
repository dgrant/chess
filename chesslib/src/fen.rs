use crate::board::Board;
use crate::board_utils::get_empty_board;
use crate::types::Color;

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Board position (8 ranks, starting from rank 8)
        for rank in (1..=8).rev() {
            // Changed to iterate from 8 down to 1
            let mut empty_count = 0;
            for file in 0..8 {
                let file_as_str = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][file];
                let coord = format!("{}{}", file_as_str, rank);
                let piece = self.get_piece_at_coordinate_as_fen(&coord);

                if piece == " " {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    // Convert Unicode pieces to FEN characters
                    fen.push_str(piece);
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 1 {
                // Changed condition to match new range
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        fen.push(' ');
        // Castling availability
        let mut castling_str = String::new();
        if self.white_kingside_castle_rights {
            castling_str.push('K');
        }
        if self.white_queenside_castle_rights {
            castling_str.push('Q');
        }
        if self.black_kingside_castle_rights {
            castling_str.push('k');
        }
        if self.black_queenside_castle_rights {
            castling_str.push('q');
        }
        if castling_str.is_empty() {
            castling_str.push('-');
        }
        fen.push_str(castling_str.as_str());
        fen.push(' ');

        // En passant target square - TODO: Implement actual en passant logic
        let en_passant_str = "-";
        fen.push_str(en_passant_str);
        fen.push(' ');

        // Halfmove clock and fullmove number - TODO: Implement actual game state tracking
        let halfmove_clock = 0; // Placeholder for halfmove clock (how many moves both players have made since the last pawn advance or piece capture)
        let fullmove_number = 1; // Placeholder for fullmove number (incremented after each black move)
        fen.push_str(&halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&fullmove_number.to_string());

        // Final FEN string
        fen
    }
}

pub fn load_fen(fen: &str) -> Result<Board, &'static str> {
    let mut board = get_empty_board();

    // Split FEN into its components
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 1 {
        return Err("Invalid FEN: not enough parts");
    }

    // Process piece placement
    let ranks: Vec<&str> = parts[0].split('/').collect();
    if ranks.len() != 8 {
        return Err("Invalid FEN: wrong number of ranks");
    }

    // Process each rank
    for (rank_idx, rank) in ranks.iter().enumerate() {
        let mut file = 0;
        for c in rank.chars() {
            if c.is_digit(10) {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                if file >= 8 {
                    return Err("Invalid FEN: rank too long");
                }
                let square = (7 - rank_idx) * 8 + file;
                let bit = 1u64 << square;

                match c {
                    'P' => board.white_pawns |= bit,
                    'N' => board.white_knights |= bit,
                    'B' => board.white_bishops |= bit,
                    'R' => board.white_rooks |= bit,
                    'Q' => board.white_queen |= bit,
                    'K' => board.white_king |= bit,
                    'p' => board.black_pawns |= bit,
                    'n' => board.black_knights |= bit,
                    'b' => board.black_bishops |= bit,
                    'r' => board.black_rooks |= bit,
                    'q' => board.black_queen |= bit,
                    'k' => board.black_king |= bit,
                    _ => return Err("Invalid FEN: invalid piece"),
                }
                file += 1;
            }
        }
        if file != 8 {
            return Err("Invalid FEN: rank wrong length");
        }
    }

    // Set side to move
    if parts.len() > 1 {
        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN: invalid side to move"),
        };
    }

    // Process castling rights
    if parts.len() > 2 {
        board.white_kingside_castle_rights = parts[2].contains('K');
        board.white_queenside_castle_rights = parts[2].contains('Q');
        board.black_kingside_castle_rights = parts[2].contains('k');
        board.black_queenside_castle_rights = parts[2].contains('q');
    }

    board.rebuild_piece_map();
    board.update_composite_bitboards();
    board.update_check_state();

    Ok(board)
}
