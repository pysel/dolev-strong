use super::GenesisStrategy;
use crate::{communication::sync::wait_delta, consensus::ConsensusNode};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_step(&self, self_node: &ConsensusNode) {
        wait_delta() // allow leader to send out a value proposal

    }
}