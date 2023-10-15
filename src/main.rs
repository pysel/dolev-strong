#![allow(non_snake_case)]
use std::env;

use consensus::ConsensusNode;

mod communication;
mod utils;
mod consensus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_config_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();
    let F = args[3].parse::<i64>().unwrap();
    let bootstrap_timestamp: u64 = args[4].parse::<u64>().unwrap(); // no nanoseconds

    let consensus_node: ConsensusNode<'_> = ConsensusNode::new_consensus_node(config_index, path_to_config_file, bootstrap_timestamp, F);
    consensus_node.launch();
}
