# Result

**Confirmed** for convergence. **Confirmed, narrowed** for identity — the claim held as stated, and
what it costs is larger than the protocol anticipated.

```text
convergence   two writers can extend one repository so that neither loses the
              other's decisions, and the result does not depend on write order   CONFIRMED

identity      a party that decides is a thing the record must hold, and holding
              it buys addressing rather than proof                               CONFIRMED,
                                                                                 NARROWED
```

The protocol wrote a refuted second half as the likely outcome. It is not refuted, and the reason is
worth being precise about rather than pleased with:

> **The boundary is authenticity. What sits on this side of it is a name, not a claim about a person.**

A recorded party is checkable as a *reference* — it names an admitted agent, known at the coordinate
the decision was taken at — and uncheckable as an *attribution*. Those are two different things, and
the record delivers the first without the second. That is exactly what the hypothesis said, which is
the only reason it survives.

## Against each criterion

**Convergence**

1. **The loss is demonstrated on a repository.** Three decisions taken and two on disk, and the
   surviving one is the last writer's. Not described — Phase 1 wrote and read files.
2. **After the repair both survive, and a party that cannot converge leaves nothing.** The merged
   repository is rebuilt in memory before any of the three files is written, and the refusal case was
   compared byte for byte against the state before it.
3. **Either order produces one repository**, compared file by file. Blinding the tie-break makes the
   two runs disagree at `lineage.json` line 32, which is the arrival order showing through.
4. **Two writers on different branches do not conflict, measured.** Each party's decision is in the
   merged file exactly as that party took it; both worlds extend the shared ancestor and neither
   extends the other; and a third party deciding what is already decided adds nothing.

**Identity**

5. **The absence is established by asking**, and it is stronger than provenance's. Every field of
   every decision and every world as a closed set, with the agents taken out of the journal rather
   than listed beside the assertion — and then the demonstration that inventories cannot give: two
   parties interleaving and one party deciding both produce the same repository, byte for byte.
6. **One field, and both questions answered.** What becomes impossible without it: a process with no
   memory cannot tell whose line is whose. What compares it on every read: the reference, and
   nothing else — stated where it stops rather than softened.
7. **What can be checked is checked inside reconstruction**, and it is one refusal covering two
   things: an identity that names no agent, and a party admitted after the decision that claims it.
   What cannot be checked is named and measured — swapping the two claims produces a repository that
   reconstructs, corroborates, and says the opposite of what happened.
8. **The arrangement reproduces whole through a fresh process**, against literals, including the
   question only the decider makes answerable.
9. **Nothing entered the engine.** `core/` is untouched across the branch.
10. **The four earlier experiments' conclusions stand.** Three are strengthened and one is
    generalized; the details are below.

**Failure conditions**, each addressed rather than assumed away: interleaving lost something; the
repair converges rather than serializing; branches were not serialized against each other; two things
about a decider are checkable; the engine knows nothing about who decides; and the second half is not
authenticity under another name, because addressing and proof came apart under measurement.

## Findings that outlast the verdict

**The contention was in the medium, not in the content.** Two decisions cannot contradict one
another — a lineage is a tree, and a second party's line is a branch. The whole-file write made two
parties compete for a byte range, and the loss looked like a concurrency problem about intention
because that is where it surfaced. The Canon's compare-and-append against one head is right one layer
down and would have manufactured a conflict one layer up.

**A tear is not a worse repository than a lost update. It is a louder one.** Corroboration compares
two representations of one fact, so a write landing between them disagrees and a write landing before
both moves them consistently. The interleaving that looks like corruption is the recoverable one.

**Nothing refers to a decision that nothing extends, and a second party's line is always a leaf.**
The lineage refers to the journal and nothing refers to the lineage, so knowledge lost alone is named
and intention lost alone is not. Both detectable cases turned out to be covered twice, by guards built
in different experiments; the leaf is covered zero times.

**Knowledge appends because a decision points at it.** A journal ordered by anything but arrival makes
standing decisions disagree with it, measured by building the journal a sorting merge would have
produced. Append-only is a requirement of the record on disk rather than a shape inherited from the
Canon.

**There is no room on a world for who decided it.** One world can be produced by two decisions, so a
per-world answer would have to choose a claim. Provenance's finding reached from the other side, for
the same reason: identity is derived from content, and a party is not content.

**Recording a party makes agreement look like duplication.** Two parties deciding the same fork
produce one world and two records. Which is a cost, and is also — for that case and only that case —
the agreement record Phase 3 said was missing.

**A field's shape can be decided by concluded experiments.** A mandatory decider would have forced
five published subjects to name parties they never had. Found twice: once in the field, once in the
subject's own entry points, at 136 lines of churn across measurements already recorded.

## What changed in the experiments before it

**Nothing was overturned.** Three conclusions are strengthened and one is generalized.

**The sequence witness, a third time and a different kind.** Convergence declined to call it redundant
with the address it duplicates; Phase 1 found that with the address blinded it is the only thing that
notices missing knowledge. Here it is not a check that catches something — it is what makes append-only
true. A repository holding only each decision's address would tolerate a reordered journal in silence.

**Source and Target are roles, not sides — and the cost is now measured.** Convergence concluded the
asymmetry is a gain, and that stands. With two parties, mutual adoption produces two worlds selecting
exactly the same commitments and differing in nothing but whose branch they are on. The parties agree
and nothing says so. Not repaired, because one world would have to pick a parent, which is picking a
party.

**A question is asked of a record — and a party can be named the same way.** The transfer machinery
needed nothing for two parties that it did not need for two lines, except that both lines be in the
archive. Which is the whole dependency between Part A and Part B: reaching each other is downstream of
converging.

**Provenance generalizes.** *There is no room on a world for where it came from* becomes *or for who
decided it*, and the underlying reason is the one provenance already gave.

**And the exclusion four protocols repeated was hiding a missing path.** Reading a repository was
complete; extending one did not exist. Not merely unmeasured — unbuilt, and unnoticed because a
laboratory needs a reader in order to compare things.

## What was not reverted, and why this differs from the experiment before

Provenance built Part B and removed it, on the argument that a guard which guards nothing keeps a
thing alive. Part B stays here, and the difference is not sentiment.

```text
provenance   the claim predicted the transfer, which every rival account agreed
             about, so the check accepted precisely what a search finds
coordination the claim predicts nothing, and the check refuses two real cases:
             an identity that is not an agent, and a party that did not exist yet
```

And there is a consumer that survives the process, which provenance's did not have. A party that was
there can compute the other's line by subtraction from its own memory; a party that terminated cannot,
and neither can a third reader. Since surviving the process is what the repository is for, the decider
is the only thing that makes *whose* survive alongside *what*.

The 02 rule is not violated: a derived value written and not compared is a liability, and a decider is
not derived. Which the protocol said plainly in advance — the rule does not forbid it, and does not
protect it either.

## Variables the protocol left open

**Whether the two notions of agent should meet.** They share a **population, not a purpose**. An
`AgentId` is a party to a commitment in the kernel and a party that decides here; in this arrangement
the two sets are disjoint. Sharing the type permits an overlap without assuming one, and a separate
population would have forbidden the ordinary case where a merchant plans its own spending. Settled
provisionally, for this boundary.

**Whether the journal and the lineage deserve the same repair.** No, and the reason is on disk rather
than in the nature of knowledge.

**The adapter's declined contract is a finding, not a debt.** The CLI runs the engine's single-threaded
conformance and declines `verify_thread_safe` in writing. That is correct and now justified: each party
replays the journal into a `Canon` of its own, so no adapter is shared between them. The thing that has
to serialize is a **file**, and the engine's port has nothing to say about files. The protocol
predicted this and it held.

**Abandoned siblings** stay deferred, with a second reason now: a party may not agree that a line is
abandoned, and that is not a claim anybody wrote.

## What the result does not cover

**Parallelism.** Deliberately, and the boundary is honest about what that leaves. Interleaving was
enough to produce the loss and is reproducible where a race is not, so every measurement here is
deterministic. What is **not** claimed is durability: the three files are still written separately, so
a converging party's write is not atomic against the filesystem. What is measured is that a torn
repository is *refused*, not that it cannot occur. An atomic commit — a rename, a lock — is a question
this experiment is arranged to be able to ask and does not answer.

**Authenticity.** Unchanged from the corroboration experiment, and now the exact boundary of the second
half rather than a neighbouring concern.

**Saying that two lines are the same plan.** Phase 3 measured the gap and Phase 5 filled a corner of
it. A statement about the relation between two lines is still not expressible.

**Two repositories meeting.** One repository, two writers. Open since convergence.

**Cost.** Deferred by six experiments now.

---

# Architectural Consequences

## Established for the CLI

* **Intention merges; knowledge appends.** Two decisions cannot contradict, so the union of two
  parties' decisions is a lineage. Knowledge may only be appended, because a decision addresses it by
  what stood before it.
* **A file is a sequence and a lineage is not**, so the record holds a linearization — chosen from what
  the decisions carry, and therefore independent of who arrived first.
* **A party that cannot converge writes nothing.** Verified before written, whole.
* **A decider is a reference, checked as one.** That the party exists and was known at the coordinate;
  never that it decided.
* **Whose survives the process only if it is written down.** *What* is derived; *whose* is not.
* **A world holds neither where it came from nor who decided it.** Both are properties of a decision,
  and a decision is not the thing it produces.

## Candidates for later experiments

* **Atomic commit** — the durability half of what Part A repaired, now the nearest unanswered question.
* **Authenticity**, unchanged, and now load-bearing for anything built on a decider.
* **Two repositories meeting**, open since convergence.
* **Saying two lines agree**, measured twice and expressible nowhere.
* **Abandoned siblings**, with a second party as a second reason.
* **Cost**, deferred by six experiments.
