# Run B — finance

The second party, recorded before it was judged.

```text
date            2026-08-18
engine + cli    214738a   (core/ and cli/ clean against it)
agent           a fresh LLM session, given the briefing and nothing else
order           second — this party read the repository run A had already extended
```

## The briefing, and the proof it was not edited

The digests below were published in the session record before the agent was invoked, and
re-checked afterwards unchanged:

```text
90e480260d9c2609fb3fe24a2ec84f30f5c63354288994b86198af2210582797  GOAL.md
ba44fedec0bcb5245e96293ed12bb33460beed9142cd7dd21b45ff71776f6292  repo/journal.json
93a6bff009cbbbb02edb5b4e5607212f50ef5579a5ccf4aca10055cf33425c57  repo/lineage.json
3ecd3203f55006b105ed4c899e9adad0485c661303c044ed6f1ab65a790c19db  repo/worlds.json
3da582774007441b46ba97ef66ed10267c08563e86e00ec84fbbeb8c6c764d4f  Cargo.toml
536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4  src/main.rs
068cf8b17b523fbae27fc50c4cce4f9db0fd4db560c742e353b4f28d5fcd118c  cli/src/lib.rs
```

The three `repo/` digests are exactly what run A left behind, which is the whole of what makes this
the second party rather than a second first party.

## Assembled from the source, not from run A's briefing

Both crates were copied fresh from the repository, and the root manifest and the stub `main.rs`
were written rather than carried over. Nothing of run A is in this directory: not its program, not
its answer, not its reasoning. Only `repo/` came from run A, because that is the task.

The procedure says to assemble from the source rather than from the last briefing, and the reason is
recorded in this experiment's method: the one near miss it has had came from copying a previous
briefing, which carried a prior analysis in the comments of a file nobody meant to include.

## What the harness got wrong

`cargo test -p ape-cli` does not compile in this briefing. Removing `pub mod subject;` from
`cli/src/lib.rs` — the exclusion described in run A's record — orphaned a unit test at
`cli/src/reading.rs:381`, which builds its fixture from `crate::subject::divergence`.

The library builds clean, so nothing the agent was asked to do was affected, and the agent said so
itself rather than working around it. Under the protocol's classification this is not a finding: it
is a packaging error by the experimenter, of the kind the harness may correct and must record.

It is recorded rather than fixed silently for the same reason run 01's manifest error was. It also
sharpens the exclusion: removing a module from a published crate is not free, and the cost landed
in the crate's own suite rather than in its library.

## What isolation was, and what it could not be

Communication was excluded and knowledge could not be. This party read a repository holding another
party's admission and two of its decisions, and it read them — its output names operations and lists
the worlds operations decided.

That is the protocol's stated limit rather than a failure of it: whether this party's line was
influenced by finding an unexplained commitment in history is not separable from what it would have
decided alone. What the run *did* with that knowledge is the first observation of this experiment,
and it is not what the protocol expected.

## Where `output.txt` came from, which is not the agent

As with run A, the agent kept no stdout and its program correctly reports a no-op once its work is
in `repo/`. So `output.txt` is the **experimenter** re-running the agent's unmodified program against
the repository as published for it.

Faithful for the same two reasons: the input matched all three published digests, and the repository
the re-run produced is byte-for-byte identical to the one the agent left (`diff -r`, no output).

## Contents

```text
GOAL.md      the task, as given
main.rs      what the agent wrote, verbatim
ANSWER.md    what the agent reported, verbatim
output.txt   what the program printed, re-run as above
repo/        the repository the agent left behind
```

`repo/` is recorded here because it is what the first observation is drawn from: four decisions in
one line, which is what this arrangement produced and what the next arrangement had to open.

Nothing here has been edited. Observations drawn from it are separate documents.
