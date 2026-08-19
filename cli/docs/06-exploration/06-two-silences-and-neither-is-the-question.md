# Observation 6 — Two silences, and neither is the question

E6 predicted one silence: *considered-and-rejected and never-considered are the same*. Confirmed — and
there is a second one, in the other arrangement, which E6's title does not name.

## The closed set, derived rather than listed

Every place the three files mention a commitment at all, found by walking their JSON for the identity's
own text. Derived on purpose: a list of the record fields that hold a `CommitmentId` would be a second
copy of the record vocabulary, and it would stop being closed the moment a field were added — silently,
which is the only way this kind of guard fails.

```text
lineage.introduced    a decision proposed it                speaks about intent
worlds.open           a world selects it, revisably          speaks about intent
lineage.witness       it had been admitted by then           speaks about knowledge
lineage.after         it was the most recent admission then  speaks about order
```

Four places, and **`journal` is not among them.** An `Admission::Commitment` carries the fields a
commitment is built from and never its identity, which is derived — so a journal admits a commitment
without ever naming it, and *which propositions are known* is answerable by replaying the file rather
than by reading it.

Of the four, only two say anything about a commitment's relation to a world. Both of them mean
**intended**.

## Arrangement A: nowhere, exactly like nothing

Twelve candidates admitted, weighed, dropped, plus one commitment admitted and never looked at:

```text
commitments known                            14
selected by any world                         1   the opening
a weighed candidate is mentioned at        { }
an incidental admission is mentioned at    { }
```

Identical, and empty. E6's refutation clause — *a query separates a rejected candidate from an
incidental admission* — is not met, and cannot be: there is nothing to query. The record does not
withhold the deliberation; it never contained a syllable of it.

## Arrangement B: the same four places as something meant

The same twelve recorded as decisions, plus one genuine intention and one incidental admission:

```text
commitments known                            16
selected by any world                        14   the opening, twelve candidates, one intention
a weighed candidate is mentioned at        { lineage.after, lineage.introduced,
                                             lineage.witness, worlds.open }
an intention is mentioned at               { the same four }
the candidate the objective chose          { the same four }
an incidental admission is mentioned at    { }
```

So arrangement B *does* answer *which propositions were ever intended* — and answers it **wrong**. It
returns fourteen where one is intended, because the application recorded consideration using the only
verb it had, and that verb means intention.

The chosen candidate is in there too, indistinguishable from the eleven it beat. Nothing records a
preference, which follows from what the provenance experiment established and is worth stating anyway:
the winner is not marked either.

## The two silences

```text
A    considered-and-rejected  ==  never-considered
B    considered-and-rejected  ==  intended
```

Neither is the question. And the reason is structural rather than an oversight: the record has exactly
two places that speak about a commitment's relation to a world, and both of them say the same thing.

## What the record *can* separate, and it is a different axis

Two of the thirteen worlds report a conflict when the repository is read back — the floor refusing the
candidates that spend 110 and 120. That distinguishes **infeasible from feasible**, which is real and
re-derivable and says nothing about why either world exists. A reader meeting an infeasible world
cannot tell a candidate that was weighed and rejected from an intention somebody got wrong.

## The variable this was waiting for

The protocol left it open:

> Whether an application wants a decision that says *I considered this* as distinct from *I intend
> this* is exactly the ontology growth the severe failure condition guards against — so the procedure
> does not reach for one, and if the result needs one, that need is the finding.

**The need is demonstrated**, and the shape of the demonstration matters as much as the fact. Nothing
failed. No primitive was missing, no invariant was violated, no call was refused. Exploration is
expressible with what exists — and what it produces is a record that answers a question nobody asked
and cannot be asked the one they did.

That is the finding, and it is not a request for `Trial` or `Scratch`. It is the observation that the
distinction lives in the *decision* rather than in the proposition, and that nothing in a decision
currently carries it.

## Two debts collected from earlier phases

* **Phase 2** — the recording watermark does not distinguish exploration from intention in either
  direction. Exploring at an instant already reached advances nothing; intending at a later instant
  advances it identically. It is derived from `recorded_at`, so it inherits the journal's silence.
* **Phase 4** — in the pruned disposition that keeps a decision taken after the exploration, the
  surviving witness names every pruned candidate. What it audits is that propositions were admitted,
  never that they were weighed, and it would say the same of admissions nobody weighed.

## What was not done here

The protocol offers an agent reading the repository and reporting what it recovers, explicitly as
something that **is not a success criterion**:

> What an agent recovers is a question about presentation rather than about the record.

It was not performed. Phase 5's result is about the record and does not depend on it, and the
presentation question remains open for whoever wants it.
