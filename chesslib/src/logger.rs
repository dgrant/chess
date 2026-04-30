use chrono::Local;
use once_cell::sync::OnceCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static LOG_PATH: OnceCell<PathBuf> = OnceCell::new();
static LOG_FILE: OnceCell<Mutex<Option<File>>> = OnceCell::new();

/// Sets the path for the log file. Must be called before any logging occurs.
pub fn set_log_path<P: Into<PathBuf>>(path: P) {
    LOG_PATH.get_or_init(|| path.into());
    // Initialize the log file mutex if not already done
    LOG_FILE.get_or_init(|| Mutex::new(None));
}

/// Gets the current log file path, or returns the default if not set
fn get_log_path() -> PathBuf {
    if let Some(base_path) = LOG_PATH.get() {
        // If a custom path is set, add date and time to it
        let datetime_str = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let path = base_path.clone();

        // Insert datetime before file extension if it exists
        if let Some(extension) = path.extension() {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));
            let filename = format!("{}_{}.{}", stem, datetime_str, extension.to_string_lossy());
            parent.join(filename)
        } else {
            // No extension, just append datetime
            let filename = format!("{}_{}", path.to_string_lossy(), datetime_str);
            PathBuf::from(filename)
        }
    } else {
        // Default path with date and time
        let datetime_str = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        PathBuf::from(format!(
            "/home/dgrant/git_personal/rust/chess/engine_{datetime_str}.log"
        ))
    }
}

/// Opens a new log file with the specified mode
fn open_log_file(truncate: bool) -> std::io::Result<File> {
    let path = get_log_path();
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(!truncate)
        .truncate(truncate)
        .open(&path)
}

/// Logs a message to the file. The file remains open for subsequent log operations.
/// If append is false, the file will be truncated and reopened.
pub fn log_to_file(message: &str, append: bool) {
    let log_file_mutex = LOG_FILE.get_or_init(|| Mutex::new(None));

    // Acquire lock in a limited scope to avoid deadlock
    let write_result = {
        let mut file_option = match log_file_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                // If the mutex is poisoned, we can still recover by using the guard
                poisoned.into_inner()
            }
        };

        // Handle file opening/reopening logic
        let file_needs_opening = file_option.is_none() || !append;

        if file_needs_opening {
            match open_log_file(!append) {
                Ok(file) => {
                    *file_option = Some(file);
                }
                Err(_) => {
                    return; // Exit early if we can't open the file
                }
            }
        }

        // Write to file if available
        if let Some(ref mut file) = *file_option {
            let write_result = writeln!(file, "{message}");
            let flush_result = file.flush();

            // Return both results so we can handle them outside the lock
            (write_result, flush_result)
        } else {
            // This shouldn't happen, but handle it gracefully
            (Err(std::io::Error::other("No file available")), Ok(()))
        }
    }; // Lock is released here

    // Handle any errors outside of the lock to avoid deadlock on panic
    if let (Err(_), _) | (_, Err(_)) = write_result {
        // Log errors could be handled here, but we avoid panicking
        // to prevent deadlocks. In a real application, you might want to
        // use a different error handling strategy.
    }
}

/// Closes the log file if it's currently open. Useful for cleanup.
pub fn close_log_file() {
    if let Some(log_file_mutex) = LOG_FILE.get() {
        let mut file_option = match log_file_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *file_option = None;
    }
}
