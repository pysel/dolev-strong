use std::fs::read_to_string;
use crate::node::peer;

pub fn lines_to_peers(filename: String) -> Vec<peer::Peer> {
    let mut result: Vec<peer::Peer> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(peer::new_peer(line.to_owned()));
    }

    result 
}

// pub fn get_listen_socket(config_index: i32) -> String {

// }