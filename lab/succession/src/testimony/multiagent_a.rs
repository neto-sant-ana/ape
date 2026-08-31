//! `04-multiagent/run-a` — operations, the first testimony written against a repository.
//!
//! The agent was asked to stand a courier slot down and put an inventory purchase in its place. It is
//! a technical report to an operator, like `01` and `02` and unlike `03`.
//!
//! # It is the testimony whose requests actually reached the queue
//!
//! Five of its wants and losses are `Tracked`, and four of those five are one sentence in
//! `QUEUE.md`'s re-run debt: *no way to say what a commitment is for; no author on an admission; no
//! on-behalf-of between agents; and `Corroborated` cannot extend what was read.* Against
//! `01-single-agent`, whose two frictions are in its own result and in no queue at all.
//!
//! **So the row queues some experiments' findings and not others', and nothing says which** — which
//! sharpens `04-a-want-has-a-standing.md` rather than softening it.
//!
//! # And it produced the first unclassified claim outside `03`
//!
//! One, and it is a bare **motivation**: *because the purchase takes the slot's place in the same
//! relationship*. No alternative named, so it is not a road not taken; nothing asked for, so it is not
//! a want. It is the reason a thing was built the way it was built, and the five kinds have no room
//! for it. See `06-motivation-is-not-one-of-the-five.md`.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn want(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Want)),
        standing: Some(standing),
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn unclassified(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Unhoused(None),
        standing: None,
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentA,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const AGENT: Carrier = Carrier::Entity("Agent");
const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const EVENT: Carrier = Carrier::Entity("Event");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const CONDITIONS: Carrier = Carrier::Derived(Derived::Conditions);

/// `QUEUE.md`'s re-run debt, which names four of this run's requests in one sentence.
const IN_THE_QUEUE: Standing = Standing::Tracked("lab/QUEUE.md");

pub const CLAIMS: &[Claim] = &[
    housed(
        "the world on file already stands the slot down, and forking it again would produce a world \
         that changes nothing",
        WORLDS,
    ),
    unhoused(
        "Run against the original `repo/`, the program reproduces the current three files **byte for \
         byte** — checked by keeping a copy and `diff -r`, which is the only claim of determinism made \
         here that was actually measured.",
        Kind::MethodLimit,
    ),
    // ---- Who is who, read out of repo/ -------------------------------------------------------
    unhoused(
        "Nothing below was assumed; every label was resolved from the journal by replaying it.",
        Kind::MethodLimit,
    ),
    housed("| house | `fe0e80f6…` | spender, from 2026-01-01 |", AGENT),
    housed(
        "| market | `0d3a24e8…` | counterparty, from 2026-01-01 |",
        AGENT,
    ),
    housed(
        "| **operations** | `326993e9…` | spender, from 2026-01-03 |",
        AGENT,
    ),
    housed(
        "| finance | `10807723…` | spender, from 2026-01-03 |",
        AGENT,
    ),
    housed(
        "One resource, `cash`, bounded to `[0, 1000]`, with one instance, `account`.",
        Carrier::Entity("Resource"),
    ),
    housed(
        "Two actions on it, `receive` (increase) and `spend` (decrease).",
        Carrier::Entity("Action"),
    ),
    housed(
        "Two statements: *counterparty receives for spender* and *spender spends for counterparty*, \
         each fulfilled by `Settled` and cancelled by `Cancelled`.",
        Carrier::Entity("Statement"),
    ),
    housed(
        "`7d86cc6c…` — market receives 100 into the account for the house, due 2026-01-02 — is \
         settled by the one Event in the chain (`ede70f26…`, `Settled`, 2026-01-02).",
        EVENT,
    ),
    housed(
        "`4de31818…` — **house spends 20 of the account to the market, due 2026-01-10** — is the \
         courier slot, and it is the single open commitment of the single world on file (`bbee1243…`, \
         a genesis cut at 2026-01-06).",
        COMMITMENT,
    ),
    unhoused(
        "That is how the slot was identified: an open commitment of the world operations holds, whose \
         magnitude is 20 and whose due date is the 10th. Two matches would have made the program \
         refuse rather than guess.",
        Kind::MethodLimit,
    ),
    housed(
        "The genesis decision on file claims **no party** (`by` absent). Both decisions added here \
         claim operations.",
        LINEAGE,
    ),
    // ---- What I recorded ---------------------------------------------------------------------
    housed(
        "**One admission, appended to the journal as entry 20** — `0df2ea53…`, admitted through \
         `Canon::admit_commitment` by way of `ape_cli::journal::replay_remaining`, recorded at \
         2026-01-07",
        JOURNAL,
    ),
    housed(
        "House is accountable and executes; the market benefits; under the *spender spends for \
         counterparty* statement; against the `account` instance of `cash`; committed 2026-01-07, due \
         2026-01-20; magnitude 60; no dependencies.",
        COMMITMENT,
    ),
    housed(
        "It is built **from the slot it replaces** — same accountable party, same executors, same \
         beneficiaries, same statement, same instance",
        COMMITMENT,
    ),
    // The reason, and it is the whole of H4 in half a sentence: no alternative is named, so it is not
    // a road not taken; nothing is asked for, so it is not a want. It is why the thing was built the
    // way it was built, and the record has nowhere to put it.
    unclassified(
        "because the purchase takes the slot's place in the same relationship, and only its size and \
         deadline differ.",
    ),
    // ---- What I decided ----------------------------------------------------------------------
    housed(
        "Two decisions, both attributed to operations (`Taken::claimed`), both taken after journal \
         entry 20 and witnessing all 20 entries",
        LINEAGE,
    ),
    housed(
        "**advance** `bbee1243…` to `known_at` 2026-01-07 → world `ddef2011…`. History imposed \
         nothing (0 commitments); the chain head is unchanged.",
        WORLDS,
    ),
    exposition(
        "This decision exists because a fork inherits its parent's cut, and the purchase was recorded \
         after 2026-01-06 — selecting it under the old cut is refused as `CommitmentNotKnownAtCut`.",
    ),
    exposition(
        "Recognizing a later day is not intending anything, which is why the engine keeps it a \
         separate step.",
    ),
    housed(
        "**fork** `ddef2011…`, omitting `4de31818…` (the courier slot) and introducing `0df2ea53…` \
         (the purchase) → world **`1cd9afbb…`**, whose frozen past is the settled 100 and whose open \
         future is the purchase alone.",
        WORLDS,
    ),
    exposition(
        "Written back with `ape_cli::converge::converge`, which merges into the repository as it \
         stands rather than overwriting what it read, and rebuilds everything in memory before writing \
         a byte.",
    ),
    housed(
        "`repo/` now holds a 20-entry journal, 3 decisions and 3 worlds; `converge::holds` confirms \
         `1cd9afbb…` is there, and the house's own binary agrees without my program",
        JOURNAL,
    ),
    // ---- For every object I construct, what it asserts ---------------------------------------
    housed(
        "the house owes the market a spend of 60 from the account, committed on 2026-01-07 and due on \
         2026-01-20, depending on nothing.",
        COMMITMENT,
    ),
    housed(
        "operations now reasons about the same intention under everything knowable on 2026-01-07.",
        WORLDS,
    ),
    housed(
        "operations no longer intends the courier slot and does intend the purchase, at the cut it \
         already recognizes.",
        WORLDS,
    ),
    housed(
        "operations took that advancement, at the point in the journal where 20 entries stood.",
        LINEAGE,
    ),
    housed(
        "operations took that substitution, at the same point.",
        LINEAGE,
    ),
    exposition("the records the house keeps live in `repo/`."),
    exposition(
        "this is one party's working copy of the repository: the knowledge it admitted, the worlds \
         its decisions produced, and how far into the journal it read.",
    ),
    exposition(
        "`WorldRecord` asserts that a world with these coordinates was produced by these decisions, \
         and `Reading` asserts what one world says about each commitment at one instant.",
    ),
    // ---- What I did not do, and why ----------------------------------------------------------
    unhoused(
        "**No Event.** Standing the slot down is not something that happened; it is something \
         operations decided. The obvious alternative was to admit an Event with observation \
         `Cancelled` against `4de31818…`, which the statement's settlement vocabulary allows. I did \
         not, for two reasons",
        Kind::RoadNotTaken,
    ),
    exposition(
        "a settlement freezes the commitment in **every** world derived from that point on, including \
         the market's, so one party would have closed a bilateral arrangement by writing into shared \
         history.",
    ),
    exposition(
        "A fork keeps the withdrawal where it belongs — in operations' intention — and leaves the \
         fact alone.",
    ),
    housed(
        "**No new statement, action, resource or role.** Everything the purchase needs already \
         existed.",
        JOURNAL,
    ),
    // ---- What I needed and could not find ----------------------------------------------------
    want(
        "**Nothing says what a commitment is *for*.** In the record, \"a courier slot costing 20\" \
         and \"an inventory purchase of 60\" are the same object with different numbers: same \
         statement, same action, same instance, same parties.",
        IN_THE_QUEUE,
    ),
    want(
        "There is no label, tag or memo field on a commitment; the vocabulary is `Role`, `Agent`, \
         `Resource`, `ResourceInstance`, `Action`, `Statement`, and only the first four carry an \
         `Identifier`.",
        IN_THE_QUEUE,
    ),
    loss(
        "If two courier slots of 20 due on the 10th ever coexist, no reader of `repo/` can tell which \
         one was stood down.",
        IN_THE_QUEUE,
    ),
    // A limit of the world this run was given, not of the record's format — and nowhere in the
    // laboratory, because no result named it.
    loss(
        "The goods themselves have no resource, so nothing in the record says inventory arrives.",
        Standing::Untracked,
    ),
    housed(
        "after this run the commitment `4de31818…` is still in canonical history, still unsettled, and \
         still selected by the two worlds that preceded the fork — read at 2026-01-20, both of them \
         report it `Breached`.",
        CONDITIONS,
    ),
    want(
        "There is no notification, acknowledgement or bilateral-agreement primitive in either crate; \
         the reconciliation of two parties' worlds is what the Synthesis layer is for, and it is asked \
         *about* worlds by someone outside them.",
        IN_THE_QUEUE,
    ),
    housed(
        "So \"the slot is stood down\" is true of operations' intention and of nothing else.",
        WORLDS,
    ),
    unhoused(
        "**The advance is attributed to operations, and attribution is a claim.** `Taken::claimed` is \
         checked for exactly two things — that the identity names an agent, and that the agent was \
         already known when the decision was taken. That the party named is the party that decided is \
         witnessed by nothing but the writer",
        Kind::Qualification,
    ),
    // Tracked as an unacted request, and ALSO ruled on by the ontology — "a scale and a delegation
    // are both of this kind. Each is expressible above the engine." Tracked wins because it is the
    // actionable one, and the ruling is why acting on it would be an application's job.
    want(
        "There is no on-behalf-of relation between agents; the lineage attribution is the only place \
         the distinction survives, and it says who decided, not who was represented.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "**2026-01-07 is my choice.** The brief gives no \"today\". […] Any date from 2026-01-06 \
         through 2026-01-20 would have worked.",
        Kind::Qualification,
    ),
    want(
        "A party that has *extended* what it read has to rebuild the struct field by field […] there \
         is no `extend`/`with_decision` on it. Not a gap in semantics, just a seam worth naming.",
        IN_THE_QUEUE,
    ),
];
