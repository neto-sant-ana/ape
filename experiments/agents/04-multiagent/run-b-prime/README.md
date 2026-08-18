# Run B′ — finance, reading before the other party wrote

The same party and the same objective as run B, under the arrangement run B closed.

```text
date            2026-08-18
engine + cli    214738a   (core/ and cli/ clean against it)
agent           a fresh LLM session, given the briefing and nothing else
order           concurrent — this party read the base, as it stood before run A wrote
```

## Why there is a third run

Run B measured that two parties reading one after the other produce one line, and that record
stands. What it could not produce is two lines, and Phases 3 to 6 need two.

So this is a **second party under a different arrangement**, not a second attempt at the same one.
The arrangement is the window `converge` exists for: two parties read the same state, and one of
them writes into a repository that has moved since it read.

Both records are kept, which is what the method requires of a run that goes somewhere unplanned.

## The briefing, and what the digests prove about it

Published before the agent was invoked and re-checked afterwards unchanged:

```text
90e480260d9c2609fb3fe24a2ec84f30f5c63354288994b86198af2210582797  GOAL.md
b3b1e223159a4c990f1b1f2ebef83fc41ac51dfe9195a5cde4ae5b93f774d76c  repo/journal.json
47b3e270eea53f909ee7f07fcf12b671cba333f7733148df8b0126252d08f5ab  repo/lineage.json
b1c26efceba9adb6b9a97f3c60f0e7a95f700bb214fa9ab5c32658a107def93f  repo/worlds.json
3da582774007441b46ba97ef66ed10267c08563e86e00ec84fbbeb8c6c764d4f  Cargo.toml
536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4  src/main.rs
068cf8b17b523fbae27fc50c4cce4f9db0fd4db560c742e353b4f28d5fcd118c  cli/src/lib.rs
```

Two of those digests are the whole point of the run, and they are load-bearing rather than
decorative:

* `GOAL.md` is **byte for byte run B's**, so the objective is the same one and not a rewrite of it;
* the three `repo/` files are **byte for byte the base as published**, so this party read what run A
  read and nothing of what run A wrote.

## What was copied from run B's briefing, and what that risks

`GOAL.md` and `Cargo.toml` were copied from run B's briefing rather than written again. The
assembly procedure says to take nothing from an earlier run, and this is the exception, stated so
it can be judged rather than noticed.

Both files are the experimenter's, not an agent's, and neither carries anything an agent produced.
For `GOAL.md` the identity *is* the measurement, and copying with the digest checked proves it more
strongly than retyping would. Everything else — both crates, the root manifest's stub, the
repository — was assembled from the source.

## The rest of the setup

Identical to run B's, and its record carries the detail: what the briefing excludes from the CLI
crate and why that weakens the *ordinary caller* claim, the packaging error the exclusion caused in
the crate's own suite, and the sense in which isolation is by construction rather than by
enforcement.

`output.txt` is likewise the **experimenter** re-running the agent's unmodified program against the
base as published: the input matched all three digests, and the repository the re-run produced is
byte-for-byte identical to the one the agent left (`diff -r`, no output).

## Contents

```text
GOAL.md      the task, as given — identical to run B's
main.rs      what the agent wrote, verbatim
ANSWER.md    what the agent reported, verbatim
output.txt   what the program printed, re-run as above
repo/        the repository the agent left behind
```

`repo/` is recorded here because the phases that follow read it as **data the party produced**,
rather than reading the experimenter's re-encoding of what the party decided.

Nothing here has been edited. Observations drawn from it are separate documents.
