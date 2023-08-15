use super::Node;
use crate::node::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

pub mod verification;

impl Node {
    pub fn broadcast_pubkey(&self) {
        let msg: PubkeyBroadcastMsg = new_pk_broadcast_msg(self.keypair.public);
        let bz: [u8; 102] = msg.serialize(&self.keypair, self.config.config_index());

        for peer in self.config.peers() {
            self.send_message(peer, bz.to_vec());
        }
    }

    fn receive_pubkeys(&mut self) {
        for s_index in 0..self.config.peers().len()  {
            self.config.read_pubkey_from_stream(s_index);
        }
    }
}