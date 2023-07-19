#![allow(dead_code)]

use std::net::TcpStream;

use ed25519_dalek::{SecretKey, PublicKey, ed25519::Error};
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
    pub peers: Option<Vec<TcpStream>> // peers is a list of tcp connections to other processes
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, mode: Mode, listen_port: i32, num_peers: i32) -> Node {
    Node {
        pubkey,
        privkey,
        mode,
        listen_port,
        num_peers,
        peers: None,
    }
} 