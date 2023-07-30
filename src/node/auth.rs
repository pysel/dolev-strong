use super::Node;
use crate::message::new_pk_broadcast_msg;

impl Node {
    pub fn broadcast_pubkey(&self) {
        let msg = new_pk_broadcast_msg(self.keypair.public);
        let bz = msg.serialize(&self.keypair);
    }
}