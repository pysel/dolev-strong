use std::net::TcpStream;

#[derive(PartialEq)]
pub enum StreamType {
    LISTEN,
    SEND,
}
pub struct Streams {
    pub streams: Vec<TcpStream>,
    pub s_type: StreamType, 
}

// unsafe
pub fn new_streams(streams: Vec<TcpStream>, s_type: StreamType) -> Streams {
    if streams.len() <= 0 {
        panic!("trying to pass empty streams vector")
    }

    Streams { streams, s_type }
}