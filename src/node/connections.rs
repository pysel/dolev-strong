use super::peer::Peer;
use super::Mode;
use std::net::TcpStream;

pub struct Connection {
    pub mode: Mode,
    pub num_peers: i32,
    pub peers: Vec<Peer>,
    pub listen_socket: String,
    pub listen_streams: Option<Vec<TcpStream>> // listen_streams is a list of tcp connections from which to expect getting messages from other processes
}

pub fn new_connection(mode: Mode, num_peers: i32, peers: Vec<Peer>, listen_socket: String, listen_streams: Option<Vec<TcpStream>>) -> Connection {
    Connection { mode, num_peers, peers, listen_socket, listen_streams }
}

fn parse_connection_from_config() -> Connection {
    unimplemented!()
}