#![allow(dead_code)]
use std::any::Any;

use ed25519_dalek::{PublicKey, Signature, Keypair};

pub mod serde;
pub mod types;

pub trait MessageI {
    fn serialize(&self, keypair: &Keypair, config_index: i32) -> Vec<u8>;
}

pub trait ReceivedMessageI {
    fn convincing(&self) -> bool; // asserts that provided signature(s) is(are) valid
    fn as_any(&self) -> &dyn Any; // required for downcasting
}

// A binary value all honest nodes must agree on
#[derive(Clone, Debug)]
pub enum Value {
    Zero,
    One,
}

pub struct ConsensusMsg { 
    value: Value,
    signatures: Vec<Signature>, // those signatures, which node already knows (not-included node's yet)
}
pub fn new_consensus_msg(value: Value, signatures: Vec<Signature>) -> ConsensusMsg {
    ConsensusMsg { value, signatures }
}

#[derive(Clone, Debug)]
pub struct ProposeMsg(Value);
pub fn new_propose_msg(value: Value) -> ProposeMsg {
    ProposeMsg(value)
}

pub struct PubkeyBroadcastMsg(PublicKey);
pub fn new_pk_broadcast_msg(pk: PublicKey) -> PubkeyBroadcastMsg {
    PubkeyBroadcastMsg(pk)
}

pub struct EncodedMsg([u8]);
