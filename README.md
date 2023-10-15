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

There are two ways to execute a protocol: by setting arbitrary values for amount of nodes in the system, and F (configurable)
and by running a default configuration (10 nodes total, tolerating 8 Byzantine nodes).

In either way, each node will write it's output into a file `output.txt` in the root of the project.
It will be recreated for every launch of the protocol.

### Configurable way

To run a protocol, execute:

```bash
make NODES=X F=Y launch
```

where `X` is a number of nodes and `Y` is a number of Byzantine nodes a protocol will be able to tolerate
(in other words, a number of stages a protocol will execute).

Note: available values for `Y`: [0; X-2].

Reason: there should be at least 2 honest nodes in the system.

Note#2: available values for `X`: 3+.

Reason: there should be at least 3 nodes in the consensus instance (otherwise, it's trivial).

### Default way

To run a protocol in default mode, execute:

```bash
    make launch-default
```
