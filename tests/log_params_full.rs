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

    let config = LogConfig::new("test_full", "debug")
        .with_file(log_file, "1M", 2)
        .with_target(true)
        .with_level_display(true)
        .with_time(true)
        .with_thread_name(true)
        .with_thread_id(true)
        .with_time_format("uptime")
        .with_ansi(true);

    config.init().expect("Failed to initialize logger");

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
