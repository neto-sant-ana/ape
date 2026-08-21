# Candidate — Witness

> **Answered.** Run as [experiment 10](../frontier/docs/10-witness/00-protocol.md); the result is
> [`99-result.md`](../frontier/docs/10-witness/99-result.md). The answer is the one this page listed last
> among its possibilities and did not expect: **no** — the two facts cannot be told apart, and the reason
> is that the witness is the only claim in a repository whose subject is its history rather than its
> intentions. What survives is that all four arrivals need a **second comparison** and nothing added to
> the record, which is now its own queue item.
>
> This page is kept as it stood when the protocol was written. It is the material that was in front of
> whoever wrote it, and editing it afterwards to match what happened would remove the only evidence that
> the question was not decided in advance.

**This is not a protocol.** A protocol is written when its experiment begins, with the previous findings
in hand, and is not edited afterwards to match what happened. This is the material that would be in front
of whoever writes it.

It exists because four arrivals turned out to be one question, and three of them arrived in a single
experiment looking like three candidates.

```text
06 exploration   request 5, as COST — what a decision costs should depend on the decision,
                 not on how much has been admitted by the time it is taken

09 collision     request 1, as CAPABILITY — a partial meeting: take the part two records
                 agree about without admitting the rest
                 request 2 — take another record's knowledge without taking its intentions
                 request 3 — say that two records agree without saying it twice
```

---

## The question

A persisted decision is a [`Taken`]: the decision, the entry that was most recent when it was applied,
and the **set of every entry that had been admitted**. The last is the witness, and the second is a
coordinate that says the same thing about the end of the prefix rather than about the whole of it.

> *A decision is recorded against what **stood**. What it is actually **about** is narrower, and nothing
> in the record says which.*

That is the whole of it, and each of the four arrivals is a consequence.

---

## Why it is one question and not four

The mechanism is one line of the application, in `cli/src/lineage.rs`:

```rust
if let Some(unexpected) = offered.difference(&witnessed).next() {   // UnwitnessedKnowledge
if let Some(missing)    = witnessed.difference(&offered).next() {   // WitnessedKnowledgeAbsent
```

Two directions, so the witness is a test of **set equality** and not of containment. Follow that one fact
into each request:

**A partial meeting** (09/1). The union of two repositories' knowledge is often one admissible journal —
measured, in Observation 10 of that experiment, where one of the two already held it. Handing it to a
rebuild with both lineages is refused at `UnwitnessedKnowledge`, because one side's decision was taken
over a shorter prefix. In the measured case the world would have come out **identical**; the rule that
would allow it cannot be stated, because nothing distinguishes an insertion that changes a world from one
that cannot.

**Knowledge without intention** (09/2). Taking another record's journal is the same refusal aimed at the
taker: once its own prefix has grown, *its own* standing decisions no longer witness the journal they sit
beside.

**Agreement said once** (09/3). Two repositories took the same decision and produced one world by
identity — and two *records*, differing in nothing but `after` and `witness`, because each was taken at a
different point in its own journal. So a merged repository answers for one world twice. Record a decision
against what it depends on and the two records **are** one record, and the duplicate never arises.

**And the cost** (06/5), which is the same fact with a number attached: 260 witnessed entries for thirteen
decisions, **18,226 of 23,553 lineage bytes** being witness — and 326 entries for the same thirteen
decisions taken later, the same record costing **21% more** for having enumerated before judging. The
dominant term of the record is driven by something the decisions are not about.

---

## What every arrival has been careful to say, and the protocol must not undo

**Removing the witness is not what is asked.** Three experiments concluded it is load-bearing, and the
last of them because it is what makes knowledge append-only: every standing decision names the entries
that stood, so a journal whose earlier entries moved makes those decisions disagree with it, and that is
what refuses a rewritten history. Convergence declined to call it redundant and three experiments have
used the refusal since.

**And the strictness is where the safety comes from.** A world is a function of the knowledge that stood
when the decision was applied: `lineage::decide` resolves a cut against the knowledge in hand and a
selection absorbs whatever that cut froze. Containment would let an inserted Event move a head in silence,
which is the shape the divergence experiment measured as *a coordinate that is wrong but well-formed is
not detectable from the record*.

So the question is not *may the witness be weaker*. It is:

> **Can a record say what a decision depends on — as a claim it can check — rather than what happened to
> have been admitted?**

---

## What is not decided here

Whether the answer is a derived summary of the prefix, a narrower witness computed from what the world
selects and cuts at, a claim about insensitivity that something can verify, or the finding that the two
cannot be told apart without re-deciding — which is a coherent answer and would close all four arrivals
with a *no*.

And one question of scope that belongs to whoever writes the protocol: three of the four arrivals are
about **two** records and one is about the size of **one**. An experiment that took only the second would
measure bytes; one that took only the first would measure meetings. What makes this a candidate rather
than either of those is that the same line of code answers both, and the sharp thing to establish first is
whether a narrower witness keeps what the current one buys.

**Related:** [`01-veracity`](01-veracity.md) — a witness that could be satisfied by a prefix it was not
taken against is exactly a record agreeing with itself and being wrong, which is that candidate's subject
approached from the writing side rather than the reading side.
