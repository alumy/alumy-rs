#![no_std]
//! Minimal FreeRTOS task integration using no_std-compatible Alumy APIs.

use alumy::freertos::fs;

/// Static configuration prepared before creating a FreeRTOS task.
#[derive(Debug, Eq, PartialEq)]
pub struct TaskConfig {
    /// Task stack allocation in bytes.
    pub stack_bytes: u64,
}

/// Builds the application task configuration.
pub fn task_config() -> Option<TaskConfig> {
    Some(TaskConfig {
        stack_bytes: fs::filesize::parse_size("8KiB")?,
    })
}

/// Portable body called by the platform-specific FreeRTOS task entry point.
pub fn task_entry() -> Option<u64> {
    Some(task_config()?.stack_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_freertos_task() {
        assert_eq!(task_entry(), Some(8192));
    }
}
