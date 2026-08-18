# Result

**Confirmed**, for the stated experimental boundary.

An application terminated completely and a fresh operating-system process rebuilt the same
operational meaning from the repository alone. The two readings agree in every coordinate
the procedure names and in every one it does not.

```text
canonical head       377850…  =  377850…
Thesis identity      6f53a4…  =  6f53a4…
Knowledge Cut        2026-01-15, head 377850…  =  same
outcome              Fulfilled  =  Fulfilled
timeliness           absent     =  absent
derived level        10.0       =  10.0
feasibility          no conflicts  =  no conflicts
```

## Against each criterion

1. **The original process can terminate completely.** Reconstruction runs in a separate
   process, so nothing the procedure forbids can carry correctness across — not because it
   was avoided, but because it cannot cross a process boundary at all. The same command
   without the repository fails, which is what distinguishes reading from remembering.
2. **A fresh process reconstructs canonical knowledge from persisted information alone.**
   Its only arguments are a repository path and the inputs needed to ask for the same
   interpretation.
3. **The Event chain has the same canonical identity and ordering.** The chain is not
   stored: replaying admissions in journal order rebuilds it, and the head that results is
   the head that was.
4. **The Thesis has the same identity and Knowledge Cut.** Neither is persisted. The
   identity is derived again from a selection resolved against a cut the decisions
   describe, which is a stronger result than reading one back would have been.
5. **Interpretation produces equivalent conditions.** Settled, and therefore under no
   deadline, at the same effective instant.
6. **The derived consequence on the resource is reproduced.** The level is folded from the
   projection rather than kept, so reproducing it required reproducing everything under it.
7. **The feasibility verdict is reproduced** — and this is the weakest of the seven. Both
   sides report no conflicts, so what was compared is an absence. A subject whose bounds
   were actually contested would test more.
8. **No previously computed projection is required.** None is written; Phase 3 asserts the
   repository holds no condition, level, verdict or identity.
9. **No process-local state is required.** There is no process left to hold any.
10. **No persistence-specific concern entered the engine.** Two engine changes were made
    while this experiment ran, and neither is about storage: the conformance suite gained
    the assertion that a refused append leaves the recording watermark untouched, and a
    commitment's movement became a public derivation. The first is a contract the suite
    stated nowhere; the second is arithmetic an application needed in order to present a
    level, which the engine already computed privately. A repository model reached neither.

## What the result does not cover

The boundary excluded alternative branches, Synthesis, mutable references, concurrent
processes, snapshots and indexes, and the exclusions held — except for Thesis advancement,
which the procedure itself required and which the boundary permits on exactly that
condition. A cut cannot recognize an Event it predates, so recognizing one is advancing.

The subject is one commitment with no dependencies, one quantifiable resource and one
settling Event. Dependency waiting, unfulfillability, cancellation and contested bounds are
untested here, and each of them is a path through the same interpretation.

Every quantity is an integer. Feasibility accumulates in `f64`, where addition is not
associative, and a subject with fractional magnitudes could differ in the last bit under a
different accumulation order. Whether reconstruction is order-stable for such a subject is
not something this experiment answers.

The cost established in Observation 1 stands unmeasured beyond its shape: opening a
repository is the length of its history, in time and in residence. Nothing here says at
what length that stops being acceptable.

---

# Architectural Consequences

Each of these is traceable to an observation, and none is accepted for making an
implementation convenient.

## Established for the CLI

* **The repository is a log, and reconstruction is replay.** From Observations 1 and 4:
  canonical knowledge cannot be hydrated and cannot be extracted, so what is written is the
  sequence of admissions as they were supplied, and what is read is that sequence replayed.
  A repository built on this boundary is not an object store.
* **A lineage is persisted the way knowledge is.** Decisions rather than Theses, for the
  same reason: a `ThesisId` is derived, and storing one would keep an answer beside the
  question it comes from.
* **No `ThesisArchive` was needed.** A repository holding decisions resolves Theses by
  replaying them. This is a fact about this experiment's boundary and not a claim about the
  port — ancestry walked across processes may still want one.

## Pressure recorded, not acted on

* **An adapter's contract coverage is the suite's coverage** (Observation 2). The engine's
  suite and port documentation are where that is addressed. The gap found was corrected; the
  general point — that an application cannot extend the proof available to it — was not.
* **Snapshots and indexes remain open.** Observation 4 positions a snapshot as a memoised
  fold, valid as a cache and disqualified as a source of truth. When repeated reconstruction
  justifies one is the open question it was, now with a measured shape rather than a guess.

## Method

* **Comparison measures the boundary, not the code that crosses it** (Observation 5). Later
  experiments that compare two derived worlds inherit the blind spot, and need an
  expectation fixed before the derivation that produces both.

## Candidates for later experiments

* A subject that contests its bounds, so a feasibility verdict is reproduced rather than an
  absence compared.
* Dependencies, cancellation and unfulfillability, which are untravelled paths through the
  same interpretation.
* Fractional magnitudes, which would answer whether reconstruction is order-stable where
  `f64` accumulation is not associative.
* Mutable references and multiple Theses, which the open questions already name and which
  this experiment needed none of.
