use std::{io::{Error, Read}, net::TcpStream};

use crate::communication::{Communication, message::{types::propose::{ProposalMsgReceived, SignedProposeBzType}, serde::deserealize}, peer::Peer};

impl Communication {
    pub fn receive_proposal(&self, leader: Peer) -> Result<ProposalMsgReceived, Error> {
        let mut stream: &TcpStream = self.config.get_listen_tcp_stream(leader)
            .expect(&format!("TcpStream does not exist with leader {:?}", leader));
        let mut buf: SignedProposeBzType = [0; 67];

        match stream.read_exact(&mut buf) { // DRY-3
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other, 
                    format!("Error when reading bytes on TCP stream in proposal phase: {}", e)
                ));
            }

            _ => {} // ignore ok
        }
        
        match deserealize(buf.to_vec()) {
            Ok(msg) => {
                if let Some(result) = msg.as_any().downcast_ref::<ProposalMsgReceived>() {
                    let mut owned_message = result.clone();
                    owned_message.sender_pubkey = leader.pubkey; // set sender_pubkey of proposal message
                    return Ok(owned_message)
                } else {
                    return Err(Error::new(std::io::ErrorKind::InvalidInput, "Trying to deserialize not a ProposalMsgReceived bytes"))
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}