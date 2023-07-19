#![allow(dead_code)]

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
    pub listen_port: String,
    // peers: Vec<PublicKey>
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, mode: Mode, listen_port: String) -> Node {
    Node {
        pubkey,
        privkey,
        mode,
        listen_port,
    }
} 