//! `03-narrative-mismatch/run-b` — the only testimony addressed to a person.
//!
//! In-memory. The agent was asked to explain to the house's principal why the account is heading
//! below its floor. It contains **no APE vocabulary at all**: no Thesis, no cut, no commitment, no
//! hypothesis. Three feasibility hypotheses become *"assuming everything simply settles eventually,
//! assuming each promise is paid on the day it falls due, and assuming the worst ordering of anything
//! falling on the same day"*.
//!
//! # This is the testimony that refutes P2, and the reason is who it is talking to
//!
//! Seven of its claims fit none of the five kinds — against zero across the first two — and they are
//! not scattered. They fall into four shapes, and every one of them is a thing a person asks of
//! another person and never of a record:
//!
//! ```text
//! accountability    "That is the failure, and it is mine, not the counterparty's"
//! recommendation    "Going forward, a decision should join the plan on the day it is made"
//! own reasoning     "The first time the answer changed my mind in time"
//! evaluation        "the system doing what it was built to do"
//! ```
//!
//! The five kinds were derived from `05-reconciliation`, which is a technical report to an operator.
//! **A testimony addressed to a principal produces different kinds**, and that is `04-a-want…`'s
//! finding pointed at H5 rather than H4. See `05-the-kinds-depend-on-the-audience.md`.
//!
//! # And one claim it asserts flatly that the audit qualified
//!
//! `02-hindsight` wrote *inferred, not read* against the date of the breach, because
//! `Conflict::OutOfBounds` carries the level and not the day. This testimony opens with *"the day it
//! happens is 20 January"*, unqualified. Same underived claim, same boundary, one hedge and one
//! assertion — which is a fact about the prose and not about the record.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::NarrativeMismatch,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::NarrativeMismatch,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::NarrativeMismatch,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

/// Fits none of the five kinds. The protocol requires this outcome to stay reachable; this testimony
/// is where it became reachable in fact.
const fn unclassified(text: &'static str) -> Claim {
    Claim {
        run: Run::NarrativeMismatch,
        text,
        verdict: Verdict::Unhoused(None),
        standing: None,
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::NarrativeMismatch,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const EVENT: Carrier = Carrier::Entity("Event");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);
const CONDITIONS: Carrier = Carrier::Derived(Derived::Conditions);
const CUT: Carrier = Carrier::Derived(Derived::Cut);

/// H4 itself: the charter is where *a home for what reaches no primitive* is written down.
const IS_H4: Standing = Standing::Tracked("lab/CHARTER.md");

pub const CLAIMS: &[Claim] = &[
    // ---- opening -----------------------------------------------------------------------------
    housed(
        "It is, by 20, and the day it happens is 20 January.",
        FEASIBILITY,
    ),
    housed(
        "Nothing has gone wrong yet — no money has left the account, nothing is late, and every one \
         of the four ways out is still open.",
        CONDITIONS,
    ),
    // ---- Where the account actually stands ---------------------------------------------------
    housed(
        "There is 100 in the account. That is not an estimate: it is the one payment that has \
         actually happened — money received and confirmed settled on 2 January.",
        CONDITIONS,
    ),
    housed("Nothing has been paid out since.", CONDITIONS),
    housed(
        "Against that 100 stand two promises the house has made, both still outstanding:",
        COMMITMENT,
    ),
    housed(
        "That is 120 promised against 100 held. On 14 January the 30 goes out and the account sits \
         at 70 — comfortable. On 20 January the 90 goes out and the account is at **-20**.",
        FEASIBILITY,
    ),
    housed(
        "That is the breach, and it is the only one: the 30 is not the problem, and no amount of \
         reordering makes it one.",
        FEASIBILITY,
    ),
    housed(
        "I checked that conclusion three different ways — assuming everything simply settles \
         eventually, assuming each promise is paid on the day it falls due, and assuming the worst \
         ordering of anything falling on the same day. All three give the same number, -20.",
        FEASIBILITY,
    ),
    housed(
        "There is no arrangement of these two payments that fits inside 100.",
        FEASIBILITY,
    ),
    // ---- How it got here, day by day ---------------------------------------------------------
    housed(
        "100 arrives and is confirmed. This is the only money the account has ever actually held.",
        EVENT,
    ),
    housed(
        "The house intends to spend 120, due 8 January. I put that intention into the plan and asked \
         whether the plan held. It did not: 100 in, 120 out, the account ends at -20.",
        FEASIBILITY,
    ),
    housed(
        "The intention was **withdrawn the same day it was made**, before a single unit of money \
         moved.",
        EVENT,
    ),
    exposition(
        "The check is not something run at the end of the month over what already happened; it runs \
         against the whole plan the moment an intention enters it.",
    ),
    housed(
        "It caught a 120 that would have overdrawn the account by 20 and it caught it on the day, not \
         on the due date two days later.",
        FEASIBILITY,
    ),
    housed(
        "It was not deleted. It is still on the record, marked cancelled, which is precisely why I \
         can show you this near-miss instead of asking you to take my word that the account has been \
         watched.",
        EVENT,
    ),
    exposition(
        "Nothing in this account has ever been erased; every version of the plan is still there, each \
         one naming the one it came from, so this story cannot be quietly rewritten after the fact — \
         including by me.",
    ),
    housed(
        "With the 120 gone, the house intends to spend 30, due 14 January. I put it into the plan and \
         asked again: 100 in, 30 out, 70 left. It held.",
        FEASIBILITY,
    ),
    housed("The house intends to spend 90, due 20 January.", COMMITMENT),
    housed(
        "The plan is brought forward to the 12th and the 90 is added to it. The answer comes back \
         immediately: **-20 on 20 January**.",
        WORLDS,
    ),
    // ---- Why the 90 is the mistake, and why it is mine ---------------------------------------
    unhoused(
        "The 30 was fine — it left 70. The 90 would also have been fine on its own — it would have \
         left 10.",
        Kind::RoadNotTaken,
    ),
    housed(
        "What does not fit is the two of them together against a single 100 that has to cover both.",
        FEASIBILITY,
    ),
    exposition(
        "The check is not fooled by that, because it always asks about the whole plan rather than \
         about the newest item, which is why it fired the moment the 90 joined the other two.",
    ),
    exposition("But firing is not preventing."),
    housed(
        "I committed the house to the 90 while the 30 was outstanding and before any money had been \
         secured to cover it.",
        WORLDS,
    ),
    // The absence IS readable: no such commitment is in the journal. What the record cannot carry is
    // the expectation as an expectation, which is the next claim.
    housed(
        "The account has a way to record money expected in; it was available on the day and it was \
         not used.",
        JOURNAL,
    ),
    loss(
        "If that 90 was taken on in the expectation of something coming in, **that expectation was \
         never written down** — and something not written down cannot be counted.",
        IS_H4,
    ),
    unclassified(
        "That is the failure, and it is mine, not the counterparty's and not the market's.",
    ),
    housed(
        "Every change to this plan was a decision by the house. Nothing was forced on it from \
         outside: no counterparty defaulted, nothing arrived late, no fact showed up that anyone had \
         to absorb.",
        JOURNAL,
    ),
    exposition(
        "the system keeps the two apart on purpose — changing what the house intends and taking in \
         what the world has done are separate acts that leave separate marks.",
    ),
    housed(
        "Across this entire history, nothing ever entered the plan that the house had not chosen to \
         put there, and the only two facts ever recorded against it are the 100 arriving and the \
         house's own withdrawal of the 120.",
        JOURNAL,
    ),
    housed(
        "It is two decisions, made three days apart, that were each defensible and are not defensible \
         together.",
        FEASIBILITY,
    ),
    housed(
        "The 90 was decided on the 9th, but the plan on the table at that moment was dated the 6th, \
         and a plan dated the 6th cannot speak about a decision made on the 9th. It had to be brought \
         forward to the 12th before the 90 could be added to it",
        CUT,
    ),
    unhoused(
        "Had I brought the plan forward on the 9th, when the decision was made, you would have had \
         this conversation three days earlier with three more days to act.",
        Kind::RoadNotTaken,
    ),
    loss(
        "Nothing was hidden in that window, but nothing was asked in it either.",
        Standing::ByDesign("lab/agents/02-hindsight/99-result.md"),
    ),
    unclassified("Going forward, a decision should join the plan on the day it is made."),
    // ---- What is still true in our favour ----------------------------------------------------
    housed("None of this has cost anything yet", CONDITIONS),
    unclassified("and that is not luck either — it is the direct result of the discipline above."),
    housed(
        "**No money has moved.** The 100 is intact. Both outstanding promises are unpaid.",
        CONDITIONS,
    ),
    housed(
        "**Nothing is late.** As of the 12th, both promises are inside their deadlines.",
        CONDITIONS,
    ),
    housed(
        "**The 30 is not at risk.** Paying it on the 14th leaves 70, which covers nothing that comes \
         after but breaches nothing either.",
        FEASIBILITY,
    ),
    housed(
        "**We have until the 20th** — over a week — and the problem is a single promise, not a \
         pattern.",
        CONDITIONS,
    ),
    // ---- The four ways out -------------------------------------------------------------------
    unhoused(
        "I tested each of these against the plan the same way everything else here was tested. All \
         four clear the floor under all three ways of asking:",
        Kind::MethodLimit,
    ),
    unhoused(
        "**Withdraw the 90.** The account ends at 70. Cleanest, and costs the house whatever the 90 \
         was for.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "**Withdraw the 30 instead.** The account ends at 10. It clears, but it leaves almost no \
         margin, and I would not recommend trading a comfortable promise for a tight one.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "**Secure 20 coming in, by 19 January.** Both promises are kept and the account lands at \
         exactly 0. Note the word *exactly*: 20 is the minimum that works, so anything that slips \
         leaves us where we are now.",
        Kind::RoadNotTaken,
    ),
    unclassified(
        "If we go this way I would want more than 20 and I would want it recorded the day it is \
         agreed, not the day it arrives.",
    ),
    unhoused(
        "**Reduce the 90 to 70.** Both counterparties keep something and the account lands at 0. \
         This one needs the other side's agreement, which the first two do not.",
        Kind::RoadNotTaken,
    ),
    loss(
        "Which of these is right is not a judgment I should make on your behalf — it depends on what \
         the 90 was for and on what the house's word is worth to that counterparty.",
        IS_H4,
    ),
    housed(
        "the process caught this before it cost anything, twice — once on the 6th, when it stopped a \
         120 that would have overdrawn the account, and once on the 12th, when it caught the \
         30-and-90 that I had put there.",
        FEASIBILITY,
    ),
    unclassified("The first time the answer changed my mind in time."),
    unclassified("That the shortfall exists at all is my error."),
    unclassified(
        "That you are hearing about it now, with the account still whole and every option still open, \
         is the system doing what it was built to do.",
    ),
];
