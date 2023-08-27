use super::GenesisStrategy;
use crate::node::{sync::wait_delta, Communication};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_step(&self, self_node: &Communication) {
        wait_delta() // allow leader to send out a value proposal

    }
}