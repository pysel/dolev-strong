use crate::communication::{LeaderByzantine, self};
use crate::communication::Mode::{LEADER, FOLLOWER};
use crate::consensus::genesis::strategy::{follower::FollowerStrategy, leader::LeaderStrategy};

use self::strategy::byzantine_leader::NullProposalStrategy;

use super::ConsensusNode;

pub mod strategy;

impl ConsensusNode<'_> {
    pub fn setup_genesis_strategy(&mut self) {
        match self.communication.get_mode() {
            LEADER => {
                self.set_genesis_strategy(&LeaderStrategy);
            }

            FOLLOWER => {
                self.set_genesis_strategy(&FollowerStrategy);
            }

            communication::Mode::ByzantineLeader(LeaderByzantine::NULLPROPOSAL) => {
                self.set_genesis_strategy(&NullProposalStrategy)
            }
        }
    }
}