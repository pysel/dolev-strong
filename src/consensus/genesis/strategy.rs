use crate::consensus::ConsensusNode;

pub mod follower;
pub mod leader;

pub trait GenesisStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode);
}
