/*
Serialization format:

2b: msg type

if pubkey broadcast: 
    2b: config_index
    32b: pubkey
    64b: single signature

if consensus message:
    1b: consensus value
    64b...: supporting signatures
*/

use std::io::{ErrorKind, Error};

use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature, PublicKey};

use super::PubkeyBroadcastMsg;
use super::types::MSG_TYPE_PB;

impl PubkeyBroadcastMsg {
    pub fn serialize(&self, keypair: &Keypair, config_index: i32) -> [u8; 98] {
        let msg_type: &[u8] = MSG_TYPE_PB;
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

use crate::node::auth::verification::valid_signature;

pub fn deserealize_pb(bz: &Vec<u8>) -> Result<PublicKey, Error> {
    let msg_type: &[u8] = &bz[..2];
    if msg_type != MSG_TYPE_PB {
        return Err(Error::new(ErrorKind::InvalidData, "Received bytes do not correspond to pubkey broadcast message"))
    }

    let pubkey_bz: &[u8] = &bz[2..34];
    let pubkey = PublicKey::from_bytes(pubkey_bz)
        .expect("Failed to parse a pubkey from bytes received");

    let signature_bz: &[u8] = &bz[34..];
    let signature: ed25519_dalek::Signature = <ed25519_dalek::Signature as Signature>::from_bytes(signature_bz)
        .expect("Failed to parse a signature from bytes received");

    let bz_to_check = &bz[..34];
    if !valid_signature(bz_to_check, pubkey, signature) {
        return Err(Error::new(ErrorKind::Other, "Public key is received with a signature made not by the corresponding private key's owner"))
    }

    Ok(pubkey)
}
