use ed25519_dalek::{Keypair, Signer, ed25519::signature::Signature};

use crate::communication::message::{ProposeMsg, types::propose::{MSG_TYPE_PROP, SignedProposeBzType}};

impl ProposeMsg {
    // unused _config_index: only to implement MessageI
    pub fn serialize(&self, keypair: &Keypair, _config_index: i32) -> SignedProposeBzType {
        let msg_type: &[u8] = MSG_TYPE_PROP;
        let proposing_value: u8 = self.0.serialize();
        let mut bz: [u8; 3] = [0; 3];
        bz[..2].copy_from_slice(msg_type);
        bz[3] = proposing_value;

        let signature = keypair.sign(&bz);
        let mut signed_bz: SignedProposeBzType = [0; 67];
        signed_bz[..3].copy_from_slice(&bz);
        signed_bz[3..67].copy_from_slice(signature.as_bytes());
        signed_bz
    }
}