//! Phase 0 — what must be true of the shared world before either party is let near it.
//!
//! Two things, and the second is the one that is easy to skip. That the world holds together, and
//! that **it holds nothing of the experiment yet**: one world, claimed by nobody, ancestor of
//! nothing, with neither party's option anywhere in it.
//!
//! The probes are the point of care. A line has to be constructible for the run to mean anything,
//! and constructing one here would be choosing for a party. So each probe is built with a
//! magnitude nothing in the experiment uses, is asserted against the *shape* a line has rather
//! than against a choice, and is never written. What it proves is that the base admits two
//! independent forks — not what either fork will say.

use std::collections::BTreeSet;

use ape::engine::hermeneia::Hypothesis;
use ape::engine::thesis::{Interpretation, ThesisId, ThesisLookup, descends_from};

use ape_cli::history::ResidentHistory;
use ape_cli::lineage::{self, Lineage};
use ape_cli::reading::{self, Corroborated};
use ape_cli::repository::Repository;

use ape_agents::coordination::{self, Line, Shared};
use ape_agents::policy::{self, Verdict};
use ape_agents::world::Intention;

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-agents-04-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// The shared world, written where a party would read it.
fn written(name: &str) -> (Repository, Shared) {
    let repository = Repository::open(scratch(name));
    let shared = coordination::shared();

    shared.write(&repository).expect("writable");

    (repository, shared)
}

/// A magnitude no objective in the experiment names, so that a probe cannot be mistaken for one.
const PROBE: f64 = 7.0;

/// One world, claimed by nobody, and nothing else decided.
#[test]
fn the_base_is_the_only_thing_decided_and_it_claims_nobody() {
    let (repository, shared) = written("base");

    let decisions = repository.read_lineage().expect("readable");

    assert_eq!(
        decisions.len(),
        1,
        "one world, decided before either party looked"
    );
    assert_eq!(
        decisions[0].by, None,
        "the base is the world as it stood, and no party decided it"
    );

    let read = reading::corroborated(&repository).expect("the repository reconstructs");

    assert_eq!(
        read.lineage.decided().len(),
        1,
        "reading it back produces the one world and no other"
    );
    assert_eq!(
        read.lineage.decided()[0].id(),
        shared.base,
        "the world read back is the world written, by identity"
    );
}

/// The base imposes the settled receipt and selects the standing arrangement.
#[test]
fn the_base_holds_one_settled_fact_and_one_open_intention() {
    let (repository, shared) = written("selection");

    let read = reading::corroborated(&repository).expect("the repository reconstructs");
    let base = read
        .lineage
        .archive()
        .thesis(shared.base)
        .expect("the archive holds the base");

    assert!(
        base.selection().is_frozen(shared.world.opening),
        "the receipt was settled, so the base could not have left it out"
    );
    assert!(
        base.selection().contains(shared.standing),
        "the base selects the standing arrangement, which is what gives a party something to drop"
    );
    assert!(
        !base.selection().is_frozen(shared.standing),
        "and it is intended rather than settled, so standing it down is available"
    );
    assert_eq!(base.selection().len(), 2);

    let interpretation =
        Interpretation::of(&base, read.canon.history()).expect("the base is interpretable");

    assert_eq!(
        policy::rule(&interpretation, Hypothesis::FinalState).expect("feasibility is derivable"),
        Verdict::MayProceed,
        "100 received and 20 intended out clears the floor, so nothing is decided by the base being tight"
    );
}

/// Nothing extends the base, which is what makes it a base rather than a position in a line.
#[test]
fn the_base_is_an_ancestor_of_nothing_yet() {
    let (repository, shared) = written("ancestor");

    let read = reading::corroborated(&repository).expect("the repository reconstructs");
    let archive = read.lineage.archive();

    for world in read.lineage.decided() {
        if world.id() == shared.base {
            continue;
        }

        assert!(
            !descends_from(archive, world.id(), shared.base).expect("ancestry is walkable"),
            "the repository holds a world descending from the base before either party decided"
        );
    }

    assert!(
        read.decisions
            .iter()
            .all(|taken| taken.decision.extends().is_none()),
        "no decision in the repository extends another"
    );
}

/// Two lines of the shapes the world offers are constructible, and neither is a party's choice.
///
/// The shapes are the two things the base makes available: leaving the standing arrangement alone
/// and adding to it, or standing it down and putting something else in its place. Which shape
/// either party reaches for is the measurement, and is not asserted here.
#[test]
fn the_base_admits_two_independent_lines() {
    let (repository, shared) = written("probes");

    let mut read = reading::corroborated(&repository).expect("the repository reconstructs");

    let additive = probe(&mut read, &shared, 12);
    let replacing = probe(&mut read, &shared, 18);

    let lines = [
        Line {
            omitted: BTreeSet::new(),
            introduced: [additive].into(),
        },
        Line {
            omitted: [shared.standing].into(),
            introduced: [replacing].into(),
        },
    ];

    let mut taken: Vec<ThesisId> = Vec::new();

    for line in &lines {
        taken.push(forked(
            &mut read.lineage,
            read.canon.history(),
            line,
            shared.base,
        ));
    }

    assert_eq!(taken.len(), 2);
    assert_ne!(
        taken[0], taken[1],
        "two lines, and not the same world twice"
    );

    let archive = read.lineage.archive();

    for line in &taken {
        assert!(
            descends_from(archive, *line, shared.base).expect("ancestry is walkable"),
            "each line extends the base, which is what makes the base their common ancestor"
        );
        assert!(
            !descends_from(archive, taken[0], taken[1]).expect("ancestry is walkable"),
            "and neither extends the other"
        );
    }

    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        1,
        "the probes were built and not written — the repository still holds only the base"
    );
}

/// An intention nothing in the experiment asked for, admitted against the reconstructed world.
fn probe(read: &mut Corroborated, shared: &Shared, due: u8) -> ape::kernel::entities::CommitmentId {
    let admission = ape_agents::world::intention(
        &shared.world,
        Intention {
            magnitude: PROBE,
            incoming: false,
            due,
            recorded_at: coordination::OPENED,
            dependencies: BTreeSet::new(),
        },
    );

    read.journal.push(admission);

    ape_cli::journal::replay_remaining(&mut read.canon, &read.journal, &mut read.admitted)
        .expect("the journal is admissible");

    *read
        .admitted
        .commitments
        .last()
        .expect("a probe was just admitted")
}

fn forked(
    lineage: &mut Lineage,
    knowledge: &ResidentHistory,
    line: &Line,
    base: ThesisId,
) -> ThesisId {
    lineage::decide(knowledge, lineage, &line.from(base))
        .expect("a line of this shape is takeable");

    lineage
        .decided()
        .last()
        .expect("a decision produces a world")
        .id()
}
