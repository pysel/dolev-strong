use std::net::{TcpListener, TcpStream};

use crate::node;

impl node::Node {
    // bind_and_wait_connection binds a listening port of this node and waits for other peers to connect to this port
    pub fn bind_and_wait_connection(&mut self) {
        let listener: TcpListener = TcpListener::bind(String::from(self.connection.listen_socket.clone()))
            .expect("Could not bind to port");

        let num_peers: i32 = self.connection.num_peers.clone();
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
        
        self.set_listen_stream(Some(peers));
        // TODO: consider adding timeout
    }

    fn connect_to_peers(self) {

    }

    fn set_listen_stream(&mut self, peers: Option<Vec<TcpStream>>) {
        match &peers {
            Some(p) => {
                if self.connection.num_peers != p.len().try_into().expect("Could not convery peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.connection.listen_socket)
                }
                self.connection.listen_streams = peers;
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
