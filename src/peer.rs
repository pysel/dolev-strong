use ed25519_dalek::PublicKey;

pub struct Peer {
    ip: String,
    pubkey: PublicKey,
}