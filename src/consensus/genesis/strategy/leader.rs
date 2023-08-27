use super::GenesisStrategy;
use crate::node::Communication;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_step(&self, self_node: &Communication) {
        // TODO

    }
}