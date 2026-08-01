//! The conformance suite for [`ThesisArchive`] adapters.
//!
//! A storage adapter proves it honors the contract by running [`verify`] against a fresh
//! instance in its own test suite:
//!
//! ```ignore
//! #[test]
//! fn my_archive_conforms() {
//!     ape::engine::thesis::conformance::verify(MyArchive::default);
//! }
//! ```
//!
//! Nothing here asserts *which* identity a Thesis has, and that restraint is deliberate.
//! A `ThesisId` is derived from the encoding of its selection today, and the encoding is
//! expected to change — a root hashed by key rather than a serialized set. A suite that
//! pinned literal id bytes would turn itself into an anchor for the current form and make
//! that change look like a regression. What is proven instead is that identity is stable
//! and derived from content: the same Thesis stored twice is one record, and what comes
//! back is what went in.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    ArchiveOutcome, ForkInput, GenesisInput, KnowledgeCut, Thesis, ThesisArchive, ThesisError,
    ThesisId,
};

use crate::canon::{Canonical, CanonicalKnowledge};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentId, CommitmentInput, Event, EventId, ResourceInstanceId,
    StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Term};

pub fn verify<A: ThesisArchive>(instance: impl Fn() -> A) {
    storing_is_idempotent(instance());
    a_stored_thesis_resolves_to_what_was_stored(instance());
    an_absent_thesis_resolves_to_nothing(instance());
    a_child_stored_before_its_parent_is_refused(instance());
    ancestry_walks_from_a_child_to_its_genesis(instance());
    diverging_lineages_are_held_side_by_side(instance());
    unrelated_lineages_do_not_meet(instance());
}

/// A Thesis is content-addressed, so storing one twice is storing one record.
///
/// The second put is not an overwrite and not a refusal: the archive already holds exactly
/// what is being offered. An adapter is free to notice that however it stores things, but
/// it must report which of the two happened, because a caller counting what it added
/// cannot ask again afterwards.
pub fn storing_is_idempotent<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let thesis = genesis(&knowledge, &[ids[0]]);

    assert_eq!(
        archive.put_thesis(thesis.clone()).unwrap(),
        ArchiveOutcome::Stored,
        "the first put of an absent Thesis stores it",
    );

    assert_eq!(
        archive.put_thesis(thesis.clone()).unwrap(),
        ArchiveOutcome::AlreadyPresent,
        "the same record offered again is already held",
    );

    let rebuilt = genesis(&knowledge, &[ids[0]]);

    assert_eq!(
        archive.put_thesis(rebuilt).unwrap(),
        ArchiveOutcome::AlreadyPresent,
        "identity follows content, so an equal Thesis built again is the same record",
    );
}

/// What a Thesis means must survive the archive.
///
/// Its cut and its partition are not decoration: a Thesis interpreted under another cut, or
/// one whose frozen past came back as open, would be a different world wearing the same id.
pub fn a_stored_thesis_resolves_to_what_was_stored<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let parent = genesis(&knowledge, &[ids[0]]);
    let child = fork(&knowledge, &parent, &[ids[1]]);

    archive.put_thesis(parent.clone()).unwrap();
    archive.put_thesis(child.clone()).unwrap();

    let resolved = archive
        .thesis(child.id())
        .expect("a stored Thesis resolves");

    assert_eq!(resolved.id(), child.id());
    assert_eq!(*resolved.parent(), Some(parent.id()));
    assert_eq!(resolved.cut(), child.cut());
    assert_eq!(
        resolved.selection().open().collect::<BTreeSet<_>>(),
        child.selection().open().collect::<BTreeSet<_>>(),
        "the open future comes back as it went in",
    );
    assert_eq!(
        resolved.selection().frozen().collect::<BTreeSet<_>>(),
        child.selection().frozen().collect::<BTreeSet<_>>(),
        "and so does the frozen past",
    );
}

pub fn an_absent_thesis_resolves_to_nothing<A: ThesisArchive>(archive: A) {
    assert!(
        archive.thesis(ThesisId::from([9; 32])).is_none(),
        "an archive answers for what it holds, and invents nothing",
    );
}

/// Ancestry is walked by resolving `parent`, so a lineage may not begin in a hole.
///
/// A child stored before its parent ends that walk indistinguishably from a genesis, and
/// nothing downstream can tell a lineage that ended because it began from one that ended
/// because a record is missing. Synthesis decides whether a Base is a common ancestor by
/// exactly this walk, so the hole would read as a definite answer.
pub fn a_child_stored_before_its_parent_is_refused<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let parent = genesis(&knowledge, &[ids[0]]);
    let child = fork(&knowledge, &parent, &[ids[1]]);

    assert!(
        matches!(
            archive.put_thesis(child.clone()),
            Err(ThesisError::ParentNotArchived { thesis, parent: absent })
                if thesis == child.id() && absent == parent.id()
        ),
        "a child may not be stored while its parent is absent",
    );

    assert!(
        archive.thesis(child.id()).is_none(),
        "a refused put leaves no trace",
    );

    archive.put_thesis(parent).unwrap();

    assert_eq!(
        archive.put_thesis(child).unwrap(),
        ArchiveOutcome::Stored,
        "and the same child is accepted once its parent is held",
    );
}

/// The walk terminates, and it terminates at a genesis rather than at exhaustion.
///
/// This is the read Synthesis performs to establish a common ancestor, so the archive must
/// answer every step of it: each stored Thesis resolves, each parent resolves in turn, and
/// the chain ends at the one Thesis that has no parent.
pub fn ancestry_walks_from_a_child_to_its_genesis<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let first = genesis(&knowledge, &[ids[0]]);
    let second = fork(&knowledge, &first, &[ids[1]]);
    let third = fork(&knowledge, &second, &[ids[2]]);

    archive.put_thesis(first.clone()).unwrap();
    archive.put_thesis(second.clone()).unwrap();
    archive.put_thesis(third.clone()).unwrap();

    assert_eq!(
        lineage_of(&archive, third.id()),
        vec![third.id(), second.id(), first.id()],
        "the walk reaches each ancestor in order and stops at the genesis",
    );
}

/// The archive holds a forest, not a chain.
///
/// The Canon keeps one Event chain under a single head, and an adapter written in its image
/// may carry that shape over. Intention branches by design: two forks of one parent are two
/// continuations of the same world, both are records, and neither displaces the other. What
/// they share is the ancestor the walk from each one reaches — which is the whole question
/// Synthesis asks of a Base.
pub fn diverging_lineages_are_held_side_by_side<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let root = genesis(&knowledge, &[ids[0]]);
    let left = fork(&knowledge, &root, &[ids[1]]);
    let right = fork(&knowledge, &root, &[ids[2]]);

    assert_ne!(
        left.id(),
        right.id(),
        "two continuations of one world are two Theses",
    );

    archive.put_thesis(root.clone()).unwrap();
    archive.put_thesis(left.clone()).unwrap();

    assert_eq!(
        archive.put_thesis(right.clone()).unwrap(),
        ArchiveOutcome::Stored,
        "a second child of the same parent is a record of its own",
    );

    assert!(
        archive.thesis(left.id()).is_some(),
        "storing the second child does not displace the first",
    );

    assert_eq!(
        lineage_of(&archive, left.id()),
        vec![left.id(), root.id()],
        "the first branch still reaches its ancestor",
    );
    assert_eq!(
        lineage_of(&archive, right.id()),
        vec![right.id(), root.id()],
        "and both branches reach the ancestor they share",
    );
}

/// Unrelated lineages coexist without meeting.
///
/// An archive imposes no single root: two Theses may descend from nothing in common, and
/// their walks end at different genesis records. Synthesis relies on exactly this to refuse
/// a Base — there is no common ancestor to find, and finding none must be an answer rather
/// than an artifact of the archive having forced the two into one tree.
pub fn unrelated_lineages_do_not_meet<A: ThesisArchive>(mut archive: A) {
    let (knowledge, ids) = ground();
    let one = genesis(&knowledge, &[ids[0]]);
    let other = genesis(&knowledge, &[ids[1]]);
    let descendant = fork(&knowledge, &other, &[ids[2]]);

    archive.put_thesis(one.clone()).unwrap();
    archive.put_thesis(other.clone()).unwrap();
    archive.put_thesis(descendant.clone()).unwrap();

    assert_eq!(lineage_of(&archive, one.id()), vec![one.id()]);
    assert_eq!(
        lineage_of(&archive, descendant.id()),
        vec![descendant.id(), other.id()],
    );

    assert!(
        !lineage_of(&archive, descendant.id()).contains(&one.id()),
        "a genesis of another lineage is not an ancestor",
    );
}

/// The walk Synthesis performs: a Thesis, then each parent in turn, ending at a genesis.
fn lineage_of<A: ThesisArchive>(archive: &A, from: ThesisId) -> Vec<ThesisId> {
    let mut cursor = archive.thesis(from).expect("the starting Thesis resolves");
    let mut lineage = vec![cursor.id()];

    while let Some(parent) = *cursor.parent() {
        cursor = archive
            .thesis(parent)
            .expect("every parent of a stored Thesis resolves");
        lineage.push(cursor.id());
    }

    lineage
}

/// Canonical knowledge holding three commitments and no Event, which is all the
/// construction of a Thesis reads.
struct Ground {
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
}
impl CanonicalKnowledge for Ground {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.commitments.get(&id).cloned()
    }

    fn canonical_event(&self, _: EventId) -> Option<Canonical<Event>> {
        None
    }

    fn head_as_of(&self, _: &Date) -> Option<EventId> {
        None
    }
}

fn ground() -> (Ground, Vec<CommitmentId>) {
    let mut commitments = BTreeMap::new();
    let mut ids = Vec::new();

    for month in 3..6u8 {
        let commitment = Commitment::create(CommitmentInput {
            assignment: Assignment::new(
                AgentId::from([1; 32]),
                [AgentId::from([2; 32])],
                [AgentId::from([3; 32])],
            )
            .unwrap(),
            statement: StatementId::from([1; 32]),
            resource: ResourceInstanceId::from([1; 32]),
            term: Term::new(date(2026, 1, 1), date(2026, month, 28)).unwrap(),
            action_value: ActionValue::none(),
            dependencies: BTreeSet::new(),
        })
        .unwrap();

        let id = commitment.id();
        ids.push(id);
        commitments.insert(id, Canonical::new(commitment, date(2026, 1, 5)).unwrap());
    }

    (Ground { commitments }, ids)
}

fn date(year: i32, month: u8, day: u8) -> Date {
    Date::from_ymd(year, month, day).unwrap()
}

fn genesis(knowledge: &Ground, selection: &[CommitmentId]) -> Thesis {
    Thesis::genesis(
        knowledge,
        GenesisInput {
            cut: KnowledgeCut::at(knowledge, date(2026, 2, 5)),
            selection: selection.iter().copied().collect(),
        },
    )
    .unwrap()
}

fn fork(knowledge: &Ground, parent: &Thesis, introduced: &[CommitmentId]) -> Thesis {
    parent
        .fork(
            knowledge,
            ForkInput {
                omitted: BTreeSet::new(),
                introduced: introduced.iter().copied().collect(),
            },
        )
        .unwrap()
}
