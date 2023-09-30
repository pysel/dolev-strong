use super::GenesisStrategy;
use crate::consensus::ConsensusNode;

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_stage(&self, mut self_node: ConsensusNode) {
        self_node.swait(0); // stage zero: allow leader to send out a value proposal.

        if self_node.self_is_leader { panic!("leader node has follower's strategy") } // sanity check
        
        let received_messages = self_node.receive_all_consensus_messages();
        println!("Received messages during genesis: {:?}", received_messages);
        if received_messages.len() != 1 {
            // TODO: this is byzantine behavior, should not panic
            panic!("more than one messages received at proposal stage");
        }
        let received_message = received_messages[0].clone();

        if !received_message.convincing(&self_node) {
            panic!("received proposal is not convincing") // TODO: Output default value - sender is Byzantine
        }

        // since the message is convincing, we can directly unwrap the received message
        let consensus_msg = received_message.1.unwrap().to_consensus_msg();

        println!("Received convincing proposal, broadcasting: {:?}", consensus_msg);
        self_node.communication.broadcast_message(&consensus_msg);

        self_node.enter_stage(1);
    }
}