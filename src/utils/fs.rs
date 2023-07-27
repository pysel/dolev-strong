use std::fs::read_to_string;
use crate::node::{peer, Mode};

fn parse_peers(config_lines: Vec<Vec<String>>) -> Vec<peer::Peer> {
    let mut result: Vec<peer::Peer> = Vec::new();

    for line in config_lines {
        result.push(peer::new_peer(line[0].clone()));
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

fn parse_listen_socket(config_lines: Vec<Vec<String>>, config_index: i32) -> String {
    config_lines[config_index as usize][0].clone()
}

pub fn parse_mode(config_lines: Vec<Vec<String>>, config_index: i32) -> Mode {
    let mode: &str = &config_lines[config_index as usize][1].clone();
    match mode {
        "leader" => Mode::LEADER,
        "follower" => Mode::FOLLOWER,
        _ => panic!("Invalid mode {mode}, should be either `follower` or `leader`")
    }
}

use crate::node::config::{Config, new_connection}; 
pub fn parse_config_from_file(filename: String, config_index: i32) -> Config {
    let config_lines = parse_config_lines(filename);

    let peers = parse_peers(config_lines.clone());
    let listen_socket = parse_listen_socket(config_lines.clone(), config_index.try_into().unwrap());
    let num_peers = parse_num_peers(config_lines.clone());
    let mode = parse_mode(config_lines.clone(), config_index);

    new_connection(mode, num_peers, peers, listen_socket, None)
}

#[cfg(test)]
mod tests {
    use std::env;
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

        let config_lines = parse_config_lines(full_config_path);
        let peers = parse_peers(config_lines);

        let expected_peers: Vec<Peer> = vec![
            new_peer("127.0.0.1:8000".to_owned()),
            new_peer("127.0.0.1:8001".to_owned()),
            new_peer("127.0.0.1:8002".to_owned()),
            new_peer("127.0.0.1:8003".to_owned()),
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
        let full_config_path = format!("{}{}", env::current_dir().unwrap().display(), TEST_CONFIG_FNAME);

        let config_lines = parse_config_lines(full_config_path);

        let socket = parse_listen_socket(config_lines, TEST_CONFIG_INDEX);
        let expected_socker = String::from("127.0.0.1:8000");

        assert_eq!(expected_socker, socket)
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