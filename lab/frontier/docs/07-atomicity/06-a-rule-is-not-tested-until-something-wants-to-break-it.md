# Observation 6 — A rule is not tested until something wants to break it

Phase 6 has to decide a repair, and every shape Phase 5's requests point at wants the repository to
record a fact it does not record today. Which ran straight into a reason written beside `Taken::by`:

> Optional, and that is not a choice made here. Four concluded experiments hold repositories whose
> decisions name nobody, and a published result whose subject moved underneath it is a result nobody
> can run again.

Read as a constraint on Part B, that says a new persisted fact would invalidate published results. It
was offered here as exactly that, in as many words, before anybody measured it.

## Measured

```text
frontier/   versioned repositories: 0
            every suite builds its own from its subject, each run, in a temporary directory
agents/     versioned repositories: 2
            04-multiagent/run-a/repo and run-b-prime/repo, read as data by that row's suites
```

So the four experiments the reason names hold **none**. They write and read with one version of the
type, in a directory that does not survive the run. A change to the record's shape cannot reach them.

What does exist is two directories, in the other row, holding what two LLM parties wrote in a run
[nobody can perform again](../../../agents/00-question/05-the-re-runs-that-did-not-happen.md). That is
a real constraint and it is a different one: not *a published result would move*, but *a record's author
is gone*. And it dissolves rather than standing, because re-encoding a record whose content does not
change is not re-running the parties that chose it.

## What this experiment claims, and what it does not

The rule that came out of this — a published claim is kept true by the commit it was taken against; a
runnable arrangement is a bill; only an irreproducible artifact vetoes — is **not** a finding of this
experiment. It is now a rule of the laboratory, in [`lab/README.md`](../../../README.md), and it belongs
there rather than here because it governs every row.

What this observation records is the measurement above, and one thing about how the reason survived.

## How a wrong reason survives four experiments

Nothing had wanted to break it.

The reason was load-bearing for exactly one decision, and that decision's **conclusion was right** —
`Taken::by` should be optional, because a decision that claims nobody is the ordinary case rather than a
legacy one, and a mandatory field would make every such decision carry an invented agent. Right answer,
wrong reason, and nothing between the coordination experiment and this one asked the question again. A
reason nobody acts against is a reason nobody checks.

Which is the same failure a guard has when it is never seen red: it produces confidence without having
been made to bear any weight. A norm is an instrument too, and the moment it is first invoked to *stop*
something is the first time anyone finds out what it says.

The sweep that followed found no second decision made this way. Every other optional field in a
persisted record is optional because the fact it carries can genuinely be absent — a magnitude for an
action that is not quantifiable, an ancestor for a genesis, a head for an empty chain, a timeliness with
no deadline in play.

And one small thing found on the way: the laboratory's rules were introduced as *the same three
constraints* with four of them written underneath. A count in prose beside the list it counts is a second
representation of one fact, and it had already drifted.

## What it does not say

Nothing about APE, and nothing about whether the field should stay optional. The type is unchanged; only
the reason beside it moved, and whether a decision should be *required* to name a party is a question
this experiment did not ask.

And nothing about Part B's shape. The constraint is now the right size — two JSON files, migratable with
their originals kept — which makes the remedy a choice about what the format should say rather than a
choice about what the past forbids. That choice is Observation 7's.
