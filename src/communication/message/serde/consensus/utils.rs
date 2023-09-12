// serialize signatures

use std::io::{ErrorKind, Error};

use ed25519_dalek::Signature;
use ed25519_dalek::ed25519::signature::Signature as SignatureTrait;

// serialize_signatures takes as an input a vector of Signature objects and returns serialized array of bytes of these signatures
pub fn serialize_signatures(sigs: &Vec<Signature>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    for sig in sigs {
        let mut bz = sig.to_bytes().to_vec();
        buf.append(&mut bz)
    }

    buf
}

// validate_consensus_bz validates the length of consensus message (3 for payload + x * 64 for signatures, where x is a stage number)
pub fn validate_consensus_bz(bz: Vec<u8>) -> bool {
    // if no payload (msg_type and proposed value), immediately discard
    if bz.len() < 3 {
        return false
    }
    
    if bz[3..].len() % Signature::BYTE_SIZE != 0 {
        return false
    }
    
    true
}

pub fn parse_signatures(bz: Vec<u8>) -> Result<Vec<Signature>, Error> {
    if bz.len() % Signature::BYTE_SIZE != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "bytes are of invalid form cannot be split into multiple signatures"))
    }

    let signatures_bz = bz.chunks(Signature::BYTE_SIZE);
    let mut result: Vec<Signature> = vec![];
    for signature_bz in signatures_bz {
        match <ed25519_dalek::Signature as SignatureTrait>::from_bytes(signature_bz) {
            Ok(signature) => {
                result.push(signature)
            }
            Err(e) => {
                return Err(Error::new(ErrorKind::InvalidInput, e));
            }
        }
    }

    Ok(result)
}