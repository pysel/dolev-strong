use crate::node::Node;

pub mod follower;
pub mod leader;

pub trait GenesisStrategy {
    fn genesis_step(&self, self_node: &Node);
}
