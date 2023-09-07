# Specification

## Definitions

`Δ` - an upper bound on maximum message delays
`f` - an upper bound on faulty processes a protocol can tolerate
`round` - a Δ length period of time when a node tries to find a convincing message and potentially convince others (1)

(1) Note: do not confuse this definition of a round with a usual definition in consensus protocols literature, where it refers to a time preiod, when nodes complete a full path from proposal to committing some value. Here, it is just an intermediary step to eventually commit a value

## How Synchronous Model works

Synchrony is enforced by each process starting with the current system's time. The timestamp of beginning of round 0 is hardcoded in the protocol (5 seconds after launching a process).

Each subsequent rounds' timestamps can be derived from this common knowledge by simple math (since protocol's expected message delay is also known):

$$TimestampRound_X = TimestampRound_0 + X *Δ$$

where Δ is an expected message delay

## Consensus

### Shared Global clock and `f`

Because we are in a synchronous model, all nodes have a notion of shared global clock and an upper bound on message delays.

For every `f` number of Byzantine processes we want to tolerate, we run a "round" of a dolev-strong strong consensus protocol,
when a node tries to get convinced of some value (that will potentially be outputted) and subsequently convince others.

### Convincing message

A node `i` is considered convinced of value `v` at some round `R` if it received such a message that:

- it references value `v`
- it is signed first by a sender
- it is signed by ``>= R` other distinct nodes, none of which are node `i`

## Consensus Rounds

A Dolev-Strong protocol can tolerate as many Byzantine nodes `F` as anyone would want. But for any Byzantine node that the protocol wants to tolerate, it has to go through one more round during consensus.
