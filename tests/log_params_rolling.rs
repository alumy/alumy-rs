mod common;
use alumy::log::LogConfig;
use std::fs;
use std::time::Duration;
use std::thread;

#[test]
fn test_log_params_rolling() {
    let log_dir = "test_logs_rolling";
    let log_file = "test_logs_rolling/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let config = LogConfig::new("test_rolling", "info")
        .with_file(log_file, "500", 3);

    config.init().expect("Failed to initialize logger");

    for i in 0..20 {
        tracing::info!("Rolling message {:02} with enough content", i);
    }

    thread::sleep(Duration::from_millis(500));

    let entries = fs::read_dir(log_dir).expect("Failed to read log directory");
    let file_count = entries.count();
    
    assert!(file_count > 1, "Should have rolled at least once");
    assert!(file_count <= 4, "Should respect max_files");
}
