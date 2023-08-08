use ed25519_dalek::PublicKey;

use super::peer::Peer;
use super::Mode;
use super::message::serde::deserealize_pb;
use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, Read};


#[derive(Debug)]
pub struct Config {
    mode: Mode,
    num_peers: i32,
    peers: Vec<Peer>,
    listen_socket: SocketAddr,
    listen_streams: Option<Vec<TcpStream>>, // listen_streams is a list of tcp connections from which to expect getting messages from other processes
    write_streams: Option<Vec<TcpStream>> // write_streams is a list of tcp connections to which to send messages to
}

pub fn new_config(mode: Mode, num_peers: i32, peers: Vec<Peer>, listen_socket: SocketAddr, listen_streams: Option<Vec<TcpStream>>, write_streams: Option<Vec<TcpStream>>) -> Config {
    Config { mode, num_peers, peers, listen_socket, listen_streams, write_streams }
}

impl Config {
    // Unsafe. Panics when trying to set None
    pub fn set_write_streams(&mut self, write_streams: Vec<TcpStream>) {
        if write_streams.len() <= 0 {
            panic!("Trying to set empty listen_streams")
        };
        self.write_streams = Some(write_streams);
    }

    pub fn set_listen_streams(&mut self, listen_streams: Vec<TcpStream>) {
        if listen_streams.len() <= 0 {
            panic!("Trying to set empty listen_streams")
        };

        self.listen_streams = Some(listen_streams);
    }

    pub fn set_listen_socket(&mut self, listen_socket: String) {
        let socket_addr: SocketAddr = listen_socket.parse().expect(&format!("Failed to parse SocketAddr from line {listen_socket}"));
        self.listen_socket = socket_addr;
    }

    pub fn listen_socket(&self) -> SocketAddr {
        self.listen_socket.clone()
    }

    pub fn set_peers(&mut self, peers: Vec<Peer>) {
        self.peers = peers
    }

    pub fn peers(&self) -> Vec<Peer> {
        self.peers.clone()
    }

    // unsafe
    pub fn set_num_peers(&mut self, num_peers: i32) {
        if num_peers <= 0 {
            panic!("num_peers can only be set positive")
        }
        self.num_peers = num_peers
    }

    pub fn num_peers(&self) -> i32 {
        self.num_peers
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }
    
    pub fn get_write_tcp_stream(&self, peer: Peer) -> Result<&TcpStream, Error> {
        if let Some(streams) = &self.write_streams {
            for conn in streams {
                if conn.peer_addr().expect("Failed to get peer's address") == peer.socket {
                    return Ok(conn)
                }
            }
        } 
        Err(Error::new(ErrorKind::InvalidData, "Trying to find peer's connection w/o established connection"))
    }

    // unsafe
    pub fn read_pubkey_from_stream(&self, s_index: usize) -> Result<PublicKey, Error> {
        let streams: &Vec<TcpStream> = self.listen_streams.as_ref().expect("Trying to read from a stream w/o setting streams");
        let mut stream = &streams[s_index];
        let mut buf: Vec<u8> = Vec::new();
        match stream.read_to_end(&mut buf) {
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other, 
                    format!("Error when reading bytes on TCP stream in pk broadcast phase: {}", e)
                ));
            }

            _ => {} // ignore ok
        }

        match deserealize_pb(&buf) {
            Ok(pubkey) => {
                return Ok(pubkey)
            }

            Err(e) => {
                Err(e)
            }
        } 
    }
}
