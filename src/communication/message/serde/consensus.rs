use std::io::{Error, ErrorKind};

use ed25519_dalek::Signer;

use crate::{communication::message::{MessageI, ConsensusMsg, types::consensus::{MSG_TYPE_CON, ConsensusMsgReceived, new_consensus_msg_received}, Value}, utils::message::bz_to_value};

pub(crate) mod utils;

impl MessageI for ConsensusMsg {
    // serialize serializes the message and signs it with this node's keypair
    fn serialize(&self, keypair: &ed25519_dalek::Keypair, _config_index: i32) -> Vec<u8> {
        let msg_type: &[u8] = MSG_TYPE_CON.as_bytes();
        let proposing_value: u8 = self.value.serialize();
        let mut bz: Vec<u8> = vec![0, 0, 0]; // TODO: think of better design here
        bz[..2].copy_from_slice(msg_type);
        bz[2] = proposing_value;

        let mut known_signatures_bz = utils::serialize_signatures(&self.signatures);
        bz.append(&mut known_signatures_bz);

        let signature: ed25519_dalek::Signature = keypair.sign(&bz);
        let mut signed_bz: Vec<u8> = vec![];
        signed_bz.append(&mut bz);
        signed_bz.append(&mut signature.to_bytes().to_vec());

        signed_bz
    }
}

pub fn deserealize_con(bz: Vec<u8>) -> Result<ConsensusMsgReceived, Error> {
    let msg_type: &[u8] = &bz[..2]; // DRY-0
    if msg_type != MSG_TYPE_CON.as_bytes() {
        return Err(Error::new(ErrorKind::InvalidData, "Received bytes do not correspond to pubkey broadcast message"))
    }

    let proposed_value: Value = match bz_to_value(&bz[2]) {
        Ok(value) => value,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
    };

    let signature_bz = &bz[3..];
    match utils::parse_signatures(signature_bz.to_vec()) {
        Ok(signatures) => {
            return Ok(new_consensus_msg_received(proposed_value, bz, signatures, None))

        },
        Err(e) => {
            return Err(Error::new(ErrorKind::InvalidInput, format!("failed to deserialize bytes to ProposalMsgReceived: {e}")));
        },
    }
}

mod tests{
    #![allow(unused_imports)] // all imports are used, for some reason my IDE doesn't see it

    use ed25519_dalek::Signer;

    use crate::{communication::message::{new_consensus_msg, MessageI}, utils::crypto::gen_keypair};

    #[test]
    fn test_serialize() {
        let keypair = gen_keypair();
        let message = "hello world";
        let signatures = vec![keypair.sign(message.as_bytes())];
        let test_msg = new_consensus_msg(crate::communication::message::Value::One, signatures);
        test_msg.serialize(&keypair, 1);
    }
}
