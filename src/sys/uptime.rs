pub fn uptime() -> u64 {
    let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };

    if unsafe { libc::sysinfo(&mut info) } != 0 {
        return 0;
    }

    info.uptime as u64
}

pub fn uptime_duration() -> std::time::Duration {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) } == 0 {
        std::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
    } else {
        std::time::Duration::from_secs(uptime())
    }
}