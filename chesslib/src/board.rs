static PAWN: &'static str = "p";
static ROOK: &'static str = "R";
static KNIGHT: &'static str = "N";
static BISHOP: &'static str = "B";
static QUEEN: &'static str = "Q";
static KING: &'static str = "K";
static SPACE: &'static str = " ";


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
        b_pawns: 0,
        b_knights: 0,
        b_bishops: 0,
        b_rooks: 0,
        b_queen: 0,
        b_king: 0,
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
        return PAWN;
    } else if is_bit_set(board.w_knights, bitboard_index) {
        return KNIGHT;
    } else if is_bit_set(board.w_bishops, bitboard_index) {
        return BISHOP;
    } else if is_bit_set(board.w_queen, bitboard_index) {
        return QUEEN;
    } else if is_bit_set(board.w_rooks, bitboard_index) {
        return ROOK;
    } else if is_bit_set(board.w_king, bitboard_index) {
        return KING;
    } else {
        return SPACE;
    }
}

pub fn print_board(board: &Board) {
    
}