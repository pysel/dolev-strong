use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use crate::node;

pub fn gen_keypair() -> Keypair {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    keypair
}

pub fn run_node(config_index: i32, path_to_config_file: String) -> node::Node {
    let keypair = gen_keypair();
    let mut node = node::new_node(keypair, config_index, path_to_config_file);
    node.establish_all_connections();
    node.broadcast_pubkey();
    node.receive_pubkeys();
    node
}