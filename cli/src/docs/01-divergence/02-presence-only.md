# Observation 2 — "Nothing derived" audits what is written, never what is missing

Phase 4 wrote a repository that passes every rule this laboratory has, and is known in
advance to be insufficient.

The rule inherited from the reconstruction experiment forbids storing a derived value: no
identity, no resolved head, no partition, no condition, no level, no verdict. Phase 4 asks
one question of each datum it keeps:

> *What semantic reconstruction becomes impossible if this datum is not preserved?*

Every datum in the repository answers it. The recording instants are assigned rather than
derived. The commitment ids are references, not cached derivations. The genesis's selection
is the proposal, the fork's request is what was asked for. Nothing there could disagree with
anything else.

And the world Phase 1 reasoned about still does not come back.

---

## The question has an inverse, and nothing was asking it

The discipline is stated over the data present. Applied honestly, it removes everything that
could drift — and it says nothing at all about what is absent, because an absent datum is
never enumerated to be asked about.

```text
asked of every datum kept    →  is this derivable?        →  drop it if so
asked of nothing             →  is anything else needed?  →  never reached
```

So "nothing derived is persisted" was quietly doing the work of "the repository is complete",
and the two are unrelated. One is a rule about honesty; the other is a claim about
sufficiency. The first is satisfiable by a repository holding almost nothing.

That is how the naive form looked finished. Every rule it had was green.

---

## What Phase 4 can name that Phase 2 could not

The lineage is two sequences in two files, each ordered within itself:

```text
journal.json    ... B recorded 01-10 ... C recorded 01-11
lineage.json    ... genesis decided 01-10 ... advance 01-15
```

The journal knows the cancellation was recorded on the tenth. The lineage knows the genesis
was decided on the tenth. Nothing in either says which came first, and no third file holds
the answer, because there was never a datum whose absence the discipline would flag.

Phase 4 pins the whole of what a decision records, field by field, as a closed set. Whatever
eventually closes this has to appear there and answer the same question — which is the point
of writing the set down while it is still wrong.

---

## Consequences to carry

* A repository needs both audits. What is written must be underivable; what is needed must be
  present. Only the first was ever stated, and the second has no method here yet.
* The insufficiency is a property of the repository's *shape* — two sequences with no relation
  between them — rather than of any field in it. No amount of adding fields to a decision is
  guaranteed to fix a shape.
* The closed field set is the thing Phase 8 will measure against. If the experiment ends by
  adding a coordinate, the diff on that assertion is the answer, stated as a change rather
  than as prose.
