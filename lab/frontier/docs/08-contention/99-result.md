# Result

**Confirmed** for divergence and **Confirmed** for exposure — and the prediction that was supposed to
be the experiment is **refuted in both halves**, with the two orderings exchanged.

```text
divergence   the two orderings do not agree: one is refused by name, and one
             lets both writers finish                                    CONFIRMED

exposure     a whole write is atomic against a process that stops and not
             against another writer                                      CONFIRMED

             …and the pointer turning makes the loss arrive as a success  CONFIRMED
```

```text
C1  two writers that both read before either writes choose the same
    generation                                                            CONFIRMED
C2  the last turn wins, and the loser is told it succeeded                CONFIRMED
    in its outcome, REFUTED in its mechanism — the last PREPARE wins
C3  a finer interleaving reopens the six states atomicity closed          CONFIRMED
    where one journal extends the other, REFUTED where they diverge
C4  the compare-and-append catches one ordering and not the other         REFUTED
    in both halves: it catches the other one
C5  what contention loses is coarser than what an interruption lost       CONFIRMED
```

## The answer

The question was *is a repository's write a compare-and-swap, or a compare followed by a swap?* The
answer is that the write had **no compare in it at all**, and that the comparison the application does
have is somewhere else and is stronger than its own note claimed.

```text
converge    reads the repository, merges, writes            a comparison, and it
                                                            is against DISK, not
                                                            against what was read

write_whole prepares a generation, turns the pointer         no comparison
```

So the two halves of the question are both true, of different code:

* **Through `converge`, the write is a compare-and-swap.** All six orderings of two parties' read and
  put-back end with both lines present, and the single refusal lands exactly where both parties read
  before either wrote. A `converge` is one call, so nothing can interleave inside it without a thread,
  and the re-read at its top is always after any earlier writer's write. A stale reading costs an
  attempt, not a decision.
* **Through `Repository`, it was a compare followed by a swap** — and the compare was about where a
  *reader* is looking rather than about who else is writing. Two writers preparing before either turns
  share a generation, the last one to prepare decides what a reader reads, both turns publish it, and
  both writers are told they committed.

C4 got this backwards because it assumed the comparison was against the party's reading. It is not, and
the correction is the most useful thing the experiment produced: the guard the application has *is* the
guard the application needs, for the parties that use it.

## Against each criterion

1. **Every interleaving is an order of calls, and no measurement depends on a scheduler.** Six
   interleavings of two prepares and two turns, six of two reads and two put-backs, twelve schedules of
   the finer grain under each of two configurations. No thread, no clock, no signal.
2. **Compared by value, never by absence of error.** Every outcome is a `State`: entries, decisions,
   worlds, the level each world intends, and the party each decision claims. That is what makes an
   outcome *the buyer's state* rather than *two worlds*, and what lets a mixture be told from both.
3. **Where the compare-and-append refuses, the coordinate is named; where it does not, the reason is
   stated in terms of what it compares.** `Diverged { position: 15 }`, at the entry each party added.
   And where it does not refuse, the reason is that one journal extends the other — which is a property
   of the two journals, not a gap in the guard.
4. **What survives of the losing party is stated for every outcome**, including the two where the answer
   is *nothing in the live repository and its whole state in the previous generation*, and the four
   where it is *nothing anywhere*.
5. **The set of interleavings is closed, and shown to be closed.** Four operations with two ordering
   constraints is six, enumerated and asserted as one table. And the closure is shown to be *as stated*
   rather than absolute: the backwards turn takes five operations and is outside it.
6. **Nothing was added to the APE engine.** `core/` is untouched across the branch: zero files.
7. **Part B was built against both criteria.** The first is measured in Phase 7 — every mixture produced
   in a shared generation, none of them reaching a reader, and the replaced generation byte-identical
   afterwards. The second is met by construction and said so: a comparison is not a lock, nothing waits,
   and a dropped `Prepared` leaves three files nothing reads.
8. **The seven earlier experiments' conclusions stand**, and atomicity's — the one at risk — is
   sharpened rather than moved. Its own result named this as the nearest unanswered question; the
   sentence *a write is all-or-nothing against a process that stops* is true and now says which writer
   it is silent about. Details below.

**Criteria 3 and 8 are the experiment**, and both hold.

**Failure conditions.** Neither severe one was met: the repair is three file reads and a `rename`,
nothing in the engine learned that knowledge lives in a directory, and nothing outside the record was
introduced — no lock file, no service, no clock. Of the ordinary ones — no measurement needs a thread;
every outcome is reported from what the record holds; the repair was chosen after Phase 5 and its
alternatives are named in Observation 6 as needs rather than shapes; the compare-and-append is **not**
reported as broken, and the correction runs the other way, since it turned out to catch the ordering the
protocol said it could not; and no interleaving is called unlikely — what each one leaves is what is
reported.

**And one honest limit.** The literals about the arrangement were written before the run, and the
**classification of the twelve mixtures** was predicted qualitatively by C3 rather than as a table. Both
tables were written down before running them and both came back as written, which is worth less than
pre-registration and more than reading them afterwards.

## Findings that outlast the verdict

**The guard compares against the repository, not against the reading.** `converge` re-reads at the
moment of writing, so what it weighs is what is *there* against what the party *holds*. That is why the
serialized ordering merges and the interleaved one is refused — the reverse of what was predicted — and
why no ordering of two converging parties loses a line.

**And what it buys is a legible refusal, not an intact repository.** Mutated so the comparison never
fires, a divergent party's write still does not land: the merge fails further down, at an entry the
journal does not hold, because a party that cannot converge writes nothing. Both comparisons are
load-bearing and only one of them keeps the record. The coordination experiment's own note implied
otherwise and is corrected in place.

**A turn is a claim about a name, and the name's contents are not part of the claim.** Preparing decides
*what* a reader will read; turning decides only *when*. Two writers sharing a name make the second
decision meaningless and the first one silent — so the pointer's turn was not the commit it looked like,
and a writer could publish bytes it had never written.

**What survives of the loser is not a property of the loss.** Separated by a turn, the loser's whole
state is the previous generation; collided, the previous generation is the base and the loser is
nowhere. Same visible outcome, and nothing on disk says which happened — so *recoverable* is not a
property of the repository. And the facility that keeps the loser was built for interruption: it keeps
the previous **write**, not the losing **writer**.

**A mixture is silent exactly where one journal extends the other.** Which is the same condition the
compare-and-append tests, read in the opposite direction: two writers dangerous enough to produce a
silent mixture are exactly the two the merge would have let through, and the parties it refuses cannot
produce one. Under extension the twelve schedules reproduce the atomicity experiment's six states cell
for cell; under divergence, none of the six is silent.

**A pointer that can be turned by a stale handle is a repository that can move backwards.** Five calls,
outside the closed set, and the only outcome in the experiment that misleads a **reader**: it
reconstructs, it corroborates, and it carries no fact saying it is a rollback.

**Ask a repository what a party decided and an empty answer means two things.** Nothing, or nothing any
more. The party itself is canonical knowledge and survives every loss; what it did is a claim and
survives none of them. The coordination experiment recorded this gap as the cost of an optional field;
it arrives here from the other side, as the cost of a write that does not compare.

## What changed in the experiments before it

**Nothing was overturned.**

**Atomicity's promise says which writer it is to.** *A whole write is all-or-nothing* is true against a
process that stops, and was silent about a second writer — which its own result said, and which is now
measured rather than suspected. Its ten phases pass unchanged against the repaired write, because the
instrument they use is the single-file path and the repair does not touch it.

**Atomicity's rollback facility acquired a second, accidental job.** Two generations exist so that the
repository before an interrupted write survives. In two of the six interleavings, that previous state is
the losing writer's whole repository — the only place a lost writer's work exists, provided by a design
that was not asked to provide it.

**Coordination's guard is stronger than its note said, and its note is fixed.** *At the moment of
writing* was load-bearing and unmeasured; the comparison is against disk, and that is what makes the
interleaved ordering refusable at all.

**Coordination's optional decider gains a second unanswerable case.** `decided_by` could not tell *not
this party's* from *unclaimed*. It also cannot tell *never decided* from *decided and overwritten*.

**Exploration's seam is reused twice.** Preparing and turning are separate for the reason weighing and
keeping are, and this experiment needed the seam in order to express an interleaving as a value — which
is the second time a public seam turned out to be what makes a measurement possible.

## Variables the protocol left open

**Whether two writers are one repository's business at all.** Still open, and now with a shape: a
repository can refuse a writer *whose generation was written over*, using nothing but what it holds. It
still cannot say whether two writers were welcome, which is Request 4 and remains unanswered.

**More than two.** Untouched. Two is the smallest number that can contend, and nothing here suggests the
third is free.

**What a writer that waits is owed.** Closed by not arising: nothing in the repair waits.

**Two repositories meeting.** Open since convergence, and untouched.

**Cost.** No longer costless to talk about. A turn was one `rename` and is now three file reads and that
`rename`, and a `Prepared` holds an encoded copy of the repository until it turns. Nothing here measured
whether that matters; what changed is that the variable now has an entry in it.

## What the result does not cover

**Threads.** Excluded, deliberately and throughout. What is refused is an interleaving of calls; the
window between the last comparison and the `rename` would take an atomic filesystem primitive, and
nothing here claims otherwise.

**Who the other writer was.** The refusal names the generation and not the writer, because nothing on
disk says. Whether it should is Request 3's neighbour and is not asked.

**`fsync`, power loss, and a partial `rename`.** Excluded by atomicity and untouched.

**Authenticity.** Named for the seventh time and unchanged; the candidate holds it.

**Benchmarking.** No timing was taken, including of the turn that now reads three files.

---

# Architectural Consequences

## Established for the CLI

* **A turn compares before it swaps.** `Prepared` keeps what it wrote, and turning refuses to publish a
  generation this write did not write — naming the generation the two writers met in.
* **The comparison that protects a write and the one that protects knowledge are two.** The first is the
  prepared generation, weighed at the turn; the second is the journal on disk, weighed at the merge. A
  writer that does not converge has only the first, and loses another line in silence.
* **What a reader reads is the state of whoever prepared last**, and the repair does not arbitrate that.
  What it changed is that the other writer is told.
* **A refused writer's recovery is to read again, decide again, and converge.** It is the same recovery
  the coordination experiment established, and there is nothing to release first.
* **Every mixture of two writers' files is out of reach through the write, and not impossible.** The
  single-file path stays, because five concluded experiments need it.

## Candidates for later experiments

* **Whether a repository may say how many writers it admits** — Request 4, and the first need in nine
  experiments that asks the record to describe its own use.
* **Telling *never decided* from *decided and overwritten*** — Request 3, and the second time this gap
  has been reached from a different direction.
* **Threads, and a turn that is atomic against one** — what this experiment excluded and what a real
  application would eventually meet.
* **A series of generations, and what would prune one** — unchanged since atomicity, and now with a
  second reason to want one.
* **Authenticity**, **two repositories meeting**, **saying two lines agree** — unchanged, all open since
  before this.
* **Cost as an absolute**, deferred by nine experiments, and now with a term somebody could measure.
