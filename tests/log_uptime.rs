use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_uptime_format() {
    let log_dir = "test_logs_uptime";
    let log_file = "test_logs_uptime/test_uptime.log";
    
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let mut config = LogConfig::new(
        Some("test_uptime".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1M".to_string()),
        Some(2),
    );
    config.time_format = Some("uptime".to_string());

    logger_init(&config).expect("Failed to initialize logger");

    tracing::info!("Testing uptime format");
    
    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    // Uptime format looks like "[   0.123456]"
    assert!(content.contains("["), "Uptime format missing opening bracket");
    assert!(content.contains("."), "Uptime format missing dot");
    assert!(content.contains("Testing uptime format"), "Log message missing");

    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
