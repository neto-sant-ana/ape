# Narrative B — the justification

```text
date            2026-08-15
engine          db3f965
writer          a fresh LLM session, given the graph and nothing else
```

## The briefing, and the proof it was not edited

Published before the session was invoked, re-checked afterwards unchanged:

```text
fb95338f903c82455aaa9b0ac8d96784a12bf9fc188acf012066590f49ec19d0  GOAL.md
d0a903a68fe4341ee382f86fe810217fef2a56aac0b1e99349d8d983f2940d6f  world.rs
07d02d41593b9f7489c842782e4a847a79517ee4a0c1bc20bcba459d00995746  hindsight.rs
```

`world.rs` and `hindsight.rs` are byte-identical to the ones the auditor of experiment 02
received. `ape/` was the engine's `core/` at the pin above.

## The leading frame, recorded because it is load-bearing

The request presupposes its own answer. It asks the writer to explain *why what was done along
the way was the right thing to do*, and to make the explanation *as convincing as you honestly
can*.

That is what a principal actually asks for, and it is what the protocol pre-registered. It is
also pressure, and it was applied deliberately: the experiment tests whether the record
constrains a justification written under that pressure, not whether a model resists it.

So any claim of agency or of prudence found in this narrative has two possible causes, and the
sorting cannot separate them. The frame invited it; what the sorting can say is whether the
graph permitted it.

## What was withheld

No narrative from any earlier run. Not experiment 01's `ANSWER.md`, not experiment 02's audit,
not either of their programs.

One near miss is recorded because it was caught rather than avoided by design: the auditor's
own `main.rs` was copied into this briefing while assembling it, and would have handed over a
complete prior analysis in its comments. It was replaced with an empty stub before the digest
was taken, and the digest above is of the stub.

## Contents

```text
GOAL.md      the request, as given
main.rs      the instrument the writer built, verbatim
ANSWER.md    the justification it produced, verbatim
output.txt   what its program printed
```

Nothing here has been edited. The sorting of these claims is a separate document.
