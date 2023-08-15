use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, Write};
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

use crate::node::peer::Peer;
use crate::node;

use utils::{new_streams, StreamType, Streams};

use super::Node;

mod utils;

impl node::Node {
    // establish_all_connections connects and accepts connections to/from other nodes
    pub fn establish_all_connections(&mut self) {
        let (tx, rx) = mpsc::channel::<Streams>();
        let (tx_bind, tx_conn) = (tx.clone(), tx);
        
        let peers: Vec<Peer> = self.config.peers();
        println!("{peers:?}");
        
        let listen_socket: SocketAddr = self.config.listen_socket();
        let num_peers: usize = self.config.peers().len();

        // run thread that waits for connections from other nodes
        thread::spawn(move || {
            let streams = Node::bind_and_wait_connection(listen_socket, num_peers.try_into().unwrap());
            match streams {
                Ok(streams) => {
                    tx_bind.send(
                        new_streams(streams, StreamType::LISTEN)
                    ).unwrap();
                }

                Err(e) => {
                    panic!("{e}");
                }
            }
        });

        // run thread that connect to other nodes
        thread::spawn(move || {
            let streams = Node::connect_until_success(peers);
            match streams {
                Ok(streams) => {
                    tx_conn.send(
                        new_streams(streams, StreamType::SEND)
                    ).unwrap();
                }

                Err(e) => {
                    panic!("{e}");
                }
            }
        });

        for received in rx {
            if received.s_type == StreamType::LISTEN {
                let listen_streams = received.streams;
                self.config.set_listen_streams(listen_streams)
            } else {
                let write_streams = received.streams;
                self.config.set_write_streams(write_streams)
            }
        }
    }

    // bind_and_wait_Config binds a listening port of this node and waits for other peers to connect to this port
    pub fn bind_and_wait_connection(listen_socket: SocketAddr, num_peers: i32) -> Result<Vec<TcpStream>, Error> {
        let listener: TcpListener = TcpListener::bind(listen_socket)
            .expect("Failed to bind");

        let mut peers: Vec<TcpStream> = vec![];
        loop { // wait until all peers are connected
            match listener.accept() {
                Ok((stream, _)) => {
                    peers.push(stream); // TODO: maybe add check if this is the accepted listener
        
                    if peers.len() == num_peers.try_into().expect("Could not convert num_peers into i32") {
                        break;
                    }
                }
                Err(e) => {
                    return Err(Error::new(ErrorKind::NotConnected, format!("Error while accepting listening stream: {e}")))
                }
            }
        }
        
        Ok(peers)
        // TODO: consider adding timeout
    }

    // connect_to_peers tries connecting to peers, returns Result of all attempts
    fn connect_to_peers(peers: &Vec<Peer>) -> Result<Vec<TcpStream>, Error> {
        let mut streams: Vec<TcpStream> = Vec::new();
        for peer in peers {
            match TcpStream::connect(peer.socket.clone()) {
                Ok(connection) => {
                    streams.push(connection);
                }

                Err(e) => {
                    return Err(Error::new(ErrorKind::NotConnected, format!("Failed to connect to peer {} with error {}", peer.socket, e)));
                }
            }
        }

        Ok(streams)
    }

    fn connect_until_success(peers: Vec<Peer>) -> Result<Vec<TcpStream>, Error> {
        let start: Instant = Instant::now();

        loop {
            let streams: Result<Vec<TcpStream>, Error> = Node::connect_to_peers(&peers);
            if let Ok(streams) = streams {
                return Ok(streams)
            }

            if start.elapsed() > Duration::from_secs(10) {
                let error = streams.unwrap_err();
                break Err(Error::new(ErrorKind::NotConnected, format!("Timeout triggered before self could connect to all peers: {error}")));
            }
        }
    }

    fn set_listen_streams(&mut self, streams: Option<Vec<TcpStream>>) {
        match streams {
            Some(streams) => {
                if self.config.peers().len() != streams.len().try_into().expect("Could not convert peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.config.listen_socket())
                }
                self.config.set_listen_streams(streams);
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }

    pub fn send_message(&self, recepient: Peer, msg: Vec<u8>) -> Option<Error> {
        match self.config.get_write_tcp_stream(recepient) {
            Ok(mut write_conn) => {
                match write_conn.write_all(&msg) {
                Err(e) => {
                    return Some(Error::new(ErrorKind::Other, format!("Failed to send message with error {e}")));
                }
                _ => return None
            }}
            
            Err(e) => {
                return Some(Error::new(ErrorKind::AddrNotAvailable, format!("Connection to {:?} not found with error {}", recepient.socket, e)))
            }
        }
    }
}
