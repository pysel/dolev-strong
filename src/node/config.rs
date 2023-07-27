use super::peer::Peer;
use super::Mode;
use std::net::{TcpStream, SocketAddr};

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub num_peers: i32,
    pub peers: Vec<Peer>,
    pub listen_socket: SocketAddr,
    listen_streams: Option<Vec<TcpStream>>, // listen_streams is a list of tcp connections from which to expect getting messages from other processes
    write_streams: Option<Vec<TcpStream>> // write_streams is a list of tcp connections to which to send messages to
}

pub fn new_config(mode: Mode, num_peers: i32, peers: Vec<Peer>, listen_socket: SocketAddr, listen_streams: Option<Vec<TcpStream>>, write_streams: Option<Vec<TcpStream>>) -> Config {
    Config { mode, num_peers, peers, listen_socket, listen_streams, write_streams }
}

impl Config {
    // Unsafe. Panics when trying to set None
    pub fn set_write_streams(&mut self, write_streams: Option<Vec<TcpStream>>) {
        if let Some(streams) = write_streams {
            self.write_streams = Some(streams);
        }

        panic!("Trying to set empty write_streams")
    }

    pub fn set_listen_streams(&mut self, listen_streams: Option<Vec<TcpStream>>) {
        if let Some(streams) = listen_streams {
            self.listen_streams = Some(streams);
        };

        panic!("Trying to set empty listen_streams")
    }

    // pub fn set_listen_socket(&mut self, listen_socket: String) {

    // }
}
