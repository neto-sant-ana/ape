//! Putting back what a party decided, without losing what arrived while it was thinking.
//!
//! Phase 1 measured a lost decision, and measured that no comparison inside a repository can find
//! one: corroboration weighs two representations of a fact against each other, and a write that
//! landed before both moved both. So the repair belongs here, at the moment of writing, where a
//! party can still find out that what it read is not what is there.
//!
//! # The two files get different repairs, and the difference is not a preference
//!
//! **Knowledge appends.** A party may add to the journal it read and may not have added to a
//! different one, which is the Canon's compare-and-append one layer up — [`Diverged`] is shaped
//! after `UnexpectedHead` deliberately. What forces it is not that knowledge is precious: it is
//! that every decision carries the set of entries that stood when it was taken, so a journal
//! whose earlier entries moved makes standing decisions disagree with it. Append-only is what the
//! sequence witness already requires; this only refuses it earlier, and by name.
//!
//! **Intention merges.** Two decisions cannot contradict one another. A lineage is a tree, so a
//! second party's line is a branch rather than a competing version, and the union of two parties'
//! decisions is a lineage in the same sense either party's was. Nothing is arbitrated because
//! there is nothing to arbitrate.
//!
//! # Order must not survive into the result
//!
//! A merge that appended each party's decisions in arrival order would remove the loss and leave
//! the order in the repository, which is a lock wearing a merge's clothes. So the merged sequence
//! is ordered by what the decisions themselves carry — where in the journal each was taken, then
//! the decision's own content — and ancestry is respected by emitting a decision only once the
//! world it extends has been.
//!
//! That order is a *linearization* of something that is not a line, and it is chosen rather than
//! observed. Two decisions on different branches have no order of their own; the repository needs
//! one because a file is a sequence.
//!
//! # A party that cannot converge writes nothing
//!
//! The merged repository is rebuilt in memory before any of it is written. A merge that does not
//! reconstruct is refused, and a refusal leaves the repository exactly as it was — which is the
//! other half of what the Canon promises a writer who lost.
//!
//! Earned by: 05-coordination (Confirmed)

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::engine::thesis::{ThesisId, ThesisLookup};

use crate::error::ConvergeError;
use crate::history::ResidentHistory;
use crate::journal::{Admission, EntryId};
use crate::lineage::{self, Taken};
use crate::reading::{Corroborated, WorldRecord};
use crate::repository::Repository;

/// Put back what a party holds, keeping whatever arrived while it was thinking.
///
/// What comes back is the party's working copy as the repository now stands, so a party that goes
/// on deciding does so against everything that is there rather than against what it last read.
pub fn converge(
    repository: &Repository,
    held: &Corroborated,
) -> Result<Corroborated, ConvergeError> {
    let arrived = crate::reading::corroborated(repository)?;

    let journal = appended(&arrived, held)?;
    let decisions = ordered(&journal.entries, union(&arrived, held))?;

    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, admitted) = lineage::rebuild(&mut canon, &journal.records, &decisions)?;

    repository.write_journal(&journal.records)?;
    repository.write_lineage(&decisions)?;
    repository.write_worlds(
        &lineage
            .decided()
            .iter()
            .map(WorldRecord::of)
            .collect::<Vec<_>>(),
    )?;

    Ok(Corroborated {
        canon,
        lineage,
        admitted,
        journal: journal.records,
        decisions,
    })
}

/// A journal and the addresses it produced, which are needed together and derived apart.
struct Sequence {
    records: Vec<Admission>,
    entries: Vec<EntryId>,
}

/// Whichever of the two journals extends the other, refusing two that diverge.
///
/// Compared by address rather than by record. An [`EntryId`] is derived from what admitting
/// produced, so two entries that agree are the same knowledge however either side spelled it.
fn appended(arrived: &Corroborated, held: &Corroborated) -> Result<Sequence, ConvergeError> {
    let (there, here) = (&arrived.admitted.entries, &held.admitted.entries);

    for (position, (found, expected)) in there.iter().zip(here).enumerate() {
        if found != expected {
            return Err(ConvergeError::Diverged {
                position,
                expected: expected.clone(),
                found: found.clone(),
            });
        }
    }

    let longer = if here.len() >= there.len() {
        held
    } else {
        arrived
    };

    Ok(Sequence {
        records: longer.journal.clone(),
        entries: longer.admitted.entries.clone(),
    })
}

/// Every decision either side holds, each with the world it produced, and each held once.
///
/// The identities come from the lineages the two sides rebuilt rather than from either side's
/// `worlds.json`. A derived value is written down so that something can compare it, and using one
/// as an input is how it stops being comparable.
fn union(arrived: &Corroborated, held: &Corroborated) -> Vec<(Taken, ThesisId)> {
    let mut merged: Vec<(Taken, ThesisId)> = Vec::new();

    for side in [arrived, held] {
        for (taken, world) in side.decisions.iter().zip(side.lineage.decided()) {
            if !merged.iter().any(|(present, _)| present == taken) {
                merged.push((taken.clone(), world.id()));
            }
        }
    }

    merged
}

/// Linearize the merged decisions: canonically, and with every parent before its children.
///
/// The sort settles what the decisions themselves say — where in the journal each was taken,
/// then the decision's own content — and it is total over distinct decisions, so no input order
/// can show through. The walk then takes the earliest one whose world exists yet, which is the
/// only constraint a lineage actually imposes.
fn ordered(
    entries: &[EntryId],
    mut pending: Vec<(Taken, ThesisId)>,
) -> Result<Vec<Taken>, ConvergeError> {
    let at = |entry: &EntryId| entries.iter().position(|held| held == entry);

    pending.sort_by(|(one, _), (other, _)| (at(&one.after), one).cmp(&(at(&other.after), other)));

    let mut decided: BTreeSet<ThesisId> = BTreeSet::new();
    let mut order = Vec::new();

    while !pending.is_empty() {
        let position = pending
            .iter()
            .position(|(taken, _)| {
                taken
                    .decision
                    .extends()
                    .is_none_or(|parent| decided.contains(&parent))
            })
            .ok_or(ConvergeError::NothingApplies {
                remaining: pending.len(),
            })?;

        let (taken, world) = pending.remove(position);

        decided.insert(world);
        order.push(taken);
    }

    Ok(order)
}

/// Whether a repository already holds a world, by identity.
///
/// A party asks this to find out whether what it decided survived, which is the question Phase 1
/// had no way to answer.
pub fn holds(repository: &Repository, world: ThesisId) -> Result<bool, ConvergeError> {
    Ok(crate::reading::corroborated(repository)?
        .lineage
        .archive()
        .thesis(world)
        .is_some())
}
