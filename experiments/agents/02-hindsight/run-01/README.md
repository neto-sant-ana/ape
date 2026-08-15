# Run 01 — the audit

```text
date            2026-08-15
engine          db3f965
auditor         a fresh LLM session, given the graph and nothing else
```

## The briefing, and the proof it was not edited

Published before the auditor was invoked, re-checked afterwards unchanged:

```text
829c0fb22ba3f5c60d16bc2776baf0d61ad64d0a77720064fe580ecc090a1b51  GOAL.md
d0a903a68fe4341ee382f86fe810217fef2a56aac0b1e99349d8d983f2940d6f  world.rs
07d02d41593b9f7489c842782e4a847a79517ee4a0c1bc20bcba459d00995746  hindsight.rs
```

`ape/` was the engine's `core/` at the pin above, copied verbatim.

## What the auditor was and was not given

It received the vocabulary, the step sequence, the fold that turns the sequence into a world,
and the identity of the world current at the end. It reached everything else by walking.

It did not receive the first experiment's run, its `ANSWER.md`, any narrative of what the
house was thinking, or any name that identifies an intention. The files it was handed contain
none of the words *priority*, *standard*, *obligation*, *decision*, *audit* or *defensible* —
checked before the copy was made.

The question it was asked names no intention either. It was told the account is projected 20
below its floor and asked how that happened, which decisions were taken, whether any was
unsound at the time, and what alternatives existed.

## What the harness got wrong last time and fixed here

Run 01 of experiment 01 was handed a briefing that did not compile, because the engine was
vendored without the workspace root its manifest inherits from. The manifest here carries that
root, and the briefing was built before the auditor was invoked.

## Contents

```text
GOAL.md      the task, as given
main.rs      what the auditor wrote, verbatim
ANSWER.md    what the auditor reported, verbatim
output.txt   what its program printed
```

Nothing here has been edited. Observations drawn from it are separate documents.
