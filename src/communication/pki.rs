// PKI - public key infrastructure (trusted setup assumption)

use super::Communication;
use crate::communication::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};
use crate::consensus::sync::wait_delta;


impl Communication {
    pub fn broadcast_pubkey(&self) {
        println!("Broadcasting self pubkey...\n");

        let msg: &PubkeyBroadcastMsg = &new_pk_broadcast_msg(self.keypair.public);
        if let Some(err) = self.broadcast_message(msg) {
            panic!("{}", format!("failed to establish PKI: error: {}", err)) // panics because PKI assumption is not met
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