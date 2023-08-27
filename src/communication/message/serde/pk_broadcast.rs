use std::io::{ErrorKind, Error};

use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature, PublicKey};

use crate::communication::message::PubkeyBroadcastMsg;
use crate::communication::message::types::pk_broadcast::{MSG_TYPE_PB, PbBroadcastResult, new_pb_broadcast_result, SignedPkBroadcastBzType};

impl PubkeyBroadcastMsg {
    pub fn serialize(&self, keypair: &Keypair, config_index: i32) -> SignedPkBroadcastBzType {
        let msg_type: &[u8] = MSG_TYPE_PB;
        let pubkey: &[u8; 32] = self.0.as_bytes();
        let config_index_bz: [u8; 4] = config_index.to_be_bytes();
        let mut bz: [u8; 38] = [0; 38];
        bz[..2].copy_from_slice(msg_type);
        bz[2..6].copy_from_slice(&config_index_bz);
        bz[6..].copy_from_slice(pubkey);
    
        let signature = keypair.sign(&bz);
        let mut signed_bz: SignedPkBroadcastBzType = [0; 102];
        signed_bz[..38].copy_from_slice(&bz);
        signed_bz[38..].copy_from_slice(signature.as_bytes());
        signed_bz
    }
}

use crate::communication::auth::verification::valid_signature;
use crate::utils::binary::bytes_to_decimal;
pub fn deserealize_pb(bz: SignedPkBroadcastBzType) -> Result<PbBroadcastResult, Error> {
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