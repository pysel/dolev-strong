// PKI - public key infrastructure (trusted setup assumption)

use super::{Communication, sync::wait_delta};
use crate::communication::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

impl Communication {
    pub fn broadcast_pubkey(&self) {
        println!("Broadcasting self pubkey...\n");

        let msg: PubkeyBroadcastMsg = new_pk_broadcast_msg(self.keypair.public);

        for peer in self.config.peers() {
            // println!("Sending message to {:?}. Communication {}", peer.socket, self.config.config_index());
            if let Some(e) = self.send_message(peer, &msg) {
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