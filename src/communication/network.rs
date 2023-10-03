use std::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind, Write};
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

use crate::communication::peer::Peer;
use crate::communication;

use utils::{new_streams, StreamType, Streams};

use self::docker::DockerSocket;

use super::Communication;
use super::message::MessageI;

mod utils;
pub mod docker;

impl communication::Communication {
    // setup establishes connections with other consensus participants and implements PKI
    pub fn setup(&mut self) {
        self.establish_all_connections();
        self.establish_pki();
        println!("Setup done!")
    }

    // establish_pki implements public key infrastructure trusted setup assumption.
    // it broadcasts the pubkey of this node and receives pubkeys of all other nodes.
    fn establish_pki(&mut self) {
        self.broadcast_pubkey();
        self.receive_pubkeys();
    }

    // establish_all_connections connects and accepts connections to/from other nodes
    fn establish_all_connections(&mut self) {
        let (tx, rx) = mpsc::channel::<Streams>();
        let (tx_bind, tx_conn) = (tx.clone(), tx);
        
        let peers: Vec<Peer> = self.config.peers.clone();
        
        let listen_socket: DockerSocket = self.config.listen_socket();
        let num_peers: usize = self.config.peers.len();

        // run thread that waits for connections from other nodes
        thread::spawn(move || {
            let streams: Result<Vec<TcpStream>, Error> = Communication::bind_and_wait_connection(listen_socket, num_peers.try_into().unwrap());
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
            let streams: Result<Vec<TcpStream>, Error> = Communication::connect_until_success(&peers);
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
    fn bind_and_wait_connection(listen_socket: DockerSocket, num_peers: i32) -> Result<Vec<TcpStream>, Error> {
        let listener: TcpListener = TcpListener::bind(listen_socket.tuple())
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
            match TcpStream::connect(peer.socket.tuple()) {
                Ok(connection) => {
                    connection.set_read_timeout(Some(Duration::new(0, 3000000)))?; // almost third of a second
                    streams.push(connection);
                }

                Err(e) => {
                    return Err(Error::new(ErrorKind::NotConnected, format!("Failed to connect to peer {:?} with error {}", peer.socket.tuple(), e)));
                }
            }
        }

        Ok(streams)
    }

    fn connect_until_success(peers: &Vec<Peer>) -> Result<Vec<TcpStream>, Error> {
        let start: Instant = Instant::now();

        loop {
            let streams: Result<Vec<TcpStream>, Error> = Communication::connect_to_peers(&peers);
            if let Ok(streams) = streams {
                return Ok(streams)
            }

            if start.elapsed() > Duration::from_secs(10) {
                let error = streams.unwrap_err();
                break Err(Error::new(ErrorKind::NotConnected, format!("Timeout triggered before self could connect to all peers: {error}")));
            }
        }
    }

    // send_message sends message to a peer
    pub fn send_message(&self, recepient: &Peer, msg: &dyn MessageI) -> Option<Error> {
        match self.config.get_write_tcp_stream(recepient) {
            Ok(mut write_conn) => {
                println!("Writing to peer: {:?} || local address: {:?} || index: {:?}", recepient.socket, write_conn.local_addr(), self.config.config_index());
                // println!("Message to send: {:?}", &msg.serialize(&self.keypair, self.config.config_index()));
                // attempt to write a message
                match write_conn.write(&msg.serialize(&self.keypair, self.config.config_index())) {
                    Err(e) => {
                        Some(Error::new(ErrorKind::Other, format!("Failed to send message with error {e}")))
                    }

                    _ => {
                        // if message was successfully written, flush writer immediately 
                        match write_conn.flush() {
                            Err(e) => {
                                return Some(Error::new(ErrorKind::Other, format!("Failed to flush connection {e}")))
                            }

                            Ok(_) => {
                                return None
                            }
                        }

                    }
                }
            }
            
            Err(e) => {
                return Some(Error::new(ErrorKind::AddrNotAvailable, format!("Connection to {:?} not found with error {}", recepient.socket, e)))
            }
        }
    }

    // broadcast_message sends message to all peers
    pub fn broadcast_message(&self, msg: &dyn MessageI) -> Option<Error>{
        for peer in &self.config.peers {
            // println!("Sending message to {:?}. Communication {}", peer.socket, self.config.config_index());
            if let Some(e) = self.send_message(peer, msg) {
                return Some(Error::new(ErrorKind::Other, format!("Failed to send message to peer {:?} with error {}", peer.socket, e)))
            }
        }
        None
    }
}
