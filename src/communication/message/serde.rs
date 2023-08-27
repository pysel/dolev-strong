/*
Serialization format:

2b: msg type

if pubkey broadcast: 
    4b: config_index
    32b: pubkey
    64b: single signature

if consensus message:
    1b: consensus value
    64b...: supporting signatures
*/

use std::io::{ErrorKind, Error};

use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature, PublicKey};

use super::PubkeyBroadcastMsg;
use super::types::{MSG_TYPE_PB, PbBroadcastResult, new_pb_broadcast_result, SignedPkBroadcastType};

impl PubkeyBroadcastMsg {
    pub fn serialize(&self, keypair: &Keypair, config_index: i32) -> [u8; 102] {
        let msg_type: &[u8] = MSG_TYPE_PB;
        let pubkey: &[u8; 32] = self.0.as_bytes();
        let config_index_bz: [u8; 4] = config_index.to_be_bytes();
        let mut bz: [u8; 38] = [0; 38];
        bz[..2].copy_from_slice(msg_type);
        bz[2..6].copy_from_slice(&config_index_bz);
        bz[6..].copy_from_slice(pubkey);
    
        let signature = keypair.sign(&bz);
        let mut signed_bz: SignedPkBroadcastType = [0; 102];
        signed_bz[..38].copy_from_slice(&bz);
        signed_bz[38..].copy_from_slice(signature.as_bytes());
        signed_bz
    }
}

use crate::communication::auth::verification::valid_signature;
use crate::utils::binary::bytes_to_decimal;
pub fn deserealize_pb(bz: SignedPkBroadcastType) -> Result<PbBroadcastResult, Error> {
    let msg_type: &[u8] = &bz[..2];
    if msg_type != MSG_TYPE_PB {
        return Err(Error::new(ErrorKind::InvalidData, "Received bytes do not correspond to pubkey broadcast message"))
    }

    let config_index_bz: &[u8] = &bz[2..6];
    let config_index: i32 = bytes_to_decimal(config_index_bz.to_vec());

    let pubkey_bz: &[u8] = &bz[6..38];
    let pubkey = PublicKey::from_bytes(pubkey_bz)
        .expect("Failed to parse a pubkey from bytes received");

    let signature_bz: &[u8] = &bz[38..];
    let signature: ed25519_dalek::Signature = <ed25519_dalek::Signature as Signature>::from_bytes(signature_bz)
        .expect("Failed to parse a signature from bytes received");

    let bz_to_check = &bz[..38];
    if !valid_signature(bz_to_check, pubkey, signature) {
        return Err(Error::new(ErrorKind::Other, "Public key is received with a signature made not by the corresponding private key's owner"))
    }

    Ok(new_pb_broadcast_result(pubkey, config_index))
}
