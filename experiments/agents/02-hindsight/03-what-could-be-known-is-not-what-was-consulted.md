# Observation 3 — The graph shows what could have been known, never what was consulted

This is the observation the two experiments were built to reach, and the auditor stated it
without being pointed at it:

> *"Whether a feasibility check was ever run before a decision. Nothing records it. This is the
> one gap that bears directly on 'should anyone have seen it coming': the answer to* could *they
> is a demonstrable yes, and the answer to* did *they is not in the graph."*

Experiment 01 closed by naming exactly this as the question it left open — that a stable reading
is not the same as being able to tell a defensible decision from a lucky one. It is now answered,
and the answer is a boundary rather than a verdict.

---

## What was already known from the engine

Operational state is projected and never stored. A projection is computed from a Thesis and
knowledge, on demand, by whoever asks.

Nothing about that is news. What it costs an audit is.

---

## The consequence of applying it

The property that makes the record trustworthy is the property that erases the evidence of
diligence. They are the same property.

Because nothing is stored, no reading can be stale, no verdict can be forged, and any claim
about what a world implied is recomputable by anyone holding the graph. And because nothing is
stored, there is no trace that anybody computed anything. An agent that examined every option
before choosing, and one that guessed and happened to guess well, leave identical graphs.

So the audit the record supports is precise and partial:

```text
counterfactual audit  → what could have been known, and what it implied      available
process audit         → what was consulted, by whom, before deciding         absent
```

The auditor performed the first exhaustively. It reconstructed each world, judged each under
its own cut, found that the house had a feasible alternative it did not take, and cross-checked
its own arithmetic against the engine's. Every one of those conclusions is reproducible by
anyone else from the same graph.

It could not begin the second.

---

## Which of the two an audit actually wants

Not obviously the missing one.

A process log is a record of a claim: *this was consulted at this time*. It is written by the
system being audited, it can be omitted, and a system that writes one for the sake of an audit
has an incentive shaping what it writes. Its trustworthiness is exactly the trustworthiness of
whoever kept it.

A counterfactual audit makes no claims to trust. It recomputes, from immutable knowledge, what
any competent examination would have found — and it answers the question that actually bears on
responsibility, which is not *did you look* but *would looking have told you*. A decision that
was unsound under what could be known is unsound whether or not anyone checked, and one that was
sound is defensible even if it was reached carelessly.

That is a narrower claim than "auditable", and it is the honest one:

> *An immutable, content-addressed operational knowledge graph makes an agent's decisions
> auditable in the counterfactual sense — what was knowable, what it implied, and what
> alternatives existed — and carries no evidence of process.*

Whether the missing half matters is a question about accountability regimes rather than about
engines, and this experiment has no standing to answer it. What it can say is that the half
that is present is the half that cannot be falsified by the party being audited.

---

## What would change this

An application that wanted process evidence would have to record it — a log of interpretations
requested, outside the engine, with all the trust problems that carries. Nothing in the ontology
would have to grow to permit it, and nothing observed here argues that it should.

The claim to carry forward is not that the gap should be filled. It is that the gap should be
stated, because a record that answers *could* while sounding like it answers *did* is worse
than one that answers neither.

---

## Smallest reproducing case

`02-hindsight/run-01/ANSWER.md`, item 7 of *What I could not determine*, and the whole of the
section above it — which is the counterfactual audit performed in full, against the same graph,
by someone who was told nothing.

---

## Narrowing — the unfalsifiability was a property of the substrate

Appended, and the original stands.

This observation ends on the counterfactual audit being *the half that cannot be falsified by the
party being audited*. That was measured over the engine's in-memory adapters, and it is narrower than
it reads, because those adapters have no durable record to forge. The claim was true of the boundary
it was measured in and was never tested against a substrate that offers a way to falsify.

One does. The CLI's corroboration experiment edits a repository and recomputes every derived value
from what it wrote: nothing refuses, a different lineage comes back, and the refusal that mattered is
gone with no file saying so. Its own words — *corroboration proves internal agreement and nothing
about who wrote it* — and closing it needs a signature and a key, which that boundary excludes.

So the honest form is a distinction the original did not draw:

```text
selective editing        →  refused, or reproduced as a disagreement
wholesale recomputation  →  accepted, and internally consistent
```

Content-addressing makes a partial edit visible because the identities stop agreeing. It does nothing
against an edit that recomputes the identities too, and the audit over that repository yields a
coherent account of something that did not happen.

What survives is the comparison the observation was actually making. A process log is a claim written
by the party being audited and is worth what that party is worth. A counterfactual audit is
recomputable by anyone holding the record — and *anyone holding the record* is doing the work that
was being credited to the record itself. Where the record can be replaced wholesale, both halves need
an anchor outside it, and neither is better than the other for lacking one.

This is published beside the original rather than in place of it. The result of experiment 02 stands
as the result of the boundary it was run in.
