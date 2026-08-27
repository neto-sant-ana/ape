# Observation 5 — Three sequences, and nowhere to say it

**U5 confirmed, and mechanically rather than by reading.** The three files a repository holds are each
a JSON **array**; every element is an object; the keys are closed.

```text
  journal.json   26 elements   25 distinct keys across all nine families
  lineage.json    1 element     5 keys
  worlds.json     1 element     6 keys
```

There is no top-level object in any of them. So there is nowhere for a statement whose subject is the
record to be written, and every claim the repository makes is about **one item of one sequence**.

```text
  journal.json   one admission's content        a fact
                 one admission's recorded_at    when a fact was learned
  lineage.json   decides / known_at / selection an intention
                 after                          where a decision stood
                 witness                        the prefix a decision stood on
                 by                             who took a decision   (absent here)
  worlds.json    thesis / thesis_parent /       a world
                 known_at / event_head /
                 frozen / open
```

Not one of them has the record as its subject, which is U5 as the closed list the protocol asked for.

## And the consequence is sharper than the absence

Because each file is a sequence, **each file's length is a property of that file**. A journal that
lost an entry is a shorter array, and a shorter array is a perfectly well-formed array. It agrees with
itself. Nothing else is in a position to disagree with it.

That is the mechanism behind Observation 2's table, stated as a property of the format rather than as
seven measurements: the reason nothing refused is not that the guards are weak, it is that **no file
is about another file's extent**. The witness is the one claim that crosses files — it lives in
`lineage.json` and is about `journal.json` — and it stops at the coordinate.

So whatever covers the tail has to cross files too, and cannot live in the journal it is about: a
tamperer editing one file would edit the claim with it. Together with Observation 4 — the author must
know the journal entire, so it is the write — that is two constraints and they leave one shape.
