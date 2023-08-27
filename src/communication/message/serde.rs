/*
Serialization format:

2b: msg type

if pubkey broadcast: 
    4b: config_index
    32b: pubkey
    64b: single signature

if consensus message:
    1b: consensus value
    64b...: supporting signatures
*/

pub mod pk_broadcast;
pub mod propose;