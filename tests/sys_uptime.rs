use alumy::sys::uptime::{uptime, uptime_duration};

#[test]
fn test_uptime() {
    let up = uptime();

    assert!(up > 0);
}

#[test]
fn test_uptime_consistency() {
    let up1 = uptime();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let up2 = uptime();
    assert!(up2 >= up1);
}

#[test]
fn test_uptime_duration() {
    let duration = uptime_duration();
    assert!(duration.as_secs() > 0);
    
    let up = uptime();

    let diff = if duration.as_secs() > up {
        duration.as_secs() - up
    } else {
        up - duration.as_secs()
    };

    assert!(diff <= 1, "uptime_duration ({}s) and uptime ({}s) differ by more than 1s", duration.as_secs(), up);
}
