use std::env;

mod node;
mod testutil;
mod utils;
mod consensus;

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();
    let peers_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();

    testutil::run_node(config_index, peers_file);
}
