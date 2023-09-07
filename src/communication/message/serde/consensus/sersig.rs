// serialize signatures

use ed25519_dalek::Signature;

pub fn serialize_signatures(sigs: &Vec<Signature>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    for sig in sigs {
        let mut bz = sig.to_bytes().to_vec();
        buf.append(&mut bz)
    }

    buf
}