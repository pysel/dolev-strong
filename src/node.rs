#![allow(dead_code)]

use ed25519_dalek::{SecretKey, PublicKey};
use crate::utils::fs::{get_listen_socket, lines_to_peers, get_num_peers, parse_mode};

mod network;
pub mod peer;
mod connections;

pub enum Mode {
    LEADER,
    FOLLOWER,
}

pub struct Node {
    pubkey: PublicKey,
    privkey: SecretKey,
    pub mode: Mode,
    pub connection: connections::Connection,
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, config_index: i32, path_to_config_file: String) -> Node {
    let peers = lines_to_peers(path_to_config_file.clone());
    let listen_socket = get_listen_socket(path_to_config_file.clone(), config_index.try_into().unwrap());
    let num_peers = get_num_peers(path_to_config_file.clone());
    let mode = parse_mode(path_to_config_file.clone(), config_index);
    
    Node {
        pubkey,
        privkey,
        mode,
        connection: connections::new_connection(num_peers, peers, listen_socket, None)
    }
} 