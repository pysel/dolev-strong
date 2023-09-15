use ed25519_dalek::{Keypair, PublicKey};
use crate::utils::fs::parse_config_from_file;

mod network;

pub mod pki;
pub mod message;
pub mod peer;
pub mod config;
pub mod exported;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    LEADER,
    FOLLOWER,
}

// Communication contains keypair used when signing messages and a configuration struct
pub struct Communication {
    keypair: Keypair,
    pub config: config::Config,
}

pub fn new_node(keypair: Keypair, config_index: i32, path_to_config_file: String) -> Communication {
    let node: Communication = Communication {
        keypair,
        config: parse_config_from_file(path_to_config_file, config_index),
    };
    node
} 

impl Communication {
    pub fn get_pubkey(&self) -> PublicKey {
        self.keypair.public
    }
}