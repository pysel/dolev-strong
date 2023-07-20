use std::fs::read_to_string;
use crate::node::{peer, Mode};

pub fn lines_to_peers(filename: String) -> Vec<peer::Peer> {
    let mut result: Vec<peer::Peer> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(peer::new_peer(line.to_owned()));
    }

    result 
}

fn parse_splitted_lines(filename: String) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_owned().split(" ").into_iter().map(|x| x.to_owned()).collect())
    }

    result    
}

pub fn get_num_peers(filename: String) -> i32 {
    (parse_splitted_lines(filename).len() - 1).try_into().unwrap()
}

pub fn get_listen_socket(filename: String, config_index: i32) -> String {
    let config_lines = parse_splitted_lines(filename);

    config_lines[config_index as usize][0].clone()
}

pub fn parse_mode(filename: String, config_index: i32) -> Mode {
    let config_lines = parse_splitted_lines(filename);
    let mode: &str = &config_lines[config_index as usize][1].clone();
    match mode {
        "leader" => Mode::LEADER,
        "follower" => Mode::FOLLOWER,
        _ => panic!("Invalid mode {mode}, should be either `follower` or `leader`")
    }
}