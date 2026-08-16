# Observation 1 — What moves a world is re-derivation, not process death

Phase 2 produced the divergence the experiment was arranged to provoke.

It produced it in one living process, with nothing persisted and nothing terminated.

```text
day 5    A, B admitted
         genesis decided at 2026-01-10   →  cut (2026-01-10, ∅)
day 10   Event cancelling B recorded
         the same decision applied again →  cut (2026-01-10, E)
```

The two cuts come from one `Decision`, one implementation, and one process. What differs
between them is when the decision was applied.

---

## What the boundary actually says

A `KnowledgeCut` holds a resolved head, and `KnowledgeCut::at` is the resolution. So a
decision recording an instant alone is not a record of a cut: it is a request to resolve one
again, and the answer depends on what canonical history holds at the moment it is asked.

That much was already visible in the engine. What Phase 2 adds is the consequence, and the
consequence is not the one the previous experiment's framing predicted:

> *The process boundary is where re-derivation becomes unavoidable, not where it originates.*

A reconstruction has no alternative — it holds no worlds, so it must re-derive all of them.
A living application does have one, and the divergence appears the moment it declines to take
it.

---

## What it costs, measured

The world Phase 1 reasoned about and the world its own decision rebuilds are not the same
world in any coordinate the experiment records:

```text
                    decided            re-derived
identity            T₀                 T₁          ≠
cut                 (10, ∅)            (10, E)     ≠
frozen              {}                 {B}
open                {A, B}             {A}
feasibility         OutOfBounds −70    ∅
```

The partition is the quieter half. Both worlds select `A` and `B`; they disagree about which
of them a fork may still revise. A comparison written to check membership would call these
the same selection.

The verdict is the loud half. A world that was refused by the resource's bounds comes back
unrefused, because the commitment that broke them is now cancelled — and a cancelled
commitment moves no level. Nothing about that is a defect: it is the correct reading of the
knowledge now standing. It is simply not the world the application reasoned about.

---

## What it forced already

The lineage had to stop being obtained by replay.

The previous experiment could obtain its live lineage by replaying its decisions, because its
subject admitted nothing during the instants those decisions named, so replay and the live
objects agreed. Here they do not, and an application that replays to find out what it decided
gets a world it never decided.

So the application holds the worlds it decided, and applies each decision once, as it is
taken. Reconstruction applies the same decisions through the same function — `lineage::decide`
is what both call — differing only in when.

That makes the missing coordinate precise. It is not a Thesis, an identity, or a projection.
It is *where in the sequence of admissions the decision was applied*.

---

## The obvious repair is not available

The first thing to reach for is to record the head alongside the instant, and the engine
appears to offer it: `KnowledgeCut::within` takes both coordinates explicitly.

It does not close this, for two reasons that hold independently.

```text
within(knowledge, known_at, event_head: EventId)
                             ↑ not Option
```

The genesis cut is `(2026-01-10, ∅)`. An absent head is not something the constructor can be
handed, and `KnowledgeCut` offers no other way in — its fields are private and `at` and
`within` are the whole of the public surface. The world Phase 1 reasoned about cannot be
named through the boundary at all.

And a present head would not fare better. `within` refuses `HeadPrecedesCut` when the named
Event is not the last of the group its instant addresses, because naming an earlier one is
retraction — a world claiming to know a day while omitting Events recorded within it. The cut
module states that as something a cut must not be able to express.

So the boundary refuses, deliberately and by documented design, the exact shape a naive
repair would need. That is not a gap to be filled: what the experiment has to answer is what
a *repository* records such that a fresh process asks the right question, and an instant plus
a head is not it.

---

## Consequences to carry

* The remaining phases must not treat a repository round-trip as the thing under test. The
  defect is reachable without one, and a fix that only works across a process boundary would
  be fixing the symptom.
* Whatever a decision records to close this must be resolvable by a fresh process holding
  nothing but the journal and the lineage. An instant is not, on its own, such a record — and
  neither, per the section above, is an instant paired with a head.
* The cascade is not yet measured. The genesis is the parent of the advancement, so a genesis
  that re-derives differently should carry every descendant with it. Phase 3 forks, and Phase
  8 is where the whole lineage is compared.
* `Advancement::imposed` is recorded by Phase 2 and established by nothing. This subject never
  imposes, so the report could be hard-wired empty and the phase would pass. Whether a
  repository needs to reproduce it is a question the experiment cannot currently ask.
