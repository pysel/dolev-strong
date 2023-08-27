use crate::node::Node;
use crate::node::Mode::{LEADER, FOLLOWER};

use crate::consensus::genesis::strategy::{follower::FollowerStrategy, leader::LeaderStrategy};

pub mod strategy;

impl Node<'_> {
    pub fn setup_genesis_strategy(&mut self) {
        match self.get_mode() {
            LEADER => {
                self.set_genesis_strategy(&LeaderStrategy);
            }

            FOLLOWER => {
                self.set_genesis_strategy(&FollowerStrategy);
            }
        }
    }
}