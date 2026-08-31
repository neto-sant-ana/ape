# Observation 04 — A want has a standing, and only two of twenty-one are in the queue

**The second amendment to the protocol, and the operator asked the question that produced it:** had
the opaque constraint already been handled? It had, in one half and not the other — and the first
commit of this experiment had reported the recurrence as fresh evidence.

## What was wrong with reporting a recurrence

`01-single-agent` and `02-hindsight` both reach for the bound behind a resource constraint, and both
work around its absence — one copies it out of the fixture by hand, the other **probes** it with
`check(-0.0001)` and `check(0.0)`. Two agents who never met, one absent accessor.

That is a real observation and it was reported as if the laboratory did not already know. **It did.**
Experiment 01's own result records it, as Observation 3:

> *A conflict names the level it reached, not the bound it left. An application that must explain a
> refusal keeps its own copy of the bounds.*

Without asking where a want already stands, this experiment re-reports the queue — and re-reporting
the queue while claiming to find something is precisely the drift [`CHARTER.md`](../../CHARTER.md) was
written to stop.

## And a correction to what was said before this was measured

The first answer given was that the read half of the constraint want appears **nowhere** in the
laboratory. That was wrong, and the way it was wrong is worth keeping:

```text
searched for   "no accessor", "accessor for its bound", "probed rather than read", "Display"
                — which are the AGENT's words

written as     "a conflict names the level it reached, not the bound it left"
                — which are the LABORATORY's words
```

The search returned nothing and was read as absence. **It is the same error as
[`03`](03-the-corpus-had-two-readers.md)**: a name — there, a directory's; here, a phrasing — was
trusted over reading the thing. Twice in one experiment, in a row whose subject is what a record is
worth to somebody who was not there.

## The amendment

Two of the five kinds **ask the record for something**: `Want` and `Loss`. Those two now carry a
`Standing`, and cannot be constructed without one — the constructors enforce it rather than a guard,
because all the data the invariant refers to is local to one claim.

```text
Tracked(where)    in QUEUE.md or a candidates/ file. TRACKED MEANS SELECTABLE
Recorded(where)   in a result document, and nowhere the queue reads
Met(what)         served since the testimony was written
ByDesign(where)   the engine or the laboratory has ruled that the record deliberately
                  does not carry it
Untracked         nowhere at all
```

## What it measured, across two testimonies of eight

```text
tracked      2
recorded     7
met          3
by design    6
untracked    3
             --
            21
```

**Two of twenty-one are in the queue.** Seven are in result documents — findable by somebody who
already knows to look, and invisible to selection, because the queue orders items *in the queue* and
ripeness cannot reach what never entered it.

**That is the drift's mechanism, measured rather than argued.** `agents/01` recorded two frictions in
its result document at the very first experiment of that row. Neither is in `QUEUE.md`. The queue has
a section named for `agents/05` — added three days ago — and nothing equivalent for 01 through 04. So
nine consecutive frontier experiments could run in one neighbourhood while those two sat where nobody
choosing work would read them.

**`Recorded` versus `Tracked` is this experiment's sharpest method finding**, and it is not about H4
at all.

## And `ByDesign` is H4's target, which is why the category earns its place

Six of twenty-one are wants the record deliberately does not serve. The laboratory said so about one
of them in `02-hindsight`'s own result, and said it in H4's words two years before H4 was written:

> *The one thing the auditor could not do — establish that anybody checked — is **absent by design
> rather than by omission**, and filling it is an application's business if any application wants it.*

H4 does not ask whether these belong in a primitive. That question is settled, and settled **no**. It
asks whether they belong **beside** the entity — which is exactly *an application's business, if any
application wants it*.

## The met ones, which are the only evidence of what happens after a boundary grows

Three, and both are chains rather than fixes:

```text
who took each decision        MET by `Taken.by` when the repository landed — and experiment 17
                              then measured that `by` names who TOOK a decision and not who
                              relayed it. The want was served, and the serving exposed the gap
                              that is now the queue's "a name for a record"

the set of alternatives is    MET by the journal, which enumerates. Queued as re-run debt S2,
a lower bound because         which says in as many words: "whether an auditor lifts the caveat
nothing enumerates            is unmeasured". The capability arrived; the measurement did not
```

**A want that was met is worth more than a want that recurred**, and neither of these could have been
seen without asking where the want stands. The first shows a boundary growing the wrong field; the
second shows one growing the right capability and nobody checking whether it helped.

## What it costs

**Nothing to the classification**, and that is checkable: no claim's verdict changed, only what is
attached to twenty-one of them.

**One thing is owed and is not paid here.** Ten of the twenty-one — the seven `Recorded` and the three
`Untracked` — belong in `QUEUE.md`, and putting them there is exactly what this observation says the
laboratory failed to do for four experiments. They are not queued yet **because eight testimonies have
not been read**, and a queue entry written from two would be the third version of the same mistake:
acting on a partial reading as though it were the finding. It is owed at Phase 5, and it is written
down here so that *owed* does not quietly become *recorded*.

**And the honest limit: the standings are a judgement too.** Whether `03-bounds` is *the same want* as
the read half is a reading, and this classification says it is not — one is about what a constraint
can **say**, the other about what it can be **asked**. A reader who disagrees can see both, because
every standing cites a document and
[`every_standing_that_cites_a_document_cites_one_that_exists`](../tests/classification.rs) refuses one
that sends them nowhere.
