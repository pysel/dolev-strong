use super::GenesisStrategy;
use crate::node::Node;

pub struct LeaderStrategy;

impl GenesisStrategy for LeaderStrategy {
    fn genesis_step(&self, self_node: &Node) {
        // TODO

    }
}