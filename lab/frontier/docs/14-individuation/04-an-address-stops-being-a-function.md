# Observation 4 — An address that says when is not a function of the entity it addresses

Twenty-five call sites stopped compiling, and the useful thing is not the number. It is that every one
of them is the same expression:

```rust
EntryId::of(commitment)    // the address of this commitment
EntryId::of(*role)         // the address of this role
EntryId::of(head)          // the address of the Event that is the chain's head
EntryId::of(repeated)      // the address of the entry that repeats
```

Not one of them is fabricating a placeholder. Every one is **deriving an address from an identity it
already holds**, which is the operation content-addressing makes free: given an entity, you know where
it is in the journal, and you need nothing else to know it.

An address that says *when* is not that. A `CommitmentId` does not know the day it was recorded on. So
the map from identity to address stops being a function and becomes a lookup.

## And for seven of the nine kinds, the port does not hand it over

This is where it stops being an inconvenience — and the first version of this section got it wrong in a
way worth keeping, because the wrong version is the one that reads as more damning.

**What was written first:** *the recording instant of seven families is not in canonical knowledge at
all.* That was read off the trait, and the trait is not where the data is.

**What is there.** The reference adapter's shelf holds `Canonical<T>` — the envelope, instant and all —
for **all nine** families. Nothing is discarded on the way in: `put_role` takes a `Canonical<Role>` and
stores it whole.

```text
stored           Canonical<T> for all nine, in the adapter

Knowledge        role, agent, resource, resource_instance, action, statement, commitment,
                 event — every one of them ends in `.assertion().clone()`, which unwraps
                 the envelope and drops the instant AT THE READ

CanonicalKnowledge   canonical_commitment, canonical_event — the envelope, for two of nine
```

So the instant is held and not offered. The map from identity to address is not impossible; it is
**unreachable through the port**, for seven of the nine kinds, and reachable for the two the port was
widened for already.

## Which makes the cost a port obligation rather than a missing getter

That reads like a smaller finding and it is a different one, so it is worth being exact about what
widening `CanonicalKnowledge` by seven methods would actually cost.

The conformance suite has `canonical_reads_expose_the_stored_record`, and it covers a Commitment and an
Event and nothing else. So **today an adapter is free not to keep** the recording instant of a Role, an
Agent, a Resource, a ResourceInstance, an Action, a Statement or an Eligibility. The reference adapter
keeps them; the port does not require it, and a durable adapter that dropped them would conform.

Widening the port turns that permission into an **obligation for every adapter, present and future** —
which is a change to what the engine promises rather than to what one implementation happens to do. It
is the engine either way, and this experiment's boundary excludes it in as many words; what the
correction changes is the size, not the side of the line.

The alternative is to carry the journal to every place an address is needed, which is the same thing as
saying an address is no longer a name.

## How the first reading happened, since the method is the point

The trait was read and the conclusion was drawn about the data. `Knowledge::role` returns `Role`, so the
instant "is not there" — except that what a port returns says what it *offers*, never what it *holds*,
and the adapter was two files away. That is the hazard named in this laboratory's own rule about
diagnosis: a measurement taken with an unchecked instrument is a false conclusion wearing the clothes of
evidence, and this one nearly went into a result document as a fact.

It changes no verdict. The trade condition and the published-result condition are untouched, and both
are independently sufficient. It changes what the third condition costs, and anybody reading this to
decide whether to try again needs the corrected number rather than the frightening one.

## The honest counter-reading, and why it is not the one taken

**It could be read as the laboratory's cost.** Every broken site is in a test suite. The application
never calls `EntryId::of` outside `admit`, where the instant is already in hand, and every test the
application has of its own stayed green. On that reading, twenty-five sites is what it costs to
re-instrument five concluded experiments, and the product pays nothing.

That reading is available and it is not the one this takes, for a reason that is on the queue rather
than in this experiment:

> **What those suites are doing is what the unbuilt capability does.** Experiment 10's dependence
> closure walks from a world's selection to the addresses it reaches — `EntryId::of(commitment)`, for
> every commitment in the selection, and then the Event chain. That is the instrument the queue's
> *recording dependence* item was measured with, and experiment 12's partial meeting needs the same
> walk. An application that ever answers *which entries is this world a function of* is doing exactly
> what those twenty-five lines do.

So the cost is not that tests broke. It is that a capability the queue holds becomes un-writable
without either the journal in hand or a wider engine — and neither was on the table.

## What this says about the prediction set

Five predictions, all five confirmed, and the answer turns on a sixth thing nobody wrote down.

That is worth keeping as a methodological note rather than as a complaint. The five were about what the
change *does* — which stage completes the pin, which refusal fires, what two records share. None of
them was about what the change **takes away from everything that is not the change**, and there was no
phase that would have found it. What found it was the compiler, on a build the protocol put in the
middle for an unrelated reason.
