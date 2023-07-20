#![allow(dead_code)]

use ed25519_dalek::{SecretKey, PublicKey};
use crate::utils::fs::{parse_connection_from_config};

mod network;
pub mod peer;
pub mod connections;

pub enum Mode {
    LEADER,
    FOLLOWER,
}

pub struct Node {
    pubkey: PublicKey,
    privkey: SecretKey,
    pub connection: connections::Connection,
}

pub fn new_node(pubkey: PublicKey, privkey: SecretKey, config_index: i32, path_to_config_file: String) -> Node {
    Node {
        pubkey,
        privkey,
        connection: parse_connection_from_config(path_to_config_file, config_index),
    }
} 