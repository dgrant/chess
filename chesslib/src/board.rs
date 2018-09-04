pub static W_PAWN: &'static str = "♙";
pub static W_ROOK: &'static str = "♖";
pub static W_KNIGHT: &'static str = "♘";
pub static W_BISHOP: &'static str = "♗";
pub static W_QUEEN: &'static str = "♕";
pub static W_KING: &'static str = "♔";
pub static W_SPACE: &'static str = " ";

pub static B_PAWN: &'static str = "♟";
pub static B_ROOK: &'static str = "♜";
pub static B_KNIGHT: &'static str = "♞";
pub static B_BISHOP: &'static str = "♝";
pub static B_QUEEN: &'static str = "♛";
pub static B_KING: &'static str = "♚";
pub static B_SPACE: &'static str = " ";

pub static A: &'static str = "a";
pub static B: &'static str = "b";
pub static C: &'static str = "c";
pub static D: &'static str = "d";
pub static E: &'static str = "e";
pub static F: &'static str = "f";
pub static G: &'static str = "g";
pub static H: &'static str = "h";


pub struct Board {
    pub w_pawns: u64,
    pub w_knights: u64,
    pub w_bishops: u64,
    pub w_rooks: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub b_knights: u64,
    pub b_bishops: u64,
    pub b_rooks: u64,
    pub b_pawns: u64,
    pub b_queen: u64,
    pub b_king: u64,
}

pub fn get_starting_board() -> Board {
    let board = Board {
        w_pawns: (1 << (8 + 0)) + (1 << (8 + 1)) + (1 << (8 + 2)) + (1 << (8 + 3)) + (1 << (8 + 4)) + (1 << (8 + 5)) + (1 << (8 + 6)) + (1 << (8 + 7)),
        w_knights: (1 << (0 + 1)) + (1 << (0 + 6)),
        w_bishops: (1 << (0 + 2)) + (1 << (0 + 5)),
        w_rooks: (1 << (0 + 0)) + (1 << (0 + 7)),
        w_queen: (1 << (0 + 3)),
        w_king: (1 << (0 + 4)),
        b_pawns: (1 << (6*8 + 0)) + (1 << (6*8 + 1)) + (1 << (6*8 + 2)) + (1 << (6*8 + 3)) + (1 << (6*8 + 4)) + (1 << (6*8 + 5)) + (1 << (6*8 + 6)) + (1 << (6*8 + 7)),
        b_knights: (1 << (7*8 + 1)) + (1 << (7*8 + 6)),
        b_bishops: (1 << (7*8 + 2)) + (1 << (7*8 + 5)),
        b_rooks: (1 << (7*8 + 0)) + (1 << (7*8 + 7)),
        b_queen: (1 << (7*8 + 3)),
        b_king: (1 << (7*8 + 4)),
    };
    board
}

pub fn string_file_to_int(file: &str) -> u8 {
    match file {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
//        TODO(dgrant): Handle this differently
        _ => 0
    }
}

pub fn int_file_to_string(file: u8) -> &'static str {
    match file {
        0 => A,
        1 => B,
        2 => C,
        3 => D,
        4 => E,
        5 => F,
        6 => G,
        7 => H,
//        TODO(dgrant): Handle this differently
        _ => W_SPACE
    }
}

pub fn convert_coordinate_to_bitboard_index(coordinate: &str) -> u8 {
    let first_char = &coordinate[0..1];
    let file_number = string_file_to_int(first_char);
    let rank: u8 = (&coordinate[1..2]).parse().unwrap();
    return (rank - 1) * 8 + file_number;
}

pub fn is_bit_set(bitboard: u64, bit: u8) -> bool {
    (1 << bit) & bitboard != 0
}

pub fn get_piece_at_coordinate(board: &Board, coordinate: &str) -> &'static str {
    let bitboard_index = convert_coordinate_to_bitboard_index(coordinate);
    if is_bit_set(board.w_pawns, bitboard_index) {
        return W_PAWN;
    } else if is_bit_set(board.w_knights, bitboard_index) {
        return W_KNIGHT;
    } else if is_bit_set(board.w_bishops, bitboard_index) {
        return W_BISHOP;
    } else if is_bit_set(board.w_queen, bitboard_index) {
        return W_QUEEN;
    } else if is_bit_set(board.w_rooks, bitboard_index) {
        return W_ROOK;
    } else if is_bit_set(board.w_king, bitboard_index) {
        return W_KING;
    } else if is_bit_set(board.b_pawns, bitboard_index) {
        return B_PAWN;
    } else if is_bit_set(board.b_knights, bitboard_index) {
        return B_KNIGHT;
    } else if is_bit_set(board.b_bishops, bitboard_index) {
        return B_BISHOP;
    } else if is_bit_set(board.b_queen, bitboard_index) {
        return B_QUEEN;
    } else if is_bit_set(board.b_king, bitboard_index) {
        return B_KING;
    } else if is_bit_set(board.b_rooks, bitboard_index) {
        return B_ROOK;
    } else {
        return W_SPACE;
    }
}

pub fn print_board(_board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let coordinate = &format!("{}{}", int_file_to_string(file), (rank + 1).to_string());
//            print!("coordinate:{}\n", coordinate);
            let piece = get_piece_at_coordinate(_board, coordinate);
            print!("{}", piece);
        }
        print!("\n");
    }
}