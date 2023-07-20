use super::peer::Peer;
use std::net::TcpStream;

pub struct Connection {
    pub num_peers: i32,
    pub peers: Vec<Peer>,
    pub listen_socket: String,
    pub listen_streams: Option<Vec<TcpStream>> // listen_streams is a list of tcp connections from which to expect getting messages from other processes
}

pub fn new_connection(num_peers: i32, peers: Vec<Peer>, listen_socket: String, listen_streams: Option<Vec<TcpStream>>) -> Connection {
    Connection { num_peers, peers, listen_socket, listen_streams }
}