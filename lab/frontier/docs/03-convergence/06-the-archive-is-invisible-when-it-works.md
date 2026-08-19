# Observation 6 — The archive is invisible when it works

The fresh process returns six worlds with the branching intact: two naming one parent, each
line's second decision finding the world it extends, and the two lines recognizing different
knowledge. Nothing in that output mentions an archive.

That is the phase's whole content. A `Thesis` does not deserialize, so an archive cannot be
opened — it is filled again by putting each world into it as the decisions produce it, and its
absence would never appear as a missing archive. It would appear as a decision naming a world
nobody can find. The rebuild succeeding is what says it was there.

## Proving that took two attempts, and the first one failed usefully

The obvious way to check that the archive is load-bearing is to break it: skip filling it for one
world and watch the rebuild refuse. It does, and it names the world it could not find.

But it refuses in the **living** process, before any repository is written. The arrangement never
gets built, the fresh process is never reached, and Phase 5's own claim — that the archive is
rebuilt *across a boundary* — is untouched by the evidence.

That is not a flaw in the mutation. It is the reconstruction experiment's finding arriving from
the other side: **there is one code path**, and an application deciding as it goes and a
reconstruction replaying afterwards differ only in *when* they run it. A defect in that path
cannot be made to appear on one side of a process boundary and not the other, because there are
not two implementations to disagree.

So the isolating check is not a mutation of the code but a move in the repository. One decision
is repointed at a world that **exists** — the other line's first — and the rebuild is asked
again. The fresh process refuses, and the refusal names which world came back wrong:

```text
world 3 disagrees with what was recorded, in what it still proposes
```

Which is the pairing the corroboration experiment established, now over a coordinate this
experiment added: `extends` is the **instruction**, and the world record is what disagrees with
it. A repointed reference that resolves is not caught by resolving — it is caught by the world it
produces not being the world that was witnessed.

## What that says about the two coordinates together

The repository now holds two references that a reader must follow and one witness that weighs the
result:

```text
after     which knowledge the decision stood on      instruction
extends   which world the decision extends           instruction
worlds    what the decision produced                 witness
```

Neither instruction can be dropped for redundancy — a reader derives from both. Neither is
self-checking beyond failing to resolve. What makes a repointing visible at all is the third
line, and this phase measured that across a process rather than assuming it.
