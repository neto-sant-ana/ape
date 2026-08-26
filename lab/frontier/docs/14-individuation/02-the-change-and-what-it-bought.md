# Observation 2 — The change, and what it bought

The whole of it, in `cli/src/journal.rs`:

```rust
pub fn of(id: impl AsRef<[u8; 32]> + std::fmt::Display, recorded_at: &str) -> Self {
    Self(format!("{id}@{recorded_at}"))
}
```

A second argument, nine call sites in one function, and a docstring that stopped being true. Nothing
else in the application: `lineage.rs`, `converge.rs`, `reading.rs` and `repository.rs` compiled
untouched, and every one of the application's own tests stayed green.

**The instant is a second argument rather than something a caller attaches afterwards**, so that an
address which does not say *when* cannot be constructed at all. That is the difference between a
change and an option, and an option would have made the measurement meaningless.

## N1 — the witness completes the pin, with no field added

Experiment 13's stage table, re-run against the new shape. Both columns were written into the subject
before the build.

```text
                 candidates admitted   bodies of knowledge   worlds
                 content  composite    content  composite    content  composite
  coordinate        5         4           4         3           2        1
  witnessed         4         2           3         1           2        1
  dated             2         2           1         1           1        1
  produced          2         2           1         1           1        1
```

Read the middle pair. Today the reference is fixed by the stage that carries a recording instant for
every witnessed entry; under an address that says *when*, it is fixed one stage earlier — by the
witness, which the record already writes down. **N1 confirmed.**

And the row below it is the other half: the dated stage stops buying anything, because a pin carrying
instants would be saying twice what its addresses already say.

The coordinate alone still leaves three bodies of knowledge standing, which is what says the
arrangement did not do this. `early-noticed-late` shares the record's coordinate exactly and is
excluded by the witness and by nothing else.

## What each of the five sites answers

One probe per site, both columns pre-registered, each probe the smallest state that reaches that site
— a state refused earlier would be measuring the site above it.

```text
                                        content              composite
  journal::replay_through               reads, answers 0     no entry of that name
  lineage::corroborate                  reads, answers 400   the prefix is not the one named
  lineage::diagnosed                    ambiguous            reads, answers 400
  lineage::diagnosed, on one day        ambiguous            ambiguous
  converge::appended                    recorded differently diverged
  converge::ordered                     reads, answers 400   reads, answers 400
```

**The first row is experiment 13's whole finding, closed.** A decision resolving against a journal
whose instants are not the ones it was taken against reads today and answers the other number; under
an address that says *when* it is refused at the coordinate, which is the cheapest guard the record
has.

**The third and fourth are N3, and it is partial exactly as predicted.** A journal that admits one
entry twice on two days becomes readable rather than ambiguous. One that admits it twice on one day
stays ambiguous — and the reason is structural rather than incidental: recording is monotonic across
admission, so everything between two same-day occurrences was learned on that day too. Within one
journal that is the only readmission an address which says *when* cannot separate.

**The last row is the control**, and it not moving is part of the measurement. A table whose every row
moved could not have said which rows the change is about.

## Two things worth saying about the fifth row

`converge::appended` refuses either way, and what changes is what it says. Today it says
`RecordedDifferently` — **this entry, and two instants**. Under the new shape it says `Diverged` —
**two addresses, at this position**. The first is the diagnosis experiment 11 built and the second is
the one experiment 13 filed a request against.

So **N2 is confirmed with its inner prediction**: the change does make experiment 11's comparison
unreachable, and what it costs is the message. Saving a comparison and losing the sentence it printed
is not obviously a saving.
