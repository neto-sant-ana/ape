//! A history of the house's spending, written as the sequence of steps that produced it,
//! and the fold that turns that sequence back into the world it describes.
//!
//! A [`Step`] is one thing that happened, in the order it happened. Intentions are addressed
//! by the position of the `Intend` that produced them, counting from zero.
//!
//! [`replay`] folds a sequence into knowledge, the worlds opened over it, and the identity of
//! the one current at the end. Every world produced is archived, including the ones that only
//! carry knowledge forward: the archive refuses a child whose parent it does not hold, and a
//! walk that skipped them would end in a hole rather than at a beginning.

use ape::canon::{Canon, EventSubmission, InMemoryHistory};
use ape::engine::thesis::{
    ForkInput, GenesisInput, InMemoryArchive, KnowledgeCut, Thesis, ThesisArchive, ThesisId,
};
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use crate::world::{self, World, cancelling};

/// One thing that happened, in the order it happened.
///
/// Intentions are addressed by the position of the `Intend` that produced them, counting
/// from zero, because a position says nothing about what the intention was for.
#[derive(Debug, Clone)]
pub enum Step {
    /// The house intends to spend `amount` by `due`, recorded at `recorded_at`.
    Intend {
        amount: f64,
        due: Date,
        recorded_at: Date,
    },
    /// It was observed that intention `which` will not happen.
    Cancel { which: usize, at: Date },
    /// A world is opened at `known_at`, proposing intentions `select`.
    Open { known_at: Date, select: Vec<usize> },
    /// The current world is carried to `known_at`, recognizing what became known.
    Carry { known_at: Date },
    /// The current world is replaced by one that adds intentions `introduce`.
    Add { introduce: Vec<usize> },
}

/// Knowledge, the archived worlds, and the identity of the one current at the end.
pub struct Graph {
    pub canon: Canon<InMemoryHistory>,
    pub archive: InMemoryArchive,
    pub current: ThesisId,
}

/// What a sequence produced, in the order it produced it.
pub struct Replay {
    pub graph: Graph,
    pub intentions: Vec<CommitmentId>,
    pub worlds: Vec<ThesisId>,
}

/// A day in January 2026.
pub fn january(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// What happened.
pub fn scenario() -> Vec<Step> {
    vec![
        Step::Intend {
            amount: 120.0,
            due: january(8),
            recorded_at: january(6),
        },
        Step::Open {
            known_at: january(6),
            select: vec![0],
        },
        Step::Intend {
            amount: 30.0,
            due: january(14),
            recorded_at: january(6),
        },
        Step::Cancel {
            which: 0,
            at: january(6),
        },
        Step::Carry {
            known_at: january(6),
        },
        Step::Add {
            introduce: vec![1],
        },
        Step::Intend {
            amount: 90.0,
            due: january(20),
            recorded_at: january(9),
        },
        Step::Carry {
            known_at: january(12),
        },
        Step::Add {
            introduce: vec![2],
        },
    ]
}

pub fn build() -> Replay {
    replay(&scenario())
}

/// Fold a sequence of steps into the world it describes.
///
/// Every thesis produced is archived, including the ones that only carry knowledge forward:
/// the archive refuses a child whose parent it does not hold, and a walk that skipped them
/// would end in a hole rather than at a beginning.
pub fn replay(steps: &[Step]) -> Replay {
    let mut world = world::build();
    let mut archive = InMemoryArchive::default();

    let mut intentions: Vec<CommitmentId> = Vec::new();
    let mut worlds: Vec<ThesisId> = Vec::new();
    let mut current: Option<Thesis> = None;

    for step in steps {
        match step {
            Step::Intend {
                amount,
                due,
                recorded_at,
            } => intentions.push(intend(&mut world, *amount, *due, *recorded_at)),

            Step::Cancel { which, at } => {
                world
                    .canon
                    .admit_event(
                        EventSubmission {
                            commitment_id: intentions[*which],
                            observation: cancelling(),
                            occurred_at: *at,
                        },
                        *at,
                    )
                    .expect("an intention may be cancelled");
            }

            Step::Open { known_at, select } => {
                let thesis = Thesis::genesis(
                    world.canon.history(),
                    GenesisInput {
                        cut: KnowledgeCut::at(world.canon.history(), *known_at),
                        selection: select.iter().map(|at| intentions[*at]).collect(),
                    },
                )
                .expect("the proposed intentions are selectable at that instant");

                current = Some(keep(&mut archive, &mut worlds, thesis));
            }

            Step::Carry { known_at } => {
                let thesis = current
                    .as_ref()
                    .expect("a world must be open before it is carried")
                    .advance(
                        world.canon.history(),
                        KnowledgeCut::at(world.canon.history(), *known_at),
                    )
                    .expect("the instant recognizes knowledge the world had not")
                    .into_thesis();

                current = Some(keep(&mut archive, &mut worlds, thesis));
            }

            Step::Add { introduce } => {
                let thesis = current
                    .as_ref()
                    .expect("a world must be open before it is added to")
                    .fork(
                        world.canon.history(),
                        ForkInput {
                            omitted: [].into(),
                            introduced: introduce.iter().map(|at| intentions[*at]).collect(),
                        },
                    )
                    .expect("the intentions are introducible at that cut");

                current = Some(keep(&mut archive, &mut worlds, thesis));
            }
        }
    }

    let current = current.expect("the sequence opens a world");

    Replay {
        graph: Graph {
            canon: world.canon,
            archive,
            current: current.id(),
        },
        intentions,
        worlds,
    }
}

fn keep(archive: &mut InMemoryArchive, worlds: &mut Vec<ThesisId>, thesis: Thesis) -> Thesis {
    worlds.push(thesis.id());

    archive
        .put_thesis(thesis.clone())
        .expect("a thesis is archived after its parent");

    thesis
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
