//! Which of a record's worlds the record means, and whose meaning it is.
//!
//! The engine builds a tree of worlds and moves nothing. Synthesis says so in a list of what it does
//! not do — *moving mutable references such as `main`, **which belongs to the application*** — and
//! until experiment 18 the application had not built one. What it had was
//! [`crate::repository`]'s `current`, which names which **generation of files** a reader reads. That
//! is a pointer at the live object store, not a reference to a world, and the two were confused for
//! one in the laboratory's own charter until the confusion was measured.
//!
//! # Why it is not a pointer
//!
//! Because a reference to a content-addressed world cannot be held by anything content-addressed,
//! and every obvious home is.
//!
//! **Not the journal.** A `ThesisId` is derived and never admitted, so nothing at the admitting layer
//! can weigh one — an admission carrying a world in a free-text field is taken in silence, aimed at a
//! world that does not exist, and read back without a word. And a plan that moves and returns leaves
//! *two* addresses where three moves happened, because designating the same world twice produces the
//! same entry twice. The journal holds the three; `custody.json` and every witness hold two.
//!
//! **Not one value in a file.** A record whose plan went `W₁ → W₂ → W₁` and a record that designated
//! `W₁` and never moved are byte-identical in every file. *What was the plan before* is not a
//! question a record with one value can be asked.
//!
//! So it is a **sequence**, and it is [`crate::lineage::Taken`]'s shape — a claim about a world, at a
//! coordinate, by somebody. That is a finding rather than a convenience: the record already knew how
//! to hold exactly this kind of claim, and had only ever held it about decisions.
//!
//! ```text
//! plan     which world            checked against `worlds.json`
//! after    where in the journal   checked against the replay
//! by       whose plan             checked against the replay, and optional
//! ```
//!
//! # Ordered by position, and dated by nothing
//!
//! Two moves with no admission between them carry the **same** `after`, for the reason above. So
//! `after` orders the log against knowledge and the file's own order is the only thing that separates
//! two moves at one coordinate. A log read as a set would answer a plan that never moved.
//!
//! It carries no instant. A recording instant is the one value nothing derives, so a designation
//! dated by its writer would be a claim no receiver can weigh — which is the class experiment 17
//! closed. What this answers is *what was the plan when the record knew this much*, which is the
//! question `Taken` was already asking about decisions.
//!
//! # `by` is optional, and the optional case is not a gap
//!
//! It is [`crate::lineage::Taken::by`]'s field and its reason: an application reasoning alone has no
//! party to name. What experiment 18 added is that the unqualified row is also the only answer a
//! record has for **a reader who is no party** — and that it is the same row a claim *about the
//! house* would be, because two records founded alike are byte-identical and a record carries no name
//! it could use to tell itself from another. *The house* and *this record* are one subject with one
//! silence, so the three candidate homes were two.
//!
//! # Absent is not empty
//!
//! A repository with no `designations.json` makes no claim, which is [`crate::repository`]'s
//! tolerance for `custody.json` and exists for the same reason: records in the workspace were written
//! by parties nobody can re-run. An empty log is a different sentence — *this record's plan never
//! moved* — and it is one a write can honestly make, where an empty custody could not.
//!
//! Earned by: 18-designation (Confirmed)

use std::collections::BTreeSet;

use ape::engine::thesis::ThesisId;
use ape::kernel::entities::AgentId;
use serde::{Deserialize, Serialize};

use crate::error::DesignationError;
use crate::journal::{EntryId, Replayed};
use crate::reading::WorldRecord;

/// One claim about which world a record means, at the coordinate it was made at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Designated {
    pub plan: ThesisId,
    /// The journal entry that was the most recent one when the plan moved.
    ///
    /// [`crate::lineage::Taken::after`]'s field, and the reasoning there is this one's: it is a
    /// position in the sequence that produces knowledge, and everything else is derived from it.
    /// What it is *not* is a witness — a designation is not a decision and stands on nothing, so
    /// there is no prefix it has to answer for.
    pub after: EntryId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<AgentId>,
}

/// Weigh every claim in a log against the record it sits beside.
///
/// Three references, and each refusal names **which** one failed and what it named. A reader told
/// only that a log is bad has to go and find out how, and the record already knows.
///
/// The check is existence, and that is a property of what a designation is rather than a shortcut.
/// `custody.json` is a function of the journal, so a replay is a second derivation of one fact and
/// the two can disagree. Nothing derives a plan — the only thing the record can contradict is that
/// the world, the coordinate and the party are its own.
pub fn corroborate(
    held: &[Designated],
    worlds: &[WorldRecord],
    admitted: &Replayed,
) -> Result<(), DesignationError> {
    let named: BTreeSet<&str> = worlds.iter().map(|world| world.thesis.as_str()).collect();
    let entries: BTreeSet<&EntryId> = admitted.entries.iter().collect();
    let parties: BTreeSet<&AgentId> = admitted.agents.iter().collect();

    for (position, claim) in held.iter().enumerate() {
        if !named.contains(claim.plan.to_string().as_str()) {
            return Err(DesignationError::PlanIsNotAWorldOfThisRecord {
                position,
                plan: claim.plan.to_string(),
            });
        }
        if !entries.contains(&claim.after) {
            return Err(DesignationError::CoordinateIsNotInTheJournal {
                position,
                after: claim.after.clone(),
            });
        }
        if let Some(party) = claim.by {
            if !parties.contains(&party) {
                return Err(DesignationError::PartyWasNeverAdmitted {
                    position,
                    by: party.to_string(),
                });
            }
        }
    }

    Ok(())
}

/// What the plan was when the record held everything up to `after`, or nothing before the first.
///
/// The last claim at or before that coordinate, by the log's own order — which is the only thing
/// that separates two moves the coordinate cannot. Nothing before the first move is `None`, and that
/// is not the same answer as a world.
pub fn plan_at(
    held: &[Designated],
    after: &EntryId,
    admitted: &Replayed,
) -> Option<(ThesisId, Option<AgentId>)> {
    let asked = admitted.entries.iter().position(|entry| entry == after)?;
    let at = |claim: &Designated| {
        admitted
            .entries
            .iter()
            .position(|entry| *entry == claim.after)
    };

    held.iter()
        .filter(|claim| at(claim).is_some_and(|position| position <= asked))
        .next_back()
        .map(|claim| (claim.plan, claim.by))
}

/// Two parties' logs, as one log.
///
/// The rule is [`crate::converge`]'s, one file over: two decisions cannot contradict one another, so
/// a second party's line is a branch rather than a competing version. Two parties hold two plans and
/// neither is wrong, so nothing is arbitrated here either — the union keeps both, and keeps each
/// party's own order, which is what tells that party's two moves at one coordinate apart.
///
/// A claim already present is not added twice, because a designation's fields are its whole content
/// and two parties that agree hold the same claim. That is the journal's rule for entries arriving
/// here without anything having arranged it.
///
/// # The limit, and it is narrow
///
/// Two **unqualified** claims from two parties at one coordinate cannot be ordered by anything either
/// record carries: neither has a party to be read under, and the coordinate is the same. This puts
/// the arrived one first, so the merge's arrival order survives into the result — which is the thing
/// [`crate::converge`] refuses to let happen to decisions.
///
/// It reaches no decision, no world and no number. It reaches which unattributed plan a reader sees
/// last, in a record where two parties both declined to say whose plan it was. Measured rather than
/// discovered later, and on the laboratory's queue.
pub fn merge(arrived: &[Designated], held: &[Designated]) -> Vec<Designated> {
    let mut merged = arrived.to_vec();

    merged.extend(
        held.iter()
            .filter(|claim| !arrived.contains(claim))
            .cloned(),
    );

    merged
}
