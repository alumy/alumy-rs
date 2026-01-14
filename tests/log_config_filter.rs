use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_config_filter() {
    let log_dir = "test_logs_filter";
    let log_file = "test_logs_filter/test_filter.log";
    
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let mut config = LogConfig::new(
        Some("test_filter".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1M".to_string()),
        Some(2),
    );
    
    config.filter = Some("info,other_target=debug".to_string());

    logger_init(&config).expect("Failed to initialize logger");

    tracing::debug!(target: "other_target", "Debug from other_target");
    tracing::debug!("Debug that should be filtered");
    tracing::info!("Info message");

    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    
    assert!(content.contains("DEBUG"), "Filter target failed");
    assert!(content.contains("Debug from other_target"), "Filter message failed");
    assert!(content.contains("Info message"), "Base level failed");
    assert!(!content.contains("Debug that should be filtered"), "Filter logic failed");

    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
