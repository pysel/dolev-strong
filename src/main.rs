use node::new_node;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

mod node;
mod message;
mod connection;

fn main() {
    run_node()
}

fn run_node() {
    let mode = message::Mode::FOLLOWER;
    let port = String::from("8000");
    let (pubkey, privkey) = gen_keypair();

    let node = new_node(pubkey, privkey, mode, port);
    node.listen();
}

fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    (keypair.public, keypair.secret)
}