use chesslib::board_utils::get_starting_board;
use chesslib::Square;
use chesslib::types::Piece;

#[test]
fn test_get_starting_board() {
    let board = get_starting_board();
    assert_eq!(board.white_pawns, 0x000000000000FF00);
    assert_eq!(board.white_knights, 0x0000000000000042);
    assert_eq!(board.white_bishops, 0x0000000000000024);
    assert_eq!(board.white_rooks, 0x0000000000000081);
    assert_eq!(board.white_queen, Square::D1.to_bitboard());
    assert_eq!(board.white_king, Square::E1.to_bitboard());

    assert_eq!(board.black_pawns, 0x00FF000000000000);
    assert_eq!(board.black_knights, 0x4200000000000000);
    assert_eq!(board.black_bishops, 0x2400000000000000);
    assert_eq!(board.black_rooks, 0x8100000000000000);
    assert_eq!(board.black_queen, Square::D8.to_bitboard());
    assert_eq!(board.black_king, Square::E8.to_bitboard());

    assert!(!board.white_king_in_check);
    assert!(!board.black_king_in_check);

    // White pieces:
    assert_eq!(board.get_piece_at_square_fast(Square::A1.to_bit_index()), Some(Piece::WhiteRook));
    assert_eq!(board.get_piece_at_square_fast(Square::B1.to_bit_index()), Some(Piece::WhiteKnight));
    assert_eq!(board.get_piece_at_square_fast(Square::C1.to_bit_index()), Some(Piece::WhiteBishop));
    assert_eq!(board.get_piece_at_square_fast(Square::D1.to_bit_index()), Some(Piece::WhiteQueen));
    assert_eq!(board.get_piece_at_square_fast(Square::E1.to_bit_index()), Some(Piece::WhiteKing));
    assert_eq!(board.get_piece_at_square_fast(Square::F1.to_bit_index()), Some(Piece::WhiteBishop));
    assert_eq!(board.get_piece_at_square_fast(Square::G1.to_bit_index()), Some(Piece::WhiteKnight));
    assert_eq!(board.get_piece_at_square_fast(Square::H1.to_bit_index()),  Some(Piece::WhiteRook));
    assert_eq!(board.get_piece_at_square_fast(Square::A2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::B2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::C2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::D2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::E2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::F2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::G2.to_bit_index()), Some(Piece::WhitePawn));
    assert_eq!(board.get_piece_at_square_fast(Square::H2.to_bit_index()), Some(Piece::WhitePawn));

    // Black pieces:
    assert_eq!(board.get_piece_at_square_fast(Square::A8.to_bit_index()), Some(Piece::BlackRook));
    assert_eq!(board.get_piece_at_square_fast(Square::B8.to_bit_index()), Some(Piece::BlackKnight));
    assert_eq!(board.get_piece_at_square_fast(Square::C8.to_bit_index()), Some(Piece::BlackBishop));
    assert_eq!(board.get_piece_at_square_fast(Square::D8.to_bit_index()), Some(Piece::BlackQueen));
    assert_eq!(board.get_piece_at_square_fast(Square::E8.to_bit_index()), Some(Piece::BlackKing));
    assert_eq!(board.get_piece_at_square_fast(Square::F8.to_bit_index()), Some(Piece::BlackBishop));
    assert_eq!(board.get_piece_at_square_fast(Square::G8.to_bit_index()), Some(Piece::BlackKnight));
    assert_eq!(board.get_piece_at_square_fast(Square::H8.to_bit_index()),  Some(Piece::BlackRook));
    assert_eq!(board.get_piece_at_square_fast(Square::A7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::B7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::C7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::D7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::E7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::F7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::G7.to_bit_index()), Some(Piece::BlackPawn));
    assert_eq!(board.get_piece_at_square_fast(Square::H7.to_bit_index()), Some(Piece::BlackPawn));

    // Castling rights
    assert!(board.white_kingside_castle_rights);
    assert!(board.white_queenside_castle_rights);
    assert!(board.black_kingside_castle_rights);
    assert!(board.black_queenside_castle_rights);
    // En passant target

    assert_eq!(board.en_passant_target, None);
    // Move history
    assert!(board.move_history.is_empty());
    // Piece map
    assert_eq!(board.piece_map.len(), 64);
    // for square in Square::all() {
    //     assert_eq!(board.piece_map[square.to_bit_index()], Some(board.get_piece_at_square_fast(square.to_bit_index())));
    // }
    // Composite bitboards
    assert_eq!(board.any_white, board.white_pawns | board.white_knights | board.white_bishops | board.white_rooks | board.white_queen | board.white_king);
    assert_eq!(board.any_black, board.black_pawns | board.black_knights | board.black_bishops | board.black_rooks | board.black_queen | board.black_king);
    assert_eq!(board.empty, !(board.any_white | board.any_black));
    // Ensure the board is valid
    // assert!(board.is_valid());
    // // Ensure the side to move is white
    // assert_eq!(board.side_to_move, chesslib::Color::White);
    // // Ensure the board is not poisoned
    // assert!(!board.is_poisoned());
    // // Ensure the board is not in check
    // assert!(!board.is_in_check(chesslib::Color::White));
    // assert!(!board.is_in_check(chesslib::Color::Black));
    // // Ensure the board is not in checkmate
    // assert!(!board.is_checkmate(chesslib::Color::White));
    // assert!(!board.is_checkmate(chesslib::Color::Black));
    // // Ensure the board is not in stalemate
    // assert!(!board.is_stalemate(chesslib::Color::White));
    // assert!(!board.is_stalemate(chesslib::Color::Black));


}