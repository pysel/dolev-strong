use std::fs::read_to_string;
use std::net::SocketAddr;

use crate::node::{peer, Mode, config};

fn parse_peers(config_lines: &mut Vec<Vec<String>>, config_index: i32) -> Vec<peer::Peer> {
    let mut result: Vec<peer::Peer> = Vec::new();
    config_lines.remove(config_index.try_into().expect("Failed to convert i32 to usize"));

    for line in config_lines {
        let addr: SocketAddr = line[0].parse().expect(&format!("Failed to parse socket address from line {}", line[0]));
        result.push(peer::new_peer(addr));
    }

    result 
}


fn parse_config_lines(filename: String) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_owned().split(" ").into_iter().map(|x| x.to_owned()).collect())
    }

    result    
}

fn parse_num_peers(config_lines: Vec<Vec<String>>) -> i32 {
    (config_lines.len() - 1).try_into().unwrap()
}

fn parse_listen_socket(config_lines: Vec<Vec<String>>, config_index: i32) -> SocketAddr {
    config_lines[config_index as usize][0].clone().parse().expect("Failed to parse listen socket")
}

pub fn parse_mode(config_lines: Vec<Vec<String>>, config_index: i32) -> Mode {
    let mode: &str = &config_lines[config_index as usize][1].clone();
    match mode {
        "leader" => Mode::LEADER,
        "follower" => Mode::FOLLOWER,
        _ => panic!("Invalid mode {mode}, should be either `follower` or `leader`")
    }
}

use crate::node::config::{Config, new_config}; 
pub fn parse_config_from_file(filename: String, config_index: i32) -> Config {
    let config_lines: Vec<Vec<String>>  = parse_config_lines(filename);

    let peers: Vec<peer::Peer> = parse_peers(&mut config_lines.clone(), config_index);
    let listen_socket: SocketAddr = parse_listen_socket(config_lines.clone(), config_index.try_into().unwrap());
    let num_peers: i32 = parse_num_peers(config_lines.clone());
    let mode: Mode = parse_mode(config_lines.clone(), config_index);

    new_config(mode, config_index, num_peers, peers, listen_socket, None, None)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net::{SocketAddr, Ipv4Addr, IpAddr};
    use super::{parse_config_lines, parse_peers, parse_num_peers, parse_listen_socket, parse_mode};
    use crate::node::peer::{Peer, new_peer};
    use crate::node::Mode;

    const TEST_CONFIG_FNAME: &'static str = "/src/testutil/test-config.txt";
    const TEST_CONFIG_INDEX: i32 = 0;

    #[test]
    fn test_parse_splitted_lines() {
        let full_config_path = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let config_lines = parse_config_lines(full_config_path);
        let expected_lines: Vec<Vec<String>> = vec![
            vec!["127.0.0.1:8000".to_owned(), "leader".to_owned()],
            vec!["127.0.0.1:8001".to_owned(), "follower".to_owned()],
            vec!["127.0.0.1:8002".to_owned(), "follower".to_owned()],
            vec!["127.0.0.1:8003".to_owned(), "follower".to_owned()],
        ];

        assert_eq!(expected_lines, config_lines)
    }

    #[test]
    fn test_parse_peers() {
        let full_config_path = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let mut config_lines = parse_config_lines(full_config_path);
        let peers = parse_peers(&mut config_lines, TEST_CONFIG_INDEX);

        let expected_peers: Vec<Peer> = vec![
            new_peer(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8001)),
            new_peer(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8002)),
            new_peer(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8003)),
        ];

        assert_eq!(peers, expected_peers)
    }

    #[test]
    fn test_parse_num_peers() {
        let full_config_path = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let config_lines = parse_config_lines(full_config_path);

        let num_peers = parse_num_peers(config_lines);
        let expected_num_peers = 3;

        assert_eq!(expected_num_peers, num_peers)
    }

    #[test]
    fn test_parse_listen_socket() {
        let full_config_path: String = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let config_lines: Vec<Vec<String>> = parse_config_lines(full_config_path);

        let socket: SocketAddr = parse_listen_socket(config_lines, TEST_CONFIG_INDEX);
        let expected_socket: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);

        assert_eq!(expected_socket, socket)
    }

    #[test]
    fn test_parse_mode() {
        let full_config_path = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let config_lines = parse_config_lines(full_config_path);

        let mode = parse_mode(config_lines, TEST_CONFIG_INDEX);
        let expected_mode = Mode::LEADER;

        assert_eq!(expected_mode, mode)
    }
}