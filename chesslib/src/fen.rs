use crate::board::Board;
use crate::board_utils::get_empty_board;
use crate::types::{Color, PieceType, Square};

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Board position (8 ranks, starting from rank 8)
        for rank in (1..=8).rev() {
            // Changed to iterate from 8 down to 1
            let mut empty_count = 0;
            for file in 0..8 {
                let file_as_str = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][file];
                let coord = format!("{file_as_str}{rank}");
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

        // En passant target square
        match self.en_passant_target {
            Some(sq) => fen.push_str(&square_to_algebraic(sq)),
            None => fen.push('-'),
        }
        fen.push(' ');

        // Halfmove clock and fullmove number
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }
}

/// Convert a Square to its algebraic name like "e3". Inverse of
/// `Square::try_from(&str)`.
fn square_to_algebraic(sq: Square) -> String {
    let idx = sq.to_bit_index();
    let file = idx % 8;
    let rank = idx / 8 + 1;
    format!("{}{}", (b'a' + file) as char, rank)
}

pub fn load_fen(fen: &str) -> Result<Board, &'static str> {
    let mut board = get_empty_board();

    // Split FEN into its components
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.is_empty() {
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
            if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                if file >= 8 {
                    return Err("Invalid FEN: rank too long");
                }
                let square = (7 - rank_idx) * 8 + file;
                let bit = 1u64 << square;

                // Translate the FEN character into a (PieceType, Color)
                // pair, then update both per-type and per-colour
                // bitboards in lock-step. Uppercase = white, lowercase
                // = black.
                let (pt, color) = match c {
                    'P' => (PieceType::Pawn, Color::White),
                    'N' => (PieceType::Knight, Color::White),
                    'B' => (PieceType::Bishop, Color::White),
                    'R' => (PieceType::Rook, Color::White),
                    'Q' => (PieceType::Queen, Color::White),
                    'K' => (PieceType::King, Color::White),
                    'p' => (PieceType::Pawn, Color::Black),
                    'n' => (PieceType::Knight, Color::Black),
                    'b' => (PieceType::Bishop, Color::Black),
                    'r' => (PieceType::Rook, Color::Black),
                    'q' => (PieceType::Queen, Color::Black),
                    'k' => (PieceType::King, Color::Black),
                    _ => return Err("Invalid FEN: invalid piece"),
                };
                board.pieces[pt.idx()] |= bit;
                board.colors[color.idx()] |= bit;
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

    // En passant target square
    if parts.len() > 3 {
        board.en_passant_target = if parts[3] == "-" {
            None
        } else {
            Some(Square::try_from(parts[3]).map_err(|_| "Invalid FEN: bad en passant square")?)
        };
    }

    // Halfmove clock
    if parts.len() > 4 {
        board.halfmove_clock = parts[4]
            .parse::<u32>()
            .map_err(|_| "Invalid FEN: bad halfmove clock")?;
    }

    // Fullmove number
    if parts.len() > 5 {
        board.fullmove_number = parts[5]
            .parse::<u32>()
            .map_err(|_| "Invalid FEN: bad fullmove number")?;
    }

    board.rebuild_piece_map();
    board.update_composite_bitboards();
    board.update_check_state();

    Ok(board)
}
