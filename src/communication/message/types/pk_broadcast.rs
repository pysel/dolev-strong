use std::any::Any;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use crate::communication::message::ReceivedMessageI;

pub const MSG_TYPE_PB: &str = "pb";

pub type SignedPkBroadcastBzType = [u8; 102];

#[derive(Debug)]
pub struct PubkeyBroadcastMsgReceived {
    pub pubkey: PublicKey,
    pub peer_index: i32,
    bytes: SignedPkBroadcastBzType,
    signature: Signature
}

impl ReceivedMessageI for PubkeyBroadcastMsgReceived {
    fn convincing(&self) -> bool {
        let bz_to_check: &[u8] = &self.bytes[..38];
        if let Ok(_) = self.pubkey.verify(bz_to_check, &self.signature) {
            return true
        }
        
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn new_pb_broadcast_result(pubkey: PublicKey, peer_index: i32, bytes: SignedPkBroadcastBzType, signature: Signature) -> PubkeyBroadcastMsgReceived {
    PubkeyBroadcastMsgReceived { pubkey, peer_index, bytes, signature }
}