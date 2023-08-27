use crate::communication::Mode;

use super::peer::Peer;

impl super::Communication {
    pub fn get_mode(&self) -> Mode {
        self.config.mode()
    }

    pub fn get_round_leader(&self) -> Option<Peer> {
        self.config.get_round_leader()
    }
}