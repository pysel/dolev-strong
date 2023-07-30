use super::Node;
use crate::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

impl Node {
    pub fn broadcast_pubkey(&self) {
        let msg: PubkeyBroadcastMsg = new_pk_broadcast_msg(self.keypair.public);
        let bz: [u8; 98] = msg.serialize(&self.keypair);

        for peer in self.config.peers() {
            self.send_message(peer, bz.to_vec());
        }
    }

    fn receive_pubkeys(&mut self) {
        for peer in self.config.peers() {
            
        }
    }
}