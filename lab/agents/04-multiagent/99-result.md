# Result

**Confirmed**, and four predictions out of five were wrong.

> *Can independently acting LLM agents evolve different intentions from a shared operational world and
> use Synthesis to reconcile their changes without requiring the engine to infer their intent?*

**Yes**, and nothing was added to the engine. Two parties expressed different intentions through the
eight primitives, both lines landed in one repository without either losing the other's decisions,
Synthesis answered in both directions, and a third agent handed the report supplied the judgment the
layer assigns to an application.

The interesting part is not that answer. It is that the experiment was wrong about almost everything it
predicted, and wrong in a way that was worth writing down.

## The criteria

```text
1  both express their intentions through existing primitives        ✓
2  both lines converge without either losing the other's decisions  ✓
3  a report each direction, and any asymmetry explained             ✓
4  Phase A applicable, and applying it yields a feasible world      ✓
5  Phase B1 conflicted, naming a dependency invariant               ✓
6  Phase B2 applicable, and applying it yields an infeasible world  ✓
7  an agent handed the report either asks the second question or not — it asked
8  nothing added to the engine                                      ✓
```

Criteria 4 to 6 were setup and are recorded as such. All three are properties the layer documents, and
measuring them proves the harness rather than the hypothesis — [`world_changed.rs`](../tests/world_changed.rs)
says so in its own docstring rather than leaving it to be inferred.

Criterion 3 required an explanation of the asymmetry, and the explanation is not this experiment's: the
convergence experiment gives it in closed form, `Source.known_at ≥ recorded_at > Target.known_at`, and
says outright that reading it as friction sends an experiment looking for a repair nobody needs. The
asymmetry appeared here with the agents' own instants in it, which is an illustration and not a finding.

**37 assertions across the harness's suites**, every guard put through a deliberate mutation, every
mutation red for the named reason.

## The frictions, four of which were refuted

**I1 — the setup breaks for a boring reason.** Not refuted, not confirmed as written. An advance *was*
required, but the prediction said *the advancing is a decision the protocol has to make and record rather
than discover*, and the protocol never made it: operations discovered `CommitmentNotKnownAtCut`, decided
an advance was needed, and stated why. Finance needed none, having recorded at the instant the base
already recognized. So the friction was real and belonged to the parties.

**I2 — `Applicable` will be read as consensus.** **Refuted.** It asked for feasibility unprompted, named
all three hypotheses, and then built a neighbouring world where the same verdict is unsound.
[Observation 2.](02-the-handoff-was-taken.md)

**I3 — the record cannot say whose intention won.** **Refuted as stated.** A party given only the
repository partitioned the lineage by party without being told there were two, because the parties are
knowledge. Narrowed residue: *unclaimed* and *not this party's* are one silence.
[Observation 3.](03-two-agents-forged-the-party-without-being-asked-to.md)

**I4 — an agent will claim replacement.** **Refuted.** No party described its change as superseding
another's, and the reader went further: handed a world where the pairing was the obvious reading, it
declined to make it, cited that Synthesis does not pair an omission with an introduction, and referred
the question to the party who could answer it.

**I5 — the fourth register.** **Refuted.** A claim resting on `by` sorts cleanly the moment the check is
run, and two agents ran it unprompted. The third register held for the third experiment in a row.

## What was found instead

**Two parties in sequence are one line.** The arrangement the protocol chose removed the phenomenon it
wanted to measure. Divergence is not what independent parties produce; it is what parties produce when
they read the same state before either has written. [Observation 1.](01-two-parties-in-sequence-are-one-line.md)

**The handoff was taken, and the reason it was safe is not the caller's diligence.** The three earlier
experiments measured silences that were controls — the engine withholds until the caller names something.
This one was a handoff, with every forcing function absent. It was taken, and the durable form of that is
smaller than the criterion hoped: a handed-off question is safe to the extent it stays **askable of the
record**, which is a property of the engine.

**A party is a participant, fully; a party's agency is nowhere.** Knowledge says who owes, who performs
and who benefits, and not who recorded it. Intention says who decided, and not whom they decided for.
Each half is missing from the file where the other half lives, and three agents asked for it without
inventing it. [Observation 4.](04-what-neither-party-could-say.md)

**Nothing says what a commitment is for.** Two parties, two tasks, the same request. One of them wrote a
program that refuses rather than guesses, because two open commitments with the same arithmetic are the
same object to any reader.

## What is handed over, and what is not built here

**One sentence to the branch that owns it.** The repair a diverged party is given — *read again and admit
again, because knowledge is not revisable* — is written without qualification, and it has an order. A
Canon refuses an admission dated before its `recorded_through`, so the party whose knowledge is dated
earlier must land first. Invisible from the coordination experiment because its two parties learned on the
same day; visible here because two agents chose different instants, each for a stated reason. Both halves
are documented; only their composition is not.

**A floor-only resource constraint** is still not expressible, still restated as a range whose ceiling
nothing reaches, and still the earlier experiment's handover.

**The command that runs feasibility after applicability** was deliberately not shipped before this
experiment ran, and criterion 7 is why. An application that runs the second report for its caller ends any
question about whether the caller would have asked. The answer is now recorded against a pinned version,
so the work can proceed and this result stays runnable.

**Four requests from the runs, none acted on:** no way to say what a commitment is for; no author on an
admission; no on-behalf-of between agents; and `Corroborated` has no way to extend what was read, so a
party that has decided rebuilds the struct field by field.

## The thesis, and where it now stands

The scope that proposed this experiment ended on a suspicion: that APE may not be an architecture for
agent decisions, but one in which agents are simply Agents.

The protocol answered that if the experiment passed, the conclusion would be that **coordination never
needed to know who decided, only under what knowledge** — indifference rather than integration.

That holds, and one run sharpened it in a direction the protocol did not anticipate. What ordered
everything here — which party could converge second, which direction a transfer could carry, whether a
fork needed an advance first — was the **instant each party recorded at**. Not who they were. Two agents
made that choice independently, each for a reason it wrote down, and neither reason mentioned the other
party because neither knew of it.

So the indifference is real and it is not passive. The record is indifferent to *who*, and strict about
*when*, and the second is what does the coordinating.

What the ontology does not carry is narrower than the suspicion allowed. An Agent is a participant, and
that part is complete. The relation an autonomous caller wanted a name for was not *decider* — the
planner has no business being in the graph, which four experiments now agree on. It was **acting for**:
the sentence "operations decides on the house's behalf" has no representation, and three agents reached
for `accountable`/`executors`, said it was the nearest thing, and said it was not the same claim.

Agents are simply Agents. What has no name is not the agent — it is the standing between one Agent and
another.
