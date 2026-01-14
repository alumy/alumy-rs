use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_file_writing() {
    let log_dir = "test_logs";
    let log_file = "test_logs/test_write.log";
    
    // Clean up before test
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let config = LogConfig::new(
        Some("test_write".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1M".to_string()),
        Some(2),
    );

    // Initialize logger
    let init_result = logger_init(&config);
    assert!(init_result.is_ok(), "Failed to initialize logger: {:?}", init_result.err());

    // Write a log message
    tracing::info!("Hello, file logging!");
    
    // Logging is non-blocking, so we need to wait a bit for the writer thread to process the message
    thread::sleep(Duration::from_millis(200));

    // Check if file exists
    assert!(Path::new(log_file).exists(), "Log file was not created at {}", log_file);

    // Check if content is correct
    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    assert!(content.contains("INFO"), "Log level missing in file");
    assert!(content.contains("Hello, file logging!"), "Log message missing in file");

    // Clean up after test
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
