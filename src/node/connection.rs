use std::net::{TcpListener, TcpStream};

use crate::node;

impl node::Node {
    pub fn bind_and_wait_connection(&mut self) {
        let listener: TcpListener = TcpListener::bind(String::from(format!("127.0.0.1:{}", self.listen_port)))
            .expect("Could not bind to port");

        let num_peers: i32 = self.num_peers.clone();
        let mut peers: Vec<TcpStream> = vec![];

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
        
        self.set_peers(Some(peers));
        // TODO: consider adding timeout
    }

    fn set_peers(&mut self, peers: Option<Vec<TcpStream>>) {
        match &peers {
            Some(p) => {
                if self.num_peers != p.len().try_into().expect("Could not convery peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.listen_port)
                }
                self.peers = peers;
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
