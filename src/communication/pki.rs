use std::{thread::sleep, time::Duration};

use ed25519_dalek::{PublicKey, Signature, Verifier};

// PKI - public key infrastructure (trusted setup assumption)
use super::Communication;
use crate::communication::message::{new_pk_broadcast_msg, PubkeyBroadcastMsg};

impl Communication {
    pub fn broadcast_pubkey(&self) {
        // println!("Broadcasting self pubkey...\n");

        let msg: &PubkeyBroadcastMsg = &new_pk_broadcast_msg(self.keypair.public);
        if let Some(err) = self.broadcast_message(msg) {
            panic!("{}", format!("failed to establish PKI: error: {}", err)) // panics because PKI assumption is not met
        }

        // sleep to make sure other processes have time to broadcast their pubkeys
        sleep(Duration::from_secs(3));
    }

    pub fn receive_pubkeys(&mut self) {
        // println!("Receiving pubkeys...\n");
        if let Err(e) = self.config.receive_pubkeys() {
            panic!("{e}");
        }
    }
}

pub fn is_valid_signature(bz: &Vec<u8>, pubkey: &PublicKey, sig: &Signature) -> bool {
    if let Ok(_) = pubkey.verify(bz, sig) {
        return true
    }
    false
}