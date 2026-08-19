# Standing the courier slot down, and putting the inventory purchase in its place

Run with `cargo run` from this directory. It reads `repo/`, acts, writes `repo/` back, and then
reads the repository again from nothing else to show what it now holds.

`repo/` already holds the result, so a run now reports *nothing to do* and writes nothing: the
world on file already stands the slot down, and forking it again would produce a world that
changes nothing (which the engine refuses as `SelectionUnchanged`). Run against the original
`repo/`, the program reproduces the current three files **byte for byte** — checked by keeping a
copy and `diff -r`, which is the only claim of determinism made here that was actually measured.

## Who is who, read out of `repo/`

Nothing below was assumed; every label was resolved from the journal by replaying it.

| party | identity | eligible |
|---|---|---|
| house | `fe0e80f6…` | spender, from 2026-01-01 |
| market | `0d3a24e8…` | counterparty, from 2026-01-01 |
| **operations** | `326993e9…` | spender, from 2026-01-03 |
| finance | `10807723…` | spender, from 2026-01-03 |

One resource, `cash`, bounded to `[0, 1000]`, with one instance, `account`. Two actions on it,
`receive` (increase) and `spend` (decrease). Two statements: *counterparty receives for spender*
and *spender spends for counterparty*, each fulfilled by `Settled` and cancelled by `Cancelled`.

Two commitments were on file. `7d86cc6c…` — market receives 100 into the account for the house,
due 2026-01-02 — is settled by the one Event in the chain (`ede70f26…`, `Settled`, 2026-01-02).
`4de31818…` — **house spends 20 of the account to the market, due 2026-01-10** — is the courier
slot, and it is the single open commitment of the single world on file (`bbee1243…`, a genesis cut
at 2026-01-06). That is how the slot was identified: an open commitment of the world operations
holds, whose magnitude is 20 and whose due date is the 10th. Two matches would have made the
program refuse rather than guess.

The genesis decision on file claims **no party** (`by` absent). Both decisions added here claim
operations.

## What I recorded

**One admission, appended to the journal as entry 20** — `0df2ea53…`, admitted through
`Canon::admit_commitment` by way of `ape_cli::journal::replay_remaining`, recorded at 2026-01-07:

> House is accountable and executes; the market benefits; under the *spender spends for
> counterparty* statement; against the `account` instance of `cash`; committed 2026-01-07, due
> 2026-01-20; magnitude 60; no dependencies.

It is built **from the slot it replaces** — same accountable party, same executors, same
beneficiaries, same statement, same instance — because the purchase takes the slot's place in the
same relationship, and only its size and deadline differ.

## What I decided

Two decisions, both attributed to operations (`Taken::claimed`), both taken after journal entry 20
and witnessing all 20 entries:

1. **advance** `bbee1243…` to `known_at` 2026-01-07 → world `ddef2011…`. History imposed nothing
   (0 commitments); the chain head is unchanged. This decision exists because a fork inherits its
   parent's cut, and the purchase was recorded after 2026-01-06 — selecting it under the old cut is
   refused as `CommitmentNotKnownAtCut`. Recognizing a later day is not intending anything, which
   is why the engine keeps it a separate step.
2. **fork** `ddef2011…`, omitting `4de31818…` (the courier slot) and introducing `0df2ea53…` (the
   purchase) → world **`1cd9afbb…`**, whose frozen past is the settled 100 and whose open future
   is the purchase alone.

Written back with `ape_cli::converge::converge`, which merges into the repository as it stands
rather than overwriting what it read, and rebuilds everything in memory before writing a byte.
`repo/` now holds a 20-entry journal, 3 decisions and 3 worlds; `converge::holds` confirms
`1cd9afbb…` is there, and the house's own binary agrees without my program:

```
cargo run -p ape-cli -- repo decided 326993e9…
["1cd9afbb…", "ddef2011…"]
```

## For every object I construct, what it asserts

- **`Admission::Commitment` (`0df2ea53…`)** — the house owes the market a spend of 60 from the
  account, committed on 2026-01-07 and due on 2026-01-20, depending on nothing.
- **`Decision::Advance { extends: bbee1243…, known_at: "2026-01-07" }`** — operations now reasons
  about the same intention under everything knowable on 2026-01-07.
- **`Decision::Fork { extends: ddef2011…, omitted: {4de31818…}, introduced: {0df2ea53…} }`** —
  operations no longer intends the courier slot and does intend the purchase, at the cut it
  already recognizes.
- **`Taken::claimed(advance, operations, …)`** — operations took that advancement, at the point in
  the journal where 20 entries stood.
- **`Taken::claimed(fork, operations, …)`** — operations took that substitution, at the same point.
- **`Repository::open("repo")`** — the records the house keeps live in `repo/`.
- **`Corroborated { … }`** (rebuilt to hand to `converge`) — this is one party's working copy of
  the repository: the knowledge it admitted, the worlds its decisions produced, and how far into
  the journal it read.

Two objects the run *reads* but does not author, worth naming because the output quotes them:
`WorldRecord` asserts that a world with these coordinates was produced by these decisions, and
`Reading` asserts what one world says about each commitment at one instant.

## What I did not do, and why

**No Event.** Standing the slot down is not something that happened; it is something operations
decided. The obvious alternative was to admit an Event with observation `Cancelled` against
`4de31818…`, which the statement's settlement vocabulary allows. I did not, for two reasons that
belong in a review rather than in a docstring: an Event is a fact, and no fact occurred; and a
settlement freezes the commitment in **every** world derived from that point on, including the
market's, so one party would have closed a bilateral arrangement by writing into shared history.
A fork keeps the withdrawal where it belongs — in operations' intention — and leaves the fact
alone. If the house wants the market to be *told*, that is a different act, and see below.

**No new statement, action, resource or role.** Everything the purchase needs already existed.

## What I needed and could not find

- **Nothing says what a commitment is *for*.** In the record, "a courier slot costing 20" and "an
  inventory purchase of 60" are the same object with different numbers: same statement, same
  action, same instance, same parties. I located the slot by magnitude-and-due-date, which works
  here only because exactly one open commitment matches — the program refuses instead of guessing
  if that stops being true. There is no label, tag or memo field on a commitment; the vocabulary
  is `Role`, `Agent`, `Resource`, `ResourceInstance`, `Action`, `Statement`, and only the first four
  carry an `Identifier`. If two courier slots of 20 due on the 10th ever coexist, no reader of
  `repo/` can tell which one was stood down.
- **"To be delivered by the 20th" is recorded as a payment, not as a delivery.** The only resource
  in `repo/` is `cash`, and the only actions on it are `receive` and `spend`. So the purchase is
  modelled as its cash leg — the house spending 60 by the 20th. The goods themselves have no
  resource, so nothing in the record says inventory arrives. That would need a second resource and
  a second statement (market delivers for spender), which is knowledge the house has not admitted,
  and I did not invent it.
- **Nothing tells the market.** The slot is an arrangement *with* the market, and a fork is
  unilateral: after this run the commitment `4de31818…` is still in canonical history, still
  unsettled, and still selected by the two worlds that preceded the fork — read at 2026-01-20, both
  of them report it `Breached`. Operations' world simply does not hold it. There is no
  notification, acknowledgement or bilateral-agreement primitive in either crate; the reconciliation
  of two parties' worlds is what the Synthesis layer is for, and it is asked *about* worlds by
  someone outside them. So "the slot is stood down" is true of operations' intention and of nothing
  else.
- **The advance is attributed to operations, and attribution is a claim.** `Taken::claimed` is
  checked for exactly two things — that the identity names an agent, and that the agent was already
  known when the decision was taken. That the party named is the party that decided is witnessed
  by nothing but the writer; the crate's own docstring says so, and I am repeating it because the
  output makes it look verified.
- **"Operations acts for the house" cannot be said.** The purchase is accountable to the **house**,
  matching the slot it replaces, while the decision is claimed by **operations**. The kernel would
  have accepted operations as accountable — operations holds the spender role from 2026-01-03 — but
  that would assert that operations owes the market 60, which is not what the brief describes.
  There is no on-behalf-of relation between agents; the lineage attribution is the only place the
  distinction survives, and it says who decided, not who was represented.
- **2026-01-07 is my choice.** The brief gives no "today". The journal stops at 2026-01-05 and the
  world on file was cut at 2026-01-06, so the recording instant had to be at least the 6th;
  recording on the 6th would have inserted knowledge into the instant an existing decision names,
  and I preferred the next day, which also makes the advance a plain move of the instant with the
  chain unchanged. Any date from 2026-01-06 through 2026-01-20 would have worked.
- **`Corroborated` had to be assembled by hand to write back.** `converge::converge` takes a
  `Corroborated`, and the only thing that produces one is `reading::corroborated`, which reads a
  repository. A party that has *extended* what it read has to rebuild the struct field by field
  (the fields are public, so this is intended rather than a workaround) — there is no
  `extend`/`with_decision` on it. Not a gap in semantics, just a seam worth naming.
