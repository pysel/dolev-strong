
use rand::Rng;

use crate::{consensus::ConsensusNode, communication::message::new_consensus_msg};
use crate::prototypes::dolevstrong::Value;
use super::GenesisStrategy;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_stage(&self, self_node: ConsensusNode) {
        if !self_node.self_is_leader { panic!("follower node has leader's strategy") } // sanity check

        let proposal_value: Value = random_proposal_value();
        let proposal_message = &new_consensus_msg(proposal_value, vec![]);

        self_node.communication.broadcast_message(proposal_message);

        self_node.enter_stage(1);
    }
}

fn random_proposal_value() -> Value {
    let num = rand::thread_rng().gen_bool(0.5);
    if num {
        return Value::One
    }

    Value::Zero
}