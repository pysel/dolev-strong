#![allow(dead_code)]

use ed25519_dalek::{SecretKey, PublicKey};
use crate::utils::fs::parse_connection_from_config;

mod network;
pub mod peer;
pub mod config;

pub enum Mode {
    LEADER,
    FOLLOWER,
}

pub struct Node {
    pubkey: PublicKey,
    privkey: SecretKey,
    pub config: config::Config,
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, config_index: i32, path_to_config_file: String) -> Node {
    let mut node = Node {
        pubkey,
        privkey,
        config: parse_connection_from_config(path_to_config_file, config_index),
    };

    node.bind_and_wait_connection();
    node
} 