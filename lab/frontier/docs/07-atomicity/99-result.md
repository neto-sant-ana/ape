# Result

**Confirmed** for divergence. **Confirmed in its first clause and refuted in its second** for exposure
— a refusal says nothing at all about the record, which is what the claim was after, and *every*
partial state costing the previous repository is false.

```text
divergence   the partial states do not agree with each other: one is refused by
             name, and one reconstructs                                    CONFIRMED

exposure     being refused protects the reader and not the record          CONFIRMED

             …so every partial state costs the repository that existed
             before the write                                              REFUTED
```

```text
A1  a truncated commit is three cases, and they do not agree             CONFIRMED
A2  the silent case is the one that loses intention                      CONFIRMED
A3  the order the files are written in decides which case is reachable   CONFIRMED
A4  being refused is not being safe                                      REFUTED as written
A5  atomicity's whole value is the case corroboration cannot see         CONFIRMED
```

## The answer

The question was *is being refused the same as being safe?* No — and the interesting part is not that
one is weaker than the other. They are **unrelated**.

```text
                  put back   lost
refused                  2      3
reconstructs             0      1
```

Three of four cells are occupied. A repository is refused when a reader could be misled, and it keeps
what was there when what survived determines what was replaced, and those two conditions have nothing
to do with each other. Safety is not a property of the refusal. It is a property of **which file the
interruption landed on**, which is in turn a property of whether anything witnesses that file's length:

> **A record rolls back exactly as far as something witnesses the length of what was replaced.** The
> worlds file is derived and comes back from the two files that produce it. The lineage appends, and
> its previous length is recorded in the worlds file — and nowhere else. The journal appends, and
> nothing records its length at all.

Which answers a question the protocol carried in as open, and answers it in the opposite direction from
the one it was posed in: a repository does not need to *start* keeping something in order to be rolled
back. The only rollback information it has is the **derived witness corroboration already keeps**, put
there for a reader. Rollback is a byproduct of corroboration rather than a requirement competing with
it.

And the empty cell is the reassuring one. No partial state both reconstructs and keeps what was there,
so the single state a reader is never warned about is also lossy.

## Against each criterion

1. **Every prefix produced deterministically, and no measurement depends on a race.** Twelve schedules
   — six write orders, two interruptible points each — leaving six distinct states. No thread, no
   signal, no clock: a prefix of a write sequence is a value, and the instrument is calling the
   application's three write methods and not making the next call.
2. **Compared by value against the whole repository, never by absence of error.** Every phase compares
   a `Reading` **and** the level each world intends, as one value per world. The intended level rather
   than the settled one, deliberately: three worlds that settled the same pledge and proposed different
   outflows report the same settled level, and a phase comparing that would have reported the lost
   world as identical to the one before it.
3. **The silent case is measured positively.** Not by failing to find an error: by writing a second
   repository in which a writer admitted the same entry and *decided nothing about it*, finishing every
   write it began, and comparing the two directories as bytes. They are equal.
4. **What survives is stated for every prefix.** All six, in Observation 3's table, including the four
   where the answer is *nothing* and the one where the loss is a single entry that cannot be attributed
   to either repository.
5. **The write order is a measured variable and the outcomes are a closed set.** Six orders enumerated
   rather than one reversed, and the six states with their outcomes are asserted as **one value** —
   because criterion 5 asks what is reachable, and a phase asserting six facts separately leaves
   whichever it forgot unmeasured.
6. **Nothing was added to the APE engine.** `core/` is untouched across the branch: zero files.
7. **Part B was built against the criterion, and the shape that failed it is reported.** It meets both
   halves, measured in Phase 6. The cheaper shape — a witness of the journal's extent, which follows
   directly from Observation 3 — meets the first half and half of the second, and is recorded in
   Observation 7 with the cost that disqualified it: it raises the atomicity requirement it was meant to
   substitute for.
8. **The six earlier experiments' conclusions stand.** One promise gained the mechanism it never had,
   two are sharpened, and one rule of the laboratory was found to be wrong about half of the laboratory.
   Details below.

**Criteria 3 and 4 are the experiment**, and both hold.

**Failure conditions.** The severe one was not met: the repair is three `fs::write` calls, one
`fs::rename` and a directory name, and nothing in the engine learned that knowledge lives in a file. Of
the ordinary ones — no measurement needs a race; the silent case is reported from what the record says;
the repair was chosen after Phase 4 and the alternative is costed in the open; a refusal's worth is
stated as *safe for a reader, and silent about the record*; and the write order is not treated as a
detail afterwards, since it is the whole reason one state is silent and Part B removes reachability
rather than reordering two lines.

**And one honest limit.** Every literal about the arrangement was written before the run and is in the
subject; the **classification of the six states** was not pre-registered as a table. A1 and A3 predicted
the cases qualitatively, and which of the six each schedule lands in was read after the run.

## Findings that outlast the verdict

**There is no fact that says a write did not finish.** The interrupted state and a repository whose
writer admitted knowledge and decided nothing about it are the same bytes. So the silence is not a gap
in the checking — it is the absence of a fact, and every reader-side remedy is bounded by it. That is
why A5 holds: atomicity's value is the case corroboration cannot see, and it cannot see it because
there is nothing there to see.

**Only a file addressed by its prefix can be replaced in silence.** The lineage addresses the journal by
an entry and by a set of entries — a prefix — so a journal that grew satisfies every reference that
survived. The worlds file answers to the lineage by its **length**, and a length is not satisfied by
growth. The journal is the only file in the first position because it is the only one that is a
**source**; sources append, and the two derived files are compared entire.

**The record rolls back what something witnesses the length of.** Two of six states put the previous
repository back byte for byte, using two rules the format already supplies. The journal has no such
rule, and what a replaced journal loses is exactly the tail no decision witnessed: 15 of 16 entries are
pinned by the last surviving decision, and the sixteenth cannot be attributed to either state.

**The record contains the rollback, and not the reason to perform it.** A lineage written before its
journal and a journal with an entry tampered away raise the **same refusal**. In the first case
truncating the lineage restores the previous repository; in the second it destroys a legitimate
decision and calls it recovery.

**A repair on the writer's side makes the states unreachable, not impossible.** Writing one file
remains possible because five concluded experiments need it in order to tamper and to prune. What
changed is that nothing in the application does it. The record still cannot tell an interruption from a
tampering; it can no longer be interrupted into one.

**A rule is not tested until something wants to break it.** A reason recorded beside `Taken::by` said
four concluded experiments hold repositories whose decisions name nobody. They hold none — every suite
in `frontier/` builds its repository from its own subject, each run, in a temporary directory. The
conclusion was right for a different reason and the reason taught the wrong rule, which is how it came
to be offered as a veto over this experiment's remedy. It survived four experiments because nothing had
tried to act against it.

**Two predictions that were one variable.** A4's claim quantified over every partial state; its
justification quantified over one write order, which is the variable A3 existed to move. The refutation
is that entanglement rather than a surprise about the record.

## What changed in the experiments before it

**Nothing was overturned.**

**Coordination's promise gained its mechanism.** *A party that cannot converge writes nothing* was true
of every refusal and of no interruption: the three writes at the end of `converge` were the one place in
the application where a process could stop and leave a repository nobody wrote — and one of the states
it could leave reconstructs. The write is now whole, and what it replaces stays on disk.

**Corroboration's promise is stated more narrowly, and is not weakened.** Six tampered repositories out
of six are still refused. What this adds is who the promise is to: a refusal is a statement about the
**reader**, and five experiments' worth of them say nothing about the record. Reporting a refusal as a
success was right; reporting it as safety would not be.

**Corroboration's tampering note gains a third intent.** It could not tell tampering from pruning
because the difference is intent; exploration measured that there is nothing to tell apart. An
interruption is now the third, on the same mechanism.

**Provenance's rule is extended and not relaxed.** *Nothing derived is persisted unless something
compares it* turns out to be what makes rollback possible at all — the derived witness kept for a reader
is the only thing in the repository that says what a previous state was.

**Convergence's verb is reused.** *What the reference turned* is what a whole write does at the end, and
the pointer is turned rather than rewritten for the same reason a reference is.

**And one rule of the laboratory was wrong.** Recorded in Observation 6 and repaired in `lab/README.md`
and in the two docstrings that stated the over-strong version: a published claim is kept true by the
commit it was taken against, a runnable arrangement is a bill, and only an irreproducible artifact can
veto a change. There are two of those, and the migration the rule authorises for them turned out not to
be needed.

## Variables the protocol left open

**Whether the three files should be one.** **Answered: no**, and by measurement rather than preference.
Atomicity did not require giving up a repository that can be read by eye — what became atomic is a
pointer, and the three files are unchanged.

**What a repository owes a writer.** Opened here and partly answered. It owes a whole write: the first
promise in seven experiments made to whoever writes rather than to whoever reads. What it does not owe
is a way to know that a write was interrupted, and nothing here found a shape for that which does not
also record more than the record records.

**Two repositories meeting.** Open since convergence, and untouched.

**Cost.** Unchanged, and now with one more term: a whole write copies three files rather than replacing
three, so a repository pays its own size per commit. Nothing here measured it, and the history nobody
has is still the thing missing.

## What the result does not cover

**Power loss and `fsync`.** Excluded by the protocol and untouched. The promise is against a program
that stops.

**Concurrency.** Two writers preparing at once would target the same generation, and nothing here
refuses it. Excluded, and now the nearest unanswered question after this one.

**More than one step of rollback.** Two generations keep one previous state. What would justify a series,
and what would prune it, is not asked.

**Authenticity.** Named again and unchanged, and this time **gathered** rather than only named: the
candidate holds what removing one of its faces did to the question.

**Benchmarking.** No timing was taken.

---

# Architectural Consequences

## Established for the CLI

* **A repository holds two generations and a pointer at the live one.** A whole write puts three files
  where nothing reads them and then turns the pointer, so a write is all-or-nothing against a process
  that stops.
* **A repository with no pointer is its own live generation.** Every repository written before
  generations existed reads unchanged, and nothing was migrated.
* **Preparing and turning are two operations.** The seam is public because an interruption is a prefix,
  and a laboratory cannot produce one against a call that does both — the same reason exploration cut
  the seam between weighing a world and keeping one.
* **Writing one file is what a record edited from outside looks like.** It stays, it writes into the
  live generation, and nothing in the application calls it.
* **Rollback reaches one state back, and no further.**
* **A refusal is a promise to a reader.** It says nothing about what the record kept, and the two are
  independent rather than ordered.

## Candidates for later experiments

* **Two writers preparing at once** — the nearest unanswered question now, and the first one in this
  sequence that needs concurrency.
* **Durability against power loss** — `fsync`, and whether the pointer's rename is the only place it
  would be needed.
* **A witness of the journal's extent** — rejected here as a substitute for atomicity, and still the
  only shape that would let a *torn* journal be rolled back.
* **A series of generations, and what would prune one** — where exploration's pruning question meets
  this one's rollback.
* **Authenticity**, **two repositories meeting**, **saying two lines agree** — unchanged, all open since
  before this.
* **Cost as an absolute**, deferred by eight experiments now.
