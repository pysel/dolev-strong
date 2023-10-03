use std::net::TcpStream;
use std::io::{Read, Error, ErrorKind};
use crate::communication::message::serde::deserealize;
use crate::communication::message::types::consensus::ConsensusMsgReceived;
use crate::communication::peer::Peer;
use crate::consensus::protocol::convincing::validate_convincing_messages;

use self::convincing::ConsensusMsgReceivedTuple;
use self::utils::current_cons_msg_size;
use crate::communication::message::Value;

use super::ConsensusNode;
use super::errors::MessageError;

pub(crate) mod utils;
pub(crate) mod convincing;

// F is the upper bound on the number of Byzantine nodes this protocol tolerates. Alias for the number of stages required. See SPEC.md for details
// TODO: make dynamic
const F: i64 = 1;

impl ConsensusNode<'_> {
    // enter_stage is used for a node to enter to stage X of consensus
    pub fn enter_stage(mut self, stage: i64) {
        println!("Entering stage {}", stage);
        if stage > F {
            panic!("maximum number of stages is {}, trying to enter bigger stage {}", F, stage)
        }

        // wait until the beginning of a stage
        self.swait(stage);
        let pending_messages = &self.receive_all_consensus_messages();
    
        for pending_message in pending_messages {
            // protocol requirement: if a node finds a convincing message, it needs to notify it's peers
            if pending_message.convincing(&self) {
                let convincing_message_rcvd: ConsensusMsgReceived = pending_message.1.clone().unwrap();
                let convincing_message = convincing_message_rcvd.to_consensus_msg();

                // message signing happens during broadcast, no need to explicitly sign here
                println!("broadcasting message: {:?} on stage {}", convincing_message.clone(), stage);
                self.communication.broadcast_message(&convincing_message.clone()); // cloning since we want to use the message later 

                self.convincing_messages.push(convincing_message_rcvd);

                // it is possible that a node receives multiple convincing messages, but we only need one 
                // to broadcast to peers, so we break here
                break;
            }
        }
        
        // check if it is time to halt
        if stage == F {
            let halting_value = validate_convincing_messages(&self.convincing_messages);

            self.halt(halting_value);
            return
        }

        self.enter_stage(stage + 1)
    }

    // receive_consensus_message receives consensus message from a peer
    fn receive_consensus_message(&self, peer: &Peer) -> Result<ConsensusMsgReceived, MessageError> {
        let mut stream: &TcpStream = self.communication.config.get_listen_tcp_stream(peer)
            .expect(&format!("TcpStream does not exist with Peer {:?}", peer));
        let current_stage = self.synchrony.get_current_stage();
        let current_msg_size = current_cons_msg_size(current_stage);
        let mut buf: Vec<u8> = vec![0u8; current_msg_size];
        println!("Reading {} bytes from {:?}", current_msg_size, peer.socket);

        match stream.read_exact(&mut buf) {
            // if timeout, return Err
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                return Err(MessageError::ErrReadExact{ e })
            },

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
                if let Some(mut consensus_msg) = msg.as_any().downcast_ref::<ConsensusMsgReceived>().cloned() {
                    consensus_msg.sender_pubkey = peer.pubkey;
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
    pub fn receive_all_consensus_messages(&self) -> Vec<ConsensusMsgReceivedTuple> {
        let mut result: Vec<ConsensusMsgReceivedTuple> = vec![];
        for peer in &self.communication.config.peers {
            match self.receive_consensus_message(&peer) {
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

    // halt stops the node and returns a final decision
    fn halt(self, decision: Value) {
        println!("Halting a node with value: {}", decision);
    }
}