use std::any::Any;
use ed25519_dalek::{Signature, PublicKey};
use crate::communication::{message::{Value, ReceivedMessageI, ConsensusMsg, new_consensus_msg}, pki::is_valid_signature};

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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ProposalMsgReceived {
    // convert_to_consensus_message converts received proposal to consensus message. Used when an honest node wants to notify peers of convincing proposal
    pub fn convert_to_consensus_message(self) -> ConsensusMsg {
        // sanity check, only needs to be signed by leader 
        if self.signatures.len() != 1 {
            panic!("proposal message received is invalid");
        }

        new_consensus_msg(self.proposed_value, self.signatures)
    }

    pub fn convincing(&self) -> bool { // TODO!: add check that sender is actually a leader
        let bz_to_check: &[u8] = &self.bytes[..3];
        let sender_pubkey: PublicKey = self.sender_pubkey.expect("set sender pubkey before trying to validate message's signature");

        if self.signatures.len() != 1 {
            return false;
        }

        let signature_to_check = &self.signatures[0];
        if !is_valid_signature(&bz_to_check.to_vec(), &sender_pubkey, signature_to_check) {
            return false
        }
        
        true
    }
}