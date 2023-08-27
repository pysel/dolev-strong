
use crate::{consensus::ConsensusNode, communication::message::{Value, new_propose_msg}};
use super::GenesisStrategy;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode) {
        let proposal_value: Value = random_proposal_value();
        let proposal_message = &new_propose_msg(proposal_value);
    }
}

// TODO: make this actually random
fn random_proposal_value() -> Value {
    Value::Zero
}