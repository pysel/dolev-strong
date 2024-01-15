#![allow(non_snake_case)]
use std::env;

use consensus::ConsensusNode;

mod communication;
mod utils;
mod consensus;
mod prototypes;

fn main() {
    println!("Starting consensus node");
    let args: Vec<String> = env::args().collect();
    let path_to_config_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();
    let F = args[3].parse::<i64>().unwrap();
    let bootstrap_timestamp: u64 = args[4].parse::<u64>().unwrap(); // no nanoseconds

    let consensus_node: ConsensusNode<'_> = ConsensusNode::new_consensus_node(config_index, path_to_config_file, bootstrap_timestamp, F);
    println!("Consensus node created");
    consensus_node.launch();
}

// fn main() {
//     let bytes = vec![99, 109, 1, 84, 229, 7, 191, 217, 36, 168, 137, 243, 254, 71, 197, 149, 43, 6, 91, 24, 135, 169, 214, 18, 192, 21, 52, 29, 211, 148, 30, 124, 148, 26, 190, 105, 199, 102, 6, 167, 121, 18, 58, 85, 132, 93, 183, 130, 3, 15, 138, 77, 158, 81, 127, 39, 132, 212, 43, 208, 56, 22, 163, 144, 4, 113, 0, 179, 198, 18, 213, 161, 73, 165, 88, 240, 177, 243, 142, 132, 32, 87, 97, 174, 84, 95, 24, 22, 106, 255, 220, 204, 28, 97, 151, 37, 43, 95, 153, 134, 140, 23, 130, 25, 69, 62, 51, 77, 16, 98, 101, 202, 97, 242, 243, 220, 248, 164, 123, 197, 3, 152, 125, 52, 114, 143, 168, 131, 229, 112, 1, 143, 128, 180, 96, 250, 24, 121, 214, 238, 18, 230, 120, 42, 182, 166, 206, 85, 111, 27, 172, 91, 177, 64, 44, 113, 144, 231, 58, 27, 145, 119, 126, 146, 61, 153, 223, 217, 200, 82, 20, 208, 60, 113, 24, 185, 204, 117, 116, 164, 76, 211, 44, 25, 57, 178, 103, 168, 219, 247, 240, 234, 239, 80, 8] ;

//     // Decode using UTF-8
//     let utf8_result = std::str::from_utf8(&bytes);
//     println!("UTF-8 Decoded: {:?}", utf8_result);
// }
