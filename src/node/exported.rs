use crate::node::Mode;

impl super::Communication {
    pub fn get_mode(&self) -> Mode {
        self.config.mode()
    }
}