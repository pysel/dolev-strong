#![allow(dead_code)]
use ed25519_dalek::{PublicKey, Signature, Keypair};

pub mod serde;
pub mod types;

pub trait MessageI {
    fn serialize(&self, keypair: &Keypair, config_index: i32);
}

// A binary value all honest nodes must agree on
pub enum Value {
    Zero,
    One,
}

pub struct ConsensusMsg { 
    proposed_value: Value,
    supporting_signatures: Vec<Signature>,
}

pub struct ProposeMsg(Value);

pub struct PubkeyBroadcastMsg(PublicKey);
pub fn new_pk_broadcast_msg(pk: PublicKey) -> PubkeyBroadcastMsg {
    PubkeyBroadcastMsg(pk)
}

pub struct EncodedMsg([u8]);
