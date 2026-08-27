# Observation 6 — Part B, and the claim the record now makes

The condition was written before the phases ran:

> **The phases show the loss changing what a later decision stands on**, and show it against a record
> that is legitimate and reads. A gap that costs nothing is a finding and not a reason to write a file.

It is met, and not narrowly. Losing the tail's reached half moves the later world and the number it
answers; losing the *unreached* half takes away a fork the record could otherwise have taken, and
refuses it with a reason that is true and is not the reason. Every one of those records is legitimate
and reads.

So Part B is built. The two constraints Observations 4 and 5 produced leave one shape, and neither of
them was named in the protocol:

```text
the author is the write        a decision's authority is what stood when it was taken, and by
                               then the tail is what stood after
it cannot live in the journal  a file is not about another file's extent, and a tamperer editing
                               one file would edit a claim inside it
```

## `custody.json`

A fourth file in the whole write: **the addresses the journal produces, whole**, compared against the
replay on every read.

```text
journal.json    [ ......... the prefix ......... ][ ... the tail ... ]
lineage.json      each entry named by a witness              nothing
custody.json      every address the journal comes to, whole
```

Four decisions in it, each with a reason the phases produced rather than a preference:

**It is derived by the write, not supplied by the writer.** The only derived value in this repository
that is not handed in — because it is a function of the journal and of nothing else, unlike a world,
which needs the lineage and the engine, and unlike a witness, which needs to know *when* the decision
was taken. That is `Taken::now`'s own argument one level up: both halves come from one reading, so
they cannot disagree when they are written, and corroboration is a property of the read.

**It covers the journal entire, not the tail.** A tail-only claim is defined relative to the last
coordinate, so editing `lineage.json` moves what it is about — a claim that a second file can
neutralize. The whole journal is checkable against the journal alone. It therefore overlaps the
witness on the prefix, and the two are still different claims: one says what a **decision** stood on,
the other what the **record** holds, and Observation 5's table is why that is two subjects and not one
fact written twice.

**Membership, not order**, for the reason `Taken::witness` gives. An entry's identity is its content,
so a journal whose entries came back in another order holds the same entries and the record's own
reader answers the same way. Sound about loss and gain, silent about a reordering — which is the same
sound-and-incomplete shape experiment 12 measured, chosen deliberately rather than met.

**Absent is not empty.** A repository with no `custody.json` makes no such claim and is read exactly
as it was; one whose file holds `[]` says it holds no entries and is refused by the first entry the
journal offers. Both measured.

## What it catches

The same eight states, against a record written whole after the change:

```text
                                          before      after
  nothing moved                           nothing     nothing
  without the whole tail                  nothing     custody
  without what nothing reaches            nothing     custody
  without what something reaches          nothing     custody
  with one more nothing reaches           nothing     custody
  with one more something reaches         nothing     custody
  with one more unsettled, on the account nothing     custody
  without one from the prefix             witness     witness
```

The prefix row is still the witness's, and that is the right order: a claim about a decision is more
specific than a claim about the record, and it sends the reader somewhere narrower.

The refusal names the entry, on both sides — `HeldKnowledgeAbsent` for a journal that was cut,
`UnheldKnowledge` for one that grew. Two variants because the two send a reader to opposite places,
which is the same reason the witness has two.

## Seen red

Making the comparison one-sided turns three phases red, naming the state (`with one more nothing
reaches: Nothing, expected Custody`). Writing the claim one entry short turns four red, including the
arrangement's own — a record whose write and whose read disagree refuses **itself**, which is the
property the whole thing rests on.

## What it costs, and what it does not do

**A whole write now replays the journal it is handed**, and every read compares two sets the size of
it. That is the second observable price in `repository.rs`, and it is paid per write rather than per
entry the way the turn's comparison is. It also narrows what `write_whole` accepts: a journal that
does not admit is now refused at the write rather than at the next read.

**It is a claim, not a proof.** A tamperer who edits `journal.json` and `custody.json` together is not
caught, exactly as one who edits `journal.json` and `lineage.json` together is not caught by the
witness. What the record buys is that a *single-file* edit no longer passes — which is the whole of
what corroboration has ever bought here, now extended to the half of the journal it did not reach.

**And it does not restore what was lost.** Experiment 07 wanted a claim about the journal's extent as
a way of rolling a torn journal back; this refuses to read a record whose journal moved, and refusing
is not recovering. 07's repair already made a torn write unreachable, so what is left of that item is
answered rather than deferred.

## What it broke, and each break is the claim working

Three concluded suites went red, and none of them is a repair of this experiment's own making. They
are recorded rather than absorbed, and each one is repaired in place with a comment naming 16.

**`08-contention`, Phase 4** enumerates the six mixtures two writers reach inside one prepared
generation, by putting one writer's files over another's. A prepared generation now carries the first
writer's claim about a journal the second replaced, so **every** mixture is refused by it. That is a
real narrowing of the door 08 reports as left open — and it is measured here rather than asserted
there, because 08's arrangement is pinned to the shape it met and a claim about a change belongs to
the change. The repair reduces the prepared generation to the three files 08 enumerates.

**`11-veracity`** enumerates the eight mixtures of two generations, the same way. Same cause, same
repair. Its two answering mixtures had become one.

**`14-individuation`** is the interesting one, and it is where the day's second finding is. It holds a
**derived** guard: a lexical scan of `cli/src` for every place an `EntryId` is compared, asserted
against a list. It caught `reading::held` and named it.

```text
  journal::replay_through   lineage::corroborate   lineage::diagnosed
  converge::appended        converge::ordered      reading::held    ← new
```

14 used those five sites to price a change it went on to refuse — an address that says when the entry
was recorded. There are now six, so that change costs more than the result that refused it measured.

**And then it stopped catching it, which is the finding.** The scan reads a function's **body** for an
address type or a field holding one. `held` names neither: its addresses arrive as **parameters**. It
was found only while the signature happened to be wrapped across three lines — and when `rustfmt`
fitted the signature onto one, the site disappeared and the guard went green at five.

> A guard that depends on where a formatter breaks a line is reading the wrong source. It was green,
> it was derived, and it was wrong — and the only reason anyone knows is that the same change made it
> red first and green afterwards, ten minutes apart.

The declaring line is now part of what is scanned. Checked: the five it reported are unchanged by the
repair, so what the fix adds is the site it was missing and nothing else. That is the second time this
laboratory has met a derived guard that read a source adjacent to the one it meant, and the first
where the guard was **already published as evidence** — 14 priced a refusal with it.

**And a fourth thing, which is a friction rather than a break.** A claim about the record makes the
record harder to tamper with *including for a laboratory that needs to*. Two of the three repairs
above are a `remove_file`. Any future experiment that needs a record edited from outside now has to
say so explicitly — which is more honest than it was, and is a cost.

## And it does not strand the four repositories nobody can re-run

Measured, not reasoned about: all four of `lab/agents/04-multiagent/run-*/repo` hold three files, read
back with no claim, and still rebuild. That is the item experiment 14 queued — *a shape change strands
four committed repositories, and nothing says so where it would be read* — met for the first time by a
change that could have caused it, and answered by making the claim optional on read rather than by
migrating anything.
