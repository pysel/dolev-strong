use ed25519_dalek::{Signature, PublicKey};

use crate::communication::{message::{Value, ReceivedMessageI, ConsensusMsg}, peer::{Peer, sanity::no_duplicate_pubkeys}, pki::is_valid_signature};

pub const MSG_TYPE_CON: &str = "cm";
const PAYLOAD_SIZE: usize = MSG_TYPE_CON.as_bytes().len() + Value::get_serialized_size();

#[derive(Clone, Debug)]
pub struct ConsensusMsgReceived { // TODO: consider merging consensus message with proposal message
    pub proposed_value: Value,
    pub bytes: Vec<u8>,
    pub signatures: Vec<Signature>,
    pub sender_pubkey: Option<PublicKey>,
}

pub fn new_consensus_msg_received(proposed_value: Value, bytes: Vec<u8>, signatures: Vec<Signature>, sender_pubkey: Option<PublicKey>) -> ConsensusMsgReceived {
    ConsensusMsgReceived { proposed_value, bytes, signatures, sender_pubkey }
}

impl ReceivedMessageI for ConsensusMsgReceived {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConsensusMsgReceived {
    // raw_with_x_signatures returns this consensus message in byte form only with x first signatures
    pub fn raw_with_x_signatures(&self, x: i64) -> Vec<u8> {
        let mut payload = self.bytes[..PAYLOAD_SIZE].to_vec();
        let mut signatures = self.bytes[PAYLOAD_SIZE..PAYLOAD_SIZE + Signature::BYTE_SIZE * x as usize].to_vec();
        let mut result = vec![];
        result.append(&mut payload);
        result.append(&mut signatures);
        result
    }

    // check_intermediary_signers asserts that intermediary signatures are valid (all signatures between first and last)
    // it checks that every signature is valid, and that every signature comes from distinct signer
    pub fn check_intermediary_signers(&self, sender_pub: &PublicKey, leader_pub: &PublicKey, peers: &Vec<Peer>) -> bool {
        let sigs_amount = self.signatures.len();
        if sigs_amount < 3 {
            // if there are no other signers other from sender and leader (should be stage 2), return true since it is a valid message
            return true
        }

        // seen_peers is used to avoid a message having multiple signatures created by the same sybil
        let mut seen_pubkeys = vec![sender_pub.clone(), leader_pub.clone()];
        for i in 1..sigs_amount-2 {
            let signature = &self.signatures[i];
            let bytes_signed = &self.raw_with_x_signatures(i.try_into().unwrap());
            if let Some(peer) = signature_belongs_to(signature, bytes_signed, peers) {
                seen_pubkeys.push(peer.pubkey.unwrap());
            }
        }

        // if total peers seen is the amount of signatures in a message and there are no duplicates, a signatures verification passes
        if seen_pubkeys.len() == sigs_amount && no_duplicate_pubkeys(&seen_pubkeys) {
            return true
        }

        false
    }

    // to_consensus_msg converts ConsensusMsgReceived to ConsensusMsg.
    // used when a node finds a convincing message at some stage and wants to notify it's peers.
    // should only be used when a message is convincing
    pub fn to_consensus_msg(&self) -> ConsensusMsg {
        ConsensusMsg { 
            value: self.proposed_value.clone(), 
            signatures: self.signatures.clone(),
        }
    }
}

// signature_belongs_to finds a peer to whom a signature belongs
fn signature_belongs_to<'a>(signature: &Signature, bytes: &Vec<u8>, peers: &'a Vec<Peer>) -> Option<&'a Peer> {
    for peer in peers {
        if is_valid_signature(bytes, &peer.pubkey.expect(&format!("pubkey not set for peer {:?}", peer)), signature) {
            return Some(peer)
        }
    }
    None
}