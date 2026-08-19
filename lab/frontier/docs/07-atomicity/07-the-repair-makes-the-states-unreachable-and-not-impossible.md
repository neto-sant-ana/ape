# Observation 7 — The repair makes the states unreachable, and not impossible

Phase 6. The criterion was set by the coordination experiment and restated in the protocol before it
could be argued with:

> **It removes a state a reader can be misled by, and the repository before an interrupted write
> survives.**

## What was built

A repository holds two **generations** and a pointer at the live one.

```text
current       a               one file, replaced by a rename
a/            journal.json  lineage.json  worlds.json      ← what a reader reads
b/            journal.json  lineage.json  worlds.json      ← what the previous write left
```

A whole write puts three files in the generation that is *not* live, where nothing reads them, and
then turns the pointer. Turning is one `rename`.

Both halves of the criterion, measured:

```text
every state Phase 4 enumerated, produced in the pending generation
  12 schedules, 6 states, and for each one:
     the live generation's three files      byte-identical to before
     what the repository answers            equal by value to Phase 0, world by world

the turn                                    one rename
after it                                    the commit landed, three worlds
the generation it replaced                  still on disk, byte for byte
```

## Where the shape came from

Not from the remedy the coordination experiment named — that named *a rename, a lock*, and naming one
before Part A would have been choosing the answer. It came from two of the observations above.

**Observation 1 decided which side of the boundary the repair goes on.** The silent state is
byte-identical to a legitimate commit, so no comparison over a repository's contents separates them —
a reader-side remedy is bounded by a distinction the record does not contain. The repair had to be on
the writer's side.

**Observation 3 decided what to do with the previous state.** Rollback exists exactly where something
determines what was replaced, and the journal has nothing that determines it. Keeping the previous
generation *whole* costs one directory and needs no witness at all, where deriving it back needs a
number the record does not carry.

And one thing the protocol left open is answered by measurement rather than by preference:

> ### Whether the three files should be one
> A single file holding journal, lineage and worlds makes a write atomic by construction on most
> filesystems, and gives up reading the repository by eye.

**No.** Atomicity did not require it. The three files are unchanged, still readable by eye, and what
became atomic is the pointer.

## What it does not remove

Writing one file is still possible, and this is the honest half of the result.

Five concluded experiments need it: it is how corroboration tampers, how exploration prunes, how
coordination forges a lineage, how convergence moves a witness. So `write_journal` and its two
neighbours stay, they write into the live generation, and nothing in the application calls them.

Which means the six states did not become **impossible**. They became **unreachable by the
application**. Producing one now takes a hand writing where the application does not write — and this
experiment's own instrument for Phases 1 to 4 is exactly that hand.

```text
before Part B   three writes, and a process that stopped between two of them
after Part B    three writes, and only something that is not the application makes them
```

So the record still cannot tell an interruption from a tampering — it now cannot *be* interrupted into
one, which is a different promise and a smaller one than it sounds. That is the authenticity candidate
for the seventh time, and it is still not this experiment's to answer.

## What an interrupted write leaves behind

Two things nothing reads: a pending generation holding some prefix of the next state, and — if the
process stopped in the one-line window inside the turn — a stray pointer written beside the real one.
The next write overwrites the first and ignores the second.

Rollback reaches **one** state back and no further. Two generations is what the criterion asked for;
a series would keep more and would make pruning a question nothing has measured a need to answer.

## What the severe failure condition asked, and the answer

> If making a commit atomic requires the engine to know that knowledge lives in files — a flush, a
> transaction, a handle, an ordering the ontology has an opinion about — then *storage is the
> application's decision* has met a real friction.

It does not. `core/` is untouched across this branch, and the repair is three `fs::write` calls, one
`fs::rename`, and a directory name. Nothing in the engine learned that knowledge lives in a file.

## The rejected shape, and why it is recorded

A **witness of the journal's extent** — one number, compared on every read — follows directly from
Observation 3, removes the silent state, and raises rollback from two of six states to four. It was not
built, for two reasons that only appear once it is costed:

* it does not detect an interruption. It works by making the silent state stop being byte-identical to
  a legitimate commit, which is a change to what the record *contains* rather than to how hard it is
  checked. That is allowed, and it is worth saying out loud that it is the only way past Observation 1;
* and it **raises the atomicity requirement it was meant to substitute for.** Today a commit that only
  admits knowledge writes one file and has no interruptible point at all. With the extent recorded
  beside the decisions, that same commit must write two, and acquires one.

It also meets only the first half of the criterion — four of six states recovered, not six. A repair
that half-satisfies the half about the record is not what the coordination experiment asked for.

## What it cost, and what it did not

A repository with no pointer is its own live generation. So every repository written before this reads
unchanged and unmigrated, including the two in `agents/04-multiagent` whose authors nobody can
re-run — which is the laboratory's new rule about preservation paying off on the first thing it was
asked about: the migration it authorised turned out not to be needed.

Nothing about `fsync`. Whether bytes handed to the operating system reach the platter is excluded by
the protocol and untouched by this: the promise is against a **program** that stops, not against a
machine that loses power mid-`rename`.
