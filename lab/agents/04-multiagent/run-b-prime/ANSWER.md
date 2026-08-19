# Finance takes on the storage invoice

The market invoiced the house for storage: **30, payable by the 14th**. Finance recorded the
intention and decided the world that holds it. Both are in `repo/` now.

Run once with `cargo run` (it appends; see *Improvised* at the end).

---

## Who was already in the record

Read out of `repo/journal.json` by replaying it — labels are not stored beside identities, so the
correspondence below is the one replay implies (nth admission of a family ↔ nth identity of that
family), and the program refuses to guess if those two counts ever disagree.

| label | kind | identity |
| --- | --- | --- |
| `house` | agent | `fe0e80f6…dd562` |
| `market` | agent | `0d3a24e8…c24ace` |
| `operations` | agent | `326993e9…37e0f8a3` |
| `finance` | agent | `108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b` |
| `spender` | role | `3d359fe8…29d6f4f` |
| `counterparty` | role | `48845bf3…4564626` |
| `spend` → `counterparty` | statement | `99c01bdc56bd31a9e1cead2132c360a9589ecc96916e3ebcdf65e0083334aebb` |
| `account` (of `cash`, bounded 0…1000) | resource instance | `13d1aa87…d2f05c` |

`finance` has held the `spender` role since 2026-01-03, which is why it can execute a `spend`.
The house's world (`bbee1243…691a98a`, known at 2026-01-06) already held one settled commitment
(the market's 100 to the house, fulfilled 2026-01-02) and one open one — the house's **20 to the
market, due 2026-01-10**, which reads as `unsettled / breached` at the 14th. Finance is taking on
the storage while that one is already late; nobody asked about it, and nothing here touches it.

## What was recorded

One admission appended to the journal (entry 20 of 20):

**Commitment `06b94c41827b09c3aeb817b8be12954c877ec94102a8f690adeb0c0b5bb5c9c7`** — the house is
accountable, **finance** is the executor, the market is the beneficiary; **30** of `cash` leaves
`account`; committed 2026-01-06, due **2026-01-14**.

## What was decided

One decision appended to the lineage, attributed to finance:

**Fork of `bbee1243…691a98a`**, introducing that commitment, omitting nothing, under the cut the
parent already recognized (2026-01-06, head `ede70f26…5ed5b02`).

It produced world **`791528450d627c4523dbfa073d231ef0884c5e78c4d564be6f65059b9875f737`**: frozen
`{7d86cc6c…}` (the fulfilled 100), open `{06b94c41… , 4de31818…}` — the storage 30 and the
overdue 20.

**Why a fork and not an advance.** `advance` moves the cut and adopts nothing: a commitment
admitted between two cuts and left unsettled does not enter a selection ("advance changes what
could be known, without deciding what should be intended"). An intention reaches a world only
through `fork`, and a fork keeps its parent's cut — so the commitment had to be *recorded at an
instant the parent's cut already recognizes*, or `ensure_selectable` would refuse it as
anachronism. Recording it at 2026-01-06 is what keeps this one decision instead of an
advance-then-fork pair. It is also why nothing in the repository says the invoice arrived after
the house last looked: the only instant available to finance is the one the house was already at.

## What the repository says now, read by a different process

Independent check, not the writing process: the `ape-cli` binary, given nothing but the
repository path, an instance and a date.

```
$ cargo run -p ape-cli -- repo decided 108077234acde…c71970b
[ "791528450d627c4523dbfa073d231ef0884c5e78c4d564be6f65059b9875f737" ]

$ cargo run -p ape-cli -- repo 13d1aa87…d2f05c 2026-01-14
… world 7915284…  06b94c41… → unsettled / within-deadline
                  4de31818… → unsettled / breached
                  7d86cc6c… → fulfilled
   level 100.0   conflicts []
```

The storage 30 is **within deadline on the 14th** (a due date is not breached by arriving), and
feasibility under the final-state hypothesis reports **no conflict** — 100 in, 20 and 30 out,
inside the 0…1000 bound of `cash`. `level` is 100 because it counts only what a projection
reports as fulfilled; neither payment has settled.

---

## Every object constructed, and what it asserts

| object | what it asserts |
| --- | --- |
| `Repository::open("repo")` | Nothing. It names the directory the records live in. |
| `Admission::Commitment{…}` | *That this commitment became known to the house on 2026-01-06, stated in exactly the fields it was supplied with* — the journal record, which is what replay re-admits. |
| the `Commitment` the Canon emitted from it (through `CommitmentInput` / `Assignment` / `Term` / `ActionValue`, inside `journal::admit`) | *The house is accountable for paying the market 30 of cash out of `account` by 2026-01-14, and finance is the party that will do it.* |
| `Decision::Fork{extends, omitted: {}, introduced: {06b94c41…}}` | *The world worth reasoning about is the house's world plus this new intention, under the same knowledge.* |
| `Taken` (via `Taken::claimed`) | *That decision was taken after journal entry `06b94c41…`, with exactly those 20 entries standing, by finance.* |
| the `Thesis` `7915284…` the fork produced | *Under what was known at 2026-01-06, this is a complete, historically closed continuation: the 100 is unavoidable, the 20 and the 30 are still proposed.* |
| the two `WorldRecord`s written to `worlds.json` | *These are the worlds the recorded decisions produce* — a witness, weighed against the rebuild on every read, never a source. |
| the `Reading`s printed at the end | *This is what each world says about each commitment, and about `account`, as of 2026-01-14.* |
| `Vocabulary` (local to `main.rs`) | Nothing about the house. It asserts that these labels in the journal resolve to these identities, and refuses to answer if the two lists have drifted apart. |

---

## What I needed and could not find

**1. There is no way to say that finance acts *for* the house.** Agents are flat: `Agent` is a
label, and the only relation between two agents is that both may hold a role. Nothing composes
one into another, delegates from one to another, or subordinates one. The nearest thing the kernel
offers is `Assignment`'s split between `accountable` and `executors`, and I used it — the house
accountable, finance executing. But that is a weaker claim than the brief makes: as far as the
kernel is concerned, finance could be named executor for *any* agent's debt, and the house could
be named accountable without ever having heard of it. "Finance took the house's invoice on" is
readable in the record only by someone who already knows that finance is part of the house.

**2. The journal has no author.** `Admission` carries `recorded_at` and no `by`. So the first half
of the task — *record what finance intends* — leaves no trace of finance in `journal.json`
whatsoever; the commitment names finance as executor, which is a fact about the payment, not about
who wrote it down. Only the **decision** carries a party (`Taken.by`). A reader of the repository
can see which worlds finance decided and cannot see which knowledge finance admitted.

**3. Attribution is a claim, and I measured how thin it is.** `lineage::attributed` checks one
thing: that the id names an agent already known at that coordinate. Two measurements on a throwaway
copy of the repository:

- rewriting `"by"` to the **market** — the counterparty being paid — reconstructs with no refusal,
  and `decided market` then answers with finance's world;
- rewriting it to the **`spender` role id** is refused, with
  `the decision is attributed to 3d359fe8…, whom nothing had admitted when it was taken`.

So the record can distinguish "not an agent" from "an agent", and cannot distinguish "the agent
that decided" from "any agent that existed". Eligibility is not consulted for a decider (only for
a commitment's executors), and nothing ties the decider to the intention it introduced. The CLI's
own docstring says as much; I am reporting that it is true in the artifact, not only in the
comment.

**4. `by` is optional, so silence and denial look the same.** The genesis in this repository claims
nobody. `decided_by` cannot tell *not finance's* from *unclaimed*, so nothing in `repo/` says who
decided the world finance forked from — presumably the house, but the record does not say it.

**5. Nothing records the invoice.** There is no claim/invoice/receivable primitive: what the market
sent is representable only as the house's own intention to pay. The direction of the paper — *they
billed us* — is not distinguishable in the record from *we decided to pay them*, and the market's
side of the arrangement (its intention to have been paid) is not written anywhere. Both parties'
`Statement`s exist in the vocabulary; only one party's `Commitment` does.

**6. `magnitude` is a bare `f64` against a bounded resource.** "30" is 30.0 of `cash`, whose
constraint is `0…1000`. There is no currency, unit or scale anywhere, so "30" is only as meaningful
as the shared assumption about what `cash` counts in.

## Improvised

- **The instant.** The brief gives a deadline and no today. I used **2026-01-06**, the instant the
  house's tip already recognizes and the earliest one the recording watermark (at 2026-01-05) still
  admits. Any later instant would have forced an `advance` before the `fork`, for a change in
  knowledge that did not happen — no event was observed. This is a choice, not a reading.
- **Extending the lineage by hand.** `converge` zips a party's `decisions` against the worlds its
  `lineage` produced, so a new `Taken` has to be pushed *and* the corresponding `Thesis` decided
  before converging, or the new decision would be dropped from the union in silence. There is no
  `extend(repository, decision)` in the crate; the three steps (`replay_remaining`,
  `lineage::decide`, push) are done in `main.rs`.
- **Run once.** `main.rs` appends unconditionally. A second `cargo run` re-appends the same
  admission record; the identity is content-derived so the Canon absorbs it as a no-op, but
  `journal.json` would end up holding the record twice. Nothing in the repository prevents that,
  and I did not add a guard.
