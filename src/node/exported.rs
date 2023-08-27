use crate::node::Mode;

impl super::Node<'_> {
    pub fn get_mode(&self) -> Mode {
        self.config.mode()
    }
}