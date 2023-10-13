use std::{env, thread::sleep};

use consensus::ConsensusNode;

mod communication;
mod utils;
mod consensus;

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();
    let peers_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();
    let bootstrap_timestamp: u64 = args[3].parse::<u64>().unwrap(); // no nanoseconds

    run_node(config_index, peers_file, bootstrap_timestamp);
}

fn run_node(config_index: i32, path_to_config_file: String, bootstrap_timestamp: u64) {
    let consensus_node: ConsensusNode<'_> = ConsensusNode::new_consensus_node(config_index, path_to_config_file, bootstrap_timestamp);
    consensus_node.launch();
}
