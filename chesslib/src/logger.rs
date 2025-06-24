use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use once_cell::sync::OnceCell;

static LOG_PATH: OnceCell<PathBuf> = OnceCell::new();

/// Sets the path for the log file. Must be called before any logging occurs.
pub fn set_log_path<P: Into<PathBuf>>(path: P) {
    LOG_PATH.get_or_init(|| path.into());
}

/// Gets the current log file path, or returns the default if not set
fn get_log_path() -> PathBuf {
    LOG_PATH
        .get()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("/home/dgrant/git_personal/rust/chess/engine.log"))
}

pub fn log_to_file(message: &str, append: bool) {
    let path = get_log_path();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .open(&path)
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}
