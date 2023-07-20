// use ed25519_dalek::PublicKey;

#[derive(Debug)]
pub struct Peer {
    ip: String,
    // pubkey: PublicKey,
}

pub fn new_peer(ip: String) -> Peer {
    Peer {
        ip
    }
}