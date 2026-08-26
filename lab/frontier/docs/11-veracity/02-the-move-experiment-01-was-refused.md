# Observation 2 — Calibration: the move experiment 01 named was made, and it closed what 01 left open

**Not a finding. An instrument.** Every state below is produced by a hand editing a file, which is
the authenticity candidate's instrument and this experiment's excluded one. It exists so that the
search knows what its target looks like, and nothing later rests on it.

Experiment 01 tampered with two repositories in one place each and neither refused. It then wrote
down what would have closed it, and why it was not allowed to:

> A coordinate that named the *state* of knowledge rather than a position in it would be
> checkable […] That value is derived, and this experiment is not permitted to persist a derived
> value.

**That move was made four experiments later, for an unrelated reason.** `Taken::witness` arrived in
the coordination experiment because two parties needed to disagree about a prefix. Nobody went back
to ask what it closed. The three rows below are that question, asked.

## The three tampers, and what refuses each

```text
tamper                                   refused by              what it says

repoint `after` at another address       the witness             the journal offers an entry
the journal holds                                                the decision does not witness

swap the two entries around the Event    the witness             an Event's identity contains its
                                                                 predecessor, so both addresses moved
                                                                 and the witness names one that is
                                                                 no longer there

move the Event's recording instant       the worlds file         the chain it recognizes
from day 3 to day 11
```

**V2 is confirmed, and the first row is the whole of it.** The state experiment 01 measured — a
coordinate that is well-formed, present, resolvable, and wrong — is refused today. What refuses it is
the second representation that experiment wanted and could not have.

## The third row is where the hypothesis was aimed, and it is refused by something else

V3 said the recording instant is the claim nothing checks, and the first half of that is exactly
right: **the addresses do not move.** Measured directly — every `EntryId` the journal produces is
identical before and after the instant is changed, because a recording instant belongs to no
identity. So the witness has nothing to disagree with, and the guard experiment 01 wanted is blind
to this one by construction.

The record refuses anyway, and the reason had not been written down anywhere:

> The one derived value a recording instant decides is `event_head` — the head a cut resolves
> against — and `event_head` is a field of `worlds.json`. The instant is unchecked; its **consequence**
> is written down twice.

That is a real closure and it is narrower than it looks, which Observation 5 is about: it holds only
where something compares the two representations, and there is one operation in the application that
writes `worlds.json` from its own derivation.

## And an instant cannot move freely

The same tamper against the *whole* faithful line does not reach any of that. It is refused before a
decision is weighed at all:

```text
recorded_at 2026-01-04 precedes 2026-01-11, through which history is already recorded
```

Recording is monotonic across admission, so an instant that overtakes a later entry's makes the
**journal** inadmissible. This was not in the hypothesis and it bounds every later phase: a false
record built on a moved instant has to keep the sequence monotonic, so the instant can only move
within the gap its neighbours leave. In the faithful line that gap is a day wide. Between two
parties, it is as wide as the two parties are apart.
