# Coordination

## Abstract

The provenance experiment refuted itself in both halves and handed one question forward. *Who was
consulted, and when* has no consumer in a repository with one writer and no party that decides, so
it was named for the experiment that has both.

This is that experiment, and the question changes shape on arrival. Provenance failed because its
answer was derivable — two routes to one content are one world, and a claim that predicts only the
transfer predicts what every rival account has in common. **Who decided is not derivable from
anything.** A decision is not knowledge; nobody admitted it; the Canon has never heard of it. If it
is not recorded it does not exist.

Which is a different kind of question, and it may have a different answer.

Underneath it sits a second thing, and it is mechanical. Four experiments have excluded concurrent
writers by name, and the exclusion has been doing more work than it looked like: this repository
writes each of its files **whole**.

> *What does a record need so that two parties can each reason, and then reach each other, without
> either erasing the other?*

---

## Question

Two halves again, and this time the second depends on the first rather than merely following it: a
question about who decided is idle if decisions are being lost.

**Can two writers converge?** Each reads the repository, decides, and writes. Nothing about that is
parallel — it is interleaved, which is a weaker thing and where the loss lives. So: what survives,
and does what survives depend on the order the writes landed in?

**Is a party that decides something a record must hold?** Not *can a field be added*. The engine
models agents that are parties to a commitment — accountable, executors, beneficiaries — and models
nothing at all about who selected a world. So: is that absence a gap, and if it is recorded, what
does it buy that a key would have to buy instead?

The two halves have different failure modes and the protocol keeps them apart. The first can be
refuted by measurement. The second can be refuted by collapsing into authenticity, which the
corroboration experiment named, measured and excluded — and which no amount of internal agreement
reaches.

---

## Hypothesis

```text
convergence   two writers can extend one repository so that neither loses the
              other's decisions, and the result does not depend on write order

identity      a party that decides is a thing the record must hold, and holding it
              buys addressing rather than proof
```

The second claim is deliberately weak, and the weakness is the point. It does **not** claim that a
recorded decider can be trusted. It claims something smaller: that naming one lets a party be
*referred to* — *apply this to the other line* — and that referring is useful even where proving is
impossible.

If that turns out to be either unnecessary or indistinguishable from an authenticity claim, the
second half is refuted and the experiment says so. The previous experiment established that this is
a real outcome and not a failure of nerve.

---

## Motivation

The engine solved the harder-looking version of the first half, one layer down, and did it
thoroughly. `append_event` is an atomic compare-and-append; a writer whose expected head has moved
gets `UnexpectedHead` and leaves no trace; and the conformance suite races many threads at one head
to prove an adapter serializes them.

The lineage has nothing of the kind. It is a file written whole by whoever writes last.

So the first half is not a research question about concurrency. It is a question about whether the
Canon's answer is the right shape one layer up — where what is being extended is not a chain of
facts but a set of decisions that may branch, and where two writers extending different branches are
not in conflict at all.

The second half is the one the previous experiment earned. Provenance measured that a repository of
decisions is enough to reproduce every world, and that nothing about a world depends on where its
intention came from. What it could not measure is whether anything depends on **whose** intention it
was, because in that boundary there was nobody for it to belong to.

---

## Experimental Boundary

This experiment exercises two parties reasoning over one repository.

It includes:

* interleaved reads and writes by two writers, deterministic and without threads;
* whatever the repository needs so that both survive;
* two lines of deliberation belonging to different parties, and a transfer between them;
* a record of who decided, if the procedure requires one;
* the corroboration discipline as it now stands, applied to whatever this experiment adds.

It deliberately excludes:

* **parallelism.** The loss this experiment is about needs interleaving, not simultaneity, and
  interleaving is reproducible where a race is not. Whether true simultaneity adds anything is a
  question this one is arranged to be able to ask and not to answer;
* **authenticity.** Who wrote a record, as opposed to what it claims. Named by the corroboration
  experiment, measured there, and unchanged;
* **two repositories meeting.** One repository, two writers. What relates two repositories has been
  an open question since convergence and stays one;
* snapshots, indexes, and any measurement of what replay costs;
* fractional magnitudes;
* mutable named references;
* production-oriented user experience.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the previous four give.

What this one must express, and no previous subject could:

```text
two parties            each deciding a line, over one body of knowledge
interleaved writing    each reading before the other has written
a shared ancestor      so the two lines are related and a transfer is coherent
a reason to reconcile  something one party holds that the other could
```

The two parties are **not** the kernel's agents. An `AgentId` is a party to a commitment — who owes
it, who executes it, who benefits — and nothing in the engine connects one to a decision. Whether
the two notions should meet is a variable this procedure leaves open and does not assume.

Everything else stays as thin as the previous subjects: integers, one quantifiable resource, no
dependencies unless the procedure demands them.

---

## Initial State

```text
repository = empty
writers    = two, neither having read anything
lineages   = none
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Part A — Can two writers converge?

#### Phase 1 — Lose a decision

Two writers read the repository, each decides, and each writes. Measure what is there afterwards.

The expected finding is a lost decision, and stating it in advance is deliberate: a phase that
expects a defect must say so, or discovering it reads as insight. What is *not* predicted is how it
presents — silently, as a refusal, or as a repository that reads and means something else — and
that is what the phase is for.

Both files are written whole. Whether the journal loses knowledge the same way the lineage loses
intention is asked here rather than assumed, because the two answer to different authorities and may
deserve different repairs.

---

#### Phase 2 — Converge

Repair it, under the discipline the corroboration experiment established and with the Canon's answer
in hand: an atomic compare-and-append, a named refusal when the expectation has moved, and no trace
left by a writer who lost.

Then measure the property that matters more than the repair: **write the same two decisions in both
orders and require the same repository.** A repair that serializes writers without converging is a
lock, and a lock is a different thing.

Whether two writers extending *different branches* are in conflict at all is part of this phase.
The Canon's chain has one head; a lineage that branches may have two writers who never met.

---

#### Phase 3 — Reach each other

One party adopts an intention from the other's line, through the transfer machinery convergence
built and provenance left alone.

This is where the earlier experiments' question finally has two parties in it. What the phase records
is what a transfer between *parties* needs that a transfer between lines did not — and the honest
answer may be nothing.

---

### Part B — Is a decider something the record must hold?

Part B runs whatever Part A concludes, on the terms the previous experiment set: a cost measured for
something unnecessary is knowledge, and what Part B builds does not stay unless Part A required it.

---

#### Phase 4 — Establish that it is not derivable

Ask the repository who decided each world, and establish by asking that nothing answers.

This is not the same shape as provenance's fifth question. There the answer was a world, and worlds
are derived; here the answer is a party, and no party appears anywhere in a decision. Confirming that
rather than asserting it is the phase.

---

#### Phase 5 — Record it, and separate addressing from proof

Record a decider, and answer both of the corroboration experiment's questions:

> *What becomes impossible if this is not preserved?*
> *What compares it, on every read?*

The second question has a known and uncomfortable answer, and the phase's job is to state it
precisely rather than to soften it: **nothing internal compares a claim about who.** So the phase
must find whatever the record *can* check — that the party named exists, that it decided the worlds
attributed to it, that a lineage's decisions do not claim two authors — and then say plainly where
the checking stops.

If what is left is only a label nothing weighs, the second half is refuted and the reason is
authenticity rather than design.

---

#### Phase 6 — Terminate, rebuild, compare

The whole arrangement, through a fresh process, against what a living one recorded — and against
literals written down before the run.

---

## Success Criteria

The two halves are judged separately, and the experiment is confirmed only if both hold.

**Convergence**

1. A decision lost under interleaved writing is demonstrated on a repository rather than described.
2. After the repair, both writers' decisions survive, and a writer who lost leaves no trace.
3. The same two decisions written in either order produce the same repository, compared whole.
4. Two writers extending different branches are shown either to conflict or not to, measured rather
   than argued.

**Identity**

5. That no existing record answers *who decided* is established by asking.
6. What a decider record adds is stated as a closed set, and answers both of Phase 5's questions —
   including where the second has no answer.
7. Whatever can be checked about it is checked inside reconstruction, and what cannot is named.
8. The arrangement reproduces whole through a fresh process.
9. No coordination-specific concern is introduced into the APE engine.
10. The four earlier experiments' conclusions stand, or the change is recorded as a result of this
    one.

---

## Failure Conditions

The hypothesis is refuted, or narrowed, if:

* interleaved writing loses nothing, so there was no convergence problem to solve;
* the repair serializes writers without converging, so write order survives into the repository;
* two writers on different branches must be serialized against each other, which would make a
  branching lineage no better than a chain;
* nothing about a decider is checkable, so the record holds a label and calls it knowledge;
* recording a decider requires the engine to know who decides;
* the second half turns out to be authenticity under another name.

**A refuted second half is a likely outcome and is a complete result.** The previous experiment ended
that way and said the strongest thing it had to say in the process. This one must not rescue identity
by inventing a consumer, and must not confuse *useful for addressing* with *worth trusting*.

---

## Variables Deliberately Left Open

### Whether the two notions of agent should meet

The kernel has agents who are parties to commitments. This experiment has parties who decide. They
may be the same population, disjoint, or overlapping, and nothing here assumes it.

### Whether the journal and the lineage deserve the same repair

Knowledge is not revisable and intention is. The Canon's compare-and-append is built for the first;
applying it unchanged to the second would be inheriting a shape rather than choosing one.

### The adapter's declined contract

The CLI runs the engine's single-threaded conformance and declines `verify_thread_safe`, saying so in
writing. That stands unless a phase needs it — and if the repository is the shared medium, two
writers need no shared adapter at all, which would make the declined contract a finding rather than
a debt.

### Abandoned siblings

Deferred by five protocols. The provenance experiment measured a cost against it for the first time,
and this one adds a second party who may not agree that a line is abandoned.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure.

The previous experiments' conclusions are not revised here. Where this one finds something that would
have changed them, it is recorded as a finding of this experiment, against the implementation as it
then stood.

And the editorial rule the last two experiments needed: before recording anything as a finding, ask
whether it is written in a docstring in `core/`, and ask whether the behaviour being described would
be better the other way. A defined behaviour reported as friction sends the next reader looking for a
repair nobody needs.

---

## Expected Pressure Points

### The shared medium is the repository, not the memory

Every prior experiment shared a `Canon` between its phases because it had one writer. Two writers
each replaying the journal into a Canon of their own is the architecture this project is modelled on,
and it means the thing that must serialize is a **file**, not a data structure. Whether the engine's
port has anything to say about that is exactly the question.

### Convergence is not serialization

A repair that makes writers take turns removes the loss and leaves the order in the result. Every
experiment so far has compared repositories whole; this is the first one where two correct runs could
legitimately differ, and the criterion is that they must not.

### A branching lineage has no single head

The Canon's compare-and-append is against one head because a chain of facts has one. A lineage
branches by design — that is what convergence established — so two writers extending different
branches may be extending different heads, and treating them as contending would be borrowing a
constraint the shape does not have.

### Nothing weighs a claim about who

The corroboration rule says a derived value written and not compared is a liability. A decider is not
derived, so the rule does not forbid it — and it does not protect it either. What the record can check
about a party is a smaller set than it looks, and the phase that finds the boundary has to be willing
to report that the boundary is close.

### Two parties may disagree about what is abandoned

One writer's discarded alternative is a world another writer may have adopted from. Provenance
measured that a claim keeps a world alive; a second party is a second reason a world cannot be
dropped, and this one is not a claim anybody wrote.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* what an exclusion four protocols repeated was hiding;
* places where the engine's answer one layer down was the wrong shape one layer up;
* what a party buys that is not proof;
* claims that could be written and not weighed;
* assumptions a single writer made without stating them.

Where possible, record the smallest reproducing case.

---

## Open Questions

* What relates two repositories, as opposed to two writers of one?
* Does a party that decides have a lifetime, and what happens to its decisions after it?
* Can two parties disagree about canonical knowledge, or only about intention?
* What does a lineage that was discarded owe to a party that adopted from it?

These are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

Reconstruction asked whether meaning survives.

Divergence asked whether deliberation survives.

Corroboration asked whether the record can be trusted without being believed.

Convergence asked whether two lines of deliberation can still reach each other.

Provenance asked whether a record that can be rebuilt perfectly is thereby understood, and answered
that it does not need to be.

```text
This one asks whether a record built for one
mind can hold two — and whether the second one
needs a name.
```

The experiment will determine what a repository must record for that to remain true.
