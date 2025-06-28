use chesslib::board::Board;
use chesslib::fen::load_fen;

fn main() {
    let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
    let mut board = load_fen(fen).expect("Valid FEN");

    println!("Board position:");
    println!("Side to move: {:?}", board.side_to_move);
    println!("White king in check: {}", board.white_king_in_check);
    println!("Black king in check: {}", board.black_king_in_check);

    // Get all raw moves
    let mut raw_moves = Vec::new();
    board.get_all_raw_moves_append(&mut raw_moves);
    println!("Raw moves found: {}", raw_moves.len());

    // Check which moves are legal
    let mut legal_count = 0;
    for mv in &raw_moves {
        if board.is_legal_move(mv) {
            legal_count += 1;
            println!("Legal move: {}", mv);
        } else {
            println!("Illegal move: {}", mv);
        }
    }

    println!("Legal moves: {}", legal_count);

    let best_move = board.find_best_move(2);
    println!("Best move found: {:?}", best_move);
}
