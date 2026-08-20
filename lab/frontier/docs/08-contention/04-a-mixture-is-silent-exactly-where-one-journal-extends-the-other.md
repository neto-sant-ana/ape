# Observation 4 — A mixture is silent exactly where one journal extends the other

C3 predicted that the finer grain — a generation two writers share, written one file at a time — reaches
**the same six states** the atomicity experiment enumerated for an interrupted write. It does, and only
under one relation between the two writers. Under the other, none of the six is silent.

Phase 4 ran both. The variable is how the two journals stand to each other.

```text
DIVERGENT — both parties read the base, so the two journals are the same length
            and differ at the entry each added

the other writer's files   what a fresh reader makes of it
──────────────────────────────────────────────────────────────────────
journal                    refused: the journal holds no entry <…>
lineage                    refused: the journal holds no entry <…>
worlds                     refused: world 1 disagrees, in what it still proposes
journal + lineage          refused: world 1 disagrees, in what it still proposes
journal + worlds           refused: the journal holds no entry <…>
lineage + worlds           refused: the journal holds no entry <…>
```

```text
EXTENDING — one writer holds what it wrote; the other read it and holds the merge,
            so one journal is a prefix of the other

journal                    reconstructs: 17 entries, 2 worlds        ← silent
lineage                    refused: the journal holds no entry <…>
worlds                     refused: 2 worlds derived, 3 recorded
journal + lineage          refused: 3 worlds derived, 2 recorded
journal + worlds           refused: 2 worlds derived, 3 recorded
lineage + worlds           refused: the journal holds no entry <…>
```

The second table is the atomicity experiment's table, cell for cell. So C3 is **confirmed** under
`extending` and **refuted** under `divergent` — and the protocol's note about it holds where it holds:
this is not a new failure, it is the old set arriving through a door the repair did not close.

## The condition is the one the guard already tests

Both halves have one cause, and the atomicity experiment already wrote it down: a file can be replaced
in silence exactly where its consumers address a **prefix** of it, and the journal is the only file
here that is a source.

A journal that *extends* leaves every address and every witness set the standing decisions name
untouched, so the surviving lineage still resolves. A journal that *diverges* moves the sixteenth entry,
and the decision beside it names an entry the journal does not hold — which is a refusal, not a
silence.

Which means the condition for a silent mixture and the condition the compare-and-append refuses are
the **same condition, read in opposite directions**:

```text
extension    the merge accepts it   ·   a mixture of it can be silent
divergence   the merge refuses it   ·   no mixture of it is silent
```

Two writers dangerous enough to produce a silent mixture are exactly the two writers `converge` would
have let through. The parties it refuses cannot produce one.

## What the silent one is

The first writer's worlds over both writers' knowledge: seventeen entries, two worlds, answering for
intentions one of those writers never learned about. A repository neither of them wrote, and neither
would recognize.

## What it does not say

That the mixture is reachable through the application. It is not: a whole write is one call, and Phase
3 measured that no ordering of two of them leaves a state nobody wrote. Reaching a mixture takes the
single-file writes — the *record edited from outside* path five experiments need — aimed at a generation
somebody else had prepared. What Phase 4 establishes is what that door leads to, and that it leads
somewhere already measured.
