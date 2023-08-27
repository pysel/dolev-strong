use ed25519_dalek::Keypair;
use crate::utils::fs::parse_config_from_file;
use crate::consensus::genesis::strategy::GenesisStrategy;

mod network;
mod message;
mod auth;
pub mod sync;

pub mod peer;
pub mod config;
pub mod exported;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    LEADER,
    FOLLOWER,
}

pub struct Node<'a> {
    keypair: Keypair,
    pub genesis_strategy: Option<&'a dyn GenesisStrategy>,
    config: config::Config,
}

pub fn new_node(keypair: Keypair, config_index: i32, path_to_config_file: String) -> Node<'static> {
    let node: Node = Node {
        keypair,
        genesis_strategy: None,
        config: parse_config_from_file(path_to_config_file, config_index),
    };
    node
} 

impl<'a> Node<'a> {
    pub fn set_genesis_strategy(&mut self, strategy: &'a dyn GenesisStrategy) {
        self.genesis_strategy = Some(strategy);
    }
}