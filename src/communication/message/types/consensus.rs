use ed25519_dalek::{Signature, PublicKey};

use crate::communication::message::{Value, ReceivedMessageI};

pub const MSG_TYPE_CON: &str = "cm";

#[derive(Clone, Debug)]
pub struct ConsensusMsgReceived { // TODO: consider merging consensus message with proposal message
    proposed_value: Value,
    bytes: Vec<u8>,
    signatures: Vec<Signature>,
    pub sender_pubkey: Option<PublicKey>,
}

pub fn new_consensus_msg_received(proposed_value: Value, bytes: Vec<u8>, signatures: Vec<Signature>, sender_pubkey: Option<PublicKey>) -> ConsensusMsgReceived {
    ConsensusMsgReceived { proposed_value, bytes, signatures, sender_pubkey }
}

impl ReceivedMessageI for ConsensusMsgReceived {
    fn convincing(&self) -> bool {
        true    // add logic 
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

