//! The world two parties share, and the repository they will both decide against.
//!
//! ```text
//! cash ∈ [floor, 1000]
//! account                the instance every intention moves
//! opening      +100      received and settled, the only money the account has held
//! standing      −20      an arrangement with the market, intended and not yet observed
//!
//! base                   one world, at the 6th, selecting the standing arrangement
//! ```
//!
//! Extended from the world the first three experiments used rather than rebuilt, so that the
//! arrangement two parties meet in is not one chosen to suit them. What is new is the standing
//! arrangement — something already intended, which a party may leave alone or stand down — and two
//! more Agents.
//!
//! # The parties, and why there are two more Agents
//!
//! `operations` and `finance` both spend from the account. Neither is an agent in the other sense:
//! an LLM acting here decides *for* one of them and has no representation of its own, so what a
//! decision records is the party, and what the party records is a name that resolves against
//! knowledge rather than a claim about who operated the session.
//!
//! A commitment still says the house owes the market. Who owes whom and who decided are different
//! questions, and this world keeps them in different places on purpose.
//!
//! # The base claims nobody
//!
//! [`Taken::now`] rather than [`Taken::claimed`], because the base is the world as it stood before
//! either party thought anything. That leaves a reader unable to tell *unclaimed* from *not this
//! party's* — which is what an optional field costs, is already measured, and is pre-registered
//! here as the shape the merge will be read through.
//!
//! # What is deliberately absent
//!
//! Neither party's option. A line is a fork of the base, and building one here would answer the
//! first question the experiment asks. The suite builds two *probes* to show the base admits two
//! independent lines; a probe is not a choice, and the record says which is which.

use std::collections::BTreeSet;
use std::path::Path;

use ape::canon::Canon;
use ape::engine::thesis::ThesisId;
use ape::kernel::entities::{AgentId, CommitmentId};

use ape_cli::converge;
use ape_cli::error::{JournalError, RepositoryError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, Replayed, ResourceKindRecord};
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::Repository;

use crate::world::{self, Constructed, Intention};

/// The instant the base recognizes.
pub const OPENED: u8 = 6;

/// When the two parties enter the record.
///
/// After the opening receipt settled, and not because either party arrived late. A Canon refuses
/// knowledge recorded before knowledge it already holds, so a world assembled in two pieces has to
/// admit the second piece after the first — which is a property of how this fixture is built and
/// of nothing in the scenario.
pub const NAMED: u8 = 3;

/// What the standing arrangement costs and when it comes due.
pub const STANDING: f64 = 20.0;
pub const STANDING_DUE: u8 = 10;

/// The world, the parties, and the one world already decided about it.
pub struct Shared {
    pub canon: Canon<ResidentHistory>,
    pub world: Constructed,
    /// The party the delivery side of the operation decides for.
    pub operations: AgentId,
    /// The party the account side decides for.
    pub finance: AgentId,
    /// An arrangement with the market, intended and not yet observed.
    pub standing: CommitmentId,
    pub base: ThesisId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub lineage: Lineage,
    pub decisions: Vec<Taken>,
}

impl Shared {
    /// Write what a party would read.
    pub fn write(&self, repository: &Repository) -> Result<(), RepositoryError> {
        repository.write_journal(&self.journal)?;
        repository.write_lineage(&self.decisions)?;
        repository.write_worlds(
            &self
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
    }
}

/// The shared world under the constraint the first three experiments ran against.
pub fn shared() -> Shared {
    under(world::cash())
}

/// The same world under a constraint the caller names.
///
/// The parameter is the whole of what the second half of the experiment changes: the same parties
/// make the same choices, and a reserve the account must keep decides whether the two of them
/// together describe something that can happen.
pub fn under(cash: ResourceKindRecord) -> Shared {
    let mut canon = Canon::new(ResidentHistory::default());
    let world = world::with_cash(&mut canon, cash).expect("the world is admissible");

    let mut journal = world.journal.clone();
    let mut admitted = world.admitted.clone();

    journal.extend([
        Admission::Agent {
            label: "operations".into(),
            recorded_at: world::day(NAMED),
        },
        Admission::Agent {
            label: "finance".into(),
            recorded_at: world::day(NAMED),
        },
    ]);
    admit(&mut canon, &journal, &mut admitted);

    let operations = named(&admitted, 2);
    let finance = named(&admitted, 3);

    journal.extend([
        Admission::Eligibility {
            agent: operations,
            roles: [world.spender].into(),
            effective_from: world::day(NAMED),
            recorded_at: world::day(NAMED),
        },
        Admission::Eligibility {
            agent: finance,
            roles: [world.spender].into(),
            effective_from: world::day(NAMED),
            recorded_at: world::day(NAMED),
        },
    ]);
    admit(&mut canon, &journal, &mut admitted);

    journal.push(world::intention(
        &world,
        Intention {
            magnitude: STANDING,
            incoming: false,
            due: STANDING_DUE,
            recorded_at: 5,
            dependencies: [].into(),
        },
    ));
    admit(&mut canon, &journal, &mut admitted);

    let standing = *admitted
        .commitments
        .last()
        .expect("the standing arrangement was just admitted");

    let mut lineage = Lineage::new();

    let genesis = Decision::Genesis {
        known_at: world::day(OPENED),
        selection: [standing].into(),
    };

    let decisions = vec![
        Taken::now(genesis.clone(), &admitted)
            .expect("the world was admitted before it was opened"),
    ];

    lineage::decide(canon.history(), &mut lineage, &genesis).expect("the base is takeable");

    let base = lineage
        .decided()
        .last()
        .expect("a decision produces a world")
        .id();

    Shared {
        canon,
        world,
        operations,
        finance,
        standing,
        base,
        journal,
        admitted,
        lineage,
        decisions,
    }
}

/// A fork of the base, in the two halves every decision has.
///
/// Not a party's choice. This is what the suite needs in order to show that the base admits two
/// independent lines, and what Phase B needs in order to put the choices the parties *did* make
/// into a world they were not made in.
pub struct Line {
    pub omitted: BTreeSet<CommitmentId>,
    pub introduced: BTreeSet<CommitmentId>,
}

impl Line {
    pub fn from(&self, base: ThesisId) -> Decision {
        Decision::Fork {
            extends: base,
            omitted: self.omitted.clone(),
            introduced: self.introduced.clone(),
        }
    }
}

/// Admit an intention against the shared world and hand back its identity.
pub fn intend(shared: &mut Shared, intention: Intention) -> CommitmentId {
    shared
        .journal
        .push(world::intention(&shared.world, intention));

    admit(&mut shared.canon, &shared.journal, &mut shared.admitted);

    *shared
        .admitted
        .commitments
        .last()
        .expect("an intention was just admitted")
}

/// Recognize a later instant as a party, which adopts nothing.
///
/// Needed wherever a party's intention was recorded after the cut its world holds, since a fork
/// inherits its parent's cut and cannot select what that cut could not have known.
pub fn carry(shared: &mut Shared, from: ThesisId, known_at: u8, by: AgentId) -> ThesisId {
    taken(
        shared,
        Decision::Advance {
            extends: from,
            known_at: world::day(known_at),
        },
        by,
    )
}

/// Take a line as a party's decision, and keep the world it produced.
pub fn decide(shared: &mut Shared, line: &Line, from: ThesisId, by: AgentId) -> ThesisId {
    taken(shared, line.from(from), by)
}

fn taken(shared: &mut Shared, decision: Decision, by: AgentId) -> ThesisId {
    shared.decisions.push(
        Taken::claimed(decision.clone(), by, &shared.admitted)
            .expect("something was admitted before anything was decided"),
    );

    lineage::decide(shared.canon.history(), &mut shared.lineage, &decision)
        .expect("the line is takeable");

    shared
        .lineage
        .decided()
        .last()
        .expect("a decision produces a world")
        .id()
}

/// The runs whose recorded repositories the later phases are built from.
pub const OPERATIONS: &str = "run-a";
pub const FINANCE: &str = "run-b-prime";

/// A repository as one of the parties left it.
///
/// Read as data. The admission a party appended and the decision it took come out of these files, so
/// nothing downstream is the experimenter's account of what a party chose.
pub fn recorded(run: &str) -> Repository {
    Repository::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("04-multiagent")
            .join(run)
            .join("repo"),
    )
}

/// A writable copy of one, since converging writes.
pub fn copied(run: &str, into: impl AsRef<Path>) -> std::io::Result<Repository> {
    let into = into.as_ref();

    std::fs::create_dir_all(into)?;

    for file in ["journal.json", "lineage.json", "worlds.json"] {
        std::fs::copy(recorded(run).root().join(file), into.join(file))?;
    }

    Ok(Repository::open(into))
}

/// What a party appended to the journal it read, and every decision it claimed, in file order.
pub fn contributed(run: &str, party: AgentId) -> (Vec<Admission>, Vec<Taken>) {
    let repository = recorded(run);

    let journal = repository.read_journal().expect("readable");
    let founded = shared().journal.len();

    let claimed = repository
        .read_lineage()
        .expect("readable")
        .into_iter()
        .filter(|taken| taken.by == Some(party))
        .collect();

    (journal[founded..].to_vec(), claimed)
}

/// Re-admit one party's contribution on top of what a repository already holds.
///
/// The repair a diverged party is given, applied verbatim: the knowledge is admitted again, and the
/// decisions are re-witnessed at the coordinate the merged journal gives.
///
/// It has an order, and the error is how the caller finds out: a Canon refuses an admission dated
/// before what it has recorded through, so the party whose knowledge is dated earlier lands first.
pub fn re_admit(
    held: &mut Corroborated,
    admissions: Vec<Admission>,
    claimed: &[Taken],
    party: AgentId,
) -> Result<(), JournalError> {
    held.journal.extend(admissions);
    journal::replay_remaining(&mut held.canon, &held.journal, &mut held.admitted)?;

    for taken in claimed {
        let decision = taken.decision.clone();

        held.decisions.push(
            Taken::claimed(decision.clone(), party, &held.admitted)
                .expect("re-witnessed where the merged journal puts it"),
        );
        lineage::decide(held.canon.history(), &mut held.lineage, &decision)
            .expect("the party's decision is takeable here too");
    }

    Ok(())
}

/// Both parties' lines in one repository, in the only order the dates admit.
///
/// The repository must already hold the earlier-dated party's line, which [`copied`] from [`FINANCE`]
/// supplies.
pub fn merge(repository: &Repository) -> Corroborated {
    let shared = shared();

    let mut held = reading::corroborated(repository).expect("reconstructs");
    let (admissions, claimed) = contributed(OPERATIONS, shared.operations);

    re_admit(&mut held, admissions, &claimed, shared.operations)
        .expect("the later party's knowledge admits onto the earlier party's");

    converge::converge(repository, &held).expect("both lines converge");

    reading::corroborated(repository).expect("the merged repository reconstructs")
}

/// The world the genesis produced, checked to be the world this experiment founded.
pub fn base_of(read: &Corroborated) -> ThesisId {
    let found = read
        .decisions
        .iter()
        .zip(read.lineage.decided())
        .find(|(taken, _)| taken.decision.extends().is_none())
        .map(|(_, world)| world.id())
        .expect("a repository holds a genesis");

    assert_eq!(
        found,
        shared().base,
        "this repository is not the shared world the experiment founded"
    );

    found
}

/// The last world a party claimed, where it claimed any.
pub fn tip_of(read: &Corroborated, party: AgentId) -> Option<ThesisId> {
    read.decisions
        .iter()
        .zip(read.lineage.decided())
        .filter(|(taken, _)| taken.by == Some(party))
        .map(|(_, world)| world.id())
        .next_back()
}

fn admit(canon: &mut Canon<ResidentHistory>, journal: &[Admission], admitted: &mut Replayed) {
    journal::replay_remaining(canon, journal, admitted).expect("the journal is admissible");
}

fn named(admitted: &Replayed, at: usize) -> AgentId {
    admitted.agents[at]
}
