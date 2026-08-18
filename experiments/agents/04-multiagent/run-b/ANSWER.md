# Finance takes on the storage invoice

`src/main.rs`, run with `cargo run`. It reads `repo/`, adds one commitment and one decision, writes
the repository back, and then reads it again from the repository alone.

## Who is who, as the records say

Nothing in the program is written in terms of an identity. Every party, resource and statement is
found by the label the journal admitted it under, so the program is answerable to the brief
("finance", "the market", "the house") rather than to the fixture's hashes — and if the records
named someone else, it would refuse instead of acting on the wrong agent.

| label | identity (head) |
| --- | --- |
| house | `fe0e80f6` |
| market | `0d3a24e8` |
| operations | `326993e9` |
| finance | `10807723` |
| spender / counterparty (roles) | `3d359fe8` / `48845bf3` |
| `account`, the one instance of `cash` (bounds `[0, 1000]`) | `13d1aa87` |
| the statement a payable is stated in — *spender **spend** for counterparty* | `99c01bdc` |

What the house already knew: a receivable of `+100` settled on 2026-01-02 (`7d86cc6c`), a payable of
`-20` due 2026-01-10 (`4de31818`), and a payable of `-60` due 2026-01-20 (`0df2ea53`). Three worlds
had been decided; the last two were claimed by **operations**, which forked away the `-20` in favour
of the `-60`. The genesis is claimed by nobody. The tip finance planned from is `1cd9afbb`.

## What I recorded

One commitment, admitted through `ape-cli`'s journal into canonical history — `a0edbe20`:

> the house commits to spend **30** out of `account` to the market, committed 2026-01-07, **due
> 2026-01-14**, executed by finance, depending on nothing.

It reuses the vocabulary the house already has (`99c01bdc`, the statement whose action is `spend`
with effect `Decrease` on `cash`). Nothing else was admitted: inventing a second way to say "the
house pays the market" would be a second version of the same fact.

## What I decided

One **fork** of the tip `1cd9afbb`, omitting nothing and introducing `a0edbe20`, recorded as
**taken by finance**. It produced world **`1f1afd4e`**: same cut (2026-01-07, event head
`ede70f26`), frozen `{+100}`, open `{-60, -30}`.

A fork, and not an advance, because `advance` recognizes later knowledge and never adds an
intention — a commitment merely admitted to history is in nobody's plan until a decision puts it
there.

Once every movement lands the account holds `100 - 60 - 30 = 10`, inside the `[0, 1000]` its
resource declares, and feasibility under `FinalState` reports no conflict for any of the four
worlds.

Put back with `converge::converge`, which re-reads the repository, appends to the journal it finds
there, merges the decisions, rebuilds the whole thing in memory and only then writes. `repo/` now
holds 21 admissions and 4 decisions. Read back two ways, neither of which sees a value this program
computed: `reading::reconstruct` in-process, and the separate `ape-cli` binary, which given only
the repository answers

```
$ cargo run -p ape-cli -- repo decided 108077234acde...  # finance
[ "1f1afd4e1f82731a26e85ccc0974eee9bd2d090dd2dda7cc5defc9b0079cd55e" ]
```

Running `cargo run` again is a no-op: it finds the invoice and the world already there and writes
nothing.

## Every object I construct, and what it asserts

| object | what it asserts |
| --- | --- |
| `Repository::open("repo")` | Nothing. It names a directory. |
| `Corroborated` (from `reading::corroborated`) | That the journal and lineage on disk rebuild *exactly* the worlds `worlds.json` records — every coordinate of every world agrees. |
| `Cast` | That these identities are the ones the house's records label house, market, operations, finance, spender, counterparty and `account`, and that `99c01bdc` is the statement whose action is `spend`. |
| `Date("2026-01-07")` and `Date("2026-01-14")` | Each asserts one civil day: the instant the invoice is recorded and committed at, and the day it falls due — which is also the day the worlds are read at, at the end. |
| `Admission::Commitment{…}` | That on 2026-01-07 this was admitted as knowledge: the house spends 30 out of `account` to the market by 2026-01-14, executed by finance. It is the input record, and it asserts *what was supplied*, which is what replay needs. |
| `Assignment` (built from it by the journal) | That the house answers for the payment, finance performs it, and the market benefits from it. |
| `Term` | That the commitment was made on 2026-01-07 and is due on 2026-01-14 — and, by construction, that the first is not after the second. |
| `ActionValue::value(30.0)` | That the movement has magnitude 30. Its *direction* is asserted elsewhere: by the action `spend`, which is `Decrease`. |
| the `Commitment` `a0edbe20` the engine emits | Intended reality: this obligation exists, its parties are eligible for the roles its statement requires, and it moves this instance of this resource. |
| `Decision::Fork{extends: 1cd9afbb, omitted: {}, introduced: {a0edbe20}}` | That the intention worth reasoning about is the tip's intention plus this payable, under the cut the tip already recognizes — an outcome, not a transition. |
| `Taken::claimed(fork, finance, …)` | That this fork was taken **by finance**, after journal entry `a0edbe20`, with exactly those 21 entries standing. The last two are checkable; the first is a claim (see below). |
| the `Thesis` `1f1afd4e` the fork produces | That there is a world at cut 2026-01-07 / head `ede70f26` whose frozen past is `{+100}` and whose open future is `{-60, -30}`, descending from `1cd9afbb`. |
| four `WorldRecord`s (written by `converge`) | What each decided world *is*, in the application's vocabulary — written down so a later reader has a second representation to weigh against the one the decisions produce. |
| four `Reading`s | What each world says at 2026-01-14: each commitment's outcome and timeliness, the settled level of `account`, and the conflicts under `FinalState`. |

## What I needed and could not find

**The journal cannot say who recorded an entry.** This is the one that bit. `Admission` has no
author field of any kind — only a decision (`Taken`) carries `by`. So half the task, "record what
finance intends", is *unattributable by construction*: the invoice sits in `journal.json` and
nothing in it says finance put it there. What I did instead was make finance the **executor** of
the commitment, which the house's eligibility record of 2026-01-03 (finance in the `spender` role)
makes admissible. I want to be plain that this is not the same claim: `executors` asserts *who
performs the payment*, not *who wrote the record*. It is the nearest thing available, and a reader
who takes it as authorship would be reading something the engine never said. The asymmetry looks
deliberate in the crates (`lineage.rs` argues at length for `by` on a decision), but the
consequence is that a party's hand shows in knowledge only when the party happens to be a
participant in it.

**Accountable vs. executor is a modelling choice the brief does not settle.** I made the house
accountable and finance the executor: the market invoiced *the house*, so the house is who answers
to the market, and finance is who takes the payment on. The alternative — finance accountable, on
the reading that "finance has to take that on" means finance answers for it — is equally
defensible, and the two produce different commitments and therefore different identities. The
existing payables in `repo/` use `accountable: house, executors: [house]`, which does not settle
it either. I did not want to pick silently.

**Nothing records what a payment is *for*.** There is no description, reference, memo or document
field anywhere in the ontology. "Storage" and "invoice" are nowhere in what I wrote; the record
says 30, out of `account`, to the market, by the 14th. Carrying the *nature* of the expense would
mean new vocabulary — a Resource, Action and Statement per kind of expense — which is a change to
the house's ontology rather than a way of taking on one invoice, so I did not do it. If the house
ever needs to tell a storage payable from a freight one, that is the shape the answer will have to
take, and it is a decision above my level here.

**Nothing checks that finance was entitled to decide.** `Taken::claimed` is verified only to the
extent that the named agent was already known at the coordinate (`attributed`, in `lineage.rs`); no
role, eligibility or authority is consulted, and the crate says so — the attribution "is witnessed
by nothing but the writer who wrote it". So the repository records that finance *claims* this
decision, not that finance could take it.

**`decided_by` cannot tell "unclaimed" from "not this party's".** The genesis names nobody, and
asking for finance's worlds simply does not list it. There is no way to ask "which decisions have
no party".

**No stored number answers "will the account hold enough".** `Reading::level` counts only what a
projection reports as *fulfilled*, so it stays `100` in every world including the one that plans to
spend `90`; feasibility reports conflicts, not levels. To print `100 - 60 - 30 = 10` I summed
`movement_of` myself. That is the documented division of labour (`level.rs`: which criterion is the
application's question), not a defect — but a reader of `repo/` alone will not find the projected
balance anywhere, and the arithmetic in my output is mine.

**I had to choose the recording instant.** The brief gives no date for the invoice, only the
deadline. I recorded at **2026-01-07**, the most recent instant the house's knowledge already
holds, so nothing was invented. Recording on a later day would have forced an `advance` before the
fork — a fork inherits its parent's cut, and a commitment recorded after that cut cannot be
selected (`CommitmentNotKnownAtCut`) — and an advance asserts recognition of later knowledge that
finance had no observation to justify.

**Two small frictions, for completeness.** A `Constraint` cannot be read back off an admitted
`Resource`, so the `[0, 1000]` in my output is read from the journal record that supplied it. And
`cargo test -p ape-cli` does not compile in this copy of the crates: `cli/src/reading.rs:381`
refers to `crate::subject::divergence`, a module the crate does not have. I did not lean on that
suite; the library itself builds clean.
