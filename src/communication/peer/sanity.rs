use super::Peer;
use std::collections::HashSet;


// no_duplicate_peers checks if peers have duplicate peers (used during consensus message validation)
pub fn no_duplicate_peers(peers: &Vec<&Peer>) -> bool {
    let mut seen_peers = HashSet::new();

    for peer in peers {
        if seen_peers.contains(&peer.socket) { // hash by socket
            return false
        }

        seen_peers.insert(&peer.socket);
    }

    true
}