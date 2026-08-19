# Observation 7 — A question is asked *of* a record, not stored in one

Both reports come back from a fresh process identical to the ones Phase 3 recorded: the three
identities, the difference, the status, and the conflict with both of its instants. The literals
written down beforehand hold too, which is the part equality cannot do — two derivations from one
implementation agree through a defect they share, and a date written down before the run does not.

What the phase had to build to get there is the finding.

## Two answers, two argument lists

The binary now answers two questions, and they do not take the same arguments:

```text
ape-cli <repository> <instance> <date>                  the worlds
ape-cli <repository> transfer <base> <source> <target>  the report
```

Reading a world needs an instance and an instant, because a world is read *of* something *at*
some time. A report needs neither, and needs three identities instead. Giving one form the
other's arguments unused would have hidden which answer depends on what.

The three identities arrive from outside because Phase 4 put them there. Reconstructing a world
needs the repository alone; reconstructing a report needs the repository **and a question**, and
a query nobody acted on is not part of the record.

That is not a gap in the repository. The identities are content-addressed, so a caller obtains
them by reading the same repository and naming what it found — which is what an application does
anyway, since it has to choose the two lines it wants to reconcile. A question is asked *of* a
record; it is not kept in one.

## The record naming its own question earns its keep

The whole-report comparison was mutated by swapping the Source and the Target where the binary
parses them — two adjacent arguments of one type, which is the defect this shape invites. The
comparison caught it, and the diff made the cause legible on its own:

```text
source: af23c630…   target: fe0187d0…      what came back
source: fe0187d0…   target: af23c630…      what was expected
```

Not "the status differs" but "the question differs". Observation 4 argued that a record naming
only the two worlds it moves between would be an answer whose question is missing; this is the
same claim measured. A record that carries its question makes a transposed question self-evident
rather than something to be inferred from a status that came out wrong.

## The ancestry walk, which a success would not show

A report at all means the rebuilt archive reached the Base from both tips, since Synthesis
establishes the Base before deriving anything. But a walk that always answered yes would produce
reports too.

So the phase also asks with a Base that only the Source descends from — the equipment line's
first decision, which the inventory line never passed through — and the fresh process refuses,
naming it:

```text
thesis 175e358d… is not a common ancestor of af23c630… and fe0187d0…
```

Which is the third success criterion measured rather than assumed: the archive a fresh process
builds resolves every world both lineages name, and its walk reaches the shared ancestor from
either side — and stops where there is nothing to reach.

## One procedure, one place

Reading the worlds and asking a transfer both begin by rebuilding the repository and weighing the
result against what it recorded. That procedure now lives in one function with two callers rather
than in two copies, because the order between the journal and the lineage is the thing the
divergence experiment spent a whole round repairing, and a second copy of it is a second place for
that order to drift.
