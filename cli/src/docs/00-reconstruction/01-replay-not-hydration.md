# Observation 1 — The public reconstruction path is replay, not hydration

The first durable `CanonicalKnowledge` adapter made a consequence of the current APE
boundary concrete.

Canonical records cannot be hydrated directly from persisted representation through the
public API.

Three existing constraints meet at that boundary:

```text
Commitment      → Serialize, not Deserialize
Date            → reconstructed through parse, not Deserialize
Canonical<T>    → cannot be constructed by the application
```

This is consistent with how the engine already reconstructs meaning.

Kernel entities are created through their ordinary construction paths, where identity is
derived again from their fields. Value objects expose public constructors or parsers.
Canonical records are produced only by `Canon::admit_*`, with `recorded_at` supplied by
the caller, and reach the history adapter through `put_*`.

Therefore a fresh application process can reconstruct canonical knowledge by replaying the
durable inputs through the same admission path that originally produced it:

```text
persisted admission inputs
        ↓
reconstruct inputs
        ↓
Axiom::emit_*
        ↓
Canon::admit_*
        ↓
Canonical records
```

Re-emitting equivalent inputs reproduces entity identity, and replaying admissions with
their original recording instants reproduces canonical records.

The experiment therefore does not reveal a missing deserializer.

It reveals an application-level consequence of the existing public boundary:

> *Through the current public API, canonical knowledge is reconstructed by replay rather
> than hydrated from storage.*

This consequence matters only once persistence crosses process lifetime.

A durable adapter cannot implement `canonical_commitment` by independently loading a
stored canonical record on demand, because the application has no public operation that
constructs that record. The adapter must first receive the record through replay and
retain something from which subsequent reads can be answered.

For the simplest implementation this implies:

```text
repository open
→ replay canonical admissions
→ rebuild readable history

open time     = O(history)
resident data = O(history)
```

The experiment has therefore established durability, but not lazy reconstruction.

---

## Consequences to carry

* The repository must preserve enough information to reproduce admission inputs rather
  than assume that serialized engine entities can simply be hydrated.
* Durable representation of those inputs is necessarily an application concern where the
  engine exposes no persistence representation for them.
* Replay must respect admission dependencies, because reconstruction uses the same
  reference-resolution boundaries as original construction and admission.
* The existing open question — *when does repeated reconstruction justify snapshots or
  indexes?* — now has its first concrete source of pressure.

None of these consequences yet justifies changing the engine or introducing a snapshot
mechanism.

The experiment should continue under the simplest replay implementation and observe
whether the pressure remains structural.
