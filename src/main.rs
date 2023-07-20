use core::panic;
use std::{env, io::Read};

mod node;
mod message;
mod testutil;
mod utils;

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
    
    let port = args[2].parse::<i32>().unwrap();
    let config_index = args[3].parse::<i32>().unwrap();
    let peers_file = args[4].clone();

    let node = testutil::run_node(mode, port, config_index, peers_file);

    print!("{:?}", node.connection.listen_streams);

    if let Some(peers) = node.connection.listen_streams {
        for mut peer in peers {
            let buf: &mut [u8] = &mut [0u8; 100];
            println!("Received {} bytes", peer.read(buf).unwrap());
            println!("{buf:?}")
        }
    }

}
