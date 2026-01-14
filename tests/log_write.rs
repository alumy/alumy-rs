use alumy::log::LogConfig;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_file_writing() {
    let log_dir = "test_logs_write_basic";
    let log_file = "test_logs_write_basic/test_write.log";
    
    // Clean up before test
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let config = LogConfig::new("test_write", "info")
        .with_file(log_file, "1M", 2);

    // Initialize logger
    let init_result = config.init();
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
