use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use crate::node;

impl node::Node {
    pub fn connect_to_peers(&mut self, waiting_for_num_connections: i32) {
        let listener = TcpListener::bind(String::from(format!("127.0.0.1:{}", self.listen_port)))
            .expect("Could not bind to port");

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                match listener.accept() {
                    Ok((stream, _)) => {
                        tx.send(stream).unwrap(); 
                    }

                    Err(e) => {
                        println!("ERROR CONNECTING: {e}")
                    }
                }
            }
        });
        
        let mut peers: Vec<TcpStream> = vec![];
        for stream in rx {
            peers.push(stream);

            if peers.len() == waiting_for_num_connections.try_into().expect("Could not convert waiting_for_num_connections into i32") {
                break;
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
