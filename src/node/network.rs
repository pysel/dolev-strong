use std::net::{TcpListener, TcpStream};
use std::io::Error;

use crate::node;

impl node::Node {
    // bind_and_wait_Config binds a listening port of this node and waits for other peers to connect to this port
    pub fn bind_and_wait_connection(&mut self) {
        let listener: TcpListener = TcpListener::bind(String::from(self.config.listen_socket.clone()))
            .expect("Failed to bind");

        let num_peers: i32 = self.config.num_peers.clone();
        let mut peers: Vec<TcpStream> = vec![];

        println!("Listening on socket {}", self.config.listen_socket);
    
        loop { // wait until all peers are connected
            match listener.accept() {
                Ok((stream, _)) => {
                    peers.push(stream);
        
                    if peers.len() == num_peers.try_into().expect("Could not convert waiting_for_num_connections into i32") {
                        break;
                    }
                }
                Err(e) => {
                    println!("ERROR CONNECTING: {e}")
                }
            }
        }
        
        self.set_listen_stream(Some(peers));
        // TODO: consider adding timeout
    }

    fn connect_to_peers(self) -> Vec<Result<TcpStream, Error>> {
        let mut streams: Vec<Result<TcpStream, Error>> = Vec::new();
        for peer in self.config.peers {
            let stream = TcpStream::connect(peer.ip.clone());
            streams.push(stream);
        }
        streams
    }

    fn set_listen_stream(&mut self, peers: Option<Vec<TcpStream>>) {
        match &peers {
            Some(p) => {
                if self.config.num_peers != p.len().try_into().expect("Could not convery peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.config.listen_socket)
                }
                self.config.listen_streams = peers;
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
