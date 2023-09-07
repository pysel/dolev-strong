# Specification

## Synchronous Model

Synchrony is enforced by each process starting with the current system's time. The timestamp of beginning of round 0 is hardcoded in the protocol (5 seconds after launching a process).

Each subsequent rounds' timestamps can be derived from this common knowledge by simple math (since protocol's expected message delay is also known):

$$TimestampRound_X = TimestampRound_0 + X *Δ$$

where Δ is an expected message delay
