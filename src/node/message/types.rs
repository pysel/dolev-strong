use ed25519_dalek::PublicKey;

pub const MSG_TYPE_PB: &[u8] = "pb".as_bytes();

pub struct PbBroadcastResult {
    pub pubkey: PublicKey,
    pub peer_index: i32,
}

pub fn new_pb_broadcast_result(pubkey: PublicKey, peer_index: i32) -> PbBroadcastResult {
    PbBroadcastResult { pubkey, peer_index }
}