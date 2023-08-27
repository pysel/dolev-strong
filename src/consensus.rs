use crate::node;

pub mod genesis;

impl node::Node<'_> {
    fn launch(&self) {
        if let Some(strategy) = self.genesis_strategy {
            strategy.genesis_step(self);
        }
    }
}