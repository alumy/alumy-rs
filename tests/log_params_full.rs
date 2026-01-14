mod common;
use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_params_full() {
    let log_dir = "test_logs_full";
    let log_file = "test_logs_full/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let mut config = LogConfig::new(
        Some("test_full".to_string()),
        Some(log_file.to_string()),
        Some("debug".to_string()),
        Some("1M".to_string()),
        Some(2),
    );
    config.display_target = Some(true);
    config.display_level = Some(true);
    config.display_time = Some(true);
    config.display_thread_name = Some(true);
    config.display_thread_id = Some(true);
    config.time_format = Some("uptime".to_string());
    config.ansi = Some(true);

    logger_init(&config).expect("Failed to initialize logger");

    let handle = thread::Builder::new()
        .name("full-feat-thread".to_string())
        .spawn(|| {
            tracing::info!("Full features message");
        })
        .unwrap();
    handle.join().unwrap();
    
    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    
    assert!(content.contains("\u{1b}"), "ANSI colors missing");
    assert!(content.contains("INFO"), "Level missing");
    assert!(content.contains("log_params_full"), "Target missing");
    assert!(content.contains("["), "Uptime bracket missing");
    assert!(content.contains("full-feat-thread"), "Thread name missing");
    assert!(content.contains("ThreadId"), "Thread ID missing");
}
