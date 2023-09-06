use crate::communication::peer::new_peer;
use crate::utils::fs::{parse_mode, parse_config_lines};

use super::message::serde::deserealize;
use super::peer::Peer;
use super::Mode;
use super::message::types::pk_broadcast::{SignedPkBroadcastBzType, PubkeyBroadcastMsgReceived};

use core::panic;
use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, Read};

#[derive(Debug)]
pub struct Config {
    mode: Mode, // TODO: consider moving to ConsensusNode
    config_index: i32,
    config_file: String,
    peers: Vec<Peer>,
    listen_socket: SocketAddr,
    listen_streams: Option<Vec<TcpStream>>, // listen_streams is a list of tcp connections from which to expect getting messages from other processes
    write_streams: Option<Vec<TcpStream>> // write_streams is a list of tcp connections to which to send messages to
}

pub fn new_config(mode: Mode, config_index: i32, config_file: String, peers: Vec<Peer>, listen_socket: SocketAddr, listen_streams: Option<Vec<TcpStream>>, write_streams: Option<Vec<TcpStream>>) -> Config {
    Config { mode, config_index, config_file, peers, listen_socket, listen_streams, write_streams }
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

    // pub fn set_listen_socket(&mut self, listen_socket: String) {
    //     let socket_addr: SocketAddr = listen_socket.parse().expect(&format!("Failed to parse SocketAddr from line {listen_socket}"));
    //     self.listen_socket = socket_addr;
    // }

    pub fn listen_socket(&self) -> SocketAddr {
        self.listen_socket.clone()
    }

    // pub fn set_peers(&mut self, peers: Vec<Peer>) {
    //     self.peers = peers
    // }

    pub fn peers(&self) -> Vec<Peer> {
        self.peers.clone()
    }

    // pub fn set_mode(&mut self, mode: Mode) {
    //     self.mode = mode
    // }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn config_index(&self) -> i32 {
        return self.config_index
    }
    
    // if write == true, returns write tcp streams (to send messages) else listen streams
    pub fn get_tcp_stream(&self, peer: Peer, write: bool) -> Result<&TcpStream, Error> {
        let streams: &Option<Vec<TcpStream>>;
        if write {
            streams = &self.write_streams;
        } else {
            streams = &self.listen_streams;
        }

        if let Some(streams) = streams {
            for conn in streams {
                if conn.peer_addr().expect("Failed to get peer's address") == peer.socket {
                    return Ok(conn)
                }
            }
        }
        Err(Error::new(ErrorKind::InvalidData, "Trying to find peer's connection w/o established connection"))
    }

    // unsafe
    pub fn receive_pubkeys(&mut self) -> Result<(), Error> {
        let streams: &Vec<TcpStream> = self.listen_streams.as_ref().expect("Trying to read from a stream w/o setting streams");
        for (i, mut stream) in streams.into_iter().enumerate() {
            let mut buf: SignedPkBroadcastBzType = [0; 102];
            // println!("Receiving message on port {:?} || From: {:?} || Index {}", stream.local_addr(), stream.peer_addr(), self.config_index);

            match stream.read_exact(&mut buf) {
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other, 
                        format!("Error when reading bytes on TCP stream in pk broadcast phase: {}", e)
                    ));
                }
    
                _ => {} // ignore ok
            }
            match deserealize(buf.to_vec()) {
                Ok(result) => {
                    if let Some(result) = result.as_any().downcast_ref::<PubkeyBroadcastMsgReceived>() {
                        let config_lines: Vec<Vec<String>> = parse_config_lines(self.config_file.to_owned());
                        let peer_mode: Mode = parse_mode(config_lines, result.peer_index);

                        self.peers[i] = new_peer(self.peers[i].socket, Some(result.pubkey), Some(peer_mode), Some(stream.peer_addr().unwrap()));

                        return Ok(())
                    }

                    panic!("failed to receive pubkeys: deserealized message failed to typecast") // trusted setup assumption not met, hence panic
                }
    
                Err(e) => {
                    return Err(e)
                }
            } 
        }
        Err(Error::new(ErrorKind::Interrupted, "Could not receive pubkeys"))
    }

    pub fn get_round_leader(&self) -> Option<Peer> {
        if self.mode() == Mode::LEADER {
            return None
        }
        
        for peer in self.peers() {
            if peer.mode.unwrap() == Mode::LEADER {
                println!("Leader");
                return Some(peer)
            }
        }  
        
        panic!("No leader found")
    }
}
