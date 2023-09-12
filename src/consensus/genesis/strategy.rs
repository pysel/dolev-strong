use crate::consensus::ConsensusNode;

pub mod follower;
pub mod leader;

pub trait GenesisStrategy {
    // genesis_stage is a "zero"'th stage, when a leader proposes a value
    fn genesis_stage(&self, self_node: ConsensusNode);
}
