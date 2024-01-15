use std::io::{Error, ErrorKind};

use crate::prototypes::dolevstrong::Value;
use Value::{Zero, One, Default};

impl Value {
    pub fn serialize(&self) -> u8 {
        match *self {
            Zero => 0,
            One => 1,
            Default => panic!("a default value should not be serialized"),
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