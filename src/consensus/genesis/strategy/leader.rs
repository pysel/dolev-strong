
use crate::{consensus::ConsensusNode, communication::message::{Value, new_propose_msg}};
use super::GenesisStrategy;
use crate::communication::message::ProposeMsg;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode) {
        let proposal_value: Value = random_proposal_value();
        let proposal_message: &ProposeMsg = &new_propose_msg(proposal_value);

        self_node.communication.broadcast_message(proposal_message);
    }
}

// TODO: make this actually random
fn random_proposal_value() -> Value {
    Value::Zero
}