#![allow(dead_code)]
use ed25519_dalek::PublicKey;

pub mod serde;
mod types;

// A binary value all honest nodes must agree on
pub enum Value {
    Zero,
    One,
}
pub struct ConsensusMsg(Value);

pub struct PubkeyBroadcastMsg(PublicKey);
pub fn new_pk_broadcast_msg(pk: PublicKey) -> PubkeyBroadcastMsg {
    PubkeyBroadcastMsg(pk)
}

pub struct EncodedMsg([u8]);
