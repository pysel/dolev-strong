# Dolev-Strong protocol

This is my attempt at implementing Dolev-Strong consensus protocol in Rust.

Dolev-Strong is a consensus protocol for a single-shot consensus problem, namely `Byzantine Broadcast`.

It works under the following assumptions:

* Permissioned (participants are known prior to run)
* Public Key Infrastructure (all nodes know other nodes' public keys prior to the run of the protocol)
* Synchronous model (shared global clock, bound on message delays)
* Known bound `f` on the number of Byzantine nodes.

## How big `f` can be?

In the said assumptions, for `f` can be arbitrarily large. Even if 2 nodes run the protocol honestly, no matter how big `f` is, honest nodes will stay in sync with one another.
