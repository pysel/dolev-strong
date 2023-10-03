use ed25519_dalek::PublicKey;

use std::net::SocketAddr;

use super::{Mode, network::docker::DockerSocket};

pub mod sanity;

#[derive(Debug, PartialEq, Clone)]
pub struct Peer {
    pub socket: DockerSocket, // a socket at which this peer is listening to incoming messages
    pub mode: Option<Mode>,
    pub pubkey: Option<PublicKey>,
    pub peer_write_socket: Option<SocketAddr> // a socket from which to expect receiving messages from this peer
}

pub fn new_peer(socket: DockerSocket, pubkey: Option<PublicKey>, mode: Option<Mode>, peer_write_socket: Option<SocketAddr>) -> Peer {
    Peer {
        socket,
        pubkey,
        mode,
        peer_write_socket
    }
}