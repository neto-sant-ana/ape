# Candidate — Authenticity

**This is not a protocol.** A protocol is written when its experiment begins, with the previous
findings in hand, and is not edited afterwards to match what happened. This is the material that
would be in front of whoever writes it.

It exists because every concluded result since corroboration names authenticity as a candidate and each
of them says *unchanged*. The candidate has been inherited and passed along by every one of them, and
nothing holds what it has accumulated on the way.

```text
02 corroboration   what signs a record, and who holds the key — Phase 7 is its motivation
03 convergence     unchanged, and now with one more derived value to forge
04 provenance      unchanged
05 coordination    unchanged, and now load-bearing for anything built on a decider
06 exploration     unchanged
07 atomicity       unchanged, and an interruption joins the same mechanism
```

---

## The faces, and they are one question

The `agents/` row gathered four cases across both rows in which a repository cannot tell an honest
state from a dishonest one, and read them as one family: *the fact that would distinguish them is not
represented at the layer that answers*. The list below is those four and whatever has been added since,
which is what this file is for.

Weighed case by case, the family is not all of a kind. The discriminator is narrower and it is
useful:

```text
                                     is the distinguishing fact in the record?
a forged `by`                        no  — it is who wrote the file
a consistently recomputed world      no  — it is the prior state, and nothing keeps it
a pruned leaf                        no  — the result is byte-identical to never having explored
an interrupted write                 no  — the same bytes as a commit that decided nothing
a separated readmission              YES — the journal holds both occurrences
```

All but one are the same question wearing different clothes. In each, the record is internally
coherent and the thing that would refute it was never in the record to begin with — so no amount of
comparison inside the repository reaches it. That is what *authenticity* names here, and it is why the
answer has to come from something the record does not contain: a key, or an anchor outside it.

The exception was different, and the difference is the finding:

> **A refusal can be made to name the right cause exactly when the distinguishing fact is somewhere in
> the record.**

The readmission's cause was a multiplicity that `corroborate` could not see, because it weighs sets —
but the journal held both occurrences all along. Moving the diagnosis to `rebuild`, which holds the
whole journal, was available *because the fact was present*. The others have no equivalent move,
and now there is a reason rather than an absence of one: there is nothing to relocate.

So when the next case of this shape appears, the question to ask first is not *which layer should
answer* — it is **whether the record contains the answer at all**. One of those is a reporting defect
worth fixing. The other is this candidate.

---

## What each result already established, and would be measured against

**02 — a consistent forgery is not refused.** Editing a repository and recomputing every derived value
from what was written produces a different lineage, and the refusal that mattered is simply gone.
Corroboration proves internal agreement and says nothing about who wrote it.

**03 — one more derived value to forge**, and the same limit.

**05 — the boundary, measured.** Every check is about the **reference** and none about the
**attribution**. A recorded party names an agent admitted and known at the decision's coordinate; that
the named party is who wrote it is witnessed by nothing. Swapping two parties' claims produces a
repository that reconstructs, corroborates, and says the opposite of what happened.

**06 — there is nothing to tell apart.** A repository pruned back to its opening and one that never
explored are the same bytes. This one is sharper than the others: it is not a check that is too weak,
it is a distinction the record does not contain.

**07 — a third intent on the same mechanism, and then one of them removed.** An interrupted write leaves
a repository byte-identical to one whose writer admitted knowledge and decided nothing about it, measured
the way 06 measured pruning. So tampering, pruning and being interrupted are one mechanism with three
intents, and the record holds none of the difference.

What that result then did is what narrows the candidate rather than lengthening it. A whole write is now
all-or-nothing, so an **interruption can no longer arise from the application at all** — the states are
unreachable by it and reachable only by something writing where it does not write. And the same is true
of the other two: nothing in the application tampers, and nothing in it prunes.

> Every intent in this family is now a **write the application did not make**.

Which is a sharper question than *tell these three apart*, and it points at a narrower answer: what the
record lacks is not a way to classify an edit, it is anything that says **who wrote this file**. That is
an argument for the *signing* face over the others, and it arrived by removing a case rather than by
adding one.

**The agents row's own stake.** Its published result rests on the counterfactual audit being *the half
that cannot be falsified by the party being audited* — which held against an in-memory adapter with no
durable record to forge, and was narrowed once there was one. And `Taken.by` is a named exception to
*nothing stored, everything recomputable, no verdict forgeable*: written, derived from nothing,
forgeable in silence.

---

## What is not decided here

Whether the answer is signing, an external anchor, an append-only log nobody in the repository can
rewrite, or accepting the limit and writing it down where consumers read it. Naming a shape before the
experiment runs is the thing the programme's rules exist to prevent.

And one honest question about scope, which belongs to whoever writes the protocol: every face in the
family — all of them but the readmission, which was a reporting defect and is fixed — is a property of a
**file on a disk**, not of APE. An experiment that measured them might find it is measuring a filesystem
and not an engine, and that finding, if it is the one, would be worth more than a remedy.

07 makes that question harder to duck rather than easier. Its repair was three `fs::write` calls, one
`fs::rename` and a directory name, and it removed a whole face of this family without the engine learning
anything.
