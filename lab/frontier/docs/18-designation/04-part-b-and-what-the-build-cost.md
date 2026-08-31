# Observation 4 — Part B, and what the build cost

The obligation is in the application. `cli/src/designation.rs`, a field on `RepositoryInput`, a
comparison on every read, and a union in the merge — `Earned by: 18-designation (Confirmed)` in each
module that carries it.

## Phase 7, which the protocol did not have

Building forced a question no phase had answered. A whole write takes the log as an input, so
`converge` has to supply one, and *what a merge does to two logs* had never been measured. The row's
rule is that an experiment may not build what it did not measure, so it was measured first.

The rule it is weighed against is `converge`'s own, one file over: two decisions cannot contradict
one another, so a second party's line is a branch rather than a competing version. Phase 6 had
established the same of designations.

```text
the union keeps both lines, and each line's own order            the planner's two moves at one
                                                                 coordinate stay in the planner's
                                                                 order, and the steward is not lost

a claim already present is not added twice                       two parties that agree hold ONE
                                                                 claim, and converging again does
                                                                 not grow the log
```

The second is the journal's rule for entries arriving without anything having arranged it: a
designation's fields are its whole content, so agreement *is* identity.

**And the limit, measured rather than left for somebody to find.** Two **unqualified** claims from
two parties at one coordinate cannot be ordered by anything either record carries — neither has a
party to be read under, and the coordinate is the same. The union puts the arrived one first, and
reversing the parties reverses the answer. That is the merge's arrival order surviving into the
result, which is the thing `converge`'s own docstring says must not happen to decisions.

It reaches no decision, no world and no number. It reaches *which unattributed plan a reader sees
last*, in a record where two parties both declined to say whose plan it was. Queued.

## What the build cost, item by item

**Seventeen call sites.** `RepositoryInput` grew a required field, so every construction of it
changed — fifteen laboratory subjects, two suites, and `converge`. That is the bill `lab/README.md`
describes: *not a veto, a bill — and it must read as a consumer breaking, never as the result having
always been different.* Each passes `&[]`, and each is a record whose plan never moved, which is
true.

**Required and not optional, and that was a choice.** A `None` meaning *carry forward whatever the
record has* would have made the default safe — a caller cannot then erase a plan by forgetting. It
was refused: a write that preserves what it was not given is implicit, and the experiment measured
the failure of exactly that shape. What the choice costs is written into the field's own docstring
rather than left for a caller to discover.

**Two guards went red that nobody touched.**

`cli/tests/pedigree.rs` caught the new module and the four new citations, in that order — `MODULES`
12 → 13, then `CITATIONS` 36 → 40. It also caught the result document before any of this, `RESULTS`
24 → 25.

And `lab/frontier/tests/individuation.rs` — experiment 14's **derived** guard, which scans
`cli/src` for every place an `EntryId` is compared — named the two new sites without being told:

```text
designation::corroborate      weighs a coordinate against the replay
designation::plan_at          walks the log to find the plan at a coordinate
```

That list is now **ten** where 14 published five. The change 14 priced and refused — an address that
says when the entry was recorded — costs twice what its own result said, and three times in three
sessions nobody had to notice. It is the argument for deriving the next one, made by the guard
rather than about it.

## The red pass, on the code that actually runs

The phases above prove the *shape* against the laboratory's prototype. A shape proven only against a
substitute leaves the code that ships with no red behind it, so six guards go through
`ape-cli`'s own read, write and merge with nothing of the prototype in the path — and three
mutations were made in `cli/src`:

```text
the plan check made dead            the refusal guard goes red, naming the plan
the merge handed an empty side      both merge guards go red, and the idempotence one with them
the write handed an empty log       FOUR go red, including the wire test
```

**And the prototype was kept rather than replaced.** `lab/README.md`'s rule is that a concluded
arrangement is pinned, not frozen — the prototype is the measurement. What is asserted instead is
that the two did not drift: the application writes what the prototype reads and reads what the
prototype writes, both directions, same file. Without that, every phase above would be evidence
about something that did not ship.

## One guard had to be repaired, and the repair is custody's

`phase_6_a_record_with_no_log_makes_no_claim` stopped being about anything the moment a whole write
started putting a log down: the record it founded now *had* one. The file is removed after the write,
which is `16-custody`'s `unclaimed()` move — *a phase measuring what a record without one does must
not be handed the thing it exists to say was missing.*

It now asserts both sentences, because the build made them different:

```text
[]        the plan never moved          what a whole write puts down
absent    the record says nothing       what every repository written before this has
```
