//! The scenario experiment 02 audits, written as the sequence that produces it.
//!
//! ```text
//! K1   the house weighs 120 and 30 against a balance of 100
//!      120 is refused, 30 is undertaken, leaving 70
//!
//! K2   an obligation of 90 is recorded, later
//!      70 - 90 = -20
//! ```
//!
//! # Why a sequence and not a program
//!
//! An auditor has to derive what happened. Handed a program, it would read the plot off the
//! variable names — `priority`, `standard`, `obligation` say which intention was wanted,
//! which was settled for and which arrived to spoil it, before a single call is made.
//! Renaming them to `c1`, `c2`, `c3` hides the plot without making it derivable, which is
//! worse: an audit that succeeds against deliberately obscured names has proved nothing
//! about a graph.
//!
//! So the scenario is [`Step`]s. Intentions are referred to by their position in the
//! sequence, decisions name the instants they are taken at, and nothing is called anything.
//! What happened is recoverable from the world the steps produce, which is the only place an
//! audit should be able to recover it from.
//!
//! The form is borrowed from the reconstruction experiment, which arrived at it under
//! different pressure: the public boundary admits constructing knowledge and does not admit
//! reading a description back out, so a durable world has to be the record of what was
//! supplied. Here nothing is being persisted — the sequence is used because a sequence
//! carries no narrative, and it is worth naming that this is the second place in the
//! workspace to fold an admission sequence. Two is not a pattern; a third would be.

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

/// What an auditor receives: knowledge, the archived worlds, and one way in.
pub struct Graph {
    pub canon: Canon<InMemoryHistory>,
    pub archive: InMemoryArchive,
    pub current: ThesisId,
}

/// What the harness keeps, to check what an auditor concludes.
pub struct Scenario {
    pub graph: Graph,
    pub intentions: Vec<CommitmentId>,
    pub worlds: Vec<ThesisId>,
}

impl Scenario {
    /// The world the decision under audit was taken in: the third opened, counting the two
    /// that only carried knowledge forward.
    pub fn decision(&self) -> ThesisId {
        self.worlds[2]
    }
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

/// The sequence. Read it and the plot is derivable; read the names and there are none.
pub fn scenario() -> Vec<Step> {
    vec![
        Step::Intend {
            amount: 120.0,
            due: january(8),
            recorded_at: decided_at(),
        },
        Step::Open {
            known_at: decided_at(),
            select: vec![0],
        },
        Step::Intend {
            amount: 30.0,
            due: january(14),
            recorded_at: decided_at(),
        },
        Step::Cancel {
            which: 0,
            at: decided_at(),
        },
        Step::Carry {
            known_at: decided_at(),
        },
        Step::Add {
            introduce: vec![1],
        },
        Step::Intend {
            amount: 90.0,
            due: january(20),
            recorded_at: obligation_recorded_at(),
        },
        Step::Carry {
            known_at: audited_at(),
        },
        Step::Add {
            introduce: vec![2],
        },
    ]
}

pub fn build() -> Scenario {
    replay(&scenario())
}

/// Fold a sequence of steps into the world it describes.
///
/// Every thesis produced is archived, including the ones that only carry knowledge forward:
/// the archive refuses a child whose parent it does not hold, and a walk that skipped them
/// would end in a hole rather than at a beginning.
pub fn replay(steps: &[Step]) -> Scenario {
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

    Scenario {
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

fn january(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}
