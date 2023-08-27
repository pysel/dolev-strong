use crate::communication::message::Value;
use Value::{Zero, One};

impl Value {
    pub fn serialize(&self) -> u8 {
        match *self {
            Zero => 0,
            One => 1,
        }
    }
}