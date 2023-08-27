use std::io::{Error, ErrorKind};

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

pub fn bz_to_value(bz: &u8) -> Result<Value, Error> {
    match bz {
        0 => Ok(Zero),
        1 => Ok(One),
        _ => {
            Err(Error::new(ErrorKind::InvalidData, "only 0 or 1 byte can be parsed as Value"))
        }
    }
}