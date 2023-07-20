#![allow(dead_code)]

use std::net::TcpStream;
use ed25519_dalek::{SecretKey, PublicKey};
use peer::Peer;
use crate::utils;

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

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, mode: Mode, config_index: i32, num_peers: i32, path_to_peers_file: String) -> Node {
    let peers = utils::fs::lines_to_peers(path_to_peers_file);
    let listen_port = 8000;
    Node {
        pubkey,
        privkey,
        mode,
        connection: connections::new_connection(num_peers, peers, listen_port, None)
    }
} 