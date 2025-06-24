extern crate chesslib;
use chesslib::{handle_uci_command, log_to_file};
use std::io::{self, BufRead, Write};

fn main() {
    chesslib::logger::set_log_path("/home/dgrant/git_personal/rust/chess/engine.log");
    
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Chess engine ready. Waiting for UCI commands...");
    log_to_file("======", false);
    loop {
        input.clear();
        if stdin.lock().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        log_to_file(&format!("Received: {}", input.trim()), true);

        let response = handle_uci_command(&input);
        log_to_file(&format!("Responded: {}", response), true);

        if input.trim() == "quit" {
            break; // Exit on "quit" command
        }

        println!("{}", response);
        io::stdout().flush().unwrap();
    }

    println!("Exiting chess engine.");
}
