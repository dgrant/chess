extern crate chesslib;
use chesslib::handle_uci_command;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

fn log_to_file(message: &str, append: bool) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true) // Ensure write mode is enabled
        .append(append) // Append if true, overwrite otherwise
        .open("/home/dgrant/git_personal/rust/chess/engine.log")
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

fn main() {
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
