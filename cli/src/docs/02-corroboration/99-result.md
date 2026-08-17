# Result

**Confirmed**, for the stated experimental boundary.

A repository that holds only what produces its worlds cannot contradict itself. This one holds
two things twice, compares both on every read, and refuses six edits out of six that change
what it means.

```text
                                                 before        after
a coordinate repointed at an entry that exists   SILENT     →  REFUSED
the genesis's intention narrowed                 SILENT     →  REFUSED
two roles reordered                              HARMLESS   →  HARMLESS
two commitments reordered                        REFUSED    →  REFUSED
an eligibility removed                           REFUSED    →  REFUSED
the cancelling Event removed                     REFUSED    →  REFUSED
```

Nothing passes in silence, and nothing that changes no world is refused.

## Against each criterion

1. **The baseline is reproduced from a fresh process.** Phase 1 measures it rather than
   inheriting it, which mattered: a phase comparing against an assumed baseline would be
   claiming a difference from nothing.
2. **Every corruption that passed in silence became a refusal.** Two of them, turned by two
   different witnesses.
3. **Every refusal names what disagrees.** The entry, for the sequence; the coordinate, for
   the world — with identity weighed last, so that what moved is reported instead of the hash
   that moves whenever anything does.
4. **Every persisted derived value is compared on every read.** Inside `reading::reconstruct`,
   not inside a harness. A refusal arrives on a child process's stderr.
5. **The `ThesisArchive` adapter conforms.**
6. **Reconstruction still derives every world**, and Observation 2 is why this is not a rule
   anyone had to keep: a `Thesis` does not deserialize, so a recorded world cannot be served
   in place of a derived one. Phase 6 measures the consequence — a repository holding its
   worlds and not its decisions produces nothing.
7. **What a decision records is a closed set**, and Phase 5 tested it by subtraction rather
   than argument: an instant, an intention, a coordinate that instructs, and a witness that
   this experiment cannot show is load-bearing.
8. **The consistent forgery of Phase 7 is undetected.** Stated here rather than buried.
9. **No persistence-specific concern entered the engine.** `core/` is untouched. Using
   `ThesisArchive` is using the boundary.
10. **The earlier experiments' conclusions stand.** Both harnesses reached them again, with the
    costs recorded below.

## The rule that replaces "nothing derived is persisted"

The old rule objected to an answer kept beside its question, free to disagree. The objection
was right about the mechanism and wrong about the sign: the freedom to disagree is the only
thing that makes disagreement visible.

> *A derived value is worth persisting where something compares it, and a derived value
> written and not compared on every read is the liability the old rule feared.*

Two consequences the experiment leaned on. A derived value must be the application's own
vocabulary, not the engine's serialized form — the answer the reconstruction experiment gave
about the journal, plus one: a witness computed over an encoding makes the encoding
load-bearing. And instruction must not be confused with witness: a repository holds some
things so a reader can derive, and others so a reader can disagree. The first cannot be
dropped for redundancy; the second cannot be kept without being compared.

## What the result does not cover

**A consistent forgery.** Phase 7 edits a repository and recomputes every derived value from
what it wrote. Nothing refuses, and a different lineage comes back — the refusal at −70 gone,
and no file saying so. Corroboration proves internal agreement and nothing about who wrote it.
Closing that needs a signature and a key, which this boundary excludes.

**Whether the sequence witness should stay.** Observation 3 shows it redundant for detection
and argues two things it still earns — naming the cause rather than the symptom, and catching
a coordinate moved to another entry that resolves the same cut. This subject cannot express
the second, so the argument is made here and measured nowhere.

**Substitution.** No corruption available to this subject swaps one entry for another, so the
difference between a witness that lists membership and one that counts it is untested.

**Concurrency.** One writer, one process. Unchanged.

## What it cost the experiments before it

Both are consequences of Criterion 4 rather than leaks: a derived value compared on every read
means every repository carries one, and there is no older format.

* The divergence experiment's closed field set gained `witness`, so a published measurement
  changed. Its own Observation 2 predicted exactly that, in those words.
* Both subjects now accumulate their admissions through one reading, so that a witness cannot
  be assembled from two readings that disagree. Additive; every assertion of both harnesses
  kept its value and its wording.

---

# Architectural Consequences

## Established for the CLI

* **The repository holds two witnesses**, over the knowledge a decision stood on and over the
  world it produced, and neither subsumes the other in what it can *say*.
* **`ThesisArchive` is an in-process port.** It resolves ancestry inside a running
  application. Nothing crosses a process as a world.
* **A refusal names a coordinate.** Half a refusal sends a reader back to the bytes.

## Candidates for later experiments

* **Authenticity** — what signs a record, and who holds the key. Phase 7 is its motivation and
  its measurement.
* **A subject that separates the two witnesses** — an arrangement where a coordinate can be
  wrong without a world being wrong. Observation 3 needs it and cannot build it.
* **Synthesis**, which now inherits an archive it can walk and a lineage that travels as
  decisions.
* **Cost**, larger again: every read derives and compares, and nothing here measures it.
