use std::io::Read;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use crate::node;

impl node::Node {
    pub fn listen(self) {
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

        for mut stream in rx {
            let buf: &mut [u8] = &mut [0u8; 100];
            stream.read(buf).unwrap();
            println!("{:?}", buf);
        }
    }
}