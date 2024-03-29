use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Error, ErrorKind, Write};
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

use crate::communication::peer::Peer;
use crate::communication;

use utils::{new_streams, StreamType, Streams};

use super::Communication;
use super::message::MessageI;

mod utils;

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
        
        let listen_socket: SocketAddr = self.config.listen_socket();
        let num_peers: usize = self.config.peers.len();

        // run thread that waits for connections from other nodes
        let listening_handle = thread::spawn(move || {
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
        let writing_handle = thread::spawn(move || {
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
                // println!("Received listen streams: {} || {:?}", listen_streams.len(), listen_streams);
                self.config.set_listen_streams(listen_streams)
            } else {
                let write_streams = received.streams;
                // println!("Received write streams: {} || {:?}", write_streams.len(), write_streams);
                self.config.set_write_streams(write_streams)
            }
        }

        listening_handle.join().unwrap();
        writing_handle.join().unwrap();
    }

    // bind_and_wait_Config binds a listening port of this node and waits for other peers to connect to this port
    fn bind_and_wait_connection(listen_socket: SocketAddr, num_peers: i32) -> Result<Vec<TcpStream>, Error> {
        let listener: TcpListener = TcpListener::bind(listen_socket)
            .expect("Failed to bind");

        let mut peers: Vec<TcpStream> = vec![];
        loop { // wait until all peers are connected
            match listener.accept() {
                Ok((stream, _)) => {
                    // println!("Accepted connection from port {:?}", stream.peer_addr().unwrap().port());
                    stream.set_read_timeout(Some(Duration::new(0, 3000000)))?; // almost third of a second
                    peers.push(stream);
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
                    connection.set_read_timeout(Some(Duration::new(0, 3000000)))?; // almost third of a second
                    streams.push(connection);
                }

                Err(e) => {
                    return Err(Error::new(ErrorKind::NotConnected, format!("Failed to connect to peer {} with error {}", peer.socket, e)));
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

            if start.elapsed() > Duration::from_secs(5) {
                let error = streams.unwrap_err();
                break Err(Error::new(ErrorKind::NotConnected, format!("Timeout triggered before self could connect to all peers: {error}")));
            }
        }
    }

    // send_message sends message to a peer
    pub fn send_message(&self, recepient: &Peer, msg: &dyn MessageI) -> Option<Error> {
        // println!("Sending message to peer: msg: {:?} ", msg.serialize(&self.keypair, self.config.config_index()));

        match self.config.get_write_tcp_stream(recepient) {
            Ok(mut write_conn) => {
                // println!("Writing to peer: {:?} || local address: {:?} || index: {:?} || msg size: {:?}", recepient.socket, write_conn.local_addr(), self.config.config_index(), &msg.serialize(&self.keypair, self.config.config_index()).len());
                // println!("Message to send: {:?}", &msg.serialize(&self.keypair, self.config.config_index()));
                // attempt to write a message
                println!("Sending message: {:?}", &msg.serialize(&self.keypair, self.config.config_index()));

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
                };

                match write_conn.flush() {
                    Err(e) => {
                        return Some(Error::new(ErrorKind::Other, format!("Failed to flush connection {e}")))
                    }

                    Ok(_) => {
                        return None
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
