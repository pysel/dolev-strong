use std::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind};
use std::time::{Instant, Duration};

use crate::utils::vec::unwrap_streams;

use crate::node;

impl node::Node {
    // establish_all_connections connects and accepts connections to/from other nodes
    pub fn establish_all_connections(&self) {

    }

    // bind_and_wait_Config binds a listening port of this node and waits for other peers to connect to this port
    pub fn bind_and_wait_connection(&self) -> Result<Vec<TcpStream>, Error> {
        let listener: TcpListener = TcpListener::bind(self.config.listen_socket())
            .expect("Failed to bind");

        let num_peers: i32 = self.config.num_peers();
        let mut peers: Vec<TcpStream> = vec![];

        println!("Listening on socket {}", self.config.listen_socket());
        loop { // wait until all peers are connected
            match listener.accept() {
                Ok((stream, _)) => {
                    peers.push(stream); // TODO: maybe add check if this is the accepted listener
        
                    if peers.len() == num_peers.try_into().expect("Could not convert waiting_for_num_connections into i32") {
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
    fn connect_to_peers(&self) -> Vec<Result<TcpStream, Error>> {
        let mut streams: Vec<Result<TcpStream, Error>> = Vec::new();
        for peer in self.config.peers() {
            let stream = TcpStream::connect(peer.socket.clone());
            streams.push(stream);
        }
        streams
    }

    fn connect_until_success(&self) -> Result<Vec<TcpStream>, Error> {
        let start = Instant::now();

        loop {
            let streams = self.connect_to_peers();
            if let Ok(streams) = unwrap_streams(streams) {
                return Ok(streams)
            }

            if start.elapsed() > Duration::from_secs(10) {
                break Err(Error::new(ErrorKind::NotConnected, "Timeout triggered before self could connect to all peers"));
            }
        }
    }

    fn set_listen_stream(&mut self, peers: Option<Vec<TcpStream>>) {
        match &peers {
            Some(p) => {
                if self.config.num_peers() != p.len().try_into().expect("Could not convert peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.config.listen_socket())
                }
                self.config.set_listen_streams(peers);
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
