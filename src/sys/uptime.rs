/// Returns the system uptime in seconds.
/// 
/// On Unix-like systems (Linux, macOS, etc.), it derives from `uptime_duration`.
/// On Windows, it uses `GetTickCount64`.
pub fn uptime() -> u64 {
    uptime_duration().as_secs()
}

/// Returns the system uptime as a `Duration`.
/// 
/// On Unix-like systems (Linux, macOS, etc.), it uses `libc::clock_gettime` with `CLOCK_MONOTONIC`.
/// On Windows, it uses `GetTickCount64`.
pub fn uptime_duration() -> std::time::Duration {
    #[cfg(unix)]
    {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) } == 0 {
            std::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
        } else {
            std::time::Duration::from_secs(0)
        }
    }

    #[cfg(target_os = "windows")]
    {
        std::time::Duration::from_millis(unsafe { GetTickCount64() })
    }

    #[cfg(not(any(unix, target_os = "windows")))]
    {
        std::time::Duration::from_secs(0)
    }
}

#[cfg(target_os = "windows")]
extern "system" {
    fn GetTickCount64() -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime() {
        let u = uptime();
        // Assume system has been up for at least 1 second
        assert!(u > 0);
    }

    #[test]
    fn test_uptime_duration() {
        let d = uptime_duration();
        assert!(d.as_millis() > 0);
    }
}
