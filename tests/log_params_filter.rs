mod common;
use alumy::log::log_init::LogConfig;
use std::fs;
use std::time::Duration;
use std::thread;

#[test]
fn test_log_params_filter() {
    let log_dir = "test_logs_filter";
    let log_file = "test_logs_filter/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let config = LogConfig::new("test_filter", "info")
        .with_file(log_file, "1M", 2)
        .with_filter("info,target_debug=debug")
        .with_level_display(true);

    config.init().expect("Failed to initialize logger");

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
