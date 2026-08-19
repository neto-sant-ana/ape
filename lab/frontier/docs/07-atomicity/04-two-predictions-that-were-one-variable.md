# Observation 4 — Two predictions that were one variable

A3 and A4 were written as independent predictions. They are not, and the way A4 fails says so.

A4, as pre-registered:

> *Prediction:* in every partial state, including the two that are refused, the repository that existed
> before the write is **gone** — the files are written whole, so the previous journal is overwritten
> before the lineage is.

The claim quantifies over **every partial state**. Its justification quantifies over **one order** —
*the previous journal is overwritten before the lineage is* is true of the order the application
happens to write in, and it is the thing A3 was written to put under test. So A4 held the variable
fixed at the value A3 was about to vary, one prediction apart.

Measured: A4 holds in four of the six states and fails in two, and the two are exactly the ones that
become reachable once the order moves.

## What the protocol got right about it, and what it did not

It called this refutation the interesting one:

> A refuted A4 is the interesting refutation: it would mean the record protects itself and nobody had
> noticed.

Half of that is what happened. The record does protect itself, in two states of six. But it is not
something nobody noticed — it is corroboration's own witness doing a second job it was never described
as doing, which is the finding of Observation 3 and reads as obvious only from that side.

The other half is worth stating plainly, because a refutation invites overreading: the record protects
itself **where** something witnesses the length of what was replaced, and the file that carries the
knowledge is not one of those. So a refuted A4 does not overturn the experiment's question. It answers
it: *being refused is not being safe*, and being safe is not a property of the refusal but of which
file the interruption landed on.

## Why this is recorded rather than corrected in place

The protocol is not edited to match what happened, and a prediction whose justification was narrower
than its claim is the kind of defect that is invisible until the measurement arrives. Two predictions,
adjacent on the page, and the second one's reason was the first one's variable.

The rule this suggests for a later protocol is small and cheap: **a prediction's justification must
quantify over as much as its claim does.** A4 could have been written as *under the application's write
order, every partial state loses the previous repository* — which is true, is what its reasoning
supports, and would have made A3 and A4 visibly one experiment rather than two.
