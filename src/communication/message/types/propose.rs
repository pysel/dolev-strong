use std::any::Any;

use ed25519_dalek::{Signature, PublicKey, Verifier};

use crate::communication::message::{Value, ReceivedMessageI};

pub const MSG_TYPE_PROP: &str = "pr";

pub type SignedProposeBzType = [u8; 67];

pub struct ProposalMsgReceived {
    proposed_value: Value,
    bytes: SignedProposeBzType,
    signature: Signature,
    sender_pubkey: Option<PublicKey>,
}

pub fn new_proposal_msg_received(
    proposed_value: Value,
    bytes: SignedProposeBzType, 
    signature: Signature, 
    sender_pubkey: Option<PublicKey>
) -> ProposalMsgReceived {
    ProposalMsgReceived { proposed_value, bytes, signature, sender_pubkey }
}

impl ReceivedMessageI for ProposalMsgReceived {
    fn valid_signatures(&self) -> bool {
        let bz_to_check = &self.bytes[..3];
        let sender_pubkey = self.sender_pubkey.expect("set sender pubkey before trying to validate message's signature");
        if let Ok(_) = sender_pubkey.verify(bz_to_check, &self.signature) {
            return true
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
