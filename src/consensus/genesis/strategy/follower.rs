use super::GenesisStrategy;
use crate::communication::message::ReceivedMessageI;
use crate::{consensus::sync::wait_delta, consensus::ConsensusNode};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode) {
        wait_delta(); // Round zero: allow leader to send out a value proposal.

        if self_node.self_is_leader { panic!("leader node has follower's strategy") } // sanity check
        
        let received_message = match self_node.communication.receive_proposal(self_node.round_leader.unwrap()) {
            Ok(msg) => msg,
            Err(e) => panic!("failed to receive proposal message with error: {e}"), // TODO: Default Value
        };

        if !received_message.convincing() {
            panic!("received proposal is not convincing") // TODO: Default Value
        }
    }
}