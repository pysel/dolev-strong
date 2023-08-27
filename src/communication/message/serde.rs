/*
Serialization format:

2b: msg type

if pubkey broadcast: 
    4b: config_index
    32b: pubkey
    64b: single signature

if consensus message: (special case - proposal message - single signature)
    1b: consensus value
    64b...: supporting signatures
*/
use std::{str, io::{Error, ErrorKind}};
use crate::communication::message::{types::{pk_broadcast::MSG_TYPE_PB, propose::{MSG_TYPE_PROP, ProposalMsgReceived}}, serde::pk_broadcast::deserealize_pb};
use crate::communication::message::types::pk_broadcast::PubkeyBroadcastMsgReceived;

use self::propose::deserealize_prop;

use super::ReceivedMessageI;

pub mod pk_broadcast;
pub mod propose;

pub fn deserealize(bz: Vec<u8>) -> Result<Box<dyn ReceivedMessageI>, Error> {
    let message_type: &str = str::from_utf8(&bz[..2]).expect("Provided bytes have invalid message type");
    let result: Result<Box<dyn ReceivedMessageI>, Error> = match message_type {
        // Tries to deserealize as pubkey broadcast message
        MSG_TYPE_PB => {
            let result: Result<PubkeyBroadcastMsgReceived, Error> = deserealize_pb(
                bz.try_into().expect("trying to deserealize a pubkey broadcast message of invalid format")
            );

            match result {
                Ok(msg) => {
                    Ok(Box::new(msg))
                }

                Err(e) => {
                    Err(Error::new(ErrorKind::InvalidInput, format!("failed to deserealize a message with error: {}", e))) // DRY-2: export this error
                }
            }
        }

        MSG_TYPE_PROP => {
            let result: Result<ProposalMsgReceived, Error> = deserealize_prop(
                bz.try_into().expect("trying to deserealize proposal message of invalid format")
            );

            match result {
                Ok(msg) => {
                    Ok(Box::new(msg))
                }

                Err(e) => {
                    Err(Error::new(ErrorKind::InvalidInput, format!("failed to deserealize a message with error: {}", e))) // DRY-2: export this error
                }
            }
        }

        _ => {
            // TODO
            unimplemented!()
        }
    };
    result
}