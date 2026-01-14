mod common;
use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_params_none() {
    let log_dir = "test_logs_none";
    let log_file = "test_logs_none/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let mut config = LogConfig::new(
        Some("test_none".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("1M".to_string()),
        Some(2),
    );
    config.display_target = Some(false);
    config.display_level = Some(false);
    config.display_time = Some(false);
    config.display_thread_name = Some(false);
    config.display_thread_id = Some(false);
    config.ansi = Some(false);

    logger_init(&config).expect("Failed to initialize logger");

    let handle = thread::Builder::new()
        .name("hidden-thread".to_string())
        .spawn(|| {
            tracing::info!("Minimal message content");
        })
        .unwrap();
    handle.join().unwrap();
    
    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    
    assert!(!content.contains("\u{1b}"), "ANSI colors should be absent");
    assert!(!content.contains("INFO"), "Level should be absent");
    assert!(!content.contains("log_params_none"), "Target should be absent");
    assert!(!content.contains("hidden-thread"), "Thread name should be absent");
    assert!(!content.contains("ThreadId"), "Thread ID should be absent");
    assert!(content.contains("Minimal message content"), "Message content missing");
}
