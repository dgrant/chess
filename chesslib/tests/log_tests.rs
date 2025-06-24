mod tests {
    use std::fs;
    use std::env::temp_dir;
    use std::path::PathBuf;
    use chesslib::log_to_file;
    use chesslib::logger::set_log_path;

    fn setup_test_log() -> PathBuf {
        let test_log = temp_dir().join("test_engine.log");
        set_log_path(&test_log);
        test_log
    }

    #[test]
    fn test_log_to_file() {
        let test_log = setup_test_log();

        // Test with append = false (should overwrite)
        log_to_file("Test message 1", false);
        let contents = fs::read_to_string(&test_log)
            .expect("Failed to read log file");
        assert_eq!(contents.trim(), "Test message 1");

        // Test with append = true (should add to file)
        log_to_file("Test message 2", true);
        let contents = fs::read_to_string(&test_log)
            .expect("Failed to read log file");
        assert!(contents.contains("Test message 1"));
        assert!(contents.contains("Test message 2"));

        // Clean up
        let _ = fs::remove_file(&test_log);
    }
}