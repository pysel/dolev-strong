use std::time::SystemTime;

const PROTOCOL_DELTA: i64 = 2; // the bound on message delays
pub const GENESIS_TIMESTAMP_DELTA: u64 = 5; // time allocated for communication setup / time before starting the process and starting initial round

#[derive(Debug)]
pub struct Synchrony {
    bootstrap_ts: u64,
    genesis_delta: u64,
}

pub fn new_synchrony(bootstrap_ts: u64) -> Synchrony {
    Synchrony { bootstrap_ts, 
        genesis_delta: GENESIS_TIMESTAMP_DELTA 
    }
}

impl Synchrony {
    // get_genesis_round_ts returns timestamp of the genesis round
    fn get_genesis_round_ts(&self) -> u64 {
        self.bootstrap_ts + self.genesis_delta
    }

    // rwait waits until the global beginning of round r. See SPEC.md for details.
    pub fn rwait(&self, r: i64) {
        let desired_timestamp = self.get_genesis_round_ts() + (PROTOCOL_DELTA * r) as u64;
        if get_current_timestamp() >= desired_timestamp {
            panic!("waiting for round that already started")
        }

        loop {
            if get_current_timestamp() >= desired_timestamp {
                break;
            }
        }
    }
}

fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            n.as_secs()
        }
        Err(_) => {
            panic!("failed to get system's time")
        }
    }
}