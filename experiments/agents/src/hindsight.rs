//! The scenario experiment 02 audits: a decision taken soundly, and knowledge arriving
//! afterwards that makes it look otherwise.
//!
//! ```text
//! K1   the house weighs 120 and 30 against a balance of 100
//!      120 is refused, 30 is undertaken, leaving 70
//!
//! K2   an obligation of 90 is recorded, later
//!      70 - 90 = -20
//! ```
//!
//! The sequence mirrors what the first run's agent did, because the point of auditing is to
//! audit something that happened rather than something arranged to be auditable. What is
//! added is only the obligation, and the recording instant that puts it out of reach of the
//! cut the decision was taken under.
//!
//! # What an auditor is handed
//!
//! One `ThesisId`, the archive, and canonical history. Nothing else, and the shortness of
//! that list is not an aesthetic choice.
//!
//! The engine's reads are entirely by identity. There is no operation that enumerates
//! commitments, no operation that enumerates theses, and no named reference — the archive's
//! own documentation says deciding what `main` points at is not its business. So a graph
//! cannot be handed over as a graph; it is handed over as an entry point, and everything
//! else is reached by walking from it.
//!
//! One entry point is therefore the honest minimum, and it is what an application keeping a
//! single current world would have. What can be reached from it is the experiment's question,
//! not its premise.

use ape::canon::{Canon, EventSubmission, InMemoryHistory};
use ape::engine::thesis::{
    ForkInput, GenesisInput, InMemoryArchive, KnowledgeCut, Thesis, ThesisArchive, ThesisId,
};
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use crate::world::{self, World, cancelling};

/// What the auditor receives.
pub struct Graph {
    pub canon: Canon<InMemoryHistory>,
    pub archive: InMemoryArchive,
    pub current: ThesisId,
}

/// What the harness keeps, so that it can check what the auditor concludes.
pub struct Scenario {
    pub graph: Graph,
    pub decision: ThesisId,
    pub considered: ThesisId,
    pub priority: CommitmentId,
    pub standard: CommitmentId,
    pub obligation: CommitmentId,
}

pub fn decided_at() -> Date {
    world::today()
}

pub fn obligation_recorded_at() -> Date {
    january(9)
}

pub fn audited_at() -> Date {
    january(12)
}

pub fn build() -> Scenario {
    let mut world = world::build();
    let mut archive = InMemoryArchive::default();

    let priority = intend(&mut world, 120.0, january(8), decided_at());

    let considered = Thesis::genesis(
        world.canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(world.canon.history(), decided_at()),
            selection: [priority].into(),
        },
    )
    .expect("the priority slot is selectable at the decision instant");

    archive
        .put_thesis(considered.clone())
        .expect("a genesis has no parent to wait for");

    let standard = intend(&mut world, 30.0, january(14), decided_at());

    world
        .canon
        .admit_event(
            EventSubmission {
                commitment_id: priority,
                observation: cancelling(),
                occurred_at: decided_at(),
            },
            decided_at(),
        )
        .expect("an intention may be cancelled");

    // Advancing produces a Thesis of its own: recognizing the cancellation is a node in the
    // lineage, distinct from the node that chooses differently because of it. Both have to be
    // archived, or the walk from a descendant ends in a hole.
    let recognized = considered
        .advance(
            world.canon.history(),
            KnowledgeCut::at(world.canon.history(), decided_at()),
        )
        .expect("the cancellation is later knowledge at the same instant")
        .into_thesis();

    archive
        .put_thesis(recognized.clone())
        .expect("its parent is held");

    let decision = recognized
        .fork(
            world.canon.history(),
            ForkInput {
                omitted: [].into(),
                introduced: [standard].into(),
            },
        )
        .expect("the standard slot is introducible at the same cut");

    archive
        .put_thesis(decision.clone())
        .expect("its ancestry is already held");

    let obligation = intend(&mut world, 90.0, january(20), obligation_recorded_at());

    // The world as it stands once the obligation is known. An application learning of an
    // obligation would want to know what it does to the world it is in, and the only way to
    // ask is to be in a world that contains it — so the current world is a fork of the
    // decision, at a cut that reaches the obligation.
    let widened = decision
        .advance(
            world.canon.history(),
            KnowledgeCut::at(world.canon.history(), audited_at()),
        )
        .expect("the obligation is later knowledge")
        .into_thesis();

    archive
        .put_thesis(widened.clone())
        .expect("its parent is held");

    let current = widened
        .fork(
            world.canon.history(),
            ForkInput {
                omitted: [].into(),
                introduced: [obligation].into(),
            },
        )
        .expect("the obligation is introducible at a cut that reaches it");

    archive
        .put_thesis(current.clone())
        .expect("its ancestry is already held");

    Scenario {
        graph: Graph {
            canon: world.canon,
            archive,
            current: current.id(),
        },
        decision: decision.id(),
        considered: considered.id(),
        priority,
        standard,
        obligation,
    }
}

/// Admit an intention of the house to spend `amount` by `due`, recorded at `recorded_at`.
fn intend(world: &mut World, amount: f64, due: Date, recorded_at: Date) -> CommitmentId {
    world
        .canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(world.house, [world.house], [world.market])
                    .expect("both sides are staffed"),
                statement: world.outbound,
                resource: world.account,
                term: Term::new(recorded_at, due).expect("committed before due"),
                action_value: ActionValue::value(amount).expect("a positive, finite magnitude"),
                dependencies: [].into(),
            },
            recorded_at,
        )
        .expect("the house may commit to spend")
}

fn january(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}
