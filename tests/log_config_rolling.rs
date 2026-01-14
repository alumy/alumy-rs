use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_config_rolling() {
    let log_dir = "test_logs_rolling";
    let log_file = "test_logs_rolling/test_rolling.log";
    
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let config = LogConfig::new(
        Some("test_rolling".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1k".to_string()),
        Some(3),
    );

    logger_init(&config).expect("Failed to initialize logger");

    for i in 0..50 {
        tracing::info!("Rolling test message {:03} with extra padding to trigger limit", i);
    }

    thread::sleep(Duration::from_millis(500));

    let entries = fs::read_dir(log_dir).expect("Failed to read log directory");
    let file_count = entries.count();
    
    assert!(file_count > 1, "Rolling failed to create multiple files");
    assert!(file_count <= 4, "Max files limit failed (expected max 4 files for max_files=3)");

    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
