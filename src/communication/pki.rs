use std::{thread::sleep, time::Duration};

// PKI - public key infrastructure (trusted setup assumption)
use super::Communication;
use crate::communication::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

impl Communication {
    pub fn broadcast_pubkey(&self) {
        println!("Broadcasting self pubkey...\n");

        let msg: &PubkeyBroadcastMsg = &new_pk_broadcast_msg(self.keypair.public);
        if let Some(err) = self.broadcast_message(msg) {
            panic!("{}", format!("failed to establish PKI: error: {}", err)) // panics because PKI assumption is not met
        }
    }

    pub fn receive_pubkeys(&mut self) {
        sleep(Duration::from_secs(1));
        
        println!("Receiving pubkeys...\n");
        if let Err(e) = self.config.receive_pubkeys() {
            panic!("{e}");
        }
    }
}