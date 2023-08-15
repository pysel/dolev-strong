use std::time::{Duration, Instant};

const PROTOCOL_DELTA: Duration = Duration::from_secs(2); // the bound on message delays

pub fn wait_delta() {
    let start: Instant = Instant::now();

    loop {
        if start.elapsed() > PROTOCOL_DELTA {
            break;
        }
    }
}