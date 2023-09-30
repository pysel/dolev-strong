#![allow(dead_code)]
use std::{any::Any, fmt::{Display, self}};

use ed25519_dalek::{PublicKey, Signature, Keypair};

pub mod serde;
pub mod types;

pub trait MessageI {
    fn serialize(&self, keypair: &Keypair, config_index: i32) -> Vec<u8>;
}

pub trait ReceivedMessageI {
    fn as_any(&self) -> &dyn Any; // required for downcasting
}

// A binary value all honest nodes must agree on
#[derive(Clone, Debug)]
pub enum Value {
    Zero,
    One,
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Zero => write!(f, "0"),
            Value::One => write!(f, "1"),
        }
    }
}

impl Value {
    pub const fn get_serialized_size() -> usize {
        1 // since it is either 1 or 0
    }
}

#[derive(Clone, Debug)]
pub struct ConsensusMsg { 
    pub value: Value,
    signatures: Vec<Signature>, // those signatures, which node already knows (not-included node's yet)
}
pub fn new_consensus_msg(value: Value, signatures: Vec<Signature>) -> ConsensusMsg {
    ConsensusMsg { value, signatures }
}

pub struct PubkeyBroadcastMsg(PublicKey);
pub fn new_pk_broadcast_msg(pk: PublicKey) -> PubkeyBroadcastMsg {
    PubkeyBroadcastMsg(pk)
}

pub struct EncodedMsg([u8]);
