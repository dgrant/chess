extern crate chesslib;
use chesslib::board::{get_starting_board, print_board};
use chesslib::handle_uci_command;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/dgrant/git_personal/rust/chess/engine.log")
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

fn flush_log_on_exit() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/dgrant/git_personal/rust/chess/engine.log")
        .unwrap();
    file.flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Chess engine ready. Waiting for UCI commands...");

    loop {
        input.clear();
        if stdin.lock().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        log_to_file(&format!("Received: {}", input.trim()));

        let response = handle_uci_command(&input);
        log_to_file(&format!("Responded: {}", response));

        if input.trim() == "quit" {
            flush_log_on_exit(); // Ensure log file is flushed before exiting
            break; // Exit on "quit" command
        }

        println!("{}", response);
        io::stdout().flush().unwrap();
    }

    println!("Exiting chess engine.");
}
