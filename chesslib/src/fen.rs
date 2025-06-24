use crate::board::Board;

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Board position (8 ranks, starting from rank 8)
        for rank in (1..=8).rev() {  // Changed to iterate from 8 down to 1
            let mut empty_count = 0;
            for file in 0..8 {
                let coord = format!("{}{}",
                    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][file],
                    rank);
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
            if rank > 1 {  // Changed condition to match new range
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(match self.side_to_move {
            crate::types::Color::White => 'w',
            crate::types::Color::Black => 'b',
        });

        // Castling availability - TODO: Implement actual castling logic
        let castling_str = "KQkq";
        fen.push(' ');
        fen.push_str(castling_str);
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
