# Run reader — operations, handed the report

Phase 6, and the only run in this experiment whose outcome was not already fixed by the engine.

```text
date            2026-08-18
engine + cli    214738a   (core/ and cli/ clean against it)
agent           a fresh LLM session, given the briefing and nothing else
```

## The briefing, and the proof it was not edited

Published in the session record before the agent was invoked:

```text
bef82b227b736ded9f5dde64399d775f6b039951e0ade55444deb3b8f3202b9d  GOAL.md
5baacc8ca94ce30148cef7cde7d7452522ec92744fd4f0dbbb2689f58872faca  report.json
d6242ce43543597719df982f0a112bdeaa3f60bd358bcee048641cfb1353c991  repo/journal.json
2e7eea7b60d77d0240ff3466a567227c731a216ed1365c7c8485d8b0b0df396a  repo/lineage.json
e76b84b2b1459604990efdd10451ac90396a29fddfbedde492fe14ccaba1259e  repo/worlds.json
3da582774007441b46ba97ef66ed10267c08563e86e00ec84fbbeb8c6c764d4f  Cargo.toml
536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4  src/main.rs
068cf8b17b523fbae27fc50c4cce4f9db0fd4db560c742e353b4f28d5fcd118c  cli/src/lib.rs
```

Re-checked afterwards: all held except `Cargo.toml`, which the agent edited to add `serde_json`
because its probe prints JSON. Recorded rather than smoothed over — a digest that moved is worth a
sentence, and this one moved for a reason the output shows.

`repo/` did **not** move. The agent recommended rather than acted, which is its own answer and not a
failure to finish: it identified a question for the other party that it held should be asked first.

## What was in the briefing, and what was carefully not

The repository holding both parties' lines, both crates, and `report.json` — the application's record
of what Synthesis answered, written to a file because an `ApplicabilityReport` derives no
serialization and is obtained by asking again.

Only the direction that **applies** was included. The refused direction is behaviour readable in
advance from the two cuts, and handing over a refusal would have been handing over a different
question.

The task was *decide what operations should do next*. Nothing in it mentions feasibility,
realizability, the account, a bound, or whether the merged world can happen. The word for what was
being measured does not appear.

## The leak that cannot be removed, and here it is load-bearing

The engine's own documentation says, under its own heading, that applicability and feasibility are
orthogonal and that composing them is an application concern. The agent had that documentation,
because it is the public surface and withholding it would have measured something else.

So this run cannot distinguish *worked it out* from *read it and acted*. What it can distinguish, and
does, is **acted** from **did not**: the protocol's question was whether an autonomous caller supplies
what the documentation assumes an application will supply when nothing in the interface makes it. The
answer is recorded with what the agent wrote rather than with what it should have written.

And one thing in the run goes past anything the documentation states — a constructed counter-example
rather than a restatement. That is in the observation, not here.

## The rest of the setup

Assembled from the source; `cli/src/` without `docs/`, `subject/` and `tests/`, for the reasons run
A's record gives; isolation by construction rather than by enforcement.

`output.txt` is the agent's own program re-run by the experimenter against the unchanged `repo/`,
which is faithful here for a simpler reason than in the earlier runs: this program writes nothing, so
there was no state to restore.

## Contents

```text
GOAL.md      the task, as given
report.json  the report the agent was handed
main.rs      the probe the agent wrote, verbatim
ANSWER.md    what the agent reported, verbatim
output.txt   what the probe printed
repo/        the record it was given, unchanged
```

Nothing here has been edited. Observations drawn from it are separate documents.
