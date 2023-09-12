use std::any::Any;
use ed25519_dalek::{Signature, PublicKey, Verifier};
use crate::communication::message::{Value, ReceivedMessageI};

pub const MSG_TYPE_PROP: &str = "pr";

pub type SignedProposeBzType = [u8; 67];

#[derive(Clone, Debug)]
pub struct ProposalMsgReceived {
    proposed_value: Value,
    bytes: SignedProposeBzType,
    signatures: Vec<Signature>,
    pub sender_pubkey: Option<PublicKey>,
}

pub fn new_proposal_msg_received(
    proposed_value: Value,
    bytes: SignedProposeBzType, 
    signatures: Vec<Signature>, 
    sender_pubkey: Option<PublicKey>
) -> ProposalMsgReceived {
    ProposalMsgReceived { proposed_value, bytes, signatures, sender_pubkey }
}

impl ReceivedMessageI for ProposalMsgReceived {
    fn convincing(&self) -> bool { // TODO!: add check that sender is actually a leader
        let bz_to_check: &[u8] = &self.bytes[..3];
        let sender_pubkey: PublicKey = self.sender_pubkey.expect("set sender pubkey before trying to validate message's signature");

        if self.signatures.len() != 1 {
            return false;
        }

        let signature_to_check = &self.signatures[0];
        if let Ok(_) = sender_pubkey.verify(bz_to_check, signature_to_check) {
            return true
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
