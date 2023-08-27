use ed25519_dalek::{Signature, PublicKey, Verifier};

pub fn valid_signature(bz: &[u8], pk: PublicKey, signature: Signature) -> bool {
    if let Ok(_) = pk.verify(bz, &signature) {
        return true
    }

    false
}