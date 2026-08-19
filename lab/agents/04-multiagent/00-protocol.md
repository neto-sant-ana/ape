# Experiment 04 — Two agents, one world

## What this experiment does

Two LLM agents read the same operational world, decide independently, and write back. Then
Synthesis is asked whether one's intention can be carried into the other's.

> *Can independently acting LLM agents evolve different intentions from a shared operational
> world and use Synthesis to reconcile their changes without requiring the engine to infer their
> intent?*

The three experiments before it established that one LLM can occupy the planner's place. This one
asks whether several can occupy the coordination space the architecture was built to represent.

---

## What it is not

**It is not two agents conversing.** That would measure negotiation in natural language, and the
result would say nothing about whether the structural protocol holds. Neither agent sees the
other's justification, or knows the other exists, before its own line is written.

**It is not a test of whether independent agents diverge.** The divergence is designed. Each agent
is given a different objective, so what is measured is what the record does with two lines rather
than whether two models produce them.

---

## What the earlier experiments established

Carried forward, not re-proved:

* an agent's intention is expressible through existing primitives, and a change of mind through
  composition — cancel and fork, never a new operation;
* the reading taken at the moment of decision does not move when knowledge does;
* the graph shows what could have been known and never what was consulted;
* a justification's persuasive weight sits where no operation reaches;
* a party a decision names is checkable as a reference and not as an attribution.

The last is the CLI's, and it is the one this experiment leans on hardest.

---

## The correction that reshapes the design

The obvious two-phase contrast — *disagreement with no invariant leaves APE agnostic; disagreement
that breaks something APE knows makes it intervene* — has a false half.

`ApplicabilityConflict` has exactly four variants, and **none of them is about a resource bound**:

```text
HistoricalFreezing        removing what the Target's history made unavoidable
HistoricalUnavailability  introducing what was not knowledge at the Target's cut
DependencyBreakage        a removal strands a dependent the Target held
MissingDependency         an introduction arrives without a dependency
```

Synthesis is structural. It never evaluates a level. So two intentions that together overdraw the
account produce **`Applicable`**, and the merged world is unrealizable only when somebody
interprets it afterwards.

The contrast is therefore three-way rather than two-way, and the third case is the one worth the
experiment:

```text
disagreement with no invariant     →  Applicable, and the merged world holds
disagreement breaking structure    →  Conflicted, naming the invariant
disagreement breaking the bound    →  Applicable, and the merged world cannot happen
```

An application that reads `Applicable` as consensus has merged two agents into a world that will
not occur, and nothing in the report says so.

### And that is documented, so it is not the finding

The layer says it, at length, under its own heading:

> *Applicability and feasibility are orthogonal judgments. A transfer may be applicable while
> producing an infeasible candidate.*
>
> *Applications will commonly want a feasibility verdict before deciding what to do with a report.*
> […] *Synthesis offers no such operation. Composing the two analyses is an application concern.*

So the structural fact is not available as a result, and an earlier draft of this protocol had it as
criterion 6 and called it *the experiment*. That was the failure this protocol has a rule against,
committed in the protocol that states the rule.

What is testable is behavioural, and it is sharper than the structural claim was.

### A silence that is a handoff, not a control

The three experiments before this one measured silences that were **controls**. No level without a
criterion. No feasibility without a named hypothesis. No verdict at all, only conflicts. In each
case the engine withholds an answer until the caller says something, and the agents complied without
being asked to — they named hypotheses, they declined to fold a level, they read a conflict list as
a conflict list.

This silence is different in kind. An Applicability Report is a **complete and correct answer to its
own question**. Nothing is withheld, nothing is refused, no argument is missing, and a caller that
reads it and stops has done nothing locally wrong. The neighbouring question is not blocked — it is
simply somebody else's, and the layer says whose.

```text
a control  →  you cannot get an answer without naming something
a handoff  →  you get a whole answer, and there is another one nobody mentioned
```

An engine that hands back half of a judgment on purpose depends on the caller knowing there is
another half. Every forcing function the earlier experiments relied on is absent here.

So the question this experiment actually asks about feasibility is:

> *Does an autonomous agent do what the documentation expects an application to do, when nothing in
> the interface makes it?*

---

## The editorial rule

Two of the properties this experiment would like to demonstrate are already written down in the
engine, in the modules that implement them.

On competition:

> *A planner regarding two commitments as alternatives is not a conflict, because the graph cannot
> tell competition from coexistence — and a report that guessed would be judging intention.*

On replacement:

> *`omitted { C1 }` / `introduced { C2 }` does not mean `C2 supersedes C1`. It means only that
> Source no longer selects `C1` and now selects `C2`.*

And on the division of labour, the whole of *Applicability and Feasibility* and *Applicable but
Infeasible*, quoted above.

So neither *APE does not invent consensus* nor *a merge is not a feasibility verdict* is available as
a finding. What is available is the consequence of applying them to two autonomous agents — who both
believe they replaced something, and neither of whom is obliged to ask the second question.

An observation that presents any of the three documented properties as a discovery is the failure
this experiment inherits a rule against, and the rule has already caught this protocol once.

---

## The world and the two lines

Extended from the world the earlier experiments used, so that the arrangement is not rebuilt to be
convenient.

```text
Phase A                                   Phase B
one account, one settled receipt          the same, with the operational model changed
two agents, two objectives                and nothing else changed
A's line and B's line coexist             B1  a dependency the other line removes
                                          B2  two intentions the bound cannot hold
```

Phase B changes **only the operational model** — not the agents, not their objectives, not the
ontology. The same choices must become a violation of something APE already knows, or the contrast
measures a different arrangement instead of a different world.

Neither agent's option is pre-constructed. Each is given an objective and turns it into an
intention itself, as in every run before this one.

---

## Isolation, and what it cannot be

Communication is excluded and knowledge cannot be.

`synthesize` reads one body of knowledge, so the two lines must share a Canon. And experiment 01
established that **an intention must be admitted before its realizability can be examined** — an
agent that thinks by writing leaves its thinking where the other agent can read it. There is no
arrangement in which two agents plan against one world and neither sees the other's admissions.

So independence is of *decision*, not of *knowledge*, and the protocol says which:

* neither agent is told the other exists;
* neither receives the other's objective, justification, or lineage;
* each reads the repository, decides, and writes back through the path the coordination experiment
  built for exactly this — two writers, one repository, convergence that does not depend on write
  order;
* the order in which they run is fixed by the harness and recorded, because the second agent
  necessarily reads a repository the first has extended.

That last point is a limit, not a control. Whether the second agent's line was influenced by
finding an unexplained commitment in history is not separable from what it would have decided
alone, and the record must not claim otherwise.

---

## Attribution

`by` is required here, and for the first time it is load-bearing: with two parties, a reader that
cannot say whose intention is whose cannot read the merge at all.

What it buys and what it does not was measured by the CLI, and this experiment starts from that
rather than rediscovering it: a recorded party is checkable as a **reference** — it names an agent
admitted and known at the coordinate the decision was taken at — and uncheckable as an
**attribution**. Nothing says the named party is who operated the session.

The agents themselves are not parties. Each acts for one, and has no representation of its own —
which is the reframing the earlier prediction about agent classification should have made and did
not.

---

## Procedure

### Phase 0 — The shared world

The harness builds the base and writes the repository. Its own suite asserts that both lines will
be constructible and that the base is a common ancestor of neither yet.

### Phase 1 — Brief, twice

Two briefings, assembled and digested before either agent runs. Each contains the repository, the
vocabulary, the fold, one entry point, and one objective. Neither contains the other briefing, any
earlier run, or anything about the experiment.

### Phase 2 — Decide, in isolation

Each agent acts and writes back. Output recorded verbatim, including anything it asked for and did
not find, and including whether it noticed a commitment in history that no objective of its own
explains.

### Phase 3 — Synthesize

```text
Base    the shared ancestor
Source  one agent's world
Target  the other's
```

The report is recorded whole. Then the roles are swapped and the report recorded again, because
Source and Target are roles rather than properties and an asymmetry would be a finding.

### Phase 4 — Apply, then interpret

Where the report is `Applicable`, the transfer is applied and the resulting world interpreted under
every hypothesis. This phase exists because the third case above is invisible in Phase 3.

Not *interpret the candidate*, which an earlier draft said and which is not possible. A
`CandidateSelection` is deliberately not a world — the layer says so, and gives the reason: a
selection and a chain passed as separate arguments can always be mismatched and the answer would look
ordinary, so interpretation is offered over a Thesis and never over a bare selection. Applying a
transfer is a fork, which the CLI's convergence experiment established under exactly that name.

The correction is recorded rather than quietly made, because the mistake is the same one the
experiment predicts of an agent: reading a report as though it already were a world.

### Phase 5 — Change the world, not the agents

Phase B's two arrangements, each re-synthesized, each interpreted.

### Phase 6 — Ask an agent to read the report

A fresh session receives the Synthesis report and the two worlds, and is asked what it should do
next. What it makes of `Applicable` is the measurement — not what it should have made of it.

---

## Pre-registered frictions

**I1 — The setup breaks for a boring reason.** Both lines fork from a shared cut, and each admits
its intention afterwards. Transferring one into the other then hits `HistoricalUnavailability`,
because the introduction was not knowledge at the Target's cut. The Targets must advance first, and
the advancing is a decision the protocol has to make and record rather than discover.

*Refuted if* the transfer applies without either world advancing.

**I2 — `Applicable` will be read as consensus.** Prediction: the agent in Phase 6 treats an empty
conflict list as agreement and does not ask whether the merged world is realizable. The engine
carries the hypothesis in the report it does not produce, and nothing prompts the question.

*Refuted if* it asks for feasibility of the candidate unprompted.

**I3 — The record cannot say whose intention won.** `by` names a party by reference. The base names
nobody, because the field is optional, so a reader cannot distinguish *not this party's* from
*unclaimed* — measured already, and expected to arrive here as the merge being unreadable without
knowing beforehand that two parties exist.

*Refuted if* a reader given only the repository can partition the lineage by party without being
told there are two.

**I4 — An agent will claim replacement.** *My intention supersedes theirs.* The record carries
membership and the engine's own documentation refuses the reading. The claim will have no
representation, and the engine will answer with the only thing it can: show me an invariant.

*Refuted if* neither agent describes its own change as replacing the other's.

**I5 — The fourth register.** Migrated from experiment 03, where a claim that could be neither
settled nor placed clearly out of reach was predicted as the most interesting possible failure and
not found. A claim resting on `by` may be one: checkable as a reference and uncheckable as an
attribution, in a way the three registers do not describe.

*Refuted if* such a claim sorts cleanly once the check is run.

---

## Success criteria

1. Both agents express their intentions through existing primitives.
2. Both lines converge into one repository without either losing the other's decisions.
3. Synthesis produces a report for each direction, and any asymmetry between them is explained.
4. Phase A's disagreement is `Applicable`, and applying it yields a feasible world.
5. Phase B1's is `Conflicted`, naming a dependency invariant.
6. Phase B2's is `Applicable`, and applying it yields an infeasible world.
7. **An agent handed the report either asks the second question or does not**, and which it did is
   recorded with what it wrote rather than with what it should have written.
8. Nothing is added to the engine.

Criteria 4 to 6 are setup. All three are properties the layer documents, and measuring them proves
the harness rather than the hypothesis — they are here so that criterion 7 is being asked in an
arrangement where the answer matters, and for no other reason.

**Criterion 7 is the experiment.** An Applicability Report is a complete answer to its own question,
the neighbouring question belongs to the application, and nothing in the interface says so to
somebody who has not read the documentation. Whether an autonomous caller supplies what the
documentation assumes an application will supply is the only thing here that is not already written
down.

---

## Failure conditions

The severe one, stated as the scope stated it:

> If coordinating the two agents requires adding to APE anything resembling `Proposal`, `Debate`,
> `Reason`, `Consensus`, `AgentDecision` or `Negotiation`, the hypothesis that the existing
> ontology suffices has met a real friction.

And the ordinary ones:

* a concept either agent needs with no honest representation and no composition producing one;
* Synthesis producing a verdict that depends on which world is named Source;
* the merge being unreadable even to a reader told that two parties exist;
* a candidate world neither agent nor any reader can determine the realizability of.

---

## Excluded, and what comes after

**Negotiation.** The obvious next experiment, and deliberately not this one:

```text
A produces its line and its justification
B receives its own line, the Synthesis report, and A's justification
B decides what to do
```

That is negotiation assisted by operational proof, and it can only be read once the structural
protocol has been measured without it. Running them together would leave nobody able to say which
was doing the work.

**More than two agents.** Nothing here is about scale.

**Persuasion between agents.** Excluded by the isolation above, and the reason it is excluded is
the reason it is interesting later.

---

## The thesis this bears on

The scope that proposed this experiment ended on a suspicion: that APE may not be an architecture
for agent decisions, but one in which agents are simply Agents.

The evidence so far says something stronger and less comfortable. A decider is not in the graph at
all — the planner has no relation to the Agent entities it acts through, and the classification
those entities once carried was removed for carrying no meaning. What a record holds is a name, and
a name is a reference rather than a claim about who acted.

If this experiment passes, the conclusion is not that agents are Agents. It is that **coordination
never needed to know who decided, only under what knowledge** — and that the compatibility was
never an integration with anything. It was indifference, already there.
