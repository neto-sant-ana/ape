//! `04-multiagent/run-b-prime` — the same task as `run-b`, run again after its arrangement was
//! corrected. The corpus's only near-duplicate.
//!
//! # What the pair is for
//!
//! Two testimonies of one task are the closest thing available to asking whether a kind belongs to
//! the task or to the agent. Two answers, and they disagree:
//!
//! ```text
//! recorded at    run-b 2026-01-07, run-b-prime 2026-01-06 — different instants for one brief
//! unclassified   run-b produced one, this produced NONE
//! ```
//!
//! **So motivation is not reliably produced even for the same task**, which is what stops two
//! occurrences in five from being a sixth kind. See `06-motivation-is-not-one-of-the-five.md`.
//!
//! # And it is the only testimony that measured a forgery
//!
//! It rewrote `by` on a throwaway copy and reported what happened: naming **the market** — the
//! counterparty being paid — reconstructs with no refusal at all, and naming a *role* is refused.
//! That is `00-authenticity`'s *forged `by`* face, demonstrated in the artifact rather than argued
//! from the docstring, by an agent that had not read the candidate.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::MultiagentBPrime,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::MultiagentBPrime,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn want(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentBPrime,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Want)),
        standing: Some(standing),
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentBPrime,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentBPrime,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const AGENT: Carrier = Carrier::Entity("Agent");
const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);
const CONDITIONS: Carrier = Carrier::Derived(Derived::Conditions);

const IN_THE_QUEUE: Standing = Standing::Tracked("lab/QUEUE.md");
const AUTHENTICITY: Standing = Standing::Tracked("lab/candidates/00-authenticity.md");

pub const CLAIMS: &[Claim] = &[
    housed(
        "Finance recorded the intention and decided the world that holds it. Both are in `repo/` now.",
        JOURNAL,
    ),
    unhoused(
        "labels are not stored beside identities, so the correspondence below is the one replay \
         implies (nth admission of a family ↔ nth identity of that family), and the program refuses \
         to guess if those two counts ever disagree.",
        Kind::MethodLimit,
    ),
    housed("| `house` | agent | `fe0e80f6…dd562` |", AGENT),
    housed("| `market` | agent | `0d3a24e8…c24ace` |", AGENT),
    housed("| `operations` | agent | `326993e9…37e0f8a3` |", AGENT),
    housed(
        "| `finance` | agent | \
         `108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b` |",
        AGENT,
    ),
    housed(
        "| `spender` | role | `3d359fe8…29d6f4f` |",
        Carrier::Entity("Role"),
    ),
    housed(
        "| `counterparty` | role | `48845bf3…4564626` |",
        Carrier::Entity("Role"),
    ),
    housed(
        "| `spend` → `counterparty` | statement | \
         `99c01bdc56bd31a9e1cead2132c360a9589ecc96916e3ebcdf65e0083334aebb` |",
        Carrier::Entity("Statement"),
    ),
    housed(
        "| `account` (of `cash`, bounded 0…1000) | resource instance | `13d1aa87…d2f05c` |",
        Carrier::Entity("ResourceInstance"),
    ),
    housed(
        "`finance` has held the `spender` role since 2026-01-03, which is why it can execute a \
         `spend`.",
        Carrier::Entity("EligibilityAssignment"),
    ),
    housed(
        "The house's world (`bbee1243…691a98a`, known at 2026-01-06) already held one settled \
         commitment (the market's 100 to the house, fulfilled 2026-01-02) and one open one — the \
         house's **20 to the market, due 2026-01-10**, which reads as `unsettled / breached` at the \
         14th.",
        WORLDS,
    ),
    unhoused(
        "Finance is taking on the storage while that one is already late; nobody asked about it, and \
         nothing here touches it.",
        Kind::Qualification,
    ),
    // ---- What was recorded / decided ---------------------------------------------------------
    housed(
        "One admission appended to the journal (entry 20 of 20):",
        JOURNAL,
    ),
    housed(
        "the house is accountable, **finance** is the executor, the market is the beneficiary; **30** \
         of `cash` leaves `account`; committed 2026-01-06, due **2026-01-14**.",
        COMMITMENT,
    ),
    housed(
        "One decision appended to the lineage, attributed to finance:",
        LINEAGE,
    ),
    housed(
        "**Fork of `bbee1243…691a98a`**, introducing that commitment, omitting nothing, under the cut \
         the parent already recognized (2026-01-06, head `ede70f26…5ed5b02`).",
        WORLDS,
    ),
    housed(
        "frozen `{7d86cc6c…}` (the fulfilled 100), open `{06b94c41… , 4de31818…}` — the storage 30 \
         and the overdue 20.",
        WORLDS,
    ),
    exposition(
        "An intention reaches a world only through `fork`, and a fork keeps its parent's cut — so the \
         commitment had to be *recorded at an instant the parent's cut already recognizes*, or \
         `ensure_selectable` would refuse it as anachronism.",
    ),
    unhoused(
        "Recording it at 2026-01-06 is what keeps this one decision instead of an advance-then-fork \
         pair.",
        Kind::RoadNotTaken,
    ),
    loss(
        "It is also why nothing in the repository says the invoice arrived after the house last \
         looked: the only instant available to finance is the one the house was already at.",
        Standing::Untracked,
    ),
    unhoused(
        "Independent check, not the writing process: the `ape-cli` binary, given nothing but the \
         repository path, an instance and a date.",
        Kind::MethodLimit,
    ),
    housed(
        "The storage 30 is **within deadline on the 14th** (a due date is not breached by arriving), \
         and feasibility under the final-state hypothesis reports **no conflict** — 100 in, 20 and 30 \
         out, inside the 0…1000 bound of `cash`.",
        FEASIBILITY,
    ),
    housed(
        "`level` is 100 because it counts only what a projection reports as fulfilled; neither payment \
         has settled.",
        CONDITIONS,
    ),
    // ---- Every object constructed, and what it asserts ---------------------------------------
    exposition(
        "| `Repository::open(\"repo\")` | Nothing. It names the directory the records live in. |",
    ),
    housed(
        "*That this commitment became known to the house on 2026-01-06, stated in exactly the fields \
         it was supplied with*",
        JOURNAL,
    ),
    housed(
        "*The house is accountable for paying the market 30 of cash out of `account` by 2026-01-14, \
         and finance is the party that will do it.*",
        COMMITMENT,
    ),
    housed(
        "*The world worth reasoning about is the house's world plus this new intention, under the \
         same knowledge.*",
        WORLDS,
    ),
    housed(
        "*That decision was taken after journal entry `06b94c41…`, with exactly those 20 entries \
         standing, by finance.*",
        LINEAGE,
    ),
    housed(
        "*Under what was known at 2026-01-06, this is a complete, historically closed continuation: \
         the 100 is unavoidable, the 20 and the 30 are still proposed.*",
        WORLDS,
    ),
    exposition(
        "*These are the worlds the recorded decisions produce* — a witness, weighed against the \
         rebuild on every read, never a source.",
    ),
    housed(
        "*This is what each world says about each commitment, and about `account`, as of 2026-01-14.*",
        CONDITIONS,
    ),
    unhoused(
        "It asserts that these labels in the journal resolve to these identities, and refuses to \
         answer if the two lists have drifted apart.",
        Kind::MethodLimit,
    ),
    // ---- What I needed and could not find ----------------------------------------------------
    want(
        "**1. There is no way to say that finance acts *for* the house.** Agents are flat: `Agent` is \
         a label, and the only relation between two agents is that both may hold a role. Nothing \
         composes one into another, delegates from one to another, or subordinates one.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "The nearest thing the kernel offers is `Assignment`'s split between `accountable` and \
         `executors`, and I used it — the house accountable, finance executing. But that is a weaker \
         claim than the brief makes",
        Kind::Qualification,
    ),
    loss(
        "as far as the kernel is concerned, finance could be named executor for *any* agent's debt, \
         and the house could be named accountable without ever having heard of it.",
        IN_THE_QUEUE,
    ),
    loss(
        "\"Finance took the house's invoice on\" is readable in the record only by someone who already \
         knows that finance is part of the house.",
        IN_THE_QUEUE,
    ),
    want(
        "**2. The journal has no author.** `Admission` carries `recorded_at` and no `by`.",
        IN_THE_QUEUE,
    ),
    loss(
        "A reader of the repository can see which worlds finance decided and cannot see which \
         knowledge finance admitted.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "**3. Attribution is a claim, and I measured how thin it is.** `lineage::attributed` checks \
         one thing: that the id names an agent already known at that coordinate. Two measurements on \
         a throwaway copy of the repository:",
        Kind::MethodLimit,
    ),
    // The forged `by`, demonstrated rather than argued — and it is `00-authenticity`'s own first face.
    loss(
        "rewriting `\"by\"` to the **market** — the counterparty being paid — reconstructs with no \
         refusal, and `decided market` then answers with finance's world;",
        AUTHENTICITY,
    ),
    housed(
        "rewriting it to the **`spender` role id** is refused, with `the decision is attributed to \
         3d359fe8…, whom nothing had admitted when it was taken`.",
        LINEAGE,
    ),
    loss(
        "So the record can distinguish \"not an agent\" from \"an agent\", and cannot distinguish \
         \"the agent that decided\" from \"any agent that existed\".",
        AUTHENTICITY,
    ),
    want(
        "Eligibility is not consulted for a decider (only for a commitment's executors), and nothing \
         ties the decider to the intention it introduced.",
        AUTHENTICITY,
    ),
    // Second occurrence, and `run-b` reached it independently on the same repository.
    want(
        "**4. `by` is optional, so silence and denial look the same.** The genesis in this repository \
         claims nobody. `decided_by` cannot tell *not finance's* from *unclaimed*",
        Standing::Untracked,
    ),
    want(
        "**5. Nothing records the invoice.** There is no claim/invoice/receivable primitive: what the \
         market sent is representable only as the house's own intention to pay.",
        Standing::Untracked,
    ),
    loss(
        "the market's side of the arrangement (its intention to have been paid) is not written \
         anywhere. Both parties' `Statement`s exist in the vocabulary; only one party's `Commitment` \
         does.",
        Standing::Untracked,
    ),
    // The `f64` half was met — `ConstraintKind` moved to `i128` — and the half that survives is the
    // one `02-scale` holds: what the count counts is the application's, and it was decided in
    // conversation and written nowhere.
    want(
        "There is no currency, unit or scale anywhere, so \"30\" is only as meaningful as the shared \
         assumption about what `cash` counts in.",
        Standing::Tracked("lab/candidates/02-scale.md"),
    ),
    // ---- Improvised --------------------------------------------------------------------------
    unhoused(
        "**The instant.** The brief gives a deadline and no today. I used **2026-01-06**, the instant \
         the house's tip already recognizes and the earliest one the recording watermark (at \
         2026-01-05) still admits. […] This is a choice, not a reading.",
        Kind::Qualification,
    ),
    unhoused(
        "Any later instant would have forced an `advance` before the `fork`, for a change in \
         knowledge that did not happen — no event was observed.",
        Kind::RoadNotTaken,
    ),
    // Second occurrence: `run-a` met the same seam and named it the same way.
    want(
        "There is no `extend(repository, decision)` in the crate; the three steps \
         (`replay_remaining`, `lineage::decide`, push) are done in `main.rs`.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "**Run once.** `main.rs` appends unconditionally. A second `cargo run` re-appends the same \
         admission record; the identity is content-derived so the Canon absorbs it as a no-op, but \
         `journal.json` would end up holding the record twice.",
        Kind::MethodLimit,
    ),
    want(
        "Nothing in the repository prevents that, and I did not add a guard.",
        Standing::Untracked,
    ),
];
