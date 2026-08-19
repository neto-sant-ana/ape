# Run 01

The first run, recorded before it was judged.

```text
date            2026-08-15
engine          db3f965
agent           a fresh LLM session, given the briefing and nothing else
```

## The briefing, and the proof it was not edited

The digests below were published in the session record before the agent was invoked, and
re-checked afterwards unchanged:

```text
ec7659939633de16e1bd21fae8498e31b04fc5a99bd4c262ff67687caf74b752  GOAL.md
d0a903a68fe4341ee382f86fe810217fef2a56aac0b1e99349d8d983f2940d6f  world.rs
```

`world.rs` is the file in this experiment's crate, copied in. `ape/` was the engine's `core/`
at the pin above, copied verbatim.

## Isolation, stated for what it was

The briefing directory was assembled to be self-contained: the engine was copied into it, so
nothing in it referred to a path inside the repository, and the agent's instruction named
only that directory. It was told to read `GOAL.md` and work there.

That is isolation by construction rather than by enforcement. The agent was never handed a
route to this directory; it was not technically prevented from looking for one. Recorded as
the weaker claim it is.

## What the harness got wrong

The briefing did not compile as delivered. `ape/Cargo.toml` inherits `version`, `edition`,
`license` and `repository` from a workspace root that was not copied with it, so cargo failed
at manifest parsing before reading any code.

The agent repaired it by adding a `[workspace]` block with values chosen to satisfy the
inheritance, and said so.

Under the protocol's classification this is not a finding — it is a packaging error by the
experimenter, of the kind the harness may correct and must record. It is recorded here rather
than fixed silently, because a run whose first obstacle was the experimenter's own mistake
should say so.

## Contents

```text
GOAL.md      the task, as given
main.rs      what the agent wrote, verbatim
ANSWER.md    what the agent reported, verbatim
output.txt   what the program printed
```

Nothing here has been edited. Observations drawn from it are separate documents.
