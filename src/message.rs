#![allow(dead_code)]

use ed25519_dalek::Signature;

// A binary value all honest nodes must agree on
pub enum Value {
    Zero,
    One,
}

pub struct Message {
    value: Value,
    signatures: Vec<Signature>
}