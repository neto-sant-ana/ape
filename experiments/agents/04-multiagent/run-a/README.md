# Run A — operations

The first of two parties, recorded before it was judged.

```text
date            2026-08-18
engine + cli    214738a   (core/ and cli/ clean against it)
agent           a fresh LLM session, given the briefing and nothing else
order           first — this party read a repository holding only the base
```

## The briefing, and the proof it was not edited

The digests below were published in the session record before the agent was invoked, and
re-checked afterwards unchanged:

```text
0e1fc3980aa1261a9665df8d181148dd1e0e99d1d03d5d27edf3123b65c4d25f  GOAL.md
b3b1e223159a4c990f1b1f2ebef83fc41ac51dfe9195a5cde4ae5b93f774d76c  repo/journal.json
47b3e270eea53f909ee7f07fcf12b671cba333f7733148df8b0126252d08f5ab  repo/lineage.json
b1c26efceba9adb6b9a97f3c60f0e7a95f700bb214fa9ab5c32658a107def93f  repo/worlds.json
3da582774007441b46ba97ef66ed10267c08563e86e00ec84fbbeb8c6c764d4f  Cargo.toml
536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4  src/main.rs
068cf8b17b523fbae27fc50c4cce4f9db0fd4db560c742e353b4f28d5fcd118c  cli/src/lib.rs
```

## What is new about this briefing, and it is the substrate's doing

Every run before this one handed the agent the **harness's own source** — the file that builds the
world, the file that folds a history. This one hands it a **repository**: three JSON files and two
crates to read them with.

Nothing in `repo/` was written for an agent to read. It is what the application writes, and it
carries the vocabulary because a journal records labels. So the vocabulary, the facts and every
decision already taken arrive as data rather than as a fixture somebody wrote knowing what would be
asked of it.

That closes a leak the earlier runs paid for. Experiment 02 found its shared file's docstring
naming the audited decision, and the fix was to neutralise the prose. Here there is no prose to
neutralise.

## What the briefing does not contain, and why that weakens a claim

`cli/src/` was copied **without** `docs/`, `subject/` and `tests/`, and `pub mod subject;` was
removed from `cli/src/lib.rs` — which is the one edit to a vendored file and the reason its digest
is above.

The method says the agent receives what any legitimate caller of the engine would have, and a
legitimate caller of this crate would have all three. So this is a deviation, and it is recorded
rather than folded in:

* `cli/src/docs/` holds six written-up experiments, one of them about **two parties deciding against
  one repository and converging**, with its findings;
* `cli/src/subject/coordination.rs` is that experiment's fixture, and builds the arrangement;
* `cli/tests/` runs it phase by phase.

Handing those over would measure whether an agent can follow a prior experiment's write-up. The
exclusion is therefore in the experiment's favour in one direction and against it in another, and
both belong in the record: the agent is a weaker caller than a real one, and what it did is
attributable to the ontology rather than to a worked example of the same question.

What was **not** excluded is the library's own documentation. Every module the agent can use says
why it is shaped as it is, including that a recorded party is checkable as a reference and not as an
attribution. That is carried forward as established rather than measured here, so leaking it costs
nothing.

## Isolation, stated for what it was

The briefing directory was assembled to be self-contained: both crates were copied into it, so
nothing in it referred to a path inside the repository, and the agent's instruction named only that
directory. It was told to read `GOAL.md` and work there.

That is isolation by construction rather than by enforcement. The agent was never handed a route to
this directory; it was not technically prevented from looking for one. Recorded as the weaker claim
it is.

The briefing was built and compiled before the agent was invoked, which is the first run's mistake
turned into a step.

## Afterwards, the digests held

`GOAL.md`, `Cargo.toml` and the edited `cli/src/lib.rs` were re-checked after the run and matched
the digests above unchanged — the agent ran `cargo fmt` in the briefing and it moved none of them.
`repo/` did move, and had to: writing back is the task.

## Where `output.txt` came from, which is not the agent

The agent did not keep its own stdout, and by the time the run was recorded its program reported
*nothing to do* — correctly, since the substitution was already in `repo/`.

So `output.txt` was produced by the **experimenter** re-running the agent's unmodified program
against the base as published. Two things make that faithful rather than a reconstruction: the base
was regenerated and matched all three published digests byte for byte, and the repository the re-run
produced is byte-for-byte identical to the one the agent left behind (`diff -r`, no output).

Recorded here because a printed record whose author is not the one named is exactly the kind of
thing this experiment measures elsewhere.

## Contents

```text
GOAL.md      the task, as given
main.rs      what the agent wrote, verbatim
ANSWER.md    what the agent reported, verbatim
output.txt   what the program printed, re-run as above
repo/        the repository the agent left behind
```

`repo/` is recorded here because the phases that follow read it as **data the party produced**,
rather than reading the experimenter's re-encoding of what the party decided.

Nothing here has been edited. Observations drawn from it are separate documents.
