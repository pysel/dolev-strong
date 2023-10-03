use std::env;

use communication::network::docker::wait_until_containers_are_up;

mod communication;
mod testutil;
mod utils;
mod consensus;

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();
    let peers_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();
    let bootstrap_timestamp: u64 = args[3].parse::<u64>().unwrap(); // no nanoseconds

    // Docker hack
    wait_until_containers_are_up();
    
    testutil::run_node(config_index, peers_file, bootstrap_timestamp);
}


