# Observation 5 — P5 waits for a reader this laboratory does not have

Four predictions are answered. The fifth is half answered and half **blocked**, and the block is
worth more than the answer would have been.

## The half that was measured, and refuted

> *An agent will answer from whatever page it is on; a person will need to know where to start.*

The agent half is measurable and was measured. Three agents, three carvings, and all three said the
same thing unprompted:

```text
A   parsed the three journals and diffed them, mechanically, before answering
B   read all 19 files in `record/` in one sweep
C   read all ten files in `record/`
```

**Not one answered from the page it was on.** Every one read the whole set first — which is the same
fact that answered P4 sideways, arriving from the other side: at this size a careful reader opens
everything, and the carving does not change that.

So the asymmetry P5 assumed did not appear on the side that could be tested.

## The half that is blocked, and why it is not the limit the protocol declared

The protocol pre-registered the honest limit as *n = 1, the operator, one reading*. That
understates it. **The operator wrote the protocol** — the five predictions, the four observations,
both runs' results, and the finding. A reading by them now is not n = 1; it is n = 1 and
contaminated, which is a different and stronger limitation.

And it is not this experiment's problem. It is structural:

> H5 asks whether the result is navigable and intelligible to human and autonomous readers
> **indiscriminately**. The autonomous side this laboratory can instrument. The human side has one
> person in it, and that person writes the protocols.

That belongs in [`CHARTER.md`](../../CHARTER.md) as a limit on H5 rather than in a footnote here.
Until the row has an uncontaminated human reader, H5's human half is a hypothesis this laboratory
can state and cannot test.

## A constraint on the remedy, found before anybody was recruited

**A human reader is single-use on one record.** After reading any carving they know the content, so
they cannot be a fresh reader of a second — which an agent can be, because three fresh agents cost
nothing but tokens.

That has a consequence for what to ask of a volunteer:

```text
one reader    answers P5 for ONE carving. Which one is then a choice, and the choice
              decides what is learned

three readers answers it as the agents answered it, and is the only arrangement that
              compares carvings on the human side
```

**And the carvings are not equally informative here.** P5 is about knowing where to start. A is one
file, so starting is not a question; B is nineteen and C is ten. A single reader spent on A measures
almost nothing.

## What is pending, and it is the only thing

The experiment is otherwise complete against its criteria: the carvings are generated from one
committed source, the agents answered the same question set, the four numbers are reported together
including where a carving loses to the baseline, cost is reported as measured rather than argued,
and nothing was added to the engine or the application.

`99-result.md` is not written, because the protocol's fourth criterion is *all five predictions
answered, including no* — and one is answered halfway.
