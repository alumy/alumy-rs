use std::fs;
use std::path::Path;
use std::process::Command;

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

pub fn run_isolated(case: &str) -> bool {
    match std::env::var("ALUMY_LINUX_TEST_CASE") {
        Ok(selected) if selected == case => true,
        Ok(_) => false,
        Err(_) => {
            let status = Command::new(std::env::current_exe().expect("test executable path"))
                .arg("--nocapture")
                .env("ALUMY_LINUX_TEST_CASE", case)
                .status()
                .expect("failed to start isolated Linux test");
            assert!(status.success(), "isolated Linux test failed: {case}");
            false
        }
    }
}
