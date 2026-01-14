use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_config_ansi_false() {
    let log_dir = "test_logs_ansi_false";
    let log_file = "test_logs_ansi_false/test_no_ansi.log";
    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }
    let mut config = LogConfig::new(Some("test_no_ansi".to_string()), Some(log_file.to_string()), Some("info".to_string()), Some("1M".to_string()), Some(2));
    config.ansi = Some(false);
    logger_init(&config).expect("Failed to initialize logger");
    tracing::info!("No ANSI test");
    thread::sleep(Duration::from_millis(200));
    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    assert!(!content.contains("\u{1b}"), "Should not contain ANSI escape codes");
    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }
}
