use core::panic;
use std::env;

mod node;
mod message;
mod testutil;

use node::Mode::{LEADER, FOLLOWER};

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();

    let mode = {
        let mode_arg = args[1].as_str();
        match mode_arg {
            "leader" => LEADER,
            "follower" => FOLLOWER,
            _ => panic!("Invalid mode {mode_arg}, should be either `follower` or `leader`")
        }
    };
    
    let port = String::from(&args[2]);

    testutil::run_node(mode, port);
}
