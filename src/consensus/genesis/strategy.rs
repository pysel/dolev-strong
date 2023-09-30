use crate::consensus::ConsensusNode;

pub mod follower;
pub mod leader;

pub trait GenesisStrategy {
    // genesis_stage is the logic for the genesis stage of the consensus protocol taken by a node depending on it's mode.
    fn genesis_stage(&self, self_node: ConsensusNode);
}
