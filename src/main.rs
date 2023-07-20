use std::{env, io::Read};

mod node;
mod message;
mod testutil;
mod utils;

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();
    let peers_file: String = args[1].clone();
    let config_index: i32 = args[2].parse::<i32>().unwrap();

    let node: node::Node = testutil::run_node(config_index, peers_file);

    print!("{:?}", node.connection.listen_streams);

    if let Some(peers) = node.connection.listen_streams {
        for mut peer in peers {
            let buf: &mut [u8] = &mut [0u8; 100];
            println!("Received {} bytes", peer.read(buf).unwrap());
            println!("{buf:?}")
        }
    }

}
