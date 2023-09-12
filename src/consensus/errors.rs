use std::io::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Error reading bytes")]
    ErrReadExact {e: Error},

    #[error("Invalid message size")]
    ErrInvalidMsgSize {size: usize},

    #[error("Error Deserializing")]
    ErrDeserializing {e: Error},

    #[error("Deserializing wrong Bytes")]
    ErrWrongBytes(&'static str),
}