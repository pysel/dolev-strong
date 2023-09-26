use ed25519_dalek::PublicKey;

use crate::communication::Mode;

use super::peer::Peer;

impl super::Communication {
    pub fn get_mode(&self) -> Mode {
        self.config.mode()
    }

    pub fn get_stage_leader(&self) -> Option<Peer> {
        self.config.get_stage_leader()
    }

    pub fn get_stage_leader_pubkey(&self) -> PublicKey {
        match self.get_stage_leader() {
            Some(leader) => leader.pubkey.unwrap(),
            None => self.get_pubkey()
        }
    }
}