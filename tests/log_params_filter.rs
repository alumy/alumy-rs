use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

struct CleanupGuard(&'static str);
impl Drop for CleanupGuard {
    fn drop(&mut self) {
        if Path::new(self.0).exists() {
            let _ = fs::remove_dir_all(self.0);
        }
    }
}

#[test]
fn test_log_params_filter() {
    let log_dir = "test_logs_filter";
    let log_file = "test_logs_filter/test.log";
    let _guard = CleanupGuard(log_dir);

    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }

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
    
    assert!(content.contains("DEBUG"), "Debug level missing for targeted message");
    assert!(content.contains("Debug message should appear"), "Debug message missing");
    assert!(content.contains("Info message should appear"), "Info message missing");
    assert!(!content.contains("Debug message should NOT appear"), "Filtered debug message present");
}
