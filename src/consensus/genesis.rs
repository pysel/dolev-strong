use crate::node::Mode::{LEADER, FOLLOWER};

use crate::consensus::genesis::strategy::{follower::FollowerStrategy, leader::LeaderStrategy};

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
        }
    }
}