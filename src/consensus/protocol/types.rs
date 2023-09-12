use crate::communication::{peer::Peer, message::types::consensus::ConsensusMsgReceived};

#[derive(Debug)]
pub struct ConsensusMsgReceivedTuple(pub Peer, pub Option<ConsensusMsgReceived>);