use std::{net::TcpStream, io::Error};


fn is_successful_connection(stream: &Result<TcpStream, Error>) -> bool {
    if let Ok(_) = stream {
        return true
    }
    false
}

pub fn unwrap_streams(streams: Vec<Result<TcpStream, Error>>) -> Result<Vec<TcpStream>, &'static str> {
    let mut result: Vec<TcpStream> = Vec::new();
    for stream in streams {
        if is_successful_connection(&stream) {
            result.push(stream.unwrap())
        }

        return Err("Not all connections were established")
    }
    Ok(result)
}