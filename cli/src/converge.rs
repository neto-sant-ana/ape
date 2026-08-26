//! Putting back what a party decided, without losing what arrived while it was thinking.
//!
//! Phase 1 measured a lost decision, and measured that no comparison inside a repository can find
//! one: corroboration weighs two representations of a fact against each other, and a write that
//! landed before both moved both. So the repair belongs here, at the moment of writing, where a
//! party can still find out that what it read is not what is there.
//!
//! *At the moment of writing* is load-bearing and was never measured until the contention experiment
//! did: what the comparison weighs is the repository **as it now stands** against what the party
//! holds, and never against what the party read, which this does not keep. A call cannot interleave
//! with another one without a thread, so the re-read below is always after any earlier writer's write
//! — and all six orderings of two parties reading and putting back through here end with both parties'
//! lines present, the single refusal landing where both read before either wrote. A stale reading
//! costs a party an attempt and nobody a decision.
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
//! **Earlier and by name is the whole of what it buys**, which is less than this note used to imply
//! and was measured by mutating it. With the comparison never firing, a divergent party's write does
//! not go through: the merge fails further down, at *the journal holds no entry …*, because the merged
//! journal is one party's and the other's decision addresses an entry that is not in it. What keeps
//! the repository intact is rebuilding before writing, below. What this supplies is a refusal at the
//! write, naming the position where the two journals disagree and both entries that disagree there,
//! instead of an address a reader would have to go and look for.
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
//! **And neither does a party that stops.** That sentence was a promise without a mechanism until the
//! atomicity experiment went looking: the three writes this function ended with were the one place in
//! the application where a process could stop and leave a repository nobody wrote — and one of the
//! states it could leave reconstructs, so nothing on the reader's side would have said so. The write
//! is now whole, and the repository this one replaces is still on disk.
//!
//! Earned by: 05-coordination (Confirmed), 07-atomicity (Confirmed), 08-contention (Confirmed),
//! 09-collision (Confirmed), 11-veracity (Confirmed)

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::engine::thesis::{ThesisId, ThesisLookup};

use crate::error::ConvergeError;
use crate::history::ResidentHistory;
use crate::journal::{Admission, EntryId};
use crate::lineage::{self, Taken};
use crate::reading::{Corroborated, WorldRecord};
use crate::repository::{Repository, RepositoryInput};

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

    let worlds = lineage
        .decided()
        .iter()
        .map(WorldRecord::of)
        .collect::<Vec<_>>();

    repository.write_whole(RepositoryInput {
        journal: &journal.records,
        lineage: &decisions,
        worlds: &worlds,
    })?;

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
/// Two comparisons over the shared prefix, and the second is not a refinement of the first.
///
/// **By address**, which settles the knowledge. An [`EntryId`] is derived from what admitting
/// produced, so two entries that agree are the same knowledge however either side spelled it.
///
/// **And by recording instant**, which the address cannot settle, because no identity contains one.
/// Two parties that learn the same fact on different days write journals that agree entry for entry
/// and are not the same journal: the instant is what a cut resolves its Event head against, so
/// keeping the converging party's would re-derive a world the other party already decided — and
/// nothing downstream would notice, because the witness is a set of addresses and a merge writes
/// its own `worlds.json`. Measured in `lab/frontier/docs/11-veracity`, where a merged record
/// answered a settled level of 0 for a world its decider had settled at 120.
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

        let (theirs, ours) = (
            arrived.journal[position].recorded_at(),
            held.journal[position].recorded_at(),
        );

        if theirs != ours {
            return Err(ConvergeError::RecordedDifferently {
                position,
                entry: found.clone(),
                held: ours.to_owned(),
                arrived: theirs.to_owned(),
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
///
/// # It answers a second question it was not built for
///
/// Handed the identity of a world **another repository** decided, this is the whole of what two
/// repositories agree about — symmetric, read-only, and needing neither of them to be told. The
/// collision experiment measured it: two records founded independently, sharing no operation and no
/// copy, agree about a world exactly where they agreed about the knowledge under it, and this is how a
/// caller finds out. It costs a read and changes nothing.
///
/// Which is worth knowing here because [`converge`] **refuses** those same two repositories, at the
/// first entry their journals do not share. The record can say what two of them have in common and has
/// no operation that takes the agreement as its subject — so this function is currently the only thing
/// in the application that a second repository is answerable to.
pub fn holds(repository: &Repository, world: ThesisId) -> Result<bool, ConvergeError> {
    Ok(crate::reading::corroborated(repository)?
        .lineage
        .archive()
        .thesis(world)
        .is_some())
}
