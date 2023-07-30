use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind};
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

use crate::utils::vec::unwrap_streams;
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
        let listen_socket: SocketAddr = self.config.listen_socket();
        let num_peers: i32 = self.config.num_peers();

        // run thread that waits for connections from other nodes
        thread::spawn(move || {
            if let Ok(conns) = Node::bind_and_wait_connection(listen_socket, num_peers) {
                tx_bind.send(
                    new_streams(conns, StreamType::LISTEN)
                ).unwrap();
            }
        });

        // run thread that connect to other nodes
        thread::spawn(move || {
            if let Ok(streams) = Node::connect_until_success(peers) {
                tx_conn.send(
                    new_streams(streams, StreamType::SEND)
                ).unwrap();
            } else {
                panic!("Failed to connect to all peers")
            }
        });

        for received in rx {
            if received.s_type == StreamType::LISTEN {
                println!("Received {received:?}");
                self.config.set_listen_streams(received.streams)
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
    fn connect_to_peers(peers: &Vec<Peer>) -> Vec<Result<TcpStream, Error>> {
        let mut streams: Vec<Result<TcpStream, Error>> = Vec::new();
        for peer in peers {
            let stream = TcpStream::connect(peer.socket.clone());
            streams.push(stream);
        }
        streams
    }

    fn connect_until_success(peers: Vec<Peer>) -> Result<Vec<TcpStream>, Error> {
        let start: Instant = Instant::now();

        loop {
            let streams: Vec<Result<TcpStream, Error>> = Node::connect_to_peers(&peers);
            if let Ok(streams) = unwrap_streams(streams) {
                return Ok(streams)
            }

            if start.elapsed() > Duration::from_secs(10) {
                break Err(Error::new(ErrorKind::NotConnected, "Timeout triggered before self could connect to all peers"));
            }
        }
    }

    fn set_listen_streams(&mut self, streams: Option<Vec<TcpStream>>) {
        match streams {
            Some(streams) => {
                if self.config.num_peers() != streams.len().try_into().expect("Could not convert peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.config.listen_socket())
                }
                self.config.set_listen_streams(streams);
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
