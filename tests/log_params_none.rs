mod common;
use alumy::log::LogConfig;
use std::fs;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_params_none() {
    let log_dir = "test_logs_none";
    let log_file = "test_logs_none/test.log";
    let _guard = common::CleanupGuard(log_dir);
    common::setup_log_dir(log_dir);

    let config = LogConfig::new("test_none", "info")
        .with_file(log_file, "1M", 2)
        .with_target(false)
        .with_level_display(false)
        .with_time(false)
        .with_thread_name(false)
        .with_thread_id(false)
        .with_ansi(false);

    config.init().expect("Failed to initialize logger");

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
