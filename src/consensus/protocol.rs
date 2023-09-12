use std::net::TcpStream;
use std::io::{Read, Error};
use crate::communication::message::serde::deserealize;
use crate::communication::message::types::consensus::ConsensusMsgReceived;
use crate::communication::peer::Peer;

use self::types::ConsensusMsgReceivedTuple;
use self::utils::current_cons_msg_size;

use super::ConsensusNode;
use super::errors::MessageError;

pub(crate) mod utils;
pub(crate) mod types;

// F is the upper bound on the number of Byzantine nodes this protocol tolerates. Alias for the number of stages required. See SPEC.md for details
// TODO: make dynamic
const F: i64 = 5;

impl ConsensusNode<'_> {
    // enter_stage is used for a node to enter to stage X of consensus
    pub fn enter_stage(mut self, stage: i64) {
        if stage > F {
            panic!("maximum number of stages is {}, trying to enter bigger stage {}", F, stage)
        }

        // wait until the beginning of a stage
        self.swait(stage);

        let pending_messages = self.receive_all_consensus_messages();
        println!("pending messages: {:?}", pending_messages)
    }

    // receive_consensus_message receives consensus message from a peer
    fn receive_consensus_message(&self, peer: Peer) -> Result<ConsensusMsgReceived, MessageError> {
        let mut stream: &TcpStream = self.communication.config.get_listen_tcp_stream(peer)
            .expect(&format!("TcpStream does not exist with Peer {:?}", peer));

        let current_stage = self.synchrony.get_current_stage();
        let current_msg_size = current_cons_msg_size(current_stage);
        let mut buf: Vec<u8> = vec![];

        match stream.read_exact(&mut buf) {
            Err(e) => {
                let e =  Error::new(
                    std::io::ErrorKind::Other, 
                    format!("Error when reading bytes on TCP stream in pk broadcast phase: {}", e)
                );

                return Err(MessageError::ErrReadExact { e })
            }

            _ => {} // ignore ok
        }

        if buf.len() != current_msg_size {
            return Err(MessageError::ErrInvalidMsgSize { size: buf.len() });
        }

        match deserealize(buf) {
            Ok(msg) => {
                if let Some(consensus_msg) = msg.as_any().downcast_ref::<ConsensusMsgReceived>().cloned() {
                    return Ok(consensus_msg)
                } else {
                    return Err(MessageError::ErrWrongBytes("Trying to deserialize not a ConsensusMessageReceived bytes"))
                }

            },
            Err(e) => {
                return Err(MessageError::ErrDeserializing { e })
            },
        }
    }

    // receive_all_consensus_messages tries to receive all consensus messages from all nodes
    fn receive_all_consensus_messages(&self) -> Vec<ConsensusMsgReceivedTuple> {
        let mut result: Vec<ConsensusMsgReceivedTuple> = vec![];
        for peer in self.communication.config.peers() {
            match self.receive_consensus_message(peer) {
                Ok(cmsg) => {
                    result.push(ConsensusMsgReceivedTuple(peer, Some(cmsg)))
                },

                Err(e) => {
                    println!("Log: failed to receive stage {} consensus message from peer {:?} with error {}", self.synchrony.get_current_stage(), peer, e);
                    result.push(ConsensusMsgReceivedTuple(peer, None))
                }
            }
        }
        result
    }
}