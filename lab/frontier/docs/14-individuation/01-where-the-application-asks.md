# Observation 1 — Where the application asks, and a subject with two varying instants

*The same entry* is not a feeling. It is a set of behaviours, and the phase that reads them derives the
set out of the application rather than listing it:

```text
journal::replay_through   where a replay stops
lineage::corroborate      which entries a prefix has
lineage::diagnosed        whether the coordinate resolves to something admitted twice
converge::appended        two journals, position by position
converge::ordered         where each decision was taken, which linearizes a merge
```

Five. The scan reads every source file in `cli/src`, finds each function whose body holds both an
address-shaped token and a comparison, and the guard is that the set it returns equals the set the
subject names — so a function that starts comparing addresses turns it red and names itself.

**Its one limit is written into the guard rather than left to be found.** The scan is lexical, so
`converge::appended` is found because its body binds `entries` and not because the line
`found != expected` says anything about an address. A comparison in a function that mentions neither
would be missed, and nothing here rules that out. Seen red by pointing the scan one directory up:
`the scan read 0 files; it did not break`.

## The subject, and the one thing it adds to experiment 13's

```text
F   receive 400   committed day 2, recorded day 2         selected
G   receive 401   committed day 3, recorded day 3 or 4     selected by nobody
E   Event settling F, occurred day 3, recorded day 4 or 9

D   Genesis { known_at: day 6, selection: { F } }, taken after E
```

**Two instants vary and only one of them is the coordinate's.** That is the addition, and it is what
lets the central prediction fail: under an address that says *when*, a coordinate whose instant is the
only one that varies would fix the reference by itself, and the witness would be credited with work
the arrangement did.

Measured rather than asserted — the phase reads the recording instant of every entry in both
candidates and reports the positions that differ:

```text
here and late                 differ at position 12, the entry the coordinate names
here and early-noticed-late   differ at position 11, an entry it does not
```

and then reads the coordinate of `early-noticed-late` and finds it equal to the record's own. So that
candidate is one the coordinate alone cannot exclude, under either shape.

Three more candidates are there for the rows the change should **not** move: a reordering, which
differs by sequence and not by membership; an insertion, which differs by an entry being there at all.

## The two records founded apart

```text
here     the whole base, recorded on the days above
later    the same content, every entry recorded one day later
```

Both read on their own and both answer `400`. Their worlds are equal **by identity**, with nothing
copied between them — which is experiment 09's ground, re-read here rather than inherited, and it is
the number the change is predicted to move.

## And what the workspace holds on disk

Counted before anything is built, because a compilation error lands in a minute and a repository that
stops replaying lands only when somebody runs the suite that reads it.

```text
4 committed repositories    lab/agents/04-multiagent/run-*/repo
2 files that name them      lab/agents/tests/merge.rs, lab/agents/src/coordination.rs
```

The phase also reads one of those files and asserts the coordinate it holds is 64 characters — a bare
address, as it was written. That is the fact that decides whether they are a cost: a shape that writes
anything else writes something these cannot be read against.
