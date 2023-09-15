use crate::communication::{peer::Peer, message::types::consensus::ConsensusMsgReceived, pki::is_valid_signature};

#[derive(Debug)]
pub struct ConsensusMsgReceivedTuple(pub Peer, pub Option<ConsensusMsgReceived>);

impl ConsensusMsgReceivedTuple {
    fn convincing(&self, s: i64, stage_leader: &Peer, peers: &Vec<Peer>) -> bool {
        let msg = match &self.1 {
            Some(msg) => msg,
            None => return false,
        };

        // convincing prerequisite: number of signatures should be equal to stage number
        if msg.signatures.len() != s.try_into().unwrap() {
            return false
        }

        let bytes_signed_by_stage_leader = &msg.raw_with_x_signatures(1);

        // convincing prerequisite: first signature should come from leader
        if !is_valid_signature(bytes_signed_by_stage_leader, &stage_leader.pubkey.expect("pubkey not set for stage's leader"), &msg.signatures[0]) {
            return false
        }

        // check that last message was signed by a sender
        let last_signature_index: usize = msg.signatures.len() - 1;
        let sender_pubkey = msg.sender_pubkey.expect("no sender pubkey set");
        
        if !is_valid_signature(&msg.bytes, &sender_pubkey, &msg.signatures[last_signature_index]) {
            return false
        }

        let msg_sender = &self.0;
        // intermediate signatures should also be valid
        if !msg.check_intermediary_signers(msg_sender, stage_leader, peers) {
            return false
        }

        true
    }
}