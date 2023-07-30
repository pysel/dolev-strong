#![allow(dead_code)]

use ed25519_dalek::Keypair;
use crate::utils::fs::parse_config_from_file;

mod network;
pub mod peer;
pub mod config;
pub mod auth;

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    LEADER,
    FOLLOWER,
}

#[derive(Debug)]
pub struct Node {
    keypair: Keypair,
    config: config::Config,
}

pub fn new_node(keypair: Keypair, config_index: i32, path_to_config_file: String) -> Node {
    let node: Node = Node {
        keypair,
        config: parse_config_from_file(path_to_config_file, config_index),
    };

    node
} 