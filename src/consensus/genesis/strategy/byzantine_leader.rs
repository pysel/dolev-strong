use std::process::exit;

use crate::consensus::ConsensusNode;

use super::GenesisStrategy;

pub struct NullProposalStrategy;

impl GenesisStrategy for NullProposalStrategy {
    // no proposal is sent by the leader
    fn genesis_stage(&self, mut _self_node: ConsensusNode) { exit(0) }
}