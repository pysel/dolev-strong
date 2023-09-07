use std::io::{ErrorKind, Error};
use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature, PublicKey};
use crate::communication::message::{PubkeyBroadcastMsg, MessageI};
use crate::communication::message::types::pk_broadcast::{MSG_TYPE_PB, PubkeyBroadcastMsgReceived, new_pb_broadcast_result, SignedPkBroadcastBzType};

impl MessageI for PubkeyBroadcastMsg {
    fn serialize(&self, keypair: &Keypair, config_index: i32) -> Vec<u8> {
        let msg_type: &[u8] = MSG_TYPE_PB.as_bytes();
        let pubkey: &[u8; 32] = self.0.as_bytes();
        let config_index_bz: [u8; 4] = config_index.to_be_bytes();
        let mut bz: [u8; 38] = [0; 38];
        bz[..2].copy_from_slice(msg_type);
        bz[2..6].copy_from_slice(&config_index_bz);
        bz[6..].copy_from_slice(pubkey);
    
        let signature: ed25519_dalek::Signature = keypair.sign(&bz);
        let mut signed_bz: SignedPkBroadcastBzType = [0; 102];
        signed_bz[..38].copy_from_slice(&bz);
        signed_bz[38..].copy_from_slice(signature.as_bytes());
        signed_bz.to_vec()
    }
}

use crate::utils::binary::bytes_to_decimal;
pub fn deserealize_pb(bz: SignedPkBroadcastBzType) -> Result<PubkeyBroadcastMsgReceived, Error> {
    let msg_type: &[u8] = &bz[..2]; // DRY-0
    if msg_type != MSG_TYPE_PB.as_bytes() {
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

    Ok(new_pb_broadcast_result(pubkey, config_index, bz, signature))
}