use super::GenesisStrategy;
use crate::node::{sync::wait_delta, Node};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_step(&self, self_node: &Node) {
        wait_delta() // allow leader to send out a value proposal

    }
}