use std::io::{Error, ErrorKind};

use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature};

use crate::{communication::message::{ProposeMsg, types::propose::{MSG_TYPE_PROP, SignedProposeBzType, ProposalMsgReceived, new_proposal_msg_received}, Value, MessageI}, utils::message::bz_to_value};

impl MessageI for ProposeMsg {
    // unused _config_index: only to implement MessageI
    fn serialize(&self, keypair: &Keypair, _config_index: i32) -> Vec<u8> {
        let msg_type: &[u8] = MSG_TYPE_PROP.as_bytes();
        let proposing_value: u8 = self.0.serialize();
        let mut bz: [u8; 3] = [0; 3];
        bz[..2].copy_from_slice(msg_type);
        bz[2] = proposing_value;

        let signature = keypair.sign(&bz);
        let mut signed_bz: SignedProposeBzType = [0; 67];
        signed_bz[..3].copy_from_slice(&bz);
        signed_bz[3..67].copy_from_slice(signature.as_bytes());
        signed_bz.to_vec()
    }
}

pub fn deserealize_prop(bz: SignedProposeBzType) -> Result<ProposalMsgReceived, Error> {
    let msg_type: &[u8] = &bz[..2]; // DRY-1
    if msg_type != MSG_TYPE_PROP.as_bytes() {
        return Err(Error::new(ErrorKind::InvalidData, "Received bytes do not correspond to pubkey broadcast message"))
    }

    let proposed_value: Value = match bz_to_value(&bz[2]) {
        Ok(value) => value,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
    };

    let signature_bz = &bz[3..];
    let signature: ed25519_dalek::Signature = <ed25519_dalek::Signature as Signature>::from_bytes(signature_bz)
        .expect("Failed to parse a signature from bytes received");

    Ok(new_proposal_msg_received(proposed_value, bz, signature, None))
}