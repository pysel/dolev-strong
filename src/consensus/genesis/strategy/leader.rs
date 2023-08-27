use crate::consensus::ConsensusNode;
use super::GenesisStrategy;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode) {
        
    }
}