use std::fs;
use std::path::Path;

pub struct CleanupGuard(pub &'static str);

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        if Path::new(self.0).exists() {
            let _ = fs::remove_dir_all(self.0);
        }
    }
}

pub fn setup_log_dir(dir: &str) {
    if Path::new(dir).exists() {
        let _ = fs::remove_dir_all(dir);
    }
}
