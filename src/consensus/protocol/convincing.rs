use crate::{communication::{peer::Peer, message::types::consensus::ConsensusMsgReceived, pki::is_valid_signature}, consensus::ConsensusNode};

#[derive(Debug)]
pub struct ConsensusMsgReceivedTuple(pub Peer, pub Option<ConsensusMsgReceived>);

impl ConsensusMsgReceivedTuple {
    pub fn convincing(&self, cnode: &ConsensusNode) -> bool {
        let msg = match &self.1 {
            Some(msg) => msg,
            None => return false,
        };

        let cur_stage = cnode.synchrony.get_current_stage();
        let leader_pubkey = match cnode.self_is_leader {
            true => {&cnode.communication.get_pubkey()},
            false => {&cnode.communication.get_stage_leader().unwrap().pubkey.unwrap()},
        };

        // convincing prerequisite: number of signatures should be equal to stage number
        if msg.signatures.len() != cur_stage.try_into().unwrap() {
            return false
        }

        let bytes_signed_by_stage_leader = &msg.raw_with_x_signatures(1);

        // let leader_signature = match self.
        // convincing prerequisite: first signature should come from a leader
        if !is_valid_signature(bytes_signed_by_stage_leader, leader_pubkey, &msg.signatures[0]) {
            return false
        }

        // check that last message was signed by a sender
        let last_signature_index: usize = msg.signatures.len() - 1;
        let sender_pubkey = msg.sender_pubkey.expect("no sender pubkey set");
        
        if !is_valid_signature(&msg.bytes, &sender_pubkey, &msg.signatures[last_signature_index]) {
            return false
        }

        let msg_sender = &self.0;
        let peers = &cnode.communication.config.peers();
        // intermediate signatures should also be valid
        if !msg.check_intermediary_signers(&sender_pubkey, leader_pubkey, peers) {
            return false
        }

        true
    }
}