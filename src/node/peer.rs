// use ed25519_dalek::PublicKey;

use std::net::SocketAddr;

#[derive(Debug, PartialEq, Clone)]
pub struct Peer {
    pub socket: SocketAddr,
    // pubkey: PublicKey,
}

pub fn new_peer(socket: SocketAddr) -> Peer {
    Peer {
        socket
    }
}