use ed25519_dalek::Signature;

use crate::communication::message::{types::consensus::MSG_TYPE_CON, Value};

// current_cons_msg_size returns the number of bytes of an expected consensus message at current stage
pub(crate) fn current_cons_msg_size(s: i64) -> usize {
    let payload = MSG_TYPE_CON.as_bytes().len() + Value::get_serialized_size();
    let signatures_size = Signature::BYTE_SIZE * (s + 1) as usize;

    payload + signatures_size
}