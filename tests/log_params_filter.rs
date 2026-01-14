mod common;
use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::time::Duration;
use std::thread;

#[test]
fn test_log_params_filter() {
    let log_dir = "test_logs_filter";
    let log_file = "test_logs_filter/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let mut config = LogConfig::new(
        Some("test_filter".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1M".to_string()),
        Some(2),
    );
    config.filter = Some("info,target_debug=debug".to_string());
    config.display_level = Some(true);

    logger_init(&config).expect("Failed to initialize logger");

    tracing::debug!(target: "target_debug", "Debug message should appear");
    tracing::debug!("Debug message should NOT appear");
    tracing::info!("Info message should appear");

    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    
    assert!(content.contains("DEBUG"), "Debug level missing");
    assert!(content.contains("Debug message should appear"), "Debug message missing");
    assert!(content.contains("Info message should appear"), "Info message missing");
    assert!(!content.contains("Debug message should NOT appear"), "Filtered debug message present");
}
