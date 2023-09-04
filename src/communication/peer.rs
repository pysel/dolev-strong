use ed25519_dalek::PublicKey;

use std::net::SocketAddr;

use super::Mode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Peer {
    pub socket: SocketAddr,
    pub mode: Option<Mode>,
    pub pubkey: Option<PublicKey>,
    peer_write_socket: Option<SocketAddr>
}

pub fn new_peer(socket: SocketAddr, pubkey: Option<PublicKey>, mode: Option<Mode>, peer_write_socket: Option<SocketAddr>) -> Peer {
    Peer {
        socket,
        pubkey,
        mode,
        peer_write_socket
    }
}