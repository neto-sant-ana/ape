# Observation 1 — One witness, two journals, and I1 refuted

The arrangement is the smallest thing that can fail: one decision, and two journals that differ in
exactly one recording instant.

```text
F   receive 400   committed day 2, recorded day 2
E   Event settling F, occurred day 3, recorded day 4 in one journal and day 9 in the other

D   Genesis { known_at: day 6, selection: { F } }, taken after E        12 entries either way
```

Nothing else differs, and no address moves — an `EventId` is derived from the commitment it settles,
the observation, its predecessor and the instant it **occurred**, and from nothing else. So the two
journals are equal entry for entry, the witness `D` carries is satisfied by both, and the coordinate
resolves in both.

```text
                    event head at day 6   settled   intended
  early                     E               400       400
  late                     none               0       400
```

**I1 is refuted.** A cut resolves its head from **recorded** instants, and `day 6` falls between the
two. The same decision selects a fund that has settled in one journal and one that is merely open in
the other, and the number it answers moves by the whole of the fund.

This was the expected result and the cheap one. What follows is the experiment.

## It is a repository, not a comparison in a test

The second journal is written whole, with the decision in it, and read back through the application's
own reader. Every guard passes. `reading::reconstruct` returns one world, `event_head` is `None`, and
the level is `0`.

So the refutation is not *two values computed differently by a phase*. It is a record somebody could
have on disk, that reads, that is nobody's forgery, and that answers the other number.

## What could have made this un-failable, in either direction

Named before the run because this is where the experiment was most likely to fool itself.

**If the two journals could not exist**, I1 would hold for a reason about the arrangement. They do
exist, and both are legitimate: recording is monotonic in each, the Event is recorded no earlier than
it occurred in each, and the fund is selectable at the cut in each.

**If they differed in something a coordinate was never about**, I1 would fail for a reason about the
arrangement. The subject asserts the hinge before anything runs —
`LEARNED_ON[0] <= KNOWN_AT && KNOWN_AT < LEARNED_ON[1]` — and asserts that the fund is committed
before the cut, so what moves is the chain and never the selection.

**Seen red by moving the second day to the near side of the cut.** With `LEARNED_ON = [4, 5]` the
subject stops compiling and names the assertion, which is the arrangement refusing to be one where
both journals answer alike.

## The three candidates that are not the pair

Three more journals exist for later phases, and each differs from `early` in exactly one nameable
way. They are here because a pin is a claim about a **family** of journals, and a family of two would
let every later stage look decisive.

```text
reordered-early    two Roles swapped. The same addresses in a different sequence
reordered-late     the same, over the late journal
inserted-early     a Commitment nothing selects, admitted before the coordinate
```

Every one of them answers the number the subject wrote down for it before the run.
