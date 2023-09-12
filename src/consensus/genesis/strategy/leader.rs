
use crate::{consensus::ConsensusNode, communication::message::{Value, new_propose_msg}};
use super::GenesisStrategy;
use crate::communication::message::ProposeMsg;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_stage(&self, self_node: ConsensusNode) {
        if !self_node.self_is_leader { panic!("follower node has leader's strategy") } // sanity check

        let proposal_value: Value = random_proposal_value();
        let proposal_message: &ProposeMsg = &new_propose_msg(proposal_value);

        self_node.communication.broadcast_message(proposal_message);

        self_node.enter_stage(1);
    }
}

// TODO: make this actually random
fn random_proposal_value() -> Value {
    Value::Zero
}