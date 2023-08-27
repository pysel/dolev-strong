use super::{Communication, sync::wait_delta};
use crate::node::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

pub mod verification;

impl Communication {
    pub fn broadcast_pubkey(&self) {
        println!("Broadcasting self pubkey...\n");

        let msg: PubkeyBroadcastMsg = new_pk_broadcast_msg(self.keypair.public);
        let bz: [u8; 102] = msg.serialize(&self.keypair, self.config.config_index());

        for peer in self.config.peers() {
            // println!("Sending message to {:?}. Communication {}", peer.socket, self.config.config_index());
            if let Some(e) = self.send_message(peer, bz.to_vec()) {
                panic!("Failed to send message to peer {:?} with error {}", peer.socket, e)
            }
        }

        wait_delta() // wait before proceeding to make sure all messages were received 
    }

    pub fn receive_pubkeys(&mut self) {
        println!("Receiving pubkeys...\n");
        if let Err(e) = self.config.receive_pubkeys() {
            panic!("{e}");
        }

        wait_delta() // TODO: maybe not needed
    }
}