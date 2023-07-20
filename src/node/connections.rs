use super::peer::Peer;
use std::net::TcpStream;

pub struct Connection {
    pub num_peers: i32,
    pub peers: Vec<Peer>,
    pub listen_port: i32,
    pub listen_streams: Option<Vec<TcpStream>> // listen_streams is a list of tcp connections from which to expect getting messages from other processes
}

pub fn new_connection(num_peers: i32, peers: Vec<Peer>, listen_port: i32, listen_streams: Option<Vec<TcpStream>>) -> Connection {
    Connection { num_peers, peers, listen_port, listen_streams }
}