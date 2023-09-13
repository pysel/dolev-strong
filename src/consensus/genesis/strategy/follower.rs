use super::GenesisStrategy;
use crate::communication::message::{ReceivedMessageI, ConsensusMsg};
use crate::consensus::ConsensusNode;

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_stage(&self, mut self_node: ConsensusNode) {
        self_node.swait(0); // stage zero: allow leader to send out a value proposal.

        if self_node.self_is_leader { panic!("leader node has follower's strategy") } // sanity check
        
        let received_message = match self_node.communication.receive_proposal(self_node.stage_leader.unwrap()) {
            Ok(msg) => msg,
            Err(e) => panic!("failed to receive proposal message with error: {e}"), // TODO: Default Value
        };

        if !received_message.convincing() {
            panic!("received proposal is not convincing") // TODO: Output default value - sender is Byzantine
        }

        let consensus_message: ConsensusMsg = received_message.convert_to_consensus_message();

        println!("Received convincing proposal, broadcasting: {:?}", consensus_message);
        self_node.communication.broadcast_message(&consensus_message);
        
        self_node.enter_stage(1);
    }
}