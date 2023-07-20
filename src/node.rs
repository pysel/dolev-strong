#![allow(dead_code)]

use std::net::TcpStream;

use ed25519_dalek::{SecretKey, PublicKey};
mod connection;

pub enum Mode {
    LEADER,
    FOLLOWER,
}

pub struct Node {
    pubkey: PublicKey,
    privkey: SecretKey,
    mode: Mode,
    pub listen_port: i32,
    num_peers: i32,
    pub listen_streams: Option<Vec<TcpStream>> // listen_streams is a list of tcp connections from which to expect getting messages from other processes
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, mode: Mode, listen_port: i32, num_peers: i32) -> Node {
    Node {
        pubkey,
        privkey,
        mode,
        listen_port,
        num_peers,
        listen_streams: None,
    }
} 