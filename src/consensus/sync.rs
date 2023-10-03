use std::time::SystemTime;

const PROTOCOL_DELTA: i64 = 2; // the bound on message delays

// time allocated for communication setup / time before starting the process and starting initial stage
// it takes 15 seconds because it is containerized
pub const GENESIS_TIMESTAMP_DELTA: u64 = 10; 

#[derive(Debug)]
pub struct Synchrony {
    current_stage: i64,
    bootstrap_ts: u64,
    genesis_delta: u64,
}

pub fn new_synchrony(bootstrap_ts: u64) -> Synchrony {
    Synchrony { 
        current_stage: -1,
        bootstrap_ts, 
        genesis_delta: GENESIS_TIMESTAMP_DELTA 
    }
}

impl Synchrony {
    // get_genesis_stage_ts returns timestamp of the genesis stage
    fn get_genesis_stage_ts(&self) -> u64 {
        self.bootstrap_ts + self.genesis_delta
    }

    // swait waits until the global beginning of stage s. See SPEC.md for details.
    pub fn swait(&mut self, s: i64) {
        let desired_timestamp = self.get_genesis_stage_ts() + (PROTOCOL_DELTA * s) as u64;
        println!("current timestamp: {}, desired timestamp: {}", get_current_timestamp(), desired_timestamp);
        if get_current_timestamp() >= desired_timestamp {
            panic!("waiting for stage that already started")
        }

        loop {
            if get_current_timestamp() >= desired_timestamp {
                break;
            }
        }
        self.set_current_stage(s)
    }

    fn set_current_stage(&mut self, s: i64) {
        if s < self.current_stage {
            panic!("cannot set stage that is less than current stage")
        }

        self.current_stage = s;
    } 

    pub fn get_current_stage(&self) -> i64 {
        self.current_stage
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