use std::process::exit;

use crate::{consensus::ConsensusNode, communication::message::{Value, new_consensus_msg}};

use super::GenesisStrategy;

pub struct NullProposalStrategy;
impl GenesisStrategy for NullProposalStrategy {
    // no proposal is sent by the leader
    fn genesis_stage(&self, mut _self_node: ConsensusNode) { exit(0) }
}

pub struct ConflictingProposalStrategy;
impl GenesisStrategy for ConflictingProposalStrategy {
    fn genesis_stage(&self, self_node: ConsensusNode) {
        if !self_node.self_is_leader { panic!("follower node has leader's strategy") } // sanity check

        // create two different proposal messages to send out to different followers
        let proposal_message_zero: &crate::communication::message::ConsensusMsg = &new_consensus_msg(Value::Zero, vec![]);
        let proposal_message_one: &crate::communication::message::ConsensusMsg = &new_consensus_msg(Value::One, vec![]);

        // send zero proposal to half of nodes, one proposal to the other half of nodes
        for (i, peer) in self_node.communication.config.peers.iter().enumerate() {
            if i % 2 == 0 {
                self_node.communication.send_message(peer, proposal_message_zero);
            } else {
                self_node.communication.send_message(peer, proposal_message_one);
            }
        }

        self_node.enter_stage(1);
    }
}