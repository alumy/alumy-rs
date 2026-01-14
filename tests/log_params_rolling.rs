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
fn test_log_params_rolling() {
    let log_dir = "test_logs_rolling";
    let log_file = "test_logs_rolling/test.log";
    let _guard = CleanupGuard(log_dir);

    if Path::new(log_dir).exists() { let _ = fs::remove_dir_all(log_dir); }

    let config = LogConfig::new(
        Some("test_rolling".to_string()),
        Some(log_file.to_string()),
        Some("info".to_string()),
        Some("500".to_string()),
        Some(3),
    );

    logger_init(&config).expect("Failed to initialize logger");

    for i in 0..20 {
        tracing::info!("Rolling message {:02} with enough content to fill 500 bytes quickly", i);
    }

    thread::sleep(Duration::from_millis(500));

    let entries = fs::read_dir(log_dir).expect("Failed to read log directory");
    let file_count = entries.count();
    
    assert!(file_count > 1, "Should have rolled at least once");
    assert!(file_count <= 4, "Should respect max_files (3 rolled + 1 current)");
}
