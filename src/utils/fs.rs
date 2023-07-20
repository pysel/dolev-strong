use std::fs::read_to_string;
use crate::node::{peer, Mode};

fn lines_to_peers(config_lines: Vec<Vec<String>>) -> Vec<peer::Peer> {
    let mut result: Vec<peer::Peer> = Vec::new();

    for line in config_lines {
        result.push(peer::new_peer(line[0].clone()));
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

fn get_num_peers(config_lines: Vec<Vec<String>>) -> i32 {
    (config_lines.len() - 1).try_into().unwrap()
}

fn get_listen_socket(config_lines: Vec<Vec<String>>, config_index: i32) -> String {
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
pub fn parse_connection_from_config(filename: String, config_index: i32) -> Config {
    let config_lines = parse_splitted_lines(filename);

    let peers = lines_to_peers(config_lines.clone());
    let listen_socket = get_listen_socket(config_lines.clone(), config_index.try_into().unwrap());
    let num_peers = get_num_peers(config_lines.clone());
    let mode = parse_mode(config_lines.clone(), config_index);

    new_connection(mode, num_peers, peers, listen_socket, None)
}