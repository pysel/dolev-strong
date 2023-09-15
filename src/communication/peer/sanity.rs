use ed25519_dalek::PublicKey;

use super::Peer;
use std::collections::HashSet;


// no_duplicate_peers checks if peers have duplicate peers (used during consensus message validation)
pub fn no_duplicate_pubkeys(pubkeys: &Vec<PublicKey>) -> bool {
    let mut seen_pks = HashSet::new();

    for pk in pubkeys {
        if seen_pks.contains(pk.as_bytes()) { // hash by socket
            return false
        }

        seen_pks.insert(pk.as_bytes());
    }

    true
}