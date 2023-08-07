/*
Serialization format:

2b: msg type

if pubkey broadcast: 
    32b: pubkey
    64b: single signature

if consensus message:
    1b: consensus value
    64b...: supporting signatures
*/

use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature};

use super::PubkeyBroadcastMsg;

impl PubkeyBroadcastMsg {
    pub fn serialize(&self, keypair: &Keypair) -> [u8; 98] {
        let msg_type: &[u8; 2] = b"pb";
        let pubkey: &[u8; 32] = self.0.as_bytes();
        let mut bz: [u8; 34] = [0; 34];
        bz[..2].copy_from_slice(msg_type);
        bz[2..].copy_from_slice(pubkey);

        let signature = keypair.sign(&bz);
        let mut signed_bz: [u8; 98] = [0; 98];
        signed_bz[..34].copy_from_slice(&bz);
        signed_bz[34..].copy_from_slice(signature.as_bytes());
        signed_bz
    }
}