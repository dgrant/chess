extern crate chesslib;
use chesslib::board::{get_starting_board, print_board};

fn main() {
    println!("Chess Game Starting!");
    let board = get_starting_board();
    print_board(&board);
    println!("Chess board initialized!");
}
