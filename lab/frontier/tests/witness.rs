//! Experiment 10 — Witness. Phases against `lab/frontier/docs/10-witness/00-protocol.md`.
//!
//! The question: can a record say what a decision **depends on**, rather than what stood when it was
//! taken — and keep what the witness buys?
//!
//! The instrument is [`dependence`], a comparison written here rather than in the application. Every
//! phase runs it over the same repositories the record's own comparison runs over, so a difference is
//! a difference of the claim and not of the arrangement.

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::engine::thesis::Thesis;

use ape_cli::error::LineageError;
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, EntryId};
use ape_cli::lineage::{self, Taken};
use ape_cli::reading::WorldRecord;

use ape_frontier::subject::witness::{
    self, DEPENDED, DIVERGES_AT, ENTRIES, ENTRIES_WITHOUT_FILINGS, FILINGS, INTENDED, WITNESSED,
    WORLDS,
};

/// The dependence comparison: what a decision's world is a function of, derived from the record.
///
/// # Two graphs, and neither implies the other
///
/// A world is produced from its cut and its selection, so the engine reads the Event chain to the
/// head and every commitment named, closed over dependencies. That is the **semantic** reach, and it
/// is a set of commitments and Events.
///
/// But an entry needs entries. An admission resolves its references through what was admitted before
/// it, so a commitment that is reached drags its statement, its instance, its agents — and the
/// eligibility that made it admissible. That is the **replay** reach, and it is what a record has to
/// hold for the world to be produced again at all.
///
/// The closure below is the second closed over the first, because a dependence set that could not be
/// replayed would not be a dependence set. Which is where a reference walk stops being enough:
/// **nothing points at an eligibility.** It is reached by the rule that admitted the commitment —
/// this agent, at this instant — rather than by a field, so the closure has to restate a rule of the
/// engine instead of following an edge.
mod dependence {
    use std::collections::{BTreeMap, BTreeSet};

    use ape::canon::{Canon, CanonicalKnowledge};
    use ape::engine::thesis::Thesis;

    use ape_cli::history::ResidentHistory;
    use ape_cli::journal::{Admission, EntryId};

    /// One journal, indexed so that an identity can be turned back into the entry that produced it.
    ///
    /// The index is the whole of what makes this derivable rather than declared: `EntryId::of` is the
    /// application's own addressing, so every reference an admission carries is already an address.
    pub struct Record<'j> {
        journal: &'j [Admission],
        entries: Vec<EntryId>,
        at: BTreeMap<String, usize>,
        canon: Canon<ResidentHistory>,
    }

    impl<'j> Record<'j> {
        pub fn of(journal: &'j [Admission]) -> Self {
            let mut canon = Canon::new(ResidentHistory::new());
            let entries = ape_cli::journal::replay(&mut canon, journal)
                .expect("the subject's journal admits")
                .entries;

            let at = entries
                .iter()
                .enumerate()
                .map(|(position, entry)| (entry.to_string(), position))
                .collect();

            Self {
                journal,
                entries,
                at,
                canon,
            }
        }

        /// Where each of `addresses` sits in this journal, dropping any it does not hold.
        pub fn positions(&self, addresses: &BTreeSet<EntryId>) -> BTreeSet<usize> {
            addresses
                .iter()
                .filter_map(|entry| self.at.get(&entry.to_string()).copied())
                .collect()
        }

        /// The entries `thesis` is a function of, as positions in this journal.
        pub fn under(&self, thesis: &Thesis) -> BTreeSet<usize> {
            let mut reached = BTreeSet::new();
            let mut pending: Vec<usize> = Vec::new();

            for commitment in thesis.selection().resolved() {
                self.seed(&EntryId::of(commitment), &mut reached, &mut pending);
            }

            // The chain, walked to its start rather than to the cut: an Event's identity contains
            // its predecessor, so reaching one reaches every Event before it.
            let mut cursor = thesis.cut().event_head();

            while let Some(event) = cursor {
                self.seed(&EntryId::of(event), &mut reached, &mut pending);
                cursor = self
                    .canon
                    .history()
                    .canonical_event(event)
                    .and_then(|record| *record.assertion().previous_event());
            }

            while let Some(position) = pending.pop() {
                for reference in self.referenced(position) {
                    self.seed(&reference, &mut reached, &mut pending);
                }
            }

            reached
        }

        /// The same, as the addresses a record would hold.
        pub fn addresses(&self, thesis: &Thesis) -> BTreeSet<EntryId> {
            self.under(thesis)
                .into_iter()
                .map(|position| self.entries[position].clone())
                .collect()
        }

        fn seed(&self, entry: &EntryId, reached: &mut BTreeSet<usize>, pending: &mut Vec<usize>) {
            let Some(position) = self.at.get(&entry.to_string()) else {
                return;
            };

            if reached.insert(*position) {
                pending.push(*position);
            }
        }

        /// Every entry the admission at `position` needs in order to be admitted.
        fn referenced(&self, position: usize) -> Vec<EntryId> {
            match &self.journal[position] {
                Admission::Role { .. } | Admission::Agent { .. } | Admission::Resource { .. } => {
                    Vec::new()
                }

                Admission::Eligibility { agent, roles, .. } => std::iter::once(EntryId::of(*agent))
                    .chain(roles.iter().map(|role| EntryId::of(*role)))
                    .collect(),

                Admission::ResourceInstance { resource, .. }
                | Admission::Action { resource, .. } => vec![EntryId::of(*resource)],

                Admission::Statement {
                    actors,
                    recipients,
                    action,
                    ..
                } => actors
                    .iter()
                    .chain(recipients)
                    .map(|role| EntryId::of(*role))
                    .chain(std::iter::once(EntryId::of(*action)))
                    .collect(),

                Admission::Commitment {
                    accountable,
                    executors,
                    beneficiaries,
                    statement,
                    resource,
                    committed_at,
                    dependencies,
                    ..
                } => {
                    let parties = std::iter::once(accountable)
                        .chain(executors)
                        .chain(beneficiaries);

                    parties
                        .clone()
                        .map(|agent| EntryId::of(*agent))
                        .chain(std::iter::once(EntryId::of(*statement)))
                        .chain(std::iter::once(EntryId::of(*resource)))
                        .chain(dependencies.iter().map(|id| EntryId::of(*id)))
                        // The edge no field records: an agent is committed for only while an
                        // eligibility says so, and the commitment does not name which one.
                        .chain(
                            parties
                                .flat_map(|agent| self.eligibility(*agent, committed_at))
                                .collect::<Vec<_>>(),
                        )
                        .collect()
                }

                Admission::Event { commitment, .. } => vec![EntryId::of(*commitment)],
            }
        }

        /// The eligibility in effect for `agent` at `at`, found the way the engine finds it.
        ///
        /// A restatement of `Knowledge::eligibility_at`: the latest assignment effective no later
        /// than the instant. It is a **query over the journal**, which is exactly the point — there is
        /// no reference from a commitment to the eligibility that let it exist.
        fn eligibility(&self, agent: ape::kernel::entities::AgentId, at: &str) -> Vec<EntryId> {
            let mut latest: Option<(&String, usize)> = None;

            for (position, admission) in self.journal.iter().enumerate() {
                let Admission::Eligibility {
                    agent: assigned,
                    effective_from,
                    ..
                } = admission
                else {
                    continue;
                };

                if *assigned != agent || effective_from.as_str() > at {
                    continue;
                }

                if latest.is_none_or(|(known, _)| effective_from >= known) {
                    latest = Some((effective_from, position));
                }
            }

            latest
                .map(|(_, position)| vec![self.entries[position].clone()])
                .unwrap_or_default()
        }
    }
}

/// The three worlds one side decided, produced again from its files.
fn rebuilt(files: &witness::Files) -> (Canon<ResidentHistory>, Vec<Thesis>) {
    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, _) = lineage::rebuild(&mut canon, &files.journal, &files.lineage)
        .expect("a side's own files rebuild");

    let decided = lineage.decided().to_vec();

    (canon, decided)
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What the repository answers, and what each decision witnesses
// ---------------------------------------------------------------------------------------------

#[test]
fn phase_0_one_history_with_far_more_in_it_than_any_decision_is_about() {
    let arranged = witness::arranged().expect("the subject is arranged");

    assert_eq!(
        arranged.left.files.journal.len(),
        ENTRIES,
        "the side that admits the filings"
    );
    assert_eq!(
        arranged.right.files.journal.len(),
        ENTRIES_WITHOUT_FILINGS,
        "the side that does not"
    );
    assert_eq!(arranged.left.filings.len(), FILINGS.len());
    assert!(arranged.right.filings.is_empty());

    // Both sides decide the same three worlds, by identity, with nothing between them.
    assert_eq!(arranged.left.worlds.len(), WORLDS);
    assert_eq!(arranged.left.worlds, arranged.right.worlds);

    let (canon, decided) = rebuilt(&arranged.left.files);

    let intended: Vec<i128> = decided
        .iter()
        .map(|thesis| {
            witness::intended(canon.history(), thesis, arranged.left.instance)
                .expect("a world reads")
        })
        .collect();

    assert_eq!(intended, INTENDED.to_vec(), "what each world intends");

    // Every world's cut resolves to the Event, so every dependence set holds a chain.
    for thesis in &decided {
        assert!(
            thesis.cut().event_head().is_some(),
            "a world whose cut resolved to nothing would make the Event half of this unmeasurable"
        );
    }

    let witnessed: Vec<usize> = arranged
        .left
        .files
        .lineage
        .iter()
        .map(|taken| taken.witness.len())
        .collect();

    assert_eq!(
        witnessed,
        WITNESSED.to_vec(),
        "how much stood when each decision was taken"
    );

    // The two journals are out of step exactly where one admits knowledge no world selects.
    let (here, there) = (
        witness::entries(&arranged.left.files.journal).expect("the left journal replays"),
        witness::entries(&arranged.right.files.journal).expect("the right journal replays"),
    );
    let diverges = here
        .iter()
        .zip(&there)
        .position(|(mine, theirs)| mine != theirs)
        .expect("the two journals differ");

    assert_eq!(diverges + 1, DIVERGES_AT);
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — Derive what each decision depends on
// ---------------------------------------------------------------------------------------------

/// W1, first half: the dependence set is a strict subset of the prefix that stood.
#[test]
fn phase_1_a_decision_is_about_far_less_than_it_came_after() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let depended: Vec<usize> = decided
        .iter()
        .map(|thesis| record.under(thesis).len())
        .collect();

    assert_eq!(
        depended,
        DEPENDED.to_vec(),
        "how much of the record each world is a function of"
    );

    for (position, thesis) in decided.iter().enumerate() {
        let reached = record.addresses(thesis);
        let stood: BTreeSet<EntryId> = side.files.lineage[position].witness.iter().cloned().collect();

        assert!(
            reached.is_subset(&stood) && reached.len() < stood.len(),
            "a world reached {} entries of the {} that stood",
            reached.len(),
            stood.len()
        );
    }

    // The closures are nested along the ancestry, so a world's own reach is the whole of what
    // producing it needs. Measured rather than assumed: two worlds forking one parent need not have
    // agreed about anything but the parent.
    let (base, early, late) = (
        record.under(&decided[0]),
        record.under(&decided[1]),
        record.under(&decided[2]),
    );

    assert!(base.is_subset(&early) && base.is_subset(&late));
    assert!(!early.is_subset(&late) && !late.is_subset(&early));
}

/// W1, second half: the closure holds, and it uses nothing the record does not.
///
/// The strongest form of *derivable* available: keep only the entries the closure reached, replay
/// **that** journal, apply the decisions that lead to the world, and require the identity back. A
/// closure that had guessed would produce a different world or fail to admit at all.
#[test]
fn phase_1_the_closure_is_enough_to_produce_the_world_again() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    for (position, thesis) in decided.iter().enumerate() {
        let pruned = pruned(&side.files.journal, &record.under(thesis));
        let ancestry = ancestry(&side.files.lineage, &decided, position);

        assert_eq!(
            pruned.len(),
            DEPENDED[position],
            "the pruned journal is the closure and nothing else"
        );

        let mut canon = Canon::new(ResidentHistory::new());

        journal::replay(&mut canon, &pruned).expect("a closure admits");

        let produced = lineage::replay(canon.history(), &ancestry).expect("a closure decides");

        assert_eq!(
            produced.decided().last().map(Thesis::id),
            Some(thesis.id()),
            "the world came back from its own dependence alone"
        );
    }
}

/// A chain is reached whole, and dropping a link does not leave a gap — it makes another world.
///
/// An Event's identity contains its predecessor, so a cut that names a head names everything under it.
/// Which puts a floor under the narrowing that has nothing to do with what anybody selected: the
/// Events before a cut are dependence whether or not any of them settled a commitment the world holds.
///
/// And the failure mode is not the one a reader would expect. A record keeping only the head would not
/// come back missing an entry; it would come back **agreeing with itself** about a different world, and
/// the identity is the only thing that says so.
#[test]
fn phase_1_the_event_chain_is_not_prunable() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let reached = record.under(&decided[0]);
    let events: BTreeSet<usize> = reached
        .iter()
        .filter(|position| matches!(side.files.journal[**position], Admission::Event { .. }))
        .copied()
        .collect();

    assert_eq!(events.len(), 2, "the whole chain, not the head");

    let head = *events.last().expect("a chain with a head");
    let without: BTreeSet<usize> = reached
        .iter()
        .filter(|position| **position == head || !events.contains(position))
        .copied()
        .collect();

    let mut canon = Canon::new(ResidentHistory::new());

    journal::replay(&mut canon, &pruned(&side.files.journal, &without))
        .expect("a journal missing an earlier Event still admits, which is the hazard");

    let produced = lineage::replay(canon.history(), &[witness::founding_decision()])
        .expect("and still decides");

    assert_ne!(
        produced.decided().first().map(Thesis::id),
        Some(decided[0].id()),
        "a world built over a shortened chain is a different world, not a broken one"
    );
}

/// The edge no field records, measured by taking it away.
///
/// Nothing in the record points at an eligibility. It is reached by restating a rule of the engine —
/// this agent, at this instant — and a closure that followed references only would drop it and look
/// perfectly well formed. What it would produce is a journal that no longer admits.
#[test]
fn phase_1_a_reference_walk_alone_would_have_produced_a_journal_that_does_not_admit() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let reached = record.under(&decided[2]);
    let eligibilities: BTreeSet<usize> = reached
        .iter()
        .filter(|position| {
            matches!(
                side.files.journal[**position],
                Admission::Eligibility { .. }
            )
        })
        .copied()
        .collect();

    assert_eq!(
        eligibilities.len(),
        2,
        "both parties to the plans are reached only through the rule"
    );

    let without: BTreeSet<usize> = reached.difference(&eligibilities).copied().collect();
    let mut canon = Canon::new(ResidentHistory::new());
    let refusal = journal::replay(&mut canon, &pruned(&side.files.journal, &without))
        .expect_err("a closure missing an eligibility cannot admit");

    assert!(
        refusal.to_string().contains("not eligible"),
        "the refusal names the rule the walk did not restate: {refusal}"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — The same decision, early and late
// ---------------------------------------------------------------------------------------------

/// W2: one decision at two points in one history. One set grows; the other does not.
///
/// The same [`ape_cli::lineage::Decision`], so the same world by identity — which is what makes this
/// a comparison rather than an argument. What differs between the two records is entirely what had
/// been admitted by the time each was written, and none of it is what the decision is about.
#[test]
fn phase_2_the_same_decision_costs_more_for_having_been_taken_later() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let early = &side.files.lineage[1];
    let late = &side.restated;

    assert_eq!(
        early.decision, late.decision,
        "the two records are of one decision"
    );

    let world = record.addresses(&decided[1]);

    assert_eq!(
        (early.witness.len(), late.witness.len()),
        (WITNESSED[1], ENTRIES),
        "what stood at each position"
    );
    assert_eq!(
        world.len(),
        DEPENDED[1],
        "what the decision is a function of, at either position"
    );

    // The gap is not a constant overhead: it is everything admitted in between.
    assert_eq!(
        late.witness.len() - early.witness.len(),
        FILINGS.len() + 1,
        "the filings, and the plan the late decision is not about"
    );

    let (broad, narrow) = (
        serde_json::to_string_pretty(&side.files.lineage).expect("a lineage encodes"),
        serde_json::to_string_pretty(&depending(&side.files.lineage, &record, &decided))
            .expect("a lineage encodes"),
    );

    let witnessed: usize = side.files.lineage.iter().map(|t| t.witness.len()).sum();
    let reached: usize = DEPENDED.iter().sum();

    assert_eq!(
        (witnessed, reached),
        (WITNESSED.iter().sum(), DEPENDED.iter().sum()),
        "addresses, one lineage over"
    );
    assert!(
        narrow.len() < broad.len(),
        "the narrow lineage is {} bytes against {}",
        narrow.len(),
        broad.len()
    );

    // Reported rather than asserted at a literal: the point is the ratio, and the ratio is a
    // property of this arrangement's tail. What is asserted is the direction and the entry counts.
    println!(
        "lineage bytes: broad {} / narrow {} ({}%)  addresses: {witnessed} / {reached}",
        broad.len(),
        narrow.len(),
        narrow.len() * 100 / broad.len(),
    );
}

/// The same lineage with each decision witnessed by what its world is a function of.
///
/// A substitution rather than a new format: only `witness` changes, so the comparison is of the claim
/// and not of an encoding somebody redesigned in between.
fn depending(
    lineage: &[Taken],
    record: &dependence::Record<'_>,
    decided: &[Thesis],
) -> Vec<Taken> {
    lineage
        .iter()
        .zip(decided)
        .map(|(taken, thesis)| Taken {
            witness: record.addresses(thesis),
            ..taken.clone()
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — Dependence in its place, over the three refused cases
// ---------------------------------------------------------------------------------------------

/// A rebuild that weighs a decision against what its world depends on.
///
/// The instrument's second half, and the shape the candidate asks for: the record **holds** the
/// dependence set, so the rebuild admits exactly what the lineage names and the derivation becomes the
/// check rather than the input. That is the only order in which it can work — a closure is a function
/// of the world, and the world is what the decision produces, so nothing can know what to admit before
/// the decision has been applied.
///
/// The comparison is two-directional like the one it replaces, and for the same reason: a record that
/// claimed less than its world needs would be unproducible, and one that claimed more would be
/// asserting a dependence the world does not have.
fn rebuild_by_dependence(
    journal: &[Admission],
    decisions: &[Taken],
) -> Result<(Canon<ResidentHistory>, Vec<Thesis>), Unmet> {
    let record = dependence::Record::of(journal);
    let claimed: BTreeSet<EntryId> = decisions
        .iter()
        .flat_map(|taken| taken.witness.iter().cloned())
        .collect();

    let mut canon = Canon::new(ResidentHistory::new());
    let wanted: Vec<Admission> = record
        .positions(&claimed)
        .into_iter()
        .map(|position| journal[position].clone())
        .collect();

    journal::replay(&mut canon, &wanted).map_err(|why| Unmet::DoesNotAdmit(why.to_string()))?;

    let mut lineage = ape_cli::lineage::Lineage::new();

    for taken in decisions {
        lineage::decide(canon.history(), &mut lineage, &taken.decision)
            .map_err(|why| Unmet::DoesNotDecide(why.to_string()))?;

        let produced = lineage
            .decided()
            .last()
            .expect("a decision that applied produced a world");
        let derived = record.addresses(produced);

        if let Some(unrecorded) = derived.difference(&taken.witness).next() {
            return Err(Unmet::Unrecorded(unrecorded.clone()));
        }

        if let Some(undepended) = taken.witness.difference(&derived).next() {
            return Err(Unmet::NotDepended(undepended.clone()));
        }
    }

    let decided = lineage.decided().to_vec();

    Ok((canon, decided))
}

/// What a dependence comparison refuses, in the vocabulary of the claim it weighs.
#[derive(Debug)]
enum Unmet {
    DoesNotAdmit(String),
    DoesNotDecide(String),
    /// The world reaches an entry the record did not claim to depend on.
    Unrecorded(EntryId),
    /// The record claims a dependence the world does not have.
    NotDepended(EntryId),
}

/// Named the way the refusal it replaces is named: the entry that disagrees, never the file.
impl std::fmt::Display for Unmet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesNotAdmit(why) => write!(f, "the claimed entries do not admit: {why}"),
            Self::DoesNotDecide(why) => write!(f, "the claimed entries do not decide: {why}"),
            Self::Unrecorded(entry) => write!(
                f,
                "the world reaches entry {entry}, which the decision does not claim to depend on"
            ),
            Self::NotDepended(entry) => write!(
                f,
                "the decision claims entry {entry}, which its world does not depend on"
            ),
        }
    }
}

/// The same lineage, each decision witnessed by what its world depends on.
///
/// `journal` is the record the claim will be weighed over, which need not be the one the decisions
/// were taken against — an address is derived from what admitting produced, so a dependence set means
/// the same entries in any journal that holds them.
fn by_dependence(journal: &[Admission], decisions: &[Taken], decided: &[Thesis]) -> Vec<Taken> {
    depending(decisions, &dependence::Record::of(journal), decided)
}

/// Every decision either side holds, each held once, linearized the way `converge` linearizes.
///
/// A replica of the application's own ordering rather than a call to it: the operation that would
/// merge these refuses one layer above, so the union has to be built here to be weighed at all.
///
/// Sorted by where each decision was taken and then by its own content, which is `converge::ordered`'s
/// own settlement. The walk that puts every parent before its children is **not** replicated: here
/// every fork extends the genesis and was taken after it, so the sort already satisfies the one
/// constraint a lineage imposes. An arrangement where it did not would be measuring the ordering rather
/// than the witness.
fn merged(journal: &[Admission], sides: [&[Taken]; 2]) -> Vec<Taken> {
    let entries = witness::entries(journal).expect("the union journal replays");
    let at = |taken: &Taken| entries.iter().position(|held| *held == taken.after);

    let mut union: Vec<Taken> = Vec::new();

    for side in sides {
        for taken in side {
            if !union.contains(taken) {
                union.push(taken.clone());
            }
        }
    }

    union.sort_by(|one, other| (at(one), one).cmp(&(at(other), other)));

    union
}

/// The dependence comparison refuses in both directions, and the two say different things.
///
/// A claim about knowledge is worth what it can be wrong about. So the instrument is weighed the way
/// `corroborate` is weighed: handed a record that claims **more** than its world depends on, and one
/// that claims **less**.
///
/// The second is the interesting one. Under-claiming is normally caught earlier — the entry is simply
/// not admitted and the world cannot be produced — and it reaches the comparison at all only because
/// another decision's claim carried the entry. Which is the shape to remember: a dependence set is
/// checked against the world it produced, not against the journal it was written beside.
#[test]
fn phase_3_a_claim_about_dependence_can_be_wrong_in_two_directions() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let Err(claiming_everything) = rebuild_by_dependence(&side.files.journal, &side.files.lineage)
    else {
        panic!("the broad witness was accepted as a dependence claim");
    };

    assert!(
        matches!(claiming_everything, Unmet::NotDepended(_)),
        "a record claiming what its world does not depend on: {claiming_everything}"
    );

    let mut claimed = by_dependence(&side.files.journal, &side.files.lineage, &decided);
    let dropped = record
        .addresses(&decided[0])
        .into_iter()
        .find(|entry| {
            let position = *record
                .positions(&[entry.clone()].into())
                .first()
                .expect("an entry of this journal");

            matches!(side.files.journal[position], Admission::Event { .. })
        })
        .expect("the founding world depends on Events");

    claimed[0].witness.remove(&dropped);

    let Err(claiming_too_little) = rebuild_by_dependence(&side.files.journal, &claimed) else {
        panic!("a claim short of its own world was accepted");
    };

    assert!(
        matches!(claiming_too_little, Unmet::Unrecorded(ref entry) if *entry == dropped),
        "and it names the entry the world reached anyway: {claiming_too_little}"
    );
}

/// The union of two twinned repositories, refused by the record and admitted by dependence.
///
/// The collision experiment's Observation 10, reproduced first: the two journals are out of step at
/// one entry no world selects, and handing their union to a rebuild is refused at
/// `UnwitnessedKnowledge` — set equality, not containment.
#[test]
fn phase_3_the_union_of_two_twinned_repositories() {
    let arranged = witness::arranged().expect("the subject is arranged");
    let union = arranged.left.files.journal.clone();
    let lineage = merged(
        &union,
        [&arranged.left.files.lineage, &arranged.right.files.lineage],
    );

    // Two of the three decisions are already one record: the tail was admitted between the second and
    // the third, so only the last of them witnessed anything the other side had not.
    assert_eq!(
        lineage.len(),
        WORLDS + 1,
        "four records for three worlds, and the duplicate is the late one"
    );

    let mut canon = Canon::new(ResidentHistory::new());
    let refusal = lineage::rebuild(&mut canon, &union, &lineage)
        .expect_err("the record refuses the union it could produce");

    assert!(
        matches!(refusal, LineageError::UnwitnessedKnowledge { .. }),
        "reproduced as the collision experiment measured it: {refusal}"
    );

    // And now the same union, weighed against what each decision depends on. Each side's worlds come
    // from its own files, because that is where its decisions were taken.
    let (_, here) = rebuilt(&arranged.left.files);
    let (_, there) = rebuilt(&arranged.right.files);
    let claimed = merged(
        &union,
        [
            &by_dependence(&union, &arranged.left.files.lineage, &here),
            &by_dependence(&union, &arranged.right.files.lineage, &there),
        ],
    );

    assert_eq!(
        claimed.len(),
        WORLDS,
        "the duplicate never arises: two records of one decision became one"
    );

    let (canon, decided) =
        rebuild_by_dependence(&union, &claimed).expect("dependence admits the union");

    assert_eq!(decided.len(), WORLDS);
    assert_eq!(
        decided.iter().map(Thesis::id).collect::<Vec<_>>(),
        arranged.left.worlds,
        "the worlds are the ones both repositories already held"
    );

    let intended: Vec<i128> = decided
        .iter()
        .map(|thesis| {
            witness::intended(canon.history(), thesis, arranged.left.instance)
                .expect("a world reads")
        })
        .collect();

    assert_eq!(intended, INTENDED.to_vec(), "and they mean what they meant");
}

/// Whether the witness a decision brought with it is still **true** in the record it arrived in.
///
/// Three conditions, and they are the most a merged record could check without being told that more
/// than one line of knowing exists: the claim names entries the journal holds, it names the coordinate
/// the decision was taken at, and the entries it names replay as a history in their own right.
///
/// A witness that passes all three is a **true statement about a journal that is not this one.**
fn coherent(journal: &[Admission], taken: &Taken) -> bool {
    let record = dependence::Record::of(journal);
    let positions = record.positions(&taken.witness);

    if positions.len() != taken.witness.len() || !taken.witness.contains(&taken.after) {
        return false;
    }

    let mut canon = Canon::new(ResidentHistory::new());

    journal::replay(&mut canon, &pruned(journal, &positions)).is_ok()
}

/// What a merge dissolves, and what it does not.
///
/// The question the collision and witness experiments circled and neither asked outright: a decision
/// arriving from another repository was taken over knowledge that repository held, and the record it
/// arrives in holds more. So what happens to *what its decider could have known*?
///
/// Measured rather than reasoned, because two answers are natural and only one is right. The claim is
/// not destroyed — it stays a true statement, and the entries it names still replay. What it stops
/// being is **checkable**, because the only thing the record can weigh it against is a prefix of a
/// journal that is not the one it was taken against.
///
/// And the sting is the last assertion. A comparison weak enough to accept an imported decision is
/// exactly weak enough to accept a journal that grew underneath a local one — the two are the same
/// shape, and nothing in the record distinguishes them.
#[test]
fn phase_3_a_merge_dissolves_what_a_decider_could_have_known() {
    let arranged = witness::arranged().expect("the subject is arranged");
    let union = &arranged.left.files.journal;

    let arriving = arranged
        .right
        .files
        .lineage
        .last()
        .expect("the right repository decided");

    // What stood where it was taken, against what stands at the same coordinate here.
    let mut canon = Canon::new(ResidentHistory::new());
    let mut admitted = ape_cli::journal::Replayed::default();

    journal::replay_through(&mut canon, union, &mut admitted, &arriving.after)
        .expect("the coordinate resolves in the union");

    assert_eq!(
        (arriving.witness.len(), admitted.entries.len()),
        (ENTRIES_WITHOUT_FILINGS, ENTRIES),
        "the same coordinate, and six entries the decider could not have seen"
    );

    // The world is untouched, so nothing derived from a world can notice.
    let (_, there) = rebuilt(&arranged.right.files);
    let (_, produced) = rebuild_by_dependence(
        union,
        &by_dependence(union, &arranged.right.files.lineage, &there),
    )
    .expect("dependence admits the arriving lineage");

    assert_eq!(
        produced.iter().map(Thesis::id).collect::<Vec<_>>(),
        arranged.right.worlds,
        "same worlds, so the worlds file agrees either way"
    );

    // The claim is still true: the entries it names are a history, and they produce its world.
    assert!(
        coherent(union, arriving),
        "the witness is a true statement about a journal that is not this one"
    );

    // And the check that accepts it accepts the other thing too. Left's own late decision, over a
    // journal that gained an entry after it was taken, is coherent by the same three conditions.
    let mut grown = union.clone();
    let a_filing = witness::entries(union)
        .expect("the union replays")
        .iter()
        .position(|entry| *entry == EntryId::of(arranged.left.filings[0]))
        .expect("the left repository filed");

    // Inserted before the late plan and after the filings, so the recording watermark holds: an
    // admission dated earlier than what precedes it is refused, and would be measuring that instead.
    grown.insert(grown.len() - 1, filing(&arranged.left, a_filing, 19));

    let local = arranged
        .left
        .files
        .lineage
        .last()
        .expect("the left repository decided");

    assert!(
        coherent(&grown, local),
        "a decision whose journal grew underneath it passes the same three conditions — so a record \
         that admits an imported witness cannot tell the two apart without being told that there is \
         more than one line of knowing"
    );

    // And the three conditions are conditions: a claim naming knowledge the record does not hold at
    // all is refused, which is what keeps the two assertions above from being satisfied by anything.
    let mut shrunk = union.clone();

    shrunk.remove(a_filing);

    assert!(
        !coherent(&shrunk, local),
        "a witness naming an entry no journal here holds is not a true statement about any journal"
    );
}

/// Knowledge taken without the lineage that witnessed it.
///
/// The right repository takes the left's journal and keeps its own three decisions. Nothing about its
/// worlds changes; what changed is what stood, and the record is answerable to that rather than to its
/// worlds.
#[test]
fn phase_3_a_journal_taken_without_its_lineage() {
    let arranged = witness::arranged().expect("the subject is arranged");
    let taken = &arranged.left.files.journal;

    let mut canon = Canon::new(ResidentHistory::new());
    let refusal = lineage::rebuild(&mut canon, taken, &arranged.right.files.lineage)
        .expect_err("the record refuses knowledge its decisions did not witness");

    assert!(
        matches!(refusal, LineageError::UnwitnessedKnowledge { .. }),
        "the entry the taking side never decided against: {refusal}"
    );

    let (_, theirs) = rebuilt(&arranged.right.files);
    let claimed = by_dependence(taken, &arranged.right.files.lineage, &theirs);
    let (_, decided) =
        rebuild_by_dependence(taken, &claimed).expect("dependence admits the taking");

    assert_eq!(
        decided.iter().map(Thesis::id).collect::<Vec<_>>(),
        arranged.right.worlds,
        "its own lineage, over knowledge it did not have when it decided"
    );
}

/// And the limit: dependence collapses a duplicate only where both sides decided at the same point.
///
/// A `Taken` is a decision, a coordinate and a claim about knowledge. Replacing the claim makes two
/// records of one decision one record — as long as the coordinate agrees. Where two sides took the
/// same decision after **different** entries, `after` still tells them apart, and the duplicate
/// survives the narrowing.
#[test]
fn phase_3_the_coordinate_still_distinguishes_two_records_of_one_decision() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let record = dependence::Record::of(&side.files.journal);
    let (_, decided) = rebuilt(&side.files);

    let early = &side.files.lineage[1];
    let late = &side.restated;

    let (narrow_early, narrow_late) = (
        Taken {
            witness: record.addresses(&decided[1]),
            ..early.clone()
        },
        Taken {
            witness: record.addresses(&decided[1]),
            ..late.clone()
        },
    );

    assert_eq!(
        narrow_early.witness, narrow_late.witness,
        "the claim about knowledge is now the same claim"
    );
    assert_ne!(
        narrow_early, narrow_late,
        "and the two are still two records, because they were taken at different points"
    );
    assert_ne!(narrow_early.after, narrow_late.after);
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — What stops being refused
// ---------------------------------------------------------------------------------------------

/// How a comparison answered one mutated record.
#[derive(Debug, PartialEq, Eq)]
enum Answered {
    /// It refused, under this name.
    Refused(&'static str),
    /// It rebuilt, and the record reads as though nothing had happened.
    Silence,
}

/// What the **third** file would say, once the narrowing has let the mutation past.
///
/// A repository holds a journal, a lineage and the worlds it says it decided, and reading one weighs
/// the third against what the first two produce — `ReadingError::WorldDisagrees`. So the witness is
/// not the only guard a mutation has to get past, and the question that decides what narrowing costs
/// is which of them would have caught it anyway.
///
/// Asked in that order on purpose. Today the record refuses at the witness and the worlds file never
/// weighs anything, so the only way to learn what it *would* have said is to let the mutation through
/// the narrow comparison first and put the result to it.
fn by_worlds(
    recorded: &[WorldRecord],
    journal: &[Admission],
    lineage: &[Taken],
    decided: &[Thesis],
) -> Answered {
    let Ok((_, produced)) =
        rebuild_by_dependence(journal, &by_dependence(journal, lineage, decided))
    else {
        return Answered::Refused("never reached");
    };

    if produced.len() != recorded.len() {
        return Answered::Refused("LineageLengthDisagrees");
    }

    for (thesis, record) in produced.iter().zip(recorded) {
        if WorldRecord::of(thesis).disagreement(record).is_some() {
            return Answered::Refused("WorldDisagrees");
        }
    }

    Answered::Silence
}

/// Every state the broad witness refuses, put through both comparisons.
///
/// A table rather than a case, because the claim is about **which** of them survive the narrowing, and
/// a phase that measured one would be reporting a sample as a closed set. Each row is a mutation of one
/// arrangement: the same side, the same three decisions, one thing done to the record afterwards.
///
/// The last row is carried forward rather than found here — a witness is a set, so a swap inside a
/// prefix was already invisible to it, which the divergence experiment measured. It is in the table
/// because leaving it out would make the narrowing look like the only thing that cannot see a
/// reordering.
#[test]
fn phase_4_what_a_narrower_claim_stops_refusing() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let (_, decided) = rebuilt(&side.files);
    let entries = witness::entries(&side.files.journal).expect("the journal replays");

    // Where the mutations go, found by identity rather than counted: the tail sits between the second
    // decision and the third, and the two plans bracket it.
    let at = |commitment| {
        entries
            .iter()
            .position(|entry| *entry == EntryId::of(commitment))
            .expect("a commitment of this journal")
    };
    let (first_filing, early_plan, late_plan) =
        (at(side.filings[0]), at(side.plans[0]), at(side.plans[1]));

    let mut table: Vec<(&'static str, Answered, Answered, Answered)> = Vec::new();

    // The lineage of the founding and the restated decision, for the two rows that need a decision
    // whose coordinate is not the entry being mutated — a missing coordinate is a different fault with
    // a name of its own. It produces the founding world and the early one, in that order.
    let paired = vec![side.files.lineage[0].clone(), side.restated.clone()];
    let pair = vec![decided[0].clone(), decided[1].clone()];
    let pair_recorded = vec![side.files.worlds[0].clone(), side.files.worlds[1].clone()];

    // An entry ADDED to a prefix, depended on by nothing.
    {
        let mut journal = side.files.journal.clone();

        // A fifth magnitude rather than a copy: readmitting an address is the fourth row, not this one.
        journal.insert(late_plan, filing(&side, first_filing, 17));

        table.push((
            "added, not depended on",
            broad(&journal, &side.files.lineage),
            narrow(&journal, &side.files.lineage, &decided),
            by_worlds(
                &side.files.worlds,
                &journal,
                &side.files.lineage,
                &decided,
            ),
        ));
    }

    // An entry REMOVED from a prefix, depended on by nothing.
    {
        let mut journal = side.files.journal.clone();

        journal.remove(first_filing);

        table.push((
            "removed, not depended on",
            broad(&journal, &side.files.lineage),
            narrow(&journal, &side.files.lineage, &decided),
            by_worlds(
                &side.files.worlds,
                &journal,
                &side.files.lineage,
                &decided,
            ),
        ));
    }

    // An entry REMOVED from a prefix, DEPENDED on.
    {
        let mut journal = side.files.journal.clone();

        journal.remove(early_plan);

        table.push((
            "removed, depended on",
            broad(&journal, &paired),
            narrow(&journal, &paired, &pair),
            by_worlds(&pair_recorded, &journal, &paired, &pair),
        ));
    }

    // A READMISSION: one address the journal admits twice, and a decision naming it.
    {
        let mut journal = side.files.journal.clone();

        journal.insert(late_plan, journal[first_filing].clone());

        let readmitted = Taken {
            decision: side.restated.decision.clone(),
            after: entries[first_filing].clone(),
            // Taken when the second occurrence was the most recent entry, so the witness holds what
            // was admitted between the two — which is what a set cannot express and the coordinate
            // cannot resolve.
            witness: witnessed(&journal, late_plan + 1),
            by: None,
        };
        let lineage = vec![side.files.lineage[0].clone(), readmitted];

        table.push((
            "a readmission",
            broad(&journal, &lineage),
            narrow(&journal, &lineage, &pair),
            by_worlds(&pair_recorded, &journal, &lineage, &pair),
        ));
    }

    // Two entries SWAPPED inside a prefix, both recorded at the same instant.
    {
        let mut journal = side.files.journal.clone();

        journal.swap(first_filing, first_filing + 1);

        table.push((
            "swapped inside a prefix",
            broad(&journal, &side.files.lineage),
            narrow(&journal, &side.files.lineage, &decided),
            by_worlds(
                &side.files.worlds,
                &journal,
                &side.files.lineage,
                &decided,
            ),
        ));
    }

    for (what, witness, dependence, worlds) in &table {
        println!("{what:26} witness {witness:?} | dependence {dependence:?} | worlds {worlds:?}");
    }

    assert_eq!(
        table,
        vec![
            (
                "added, not depended on",
                Answered::Refused("UnwitnessedKnowledge"),
                Answered::Silence,
                Answered::Silence,
            ),
            (
                "removed, not depended on",
                Answered::Refused("WitnessedKnowledgeAbsent"),
                Answered::Silence,
                Answered::Silence,
            ),
            (
                "removed, depended on",
                Answered::Refused("WitnessedKnowledgeAbsent"),
                Answered::Refused("DoesNotDecide"),
                Answered::Refused("never reached"),
            ),
            (
                "a readmission",
                Answered::Refused("ReadmittedEntryIsAmbiguous"),
                Answered::Silence,
                Answered::Silence,
            ),
            (
                "swapped inside a prefix",
                Answered::Silence,
                Answered::Silence,
                Answered::Silence,
            ),
        ],
        "the closed table of what each guard can see"
    );
}

/// A readmission the record does not diagnose, because nothing refused first.
///
/// `diagnosed` runs only on the error path of `corroborate`, so the cause it names is reachable only
/// through a refusal it happens to be able to explain. Where the readmission sits past the last
/// coordinate, every decision corroborates and the journal admits one address twice with nothing saying
/// so — which is true today and has nothing to do with narrowing anything.
#[test]
fn phase_4_a_readmission_nothing_refused_is_a_readmission_nothing_names() {
    let side = witness::side(witness::Filings::All).expect("the subject is arranged");
    let mut journal = side.files.journal.clone();

    journal.push(journal[22].clone());

    let entries = witness::entries(&journal).expect("a readmitted journal replays");
    let distinct: BTreeSet<&EntryId> = entries.iter().collect();

    assert_eq!(
        (entries.len(), distinct.len()),
        (ENTRIES + 1, ENTRIES),
        "one address, admitted twice"
    );

    assert_eq!(
        broad(&journal, &side.files.lineage),
        Answered::Silence,
        "the record rebuilds, and no reader is told the journal repeats itself"
    );
}

/// What the record's own comparison makes of a mutated repository.
fn broad(journal: &[Admission], lineage: &[Taken]) -> Answered {
    let mut canon = Canon::new(ResidentHistory::new());

    match lineage::rebuild(&mut canon, journal, lineage) {
        Ok(_) => Answered::Silence,
        Err(LineageError::UnwitnessedKnowledge { .. }) => {
            Answered::Refused("UnwitnessedKnowledge")
        }
        Err(LineageError::WitnessedKnowledgeAbsent { .. }) => {
            Answered::Refused("WitnessedKnowledgeAbsent")
        }
        Err(LineageError::ReadmittedEntryIsAmbiguous { .. }) => {
            Answered::Refused("ReadmittedEntryIsAmbiguous")
        }
        Err(why) => panic!("a refusal this table does not account for: {why}"),
    }
}

/// What the dependence comparison makes of the same one.
fn narrow(journal: &[Admission], lineage: &[Taken], decided: &[Thesis]) -> Answered {
    match rebuild_by_dependence(journal, &by_dependence(journal, lineage, decided)) {
        Ok(_) => Answered::Silence,
        Err(Unmet::DoesNotAdmit(_)) => Answered::Refused("DoesNotAdmit"),
        Err(Unmet::DoesNotDecide(_)) => Answered::Refused("DoesNotDecide"),
        Err(Unmet::Unrecorded(_)) => Answered::Refused("Unrecorded"),
        Err(Unmet::NotDepended(_)) => Answered::Refused("NotDepended"),
    }
}

/// The addresses the first `upto` admissions of a journal produce, as a witness would hold them.
fn witnessed(journal: &[Admission], upto: usize) -> BTreeSet<EntryId> {
    witness::entries(&journal[..upto])
        .expect("a prefix replays")
        .into_iter()
        .collect()
}

/// One more filing, of a magnitude the subject does not hold.
fn filing(side: &witness::Side, like: usize, magnitude: u128) -> Admission {
    let Admission::Commitment {
        accountable,
        executors,
        beneficiaries,
        statement,
        resource,
        committed_at,
        due_date,
        dependencies,
        recorded_at,
        ..
    } = side.files.journal[like].clone()
    else {
        panic!("the entry copied here is a filing");
    };

    Admission::Commitment {
        accountable,
        executors,
        beneficiaries,
        statement,
        resource,
        committed_at,
        due_date,
        magnitude: Some(magnitude),
        dependencies,
        recorded_at,
    }
}

/// The entries at `positions`, in the order the journal admitted them.
fn pruned(journal: &[Admission], positions: &BTreeSet<usize>) -> Vec<Admission> {
    positions
        .iter()
        .map(|position| journal[*position].clone())
        .collect()
}

/// The decisions that lead to the world at `position`, oldest first.
///
/// Derived by walking the world's parents rather than by taking a prefix of the lineage: a fork's
/// siblings are not on the way to it, and taking them along would be measuring a different journal.
fn ancestry(
    lineage: &[Taken],
    decided: &[Thesis],
    position: usize,
) -> Vec<ape_cli::lineage::Decision> {
    let mut wanted = BTreeSet::new();
    let mut cursor = Some(decided[position].id());

    while let Some(id) = cursor {
        let at = decided
            .iter()
            .position(|thesis| thesis.id() == id)
            .expect("a world of this lineage");

        wanted.insert(at);
        cursor = *decided[at].parent();
    }

    wanted
        .into_iter()
        .map(|at| lineage[at].decision.clone())
        .collect()
}
