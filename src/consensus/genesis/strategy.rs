use crate::consensus::ConsensusNode;

pub mod follower;
pub mod leader;

pub trait GenesisStrategy {
    fn genesis_step(&self, self_node: &ConsensusNode);
}
