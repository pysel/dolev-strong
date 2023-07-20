use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use crate::node;


pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    (keypair.public, keypair.secret)
}

pub fn run_node(mode: node::Mode, port: i32, config_index: i32, path_to_peers_file: String) -> node::Node {
    let (pubkey, privkey) = gen_keypair();
    let mut node = node::new_node(pubkey, privkey, mode, port, config_index, path_to_peers_file);
    println!("{:?}", node.connection.peers);
    node.bind_and_wait_connection();
    node
}