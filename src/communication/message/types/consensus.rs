use ed25519_dalek::{Signature, PublicKey};

use crate::communication::{message::{Value, ReceivedMessageI}, peer::Peer, pki::is_valid_signature};

pub const MSG_TYPE_CON: &str = "cm";
const PAYLOAD_SIZE: usize = MSG_TYPE_CON.as_bytes().len() + Value::get_serialized_size();

#[derive(Clone, Debug)]
pub struct ConsensusMsgReceived { // TODO: consider merging consensus message with proposal message
    proposed_value: Value,
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
    fn raw_with_x_signatures(&self, x: i64) -> Vec<u8> {
        let mut payload = self.bytes[..PAYLOAD_SIZE].to_vec();
        let mut signatures = self.bytes[PAYLOAD_SIZE..PAYLOAD_SIZE + Signature::BYTE_SIZE * x as usize].to_vec();
        let mut result = vec![];
        result.append(&mut payload);
        result.append(&mut signatures);
        result
    }

    // pub fn signed_by(&self, peer: Peer) -> bool {
    //     let signed_bytes = &self.bytes;
    //     if !is_valid_signature(signed_bytes, peer.pubkey, sig);
    //     unimplemented!()
    // }
}

