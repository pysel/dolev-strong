use super::GenesisStrategy;
use crate::{consensus::{ConsensusNode, protocol::convincing::ConsensusMsgReceivedTuple}, communication::message::ConsensusMsg};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_stage(&self, mut self_node: ConsensusNode) {
        self_node.swait(0); // stage zero: allow leader to send out a value proposal.

        if self_node.self_is_leader { panic!("leader node has follower's strategy") } // sanity check
        
        let received_messages = self_node.receive_all_consensus_messages();
        println!("Received messages during genesis: {:?}", received_messages);

        // since the message is convincing, we can directly unwrap the received message
        let consensus_msg = validate_messages_received_on_proposal(received_messages, &self_node);

        println!("Received convincing proposal, broadcasting: {:?}", consensus_msg);
        self_node.communication.broadcast_message(&consensus_msg);

        self_node.enter_stage(1);
    }
}

// validate_messages_received_on_proposal checks that the messages received on proposal stage are valid and returns the consensus message
// to be broadcasted to peers if the proposal is convincing
fn validate_messages_received_on_proposal(msgs: Vec<ConsensusMsgReceivedTuple>, cnode: &ConsensusNode) -> ConsensusMsg {
    // should only receive one message
    if msgs.len() != 1 {
        // TODO: this is a byzantine behavior, need to find a message that came from a leader, and discard others
        panic!("more than one messages received at proposal stage");
    }

    let received_message: ConsensusMsgReceivedTuple<'_> = msgs[0].clone();
    if !received_message.convincing(&cnode) {
        panic!("received proposal is not convincing") // TODO: Output default value - sender is Byzantine
    }

    // since the message is convincing, we can directly unwrap the received message
    let consensus_msg: crate::communication::message::ConsensusMsg = received_message.1.unwrap().to_consensus_msg();
    consensus_msg
}