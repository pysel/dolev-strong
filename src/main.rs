use node::new_node;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use core::panic;
use std::env;

mod node;
mod message;

use message::Mode::{LEADER, FOLLOWER};

fn main() {
    // first arg - node's mode (leader/follower), second - port
    let args: Vec<String> = env::args().collect();

    let mode = {
        let mode_arg = args[1].as_str();
        match mode_arg {
            "leader" => LEADER,
            "follower" => FOLLOWER,
            _ => panic!("Invalid mode {mode_arg}, should be either `follower` or `leader`")
        }
    };
    
    let port = String::from(&args[2]);

    run_node(mode, port);
}

fn run_node(mode: message::Mode, port: String) {
    let (pubkey, privkey) = gen_keypair();

    let node = new_node(pubkey, privkey, mode, port);
    node.listen();
}

fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    (keypair.public, keypair.secret)
}