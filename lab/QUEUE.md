# Queue

Everything that can be worked on, in one place, with where it was named and what holds its material.

**This holds no question of its own.** Every item's content stays in the result, protocol or candidate
that produced it, and this points at it — because a queue that restated nine results would be nine
places for the same item to drift. What it holds that nothing else does is the **count**, the **kind**,
and whether anything is holding the material.

**Why it exists.** A candidate that is inherited and never held is the failure
[`candidates/`](candidates) exists to fix, one item at a time. This is the same failure one level up:
choosing what to do next required reading nine results, two rows and eight layer documents — and the
first sweep found four items recorded nowhere at all, which is what
[`01-veracity`](candidates/01-veracity.md) through [`04-training`](candidates/04-training.md) now hold.

**It is not a plan and not a priority order.** Ripeness is reported — how many results named a thing,
and whether the later ones sharpened it or repeated it — and what to do with that is a decision this
file does not make.

---

## The kinds

```text
experiment   a question with a measurement, owned by a row
candidate    the same, with its material gathered in candidates/ because nothing else held it
engine       a gap in the ontology, which no experiment may close by itself
unblocked    an obligation whose reason not to build it was removed, and which nothing has asked for
debt         a published caveat a later substrate made liftable, and nobody lifted
```

The distinction that matters most is **experiment** against **unblocked**. A request from an experiment
is not a feature: it is an input to that experiment's Part B, and the ones Part B declined were
declined *for want of a measurement*. Building one anyway would put code in the application citing an
experiment that did not measure it, which is what `cli/tests/pedigree.rs` exists to refuse. And a
result that removes the reason **not** to build something has not thereby asked for it — see the one
`unblocked` item below, which stays unbuilt for exactly that reason.

---

## Held in `candidates/`

Four questions with their material gathered, because each was being inherited and dropped.

| | question | where it began |
|---|---|---|
| [`00-authenticity`](candidates/00-authenticity.md) | what signs a record, and who holds the key | 02, and every result since says *unchanged* |
| [`01-veracity`](candidates/01-veracity.md) | does a record that agrees with itself thereby say something true | 07 and 08 asked it identically; 01 had already measured an instance; and 10 handed it its **first reachable case from the writing side** — knowledge present at a decision, not depended on, and later removed |
| [`02-scale`](candidates/02-scale.md) | an application must say what it counts somewhere — is that somewhere the record | the `i128` change, decided in conversation and written nowhere |
| [`03-bounds`](candidates/03-bounds.md) | a floor with no ceiling cannot be stated, and the workaround writes down a bound that is false | handed over twice by the agents row, verbatim |
| [`04-training`](candidates/04-training.md) | is this structure a harness for training decision reasoning | 06 was written to begin the question |

**And one was held and answered.** [`05-witness`](candidates/05-witness.md) was run as experiment 10, and
the answer is **no** — a record cannot say what a decision depends on *instead of* what stood, because the
witness is the only guard whose subject is the repository's history. What survives is smaller and better
placed: the three capabilities need a **second comparison** — and, if the merge is not to dissolve what a
decider could have known, one thing the record does not say. Both are below, under the frontier row.

---

## Frontier row — experiment candidates

Ordered by how many concluded results named the item, which is the only ripeness signal available.

| named in | item |
|---|---|
| 05 06 07 08 09 | **Saying two lines agree** — measured three times, expressible nowhere; and the face 09 handed over is now the item below |
| 09 10 | **A comparison whose subject is what two decisions are about** — what experiment 10 leaves standing. The dependence set is derived from the coordinate and the journal, and all three of 09's blocked cases admit under it with nothing written down. Two boundaries, both measured: it must not replace the witness; and on its own it dissolves *what a decider could have known*, because a comparison weak enough to admit an imported witness is the same shape as one that admits a journal grown underneath a local decision. And one hazard it must rule out and this experiment did not reach: an **Event** arriving before a non-Event coordinate moves a cut and produces a world nobody decided |
| 07 10 | **Which line of knowing a claim is about** — the thing that would let a merge keep the context instead of dissolving it. Reached from three directions now: 07 wanted a witness of the journal's extent, 10 measured why a merge needs one, and the authenticity candidate asks the same question as *whose record is this* |
| 03 04 05 | **What a discarded or discredited lineage owes** the ones that took intention from it |
| 07 08 | **A series of generations, and what would prune one** — where exploration's pruning question meets atomicity's rollback |
| 07 | **A witness of the journal's extent** — recording how long a journal was, rejected in 07 as a substitute for atomicity, and still the only shape that would roll back a torn journal. It pointed the opposite way from [`05-witness`](candidates/05-witness.md), and experiment 10's answer makes it the surviving direction: the record's one history-shaped claim is the witness, and this asks for a second |
| 05 08 | **Telling *never decided* from *decided and overwritten*** — the same unanswerable query reached from two directions |
| 07 08 | **Durability against power loss** — `fsync`, and whether the pointer's rename is the only place it is needed |
| 00 06 | **Snapshots or indexes**, and when repeated reconstruction justifies one |
| 00 01 | **What a world is called** when an application holds several, and human-readable names for persisted identities |
| 03 04 | **Does a transfer have an identity**, and would provenance be it |
| 10 | **The readmission diagnosis is reachable only through a refusal** — `diagnosed` runs on the error path of `corroborate`, so a journal admitting one address twice past the last coordinate rebuilds in silence. Small, real, and unrelated to the witness |
| 08 | **Whether a repository may say how many writers it admits** — the first need that asks the record to describe its own use |
| 08 | **Threads, and a turn that is atomic against one** — excluded throughout, and what a real application meets |
| 06 | **A decision that says it weighed rather than meant** — the request nearest the ontology, and load-bearing for [`04-training`](candidates/04-training.md) |
| 06 | **A preference, recorded as a claim** |
| 06 | **Whether a bounded record owes the worlds nobody chose anything at all** — the only item where *no* is a coherent answer rather than a deferral |
| 06 | **Whether the journal should deduplicate** |
| 06 | **Generated Events and simulated time** — the sharpest dependency exploration-at-scale has |
| 03 | **Choosing between Bases** — an arrangement branching twice on one side |
| 02 | **Substitution** — one entry swapped for another, which no subject so far can produce |
| 00 01 | **Dependencies, cancellation and unfulfillability** — untravelled paths through the same interpretation |

**Deferred by all nine: cost.** No experiment has measured it and every one of them says so. It now
has terms somebody could measure: reconstruction admits the journal in step with the lineage; every
read derives and compares; a whole write copies three files rather than replacing three; and a turn
reads three files before renaming. What is missing is not a method — it is a reason to want the number.

---

## Engine — nothing queued, and the test that keeps it that way

The engine's layer sequence is **finished as documented**: `00-philosophy` through `07-synthesis` in
[`../core/src/docs/`](../core/src/docs), each with its implementation. There is no next layer waiting,
which is why nine experiments have produced findings for the application and four consecutive ones left
`core/` untouched. That is the structure rather than drift.

**And nothing is queued against the ontology**, because the test for entering it is not the one the
laboratory's evidence naturally satisfies:

> A concept enters the ontology only if operational coordination cannot be **represented** without it.
> Not whether a caller wanted it named, and not how many callers wanted it.

That is [`01-ontology.md`](../core/src/docs/01-ontology.md), and the laboratory's own reading of it is
in [`README.md`](README.md) — *a friction is evidence of a want, not of a need*. Every finding here
arrives as a friction, which is by construction somebody reaching for something, so the test is the
thing that stops nine experiments' worth of reaching from accumulating into a larger engine.

Applied to what the queue held, it removes the only item that would have touched the kernel and
predicts the shape of the two largest candidates:

```text
acting for      closed. Three agents wanted to say whose behalf they acted on, which is
                a want they had BECAUSE they were agents. Coordination between commitments
                and events is representable without it — and a delegation is a type an
                application holds over two Agent identities

authenticity    whatever signs a record, coordination is representable without it — so the
                remedy is an application's, and the candidate's own material already says
                the answer comes from outside the record

scale           the engine adds and compares along one axis and there is no second axis to
                reconcile, so a unit is not necessary to represent coordination. Which is
                the answer the engine already gives, and what is left is an application's
```

And *belongs to an application* is not *belongs to nowhere*: the ontology's note says an application
may compose the primitives **or wrap them** in types of its own, with one boundary that matters here —
a wrapper is the application's vocabulary and does not enter the record, so a meaning that reaches no
primitive is a meaning the next reader does not have. Which is why the remaining half of `02-scale` is
about the **label** and not about the wrapper.

The test has a cost worth stating: it makes the ontology very hard to grow, and a real need would have
to arrive as *a coordination that cannot be expressed at all* rather than as anybody's difficulty. That
is the intent — the ontology is radically minimal, and the burden is on the extension.

---

## Unblocked, and unrequested — one item

**Feasibility after applicability, in the CLI.** `agents/04` removed the reason not to build it: the
command was *deliberately* withheld because criterion 7 asked whether an agent handed an applicability
report would go on to ask the second question, and an application that asked for the caller would have
ended the question. The agent asked, and the answer is pinned.

> The answer is now recorded against a pinned version, so the work can proceed and this result stays
> runnable.

Today `ape-cli <repo> transfer <base> <source> <target>` renders applicability and stops. And it stays
that way, because *the work may proceed* is not *something asked for it*. No experiment has been
obstructed by its absence, and coherence is not pressure — which is the same standard applied to every
declined Part B in the row.

**One blocker would also have to move first, and it is structural.** `cli/tests/pedigree.rs` resolves a
citation against `lab/frontier/docs/<experiment>/99-result.md` — the **frontier row only**. So nothing
the agents row earns can be cited by the application as it stands. That is a finding about the guard
rather than about the work, and it is the first time it has mattered.

---

## Re-run debt — agents row

Recorded in [`agents/00-question/05-the-re-runs-that-did-not-happen.md`](agents/00-question/05-the-re-runs-that-did-not-happen.md),
which also states why the debt is acceptable and where it is not. Three caveats the repository
substrate made liftable, and nobody has lifted:

```text
S2   an auditor's set of alternatives was a lower bound because nothing enumerated.
     A journal enumerates. Whether an auditor lifts the caveat is unmeasured
S3   a persisted Decision names its operation, so what an auditor was inferring by
     comparing states now exists. Whether it prefers the record is unmeasured
S4   a claim sorted as unsupported partly because nothing enumerated is now decidable
```

Its own note says what would make paying the debt worth it, and it is none of the three: whether an
agent handed a **larger boundary uses more of it**. Every earlier run met silences that were controls.

**And four requests from those runs, none acted on** — no way to say what a commitment is *for*; no
author on an admission; no *on-behalf-of* between agents; and `Corroborated` cannot extend what was
read, so a party that has decided rebuilds the struct field by field.

**One composition handed over and not written**: the repair *read again and admit again* has an order,
because a Canon refuses an admission dated before its `recorded_through` — so the party whose knowledge
is dated earlier must land first. Both halves are documented; only their composition is not.

---

## Closed, and worth not reopening

An inventory that only grows teaches nobody. These were candidates and are answered:

```text
whether a decider belongs in the record
                            settled across four experiments and stated in three
                            documents: it does not
acting for                  the relation that survived that conclusion, and it does not
                            survive the ontology's test for entering it — see above
abandoned siblings          06 — no longer deferred; they are eleven of thirteen worlds
concurrency, interleaved    08 — every ordering of two writers, and what each leaves
whether three files are one 07 — no, and by measurement rather than preference
what a writer that waits is owed
                            08 — closed by not arising: nothing in the repair waits
fractional magnitudes       superseded by the i128 representation; what is left is the
                            unit, in candidates/02-scale.md
a separated readmission     the one authenticity face that WAS in the record, and was
                            relocated to where the fact lived
two repositories meeting    09 — they share the history they said the same way, agree about
                            a world where they agreed about the knowledge under it, and are
                            refused by the journal compared entire. No authority needed, and
                            the merge-base derived rather than found
recording dependence in
the witness's place         10 — no. Dependence is narrower, derivable and admits all three
                            blocked cases; and the witness is the only guard whose subject
                            is history, so what it stops refusing nothing else refuses.
                            The capability survives as an operation, not as a record
```

**And nothing is unrecorded any more.** The first sweep of this file found four items that existed in
conversation and in no document. They are the first four candidate files, which is what a queue is for.

**The fifth was found the other way**, and it is the more useful kind of finding: three items this queue
would have listed as three turned out to be **one**, and one that was already in it. Experiment 09 ended
with three requests — a partial meeting, knowledge without intention, agreement said once — and every one
of them is blocked by the same line of `corroborate`; experiment 06 had asked the same question as a cost.
Four arrivals, one candidate, [`05-witness`](candidates/05-witness.md).

Which is worth stating as a habit rather than an anecdote: **a queue's failure mode is not only losing an
item, it is holding one question as several.** Before a result's candidate list is copied here, the
question to ask of any two of its entries is *what would have to change for both of these to be
answered* — and if the answer is one line, they are one item.
