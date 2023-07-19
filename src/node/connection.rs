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

#[cfg(test)]
mod tests{
    use crate::testutil;
    use crate::node::Mode;
    use crate::node;

    #[test]
    fn message_passing(){
        let (pubkey, privkey) = testutil::gen_keypair();
        let mode = Mode::LEADER;
        let listen_port = 8000.to_string();
        let node = node::new_node(pubkey, privkey, mode, listen_port);
        node.listen();

        print!("hello")
    }
}