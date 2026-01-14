use alumy::log::log_init::{LogConfig, logger_init};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
fn test_log_config_display() {
    let log_dir = "test_logs_display";
    let log_file = "test_logs_display/test_display.log";
    
    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }

    let mut config = LogConfig::new(
        Some("test_display".to_string()),
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
    config.ansi = Some(false);

    logger_init(&config).expect("Failed to initialize logger");

    let handle = thread::Builder::new()
        .name("test-thread".to_string())
        .spawn(|| {
            tracing::info!("Message from test-thread");
        })
        .unwrap();
    handle.join().unwrap();
    
    thread::sleep(Duration::from_millis(200));

    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    
    assert!(content.contains("INFO"), "Display level failed");
    assert!(content.contains("log_config_display"), "Display target failed");
    assert!(content.contains("["), "Uptime format failed");
    assert!(content.contains("test-thread"), "Display thread name failed");
    assert!(content.contains("ThreadId"), "Display thread ID failed");

    if Path::new(log_dir).exists() {
        let _ = fs::remove_dir_all(log_dir);
    }
}
