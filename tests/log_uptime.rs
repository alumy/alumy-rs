use alumy::log::LogConfig;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_uptime_format() {
    let log_dir = "test_logs_uptime_basic";
    let log_file = "test_logs_uptime_basic/test_uptime.log";
    
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let config = LogConfig::new("test_uptime", "info")
        .with_file(log_file, "1M", 2)
        .with_time_format("uptime");

    config.init().expect("Failed to initialize logger");

    tracing::info!("Testing uptime format");
    
    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    // Uptime format looks like "[   0.123]"
    assert!(content.contains("["), "Uptime format missing opening bracket");
    assert!(content.contains("."), "Uptime format missing dot");
    assert!(content.contains("Testing uptime format"), "Log message missing");

    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
