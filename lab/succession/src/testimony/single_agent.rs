//! `01-single-agent/run-01` — the first testimony, and the only one whose boundary no longer exists.
//!
//! In-memory: there are no four files, so a housed claim names the equivalent — the admitted
//! knowledge, the decided worlds, and what the engine computes from them. The run's task was one
//! decision the house wanted and could not carry out, and a fallback it could.
//!
//! # What reading it changed about the method
//!
//! This is the testimony that produced [`Verdict::Exposition`]. Thirteen of its claims are the agent
//! explaining what the engine is — *"a Commitment is knowledge, a Thesis is which knowledge you
//! mean"* — and asking whether a record houses that is the wrong question. The reasoning is in
//! `classification.rs` and the cost is in `02-the-third-verdict.md`.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::SingleAgent,
        text,
        verdict: Verdict::Housed(carrier),
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::SingleAgent,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::SingleAgent,
        text,
        verdict: Verdict::Exposition,
    }
}

const ENTITY_COMMITMENT: Carrier = Carrier::Entity("Commitment");
const ENTITY_EVENT: Carrier = Carrier::Entity("Event");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);

pub const CLAIMS: &[Claim] = &[
    // ---- The arrangement chosen -------------------------------------------------------------
    housed(
        "The house wanted the priority slot and must undertake the standard one.",
        LINEAGE,
    ),
    unhoused(
        "the priority slot spares a penalty owed to a third party — and nothing else in the world \
         tells against it. So that is the decision the program forms first",
        Kind::RoadNotTaken,
    ),
    housed(
        "It cannot be carried out. The account holds 100, its resource is bounded by `cash >= 0`, \
         and spending 120 puts it at −20.",
        FEASIBILITY,
    ),
    housed(
        "That is not a preference to weigh against the penalty; it is a bound the world declares, \
         and every hypothesis the engine offers agrees on it.",
        FEASIBILITY,
    ),
    housed(
        "The house therefore falls back to the standard slot",
        LINEAGE,
    ),
    // Housed narrowly: no accessor offers a level, but a reader with the record can fold
    // `movement_of` over the selection. That the crate does not OFFER it is a separate claim, and it
    // is a want — see the last section.
    housed(
        "which leaves the account at 70",
        Carrier::Derived(Derived::Interpretation),
    ),
    housed("against which nothing was found", FEASIBILITY),
    unhoused(
        "The penalty has no number. \"Spares a late penalty\" is the only reason to prefer paying \
         120 over 30, and the world carries no third party, no penalty commitment and no amount. I \
         could not compare the arrangements; I could only rank them",
        Kind::Qualification,
    ),
    unhoused(
        "The comparison would have flipped on that number. […] Any penalty above 70 makes the \
         standard path *also* end below zero, and then neither arrangement is realizable as the \
         world stands",
        Kind::Qualification,
    ),
    // ---- What each constructed object asserts -----------------------------------------------
    housed(
        "| `Assignment` | The house is accountable for this spending and performs it; the market is \
         the one it benefits. |",
        ENTITY_COMMITMENT,
    ),
    housed(
        "| `Term` | This spending was committed today and is due on the slot's date. |",
        ENTITY_COMMITMENT,
    ),
    housed(
        "| `ActionValue` | The magnitude moved is 120 (resp. 30) — the direction is the Action's, \
         not this value's. |",
        ENTITY_COMMITMENT,
    ),
    housed(
        "| `Commitment` (priority) | The house intends to spend 120 from the account to the market \
         by 2026-01-08. |",
        ENTITY_COMMITMENT,
    ),
    exposition(
        "| `KnowledgeCut::at(today)` | What could be known today: every commitment recorded by \
         today, against the event chain current at it. |",
    ),
    housed(
        "| `Thesis` (genesis, *intended*) | One complete world: the settled past that cut makes \
         unavoidable, plus the priority slot as its only open intention. |",
        WORLDS,
    ),
    housed(
        "| `Commitment` (standard) | The house intends to spend 30 from the account to the market by \
         2026-01-12. |",
        ENTITY_COMMITMENT,
    ),
    housed(
        "| `Event` (cancellation) | It was observed today that the priority slot will not happen. |",
        ENTITY_EVENT,
    ),
    housed(
        "| `KnowledgeCut::at(today)`, again | The same instant against a longer chain: the head is \
         now the cancellation. |",
        Carrier::Derived(Derived::Cut),
    ),
    housed(
        "| `Advancement` → `Thesis` | The same intention under later knowledge — the world now \
         recognizes the cancellation, and nothing new was chosen in doing so. |",
        WORLDS,
    ),
    housed(
        "| `Thesis` (fork, *undertaken*) | The world the house undertakes: that frozen past, plus \
         the standard slot as its only open intention. |",
        WORLDS,
    ),
    exposition(
        "| `Interpretation` | Nothing. It is the fold of the knowledge one world recognizes — the \
         thing questions are put to, not a claim. |",
    ),
    exposition(
        "| `FeasibilityReport` | Under one named hypothesis, at one named head, these are the \
         conflicts that world carries. |",
    ),
    exposition(
        "| `ProjectedConditions` | As of today, each selected commitment stands in this condition. |",
    ),
    housed(
        "The two candidate commitments coexist in canonical history, and only one of them is \
         selected by the world the house ends up in.",
        WORLDS,
    ),
    exposition(
        "That is the engine's division and not a workaround: a Commitment is knowledge, a Thesis is \
         which knowledge you mean.",
    ),
    // ---- How one determines whether it can be carried out ------------------------------------
    exposition(
        "Not from the Commitment. Whether an intention is realizable is a property of the whole \
         graph a world selects",
    ),
    exposition(
        "the determination is: name the cut you are asking under, build the `Thesis` that selects \
         the intention at that cut, hand it to `Interpretation::of`, and ask `feasibility_under` for \
         each hypothesis.",
    ),
    exposition(
        "Three hypotheses exist and they differ in what they assume about *when* unsettled \
         commitments land, never in what is known",
    ),
    exposition(
        "Asking all three is cheap — one fold answers them all — and it is what separates \"this is \
         impossible\" from \"this is impossible only in some order\".",
    ),
    housed(
        "The priority world **cannot** be carried out. All three hypotheses report the account \
         leaving its bounds at −20, so no ordering and no deadline rescues it.",
        FEASIBILITY,
    ),
    exposition(
        "There is no `Feasible` verdict in this crate on purpose: an empty conflict list means \
         nothing was found under the hypothesis asked, not that the graph is realizable.",
    ),
    unhoused(
        "The strongest honest claim is that three hypotheses were asked of the standard slot and \
         none of them found anything.",
        Kind::Qualification,
    ),
    unhoused(
        "I checked that these verdicts are live rather than decorative, by moving the numbers and \
         watching the answers move with them […] Both mutations were reverted.",
        Kind::MethodLimit,
    ),
    exposition(
        "The money exists only because the cut recognizes the event that made it. The +100 is not a \
         stored balance; it is the movement of a fulfilled commitment",
    ),
    exposition(
        "A cut taken before that event would make every spend look out of bounds, and the report \
         would look identical.",
    ),
    exposition(
        "Cancelling is what removes an intention's movement; omitting is what removes the intention.",
    ),
    unhoused(
        "I cancelled rather than omitted because the house did form the intention, and the fact that \
         it will not happen is worth recording; the omit-in-a-fork route was available up to the \
         moment the event was admitted.",
        Kind::RoadNotTaken,
    ),
    // ---- What I could not find ---------------------------------------------------------------
    unhoused(
        "The vendored crate does not compile as delivered. `ape/Cargo.toml` inherits `version`, \
         `edition`, `license` and `repository` from a workspace root that was not vendored with it",
        Kind::MethodLimit,
    ),
    unhoused(
        "There is no way to ask \"would this be feasible?\" without asserting it first. […] there is \
         no dry run.",
        Kind::Want,
    ),
    housed(
        "history now permanently contains an intention that never could be realized",
        JOURNAL,
    ),
    unhoused(
        "An application that wanted one would have to implement `Knowledge` + `CanonicalKnowledge` \
         itself as an overlay of the real history plus the candidate […] nothing in the crate offers \
         it.",
        Kind::Want,
    ),
    unhoused(
        "The penalty is not expressible from the world as given. The third party is not an admitted \
         Agent, holds no role, and the penalty has no amount.",
        Kind::Loss,
    ),
    unhoused(
        "the amount would be invented by me, and the whole comparison turns on it, so I left it out \
         rather than fabricate the number that decides the answer.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "`Conflict::OutOfBounds` names the level but not the bound it left. […] `Constraint` exposes \
         only `check(value)` — no accessor for its bound and no `Display`.",
        Kind::Want,
    ),
    unhoused(
        "The bound in my printed prose is therefore copied by hand out of `world.rs`, which is \
         exactly the kind of second copy that goes stale.",
        Kind::MethodLimit,
    ),
    unhoused(
        "There is no \"what is the level now\". `Accumulation` answers conditions and feasibility; a \
         projected balance is not among them.",
        Kind::Want,
    ),
    unhoused(
        "It did not block me — feasibility answered the question I had — but I expected to be able \
         to print the account's level and could not, without writing the sum myself.",
        Kind::Qualification,
    ),
    unhoused(
        "`world.rs` exposes `today()` but keeps its `day()` helper private, so `src/main.rs` \
         re-derives January 2026 dates of its own. Two copies of one calendar.",
        Kind::Want,
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// This testimony produced no claim fitting none of the five kinds, which is a result rather than
    /// an omission — asserted so that *zero* is measured rather than assumed.
    ///
    /// The outcome stays reachable through the type, `Verdict::Unhoused(None)`, and not through a
    /// constructor: a helper nobody calls is dead code, and the thing that has to remain possible is
    /// the value, which this test names directly.
    #[test]
    fn this_testimony_produced_no_claim_that_fits_none_of_the_five_kinds() {
        let orphans: Vec<&str> = CLAIMS
            .iter()
            .filter(|claim| claim.verdict == Verdict::Unhoused(None))
            .map(|claim| claim.text)
            .collect();

        assert!(
            orphans.is_empty(),
            "claims fitting none of the five kinds: {orphans:#?}"
        );
    }
}
