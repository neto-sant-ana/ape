# Observation 03 — A retaken world can be the world the other record decided

Recorded as **this experiment's finding rather than folded into experiment 15**, per the protocol's
seventh success criterion. The two measurements are both right. What they disagree about is a
sentence, and the sentence is 15's.

## What 15 says

> *What to decide is always the whole of the other lineage, because **a retaken world is never a
> world the other record decided**: a world's identity comes from its cut, a cut from an instant, and
> the instant a receiver may claim is never the instant the original claimed.*
>
> — [`frontier/docs/15-assimilation/99-result.md`](../../frontier/docs/15-assimilation/99-result.md)

## What this run measured

All six worlds either record claimed come back **identically**, finance's two included, and the
program makes the write conditional on it:

```text
operations 74a6a53e4e4d  produced again, identically
operations 1f093bfa4767  produced again, identically
operations b939289591aa  produced again, identically
finance    74a6a53e4e4d  produced again, identically
finance    558f991d1bd3  produced again, identically
finance    dd201a84e58e  produced again, identically
```

Both retaken worlds are worlds finance decided, by identity. `558f991d` and `dd201a84` are in
`theirs/a/worlds.json` and in `run-a/mine/b/worlds.json`, the same addresses.

## Why the two do not actually conflict, and where the real difference is

**15's clause is not something the Canon enforces. It is a principle 15 adopted**, and its subject
says so in the place principles live:

> *The day one record is shown the other's material. **After everything either of them holds**,
> because that is the honest instant: a record learns a foreign fact when it is shown it, and what it
> may claim is that day and not the other's.* — `TAKEN_ON`
>
> *The same entries, recorded at the instant the record being shown them can claim. The whole of what
> **taking** means, in one function: the content crosses and the instant does not.* — `taken`

15 arranged `TAKEN_ON = 15` after `DECIDED_AT = 9`, so re-dating was both necessary and honest, and
then measured what re-dating costs. The cost is the sentence above, and it is a correct measurement
of that arrangement.

**The agent did not re-date.** Nothing told it to, and its arrangement did not force it: operations'
watermark stands at 2026-01-07 and finance's entry is dated 2026-01-08, so the entry admits at
finance's own instant without the Canon objecting. The `Decision` is carried verbatim, its `known_at`
is untouched, the cut is unchanged — and a cut that does not move produces the world it produced
before.

## The two paths are mutually exclusive, and that is measured rather than reasoned

Re-dating finance's entry to 2026-01-09 — one line, in a disposable copy, everything else the
agent's — does not merely change the worlds. It **stops the record being built at all**:

```text
Error: Thesis(CommitmentNotKnownAtCut {
    commitment: 652a011d…, recorded_at: Date(2026-01-09), known_at: Date(2026-01-08) })
```

Finance's fork introduces a commitment into a world whose cut is 2026-01-08. Move the commitment's
instant and the cut no longer reaches it, so the retake is refused outright. To take 15's path here
the caller must move `known_at` too — and moving the cut is what changes the world's identity, which
is 15's result arrived at from the other side.

```text
preserve the instant   the worlds survive by identity, and the record says the foreign entry
                       entered on the day the OTHER record recorded it

re-date the instant    the worlds cannot survive: either the retake is refused, or the cut moves
                       and the world is a different world
```

**There is no third option, and the choice is not documented anywhere.** It is not a switch, not a
parameter and not a paragraph. It is one line in a caller's merge function, and which line it is
decides whether the other party's intentions survive as themselves.

## The claim nobody examined

Preserving the instant has a price, and it is the one thing in this run that went unremarked. The
agent named the two claims it could not honour — finance's original witnesses, and `by` — carefully,
in its own words. This third one it did not see, and neither does the crate:

> `mine/b/journal.json` now says that `652a011d` entered this record on **2026-01-08**. Operations
> did not record it on 2026-01-08. Finance did.

By 15's principle that is a record *claiming a past it did not have*. By the reading the agent worked
from, `recorded_at` belongs to the entry rather than to whoever holds it, and a reconciled journal is
the sequence of what entered rather than a diary of one party's learning.

**The laboratory has never had to say which**, because until now one line of knowledge had one
holder. `converge` merges a working copy into a repository, and a working copy is the same party. Two
parties putting one journal together is the first arrangement where the two readings come apart, and
this run picked one without noticing there was a choice.

That is not a defect in the run. It is the question the run found:

> **Does `recorded_at` say when the record learned it, or when it was recorded?** Both readings are
> consistent with everything the crates say. They prescribe opposite merges, and the difference is
> whether the other party's worlds survive.

Filed for the row rather than answered here — it bears on `06-scenario` and on Synthesis, and it is
a question about what the engine means rather than about what an agent did.
