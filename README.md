# Dolev-Strong protocol

This is my attempt at implementing Dolev-Strong consensus protocol in Rust.

Dolev-Strong is a consensus protocol for a single-shot consensus problem `Byzantine Broadcast`.

## Assumptions

* Permissioned (participants are known prior to run).
* Public Key Infrastructure - PKI (all nodes know other nodes' public keys prior to the run of the protocol).
* Synchronous model (shared global clock, bound on message delays).
* Known bound `f` on the number of Byzantine nodes.

## Byzantine Tolerance

Notation: let `f` be the number of Byzantine nodes that can be tolerated by this protocol (note: `f` is not a % of total number of nodes, but an actual number of Byzantine nodes).

Under said assumptions, `f` can be arbitrarily large. Even if 2 nodes run the protocol honestly, no matter how big `f` is, honest nodes will always stay in sync with one another.

## Running a Protocol

TODO: add instructions on how to run the protocol.

Currently, the protocol is still under development, so it is not possible to run it yet.
Once I finish implementing it, I will add instructions on how to run it.
