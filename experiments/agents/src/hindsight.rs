//! A history of the house's spending, written as the sequence of steps that produced it, and
//! the fold that turns that sequence into the repository it describes.
//!
//! A [`Step`] is one thing that happened, in the order it happened. Intentions are addressed by
//! the position of the `Intend` that produced them, counting from zero.
//!
//! [`replay`] folds a sequence into two records rather than one, because the substrate keeps
//! them apart: a journal of admissions, and a lineage of decisions each carrying the journal
//! entry that stood when it was taken. That coordinate is the half a decision cannot re-derive,
//! and folding the two together in one pass is what makes them agree.
//!
//! Every decision here is attributed to the house, through the constructor whose name says it is
//! a claim. Nothing about a decision produces the party, so nothing about it can disagree —
//! which is the property the re-run is here to measure rather than to rely on.

use ape::canon::Canon;
use ape::engine::thesis::{Thesis, ThesisId, ThesisLookup};
use ape::kernel::entities::CommitmentId;

use ape_cli::archive::ResidentArchive;
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, EntryId, Replayed};
use ape_cli::lineage::{self, Decision, Lineage, Taken};

use crate::world::{self, CANCELLING, Constructed};

/// One thing that happened, in the order it happened.
#[derive(Debug, Clone)]
pub enum Step {
    /// The house intends to move `magnitude` on the account by `due`.
    Intend {
        magnitude: f64,
        incoming: bool,
        due: u8,
        recorded_at: u8,
    },
    /// It was observed that intention `which` will not happen.
    Cancel { which: usize, at: u8 },
    /// A world is opened at `known_at`, proposing intentions `select`.
    Open { known_at: u8, select: Vec<usize> },
    /// The current world is carried to `known_at`, recognizing what became known.
    Carry { known_at: u8 },
    /// The current world is replaced by one that drops `omit` and adds `introduce`.
    Add {
        omit: Vec<usize>,
        introduce: Vec<usize>,
    },
}

/// What a sequence produced: the two records, and the handles the harness needs.
pub struct Built {
    pub canon: Canon<ResidentHistory>,
    pub world: Constructed,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub taken: Vec<Taken>,
    pub lineage: Lineage,
    pub intentions: Vec<CommitmentId>,
    pub worlds: Vec<ThesisId>,
}

impl Built {
    /// The worlds by identity, as Synthesis and a walk both read them.
    pub fn archive(&self) -> &ResidentArchive {
        self.lineage.archive()
    }

    /// The world current at the end of the sequence.
    pub fn current(&self) -> ThesisId {
        *self.worlds.last().expect("the sequence opens a world")
    }

    /// Whether the journal holds the entry an address names.
    pub fn witnessed(&self, entry: &EntryId) -> bool {
        self.admitted.entries.contains(entry)
    }

    /// Resolve a world the sequence produced.
    pub fn world_at(&self, id: ThesisId) -> Thesis {
        self.archive()
            .thesis(id)
            .expect("the archive holds every world the lineage decided")
    }
}

/// The world, with nothing decided about it.
pub fn nothing_decided() -> Built {
    replay(&[])
}

/// What happened.
pub fn scenario() -> Vec<Step> {
    vec![
        Step::Intend {
            magnitude: 120.0,
            incoming: false,
            due: 8,
            recorded_at: 6,
        },
        Step::Open {
            known_at: 6,
            select: vec![0],
        },
        Step::Intend {
            magnitude: 30.0,
            incoming: false,
            due: 14,
            recorded_at: 6,
        },
        Step::Cancel { which: 0, at: 6 },
        Step::Carry { known_at: 6 },
        Step::Add {
            omit: vec![],
            introduce: vec![1],
        },
        Step::Intend {
            magnitude: 90.0,
            incoming: false,
            due: 20,
            recorded_at: 9,
        },
        Step::Carry { known_at: 12 },
        Step::Add {
            omit: vec![],
            introduce: vec![2],
        },
    ]
}

pub fn build() -> Built {
    replay(&scenario())
}

/// Fold a sequence of steps into the records it describes.
pub fn replay(steps: &[Step]) -> Built {
    let mut canon = Canon::new(ResidentHistory::default());
    let world = world::construct(&mut canon).expect("the world is admissible");

    let mut journal = world.journal.clone();
    let mut admitted = world.admitted.clone();
    let mut lineage = Lineage::new();

    let mut intentions: Vec<CommitmentId> = Vec::new();
    let mut worlds: Vec<ThesisId> = Vec::new();
    let mut taken: Vec<Taken> = Vec::new();

    for step in steps {
        match step {
            Step::Intend {
                magnitude,
                incoming,
                due,
                recorded_at,
            } => {
                journal.push(world::intention(
                    &world,
                    *magnitude,
                    *incoming,
                    *due,
                    *recorded_at,
                ));
                admit(&mut canon, &journal, &mut admitted);

                intentions.push(
                    *admitted
                        .commitments
                        .last()
                        .expect("an intention was just admitted"),
                );
            }

            Step::Cancel { which, at } => {
                journal.push(Admission::Event {
                    commitment: intentions[*which],
                    observation: CANCELLING.into(),
                    occurred_at: world::day(*at),
                    recorded_at: world::day(*at),
                });
                admit(&mut canon, &journal, &mut admitted);
            }

            Step::Open { known_at, select } => decide(
                Decision::Genesis {
                    known_at: world::day(*known_at),
                    selection: select.iter().map(|at| intentions[*at]).collect(),
                },
                &canon,
                &world,
                &admitted,
                &mut lineage,
                &mut taken,
                &mut worlds,
            ),

            Step::Carry { known_at } => decide(
                Decision::Advance {
                    extends: *worlds.last().expect("a world must be open to be carried"),
                    known_at: world::day(*known_at),
                },
                &canon,
                &world,
                &admitted,
                &mut lineage,
                &mut taken,
                &mut worlds,
            ),

            Step::Add { omit, introduce } => decide(
                Decision::Fork {
                    extends: *worlds.last().expect("a world must be open to be added to"),
                    omitted: omit.iter().map(|at| intentions[*at]).collect(),
                    introduced: introduce.iter().map(|at| intentions[*at]).collect(),
                },
                &canon,
                &world,
                &admitted,
                &mut lineage,
                &mut taken,
                &mut worlds,
            ),
        }
    }

    Built {
        canon,
        world,
        journal,
        admitted,
        taken,
        lineage,
        intentions,
        worlds,
    }
}

fn admit(canon: &mut Canon<ResidentHistory>, journal: &[Admission], admitted: &mut Replayed) {
    journal::replay_remaining(canon, journal, admitted).expect("the journal is admissible");
}

/// Take a decision, record it against the entry that stood, and keep the world it produced.
///
/// The two happen together on purpose. A decision and the coordinate it was taken at are one
/// record, and writing them at different moments is how a lineage comes to read back as a
/// different lineage.
fn decide(
    decision: Decision,
    canon: &Canon<ResidentHistory>,
    world: &Constructed,
    admitted: &Replayed,
    lineage: &mut Lineage,
    taken: &mut Vec<Taken>,
    worlds: &mut Vec<ThesisId>,
) {
    taken.push(
        Taken::claimed(decision.clone(), world.house, admitted)
            .expect("something was admitted before anything was decided"),
    );

    lineage::decide(canon.history(), lineage, &decision).expect("the decision is takeable");

    worlds.push(
        lineage
            .decided()
            .last()
            .expect("a decision produces a world")
            .id(),
    );
}
