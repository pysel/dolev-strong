use crate::communication::{peer::Peer, message::types::consensus::ConsensusMsgReceived, pki::is_valid_signature};

#[derive(Debug)]
pub struct ConsensusMsgReceivedTuple(pub Peer, pub Option<ConsensusMsgReceived>);

impl ConsensusMsgReceivedTuple {
    fn convincing(&self, s: i64, round_leader: Peer) -> bool {
        let msg = match &self.1 {
            Some(msg) => msg,
            None => return false,
        };

        // this is a requirement for a message to be convincing
        if msg.signatures.len() != s.try_into().unwrap() {
            return false
        }

        let last_signature_index: usize = msg.signatures.len() - 1;
        let sender_pubkey = msg.sender_pubkey.expect("no sender pubkey set");
        
        if !is_valid_signature(&msg.bytes, &sender_pubkey, &msg.signatures[last_signature_index]) {
            return false
        }

        // TODO: add check for all other signatures

        true
    }
}