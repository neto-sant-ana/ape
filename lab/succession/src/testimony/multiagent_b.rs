//! `04-multiagent/run-b` — finance, the second party, working from the tip operations left.
//!
//! A technical report to an operator, and the testimony where **the same three wants arrive for the
//! third time**: no author on an admission, nothing that says what a payment is for, and a
//! `Constraint` that cannot be read back off the `Resource` it was admitted with.
//!
//! # It is also where two agents disagree about what the record should have said
//!
//! `run-a` made the house accountable and the house the executor. This run made the house
//! accountable and **finance** the executor, and says outright that the alternative — finance
//! accountable — *is equally defensible, and the two produce different commitments and therefore
//! different identities.* Two agents, one vocabulary, two admissible readings of one brief, and the
//! record has no way to hold which reading was meant.
//!
//! That is a road not taken by the classification's letter. It is also the clearest thing in the
//! corpus about **why** H4's material matters: the two identities differ, so the choice is not a
//! matter of presentation.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn want(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Want)),
        standing: Some(standing),
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn unclassified(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Unhoused(None),
        standing: None,
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentB,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const AGENT: Carrier = Carrier::Entity("Agent");
const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const STATEMENT: Carrier = Carrier::Entity("Statement");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);
const CONDITIONS: Carrier = Carrier::Derived(Derived::Conditions);

const IN_THE_QUEUE: Standing = Standing::Tracked("lab/QUEUE.md");
const IN_01: Standing = Standing::Recorded("lab/agents/01-single-agent/99-result.md");

pub const CLAIMS: &[Claim] = &[
    unhoused(
        "Every party, resource and statement is found by the label the journal admitted it under, so \
         the program is answerable to the brief (\"finance\", \"the market\", \"the house\") rather \
         than to the fixture's hashes — and if the records named someone else, it would refuse \
         instead of acting on the wrong agent.",
        Kind::MethodLimit,
    ),
    housed("| house | `fe0e80f6` |", AGENT),
    housed("| market | `0d3a24e8` |", AGENT),
    housed("| operations | `326993e9` |", AGENT),
    housed("| finance | `10807723` |", AGENT),
    housed(
        "| spender / counterparty (roles) | `3d359fe8` / `48845bf3` |",
        Carrier::Entity("Role"),
    ),
    housed(
        "| `account`, the one instance of `cash` (bounds `[0, 1000]`) | `13d1aa87` |",
        Carrier::Entity("ResourceInstance"),
    ),
    housed(
        "| the statement a payable is stated in — *spender **spend** for counterparty* | `99c01bdc` |",
        STATEMENT,
    ),
    housed(
        "What the house already knew: a receivable of `+100` settled on 2026-01-02 (`7d86cc6c`), a \
         payable of `-20` due 2026-01-10 (`4de31818`), and a payable of `-60` due 2026-01-20 \
         (`0df2ea53`).",
        COMMITMENT,
    ),
    housed(
        "Three worlds had been decided; the last two were claimed by **operations**, which forked \
         away the `-20` in favour of the `-60`. The genesis is claimed by nobody. The tip finance \
         planned from is `1cd9afbb`.",
        LINEAGE,
    ),
    // ---- What I recorded ---------------------------------------------------------------------
    housed(
        "One commitment, admitted through `ape-cli`'s journal into canonical history — `a0edbe20`:",
        JOURNAL,
    ),
    housed(
        "the house commits to spend **30** out of `account` to the market, committed 2026-01-07, \
         **due 2026-01-14**, executed by finance, depending on nothing.",
        COMMITMENT,
    ),
    housed(
        "It reuses the vocabulary the house already has (`99c01bdc`, the statement whose action is \
         `spend` with effect `Decrease` on `cash`).",
        STATEMENT,
    ),
    unhoused(
        "Nothing else was admitted: inventing a second way to say \"the house pays the market\" would \
         be a second version of the same fact.",
        Kind::RoadNotTaken,
    ),
    // ---- What I decided ----------------------------------------------------------------------
    housed(
        "One **fork** of the tip `1cd9afbb`, omitting nothing and introducing `a0edbe20`, recorded as \
         **taken by finance**. It produced world **`1f1afd4e`**: same cut (2026-01-07, event head \
         `ede70f26`), frozen `{+100}`, open `{-60, -30}`.",
        WORLDS,
    ),
    exposition(
        "A fork, and not an advance, because `advance` recognizes later knowledge and never adds an \
         intention — a commitment merely admitted to history is in nobody's plan until a decision \
         puts it there.",
    ),
    housed(
        "Once every movement lands the account holds `100 - 60 - 30 = 10`, inside the `[0, 1000]` its \
         resource declares, and feasibility under `FinalState` reports no conflict for any of the \
         four worlds.",
        FEASIBILITY,
    ),
    exposition(
        "Put back with `converge::converge`, which re-reads the repository, appends to the journal it \
         finds there, merges the decisions, rebuilds the whole thing in memory and only then writes.",
    ),
    housed("`repo/` now holds 21 admissions and 4 decisions.", JOURNAL),
    unhoused(
        "Read back two ways, neither of which sees a value this program computed: \
         `reading::reconstruct` in-process, and the separate `ape-cli` binary, which given only the \
         repository answers",
        Kind::MethodLimit,
    ),
    housed(
        "Running `cargo run` again is a no-op: it finds the invoice and the world already there and \
         writes nothing.",
        WORLDS,
    ),
    // ---- Every object I construct, and what it asserts ---------------------------------------
    exposition("| `Repository::open(\"repo\")` | Nothing. It names a directory. |"),
    exposition(
        "That the journal and lineage on disk rebuild *exactly* the worlds `worlds.json` records — \
         every coordinate of every world agrees.",
    ),
    housed(
        "That these identities are the ones the house's records label house, market, operations, \
         finance, spender, counterparty and `account`, and that `99c01bdc` is the statement whose \
         action is `spend`.",
        AGENT,
    ),
    housed(
        "Each asserts one civil day: the instant the invoice is recorded and committed at, and the \
         day it falls due — which is also the day the worlds are read at, at the end.",
        COMMITMENT,
    ),
    housed(
        "That on 2026-01-07 this was admitted as knowledge: the house spends 30 out of `account` to \
         the market by 2026-01-14, executed by finance.",
        JOURNAL,
    ),
    housed(
        "That the house answers for the payment, finance performs it, and the market benefits from \
         it.",
        COMMITMENT,
    ),
    housed(
        "That the commitment was made on 2026-01-07 and is due on 2026-01-14 — and, by construction, \
         that the first is not after the second.",
        COMMITMENT,
    ),
    housed(
        "That the movement has magnitude 30. Its *direction* is asserted elsewhere: by the action \
         `spend`, which is `Decrease`.",
        COMMITMENT,
    ),
    housed(
        "Intended reality: this obligation exists, its parties are eligible for the roles its \
         statement requires, and it moves this instance of this resource.",
        COMMITMENT,
    ),
    housed(
        "That the intention worth reasoning about is the tip's intention plus this payable, under the \
         cut the tip already recognizes — an outcome, not a transition.",
        WORLDS,
    ),
    housed(
        "That this fork was taken **by finance**, after journal entry `a0edbe20`, with exactly those \
         21 entries standing. The last two are checkable; the first is a claim (see below).",
        LINEAGE,
    ),
    housed(
        "That there is a world at cut 2026-01-07 / head `ede70f26` whose frozen past is `{+100}` and \
         whose open future is `{-60, -30}`, descending from `1cd9afbb`.",
        WORLDS,
    ),
    exposition(
        "What each decided world *is*, in the application's vocabulary — written down so a later \
         reader has a second representation to weigh against the one the decisions produce.",
    ),
    housed(
        "What each world says at 2026-01-14: each commitment's outcome and timeliness, the settled \
         level of `account`, and the conflicts under `FinalState`.",
        CONDITIONS,
    ),
    // ---- What I needed and could not find ----------------------------------------------------
    want(
        "**The journal cannot say who recorded an entry.** This is the one that bit. `Admission` has \
         no author field of any kind — only a decision (`Taken`) carries `by`.",
        IN_THE_QUEUE,
    ),
    loss(
        "So half the task, \"record what finance intends\", is *unattributable by construction*: the \
         invoice sits in `journal.json` and nothing in it says finance put it there.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "I want to be plain that this is not the same claim: `executors` asserts *who performs the \
         payment*, not *who wrote the record*. It is the nearest thing available, and a reader who \
         takes it as authorship would be reading something the engine never said.",
        Kind::Qualification,
    ),
    exposition(
        "the consequence is that a party's hand shows in knowledge only when the party happens to be \
         a participant in it.",
    ),
    unhoused(
        "**Accountable vs. executor is a modelling choice the brief does not settle.** I made the \
         house accountable and finance the executor: the market invoiced *the house*, so the house is \
         who answers to the market, and finance is who takes the payment on.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "The alternative — finance accountable, on the reading that \"finance has to take that on\" \
         means finance answers for it — is equally defensible, and the two produce different \
         commitments and therefore different identities.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "The existing payables in `repo/` use `accountable: house, executors: [house]`, which does \
         not settle it either. I did not want to pick silently.",
        Kind::Qualification,
    ),
    want(
        "**Nothing records what a payment is *for*.** There is no description, reference, memo or \
         document field anywhere in the ontology. \"Storage\" and \"invoice\" are nowhere in what I \
         wrote; the record says 30, out of `account`, to the market, by the 14th.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "Carrying the *nature* of the expense would mean new vocabulary — a Resource, Action and \
         Statement per kind of expense — which is a change to the house's ontology rather than a way \
         of taking on one invoice, so I did not do it.",
        Kind::RoadNotTaken,
    ),
    // Fits none of the five: a recommendation about future modelling, handed upward. The same shape
    // `03-narrative-mismatch` produced four times — and here it is in a technical report, which is
    // what `05-the-kinds-depend-on-the-audience.md` has to answer for.
    unclassified(
        "If the house ever needs to tell a storage payable from a freight one, that is the shape the \
         answer will have to take, and it is a decision above my level here.",
    ),
    want(
        "**Nothing checks that finance was entitled to decide.** `Taken::claimed` is verified only to \
         the extent that the named agent was already known at the coordinate (`attributed`, in \
         `lineage.rs`); no role, eligibility or authority is consulted",
        Standing::Tracked("lab/candidates/00-authenticity.md"),
    ),
    want(
        "**`decided_by` cannot tell \"unclaimed\" from \"not this party's\".** The genesis names \
         nobody, and asking for finance's worlds simply does not list it. There is no way to ask \
         \"which decisions have no party\".",
        Standing::Untracked,
    ),
    want(
        "**No stored number answers \"will the account hold enough\".** `Reading::level` counts only \
         what a projection reports as *fulfilled*, so it stays `100` in every world including the one \
         that plans to spend `90`; feasibility reports conflicts, not levels.",
        Standing::Untracked,
    ),
    unhoused(
        "a reader of `repo/` alone will not find the projected balance anywhere, and the arithmetic \
         in my output is mine.",
        Kind::Qualification,
    ),
    unhoused(
        "**I had to choose the recording instant.** The brief gives no date for the invoice, only the \
         deadline. I recorded at **2026-01-07**, the most recent instant the house's knowledge already \
         holds, so nothing was invented.",
        Kind::Qualification,
    ),
    unhoused(
        "Recording on a later day would have forced an `advance` before the fork […] and an advance \
         asserts recognition of later knowledge that finance had no observation to justify.",
        Kind::RoadNotTaken,
    ),
    // The third time this want arrives, from the third agent, and it is still only in 01's result.
    want(
        "A `Constraint` cannot be read back off an admitted `Resource`, so the `[0, 1000]` in my \
         output is read from the journal record that supplied it.",
        IN_01,
    ),
    unhoused(
        "`cargo test -p ape-cli` does not compile in this copy of the crates: `cli/src/reading.rs:381` \
         refers to `crate::subject::divergence`, a module the crate does not have.",
        Kind::MethodLimit,
    ),
];
