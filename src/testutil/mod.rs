use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use crate::node;

pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    (keypair.public, keypair.secret)
}

pub fn run_node(mode: node::Mode, port: i32, peers: i32) -> node::Node {
    let (pubkey, privkey) = gen_keypair();

    let mut node = node::new_node(pubkey, privkey, mode, port, peers);
    node.connect_to_peers();
    node
}