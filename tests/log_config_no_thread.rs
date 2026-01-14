use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_config_no_thread() {
    let log_dir = "test_logs_no_thread";
    let log_file = "test_logs_no_thread/test_no_thread.log";
    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }
    let mut config = LogConfig::new(Some("test_no_thread".to_string()), Some(log_file.to_string()), Some("info".to_string()), Some("1M".to_string()), Some(2));
    config.display_thread_name = Some(false);
    config.display_thread_id = Some(false);
    logger_init(&config).expect("Failed to initialize logger");
    let handle = thread::Builder::new().name("hidden-thread".to_string()).spawn(|| { tracing::info!("Message from anonymous thread"); }).unwrap();
    handle.join().unwrap();
    thread::sleep(Duration::from_millis(200));
    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    println!("Log content: {}", content);
    assert!(!content.contains("hidden-thread"), "Should not contain thread name in prefix");
    assert!(!content.contains("ThreadId"), "Should not contain thread ID");
    assert!(content.contains("Message from anonymous thread"), "Should contain message");
    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }
}
