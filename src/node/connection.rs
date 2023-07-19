use std::net::TcpListener;

use crate::node;

impl node::Node {
    pub fn listen(self) {
        let listener = match TcpListener::bind(String::from(format!("127.0.0.1:{}", self.listen_port))) {
            Ok(listener) => listener,
            Err(e) => {
                panic!("ERROR WHEN BINDING {e}");
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("{:?}", stream)
                }

                Err(e) => {
                    println!("ERROR CONNECTING: {e}")
                }
            }
        }
    }
}