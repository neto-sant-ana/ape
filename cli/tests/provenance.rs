//! The provenance experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is that half the experiment
//! asks whether anything needs to be built at all, so a phase may end by recording that a
//! question already has an answer — or that it has no content.

use std::collections::BTreeSet;

use ape::engine::synthesis::{ApplicabilityStatus, synthesize};
use ape::engine::thesis::{ThesisId, descends_from};
use ape::kernel::value_objects::Date;

use ape_cli::lineage::{self, Decision, Lineage};
use ape_cli::reading::{self, ConflictRecord, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli::subject::provenance::{self, Adopted};
use ape_cli::transfer::{self, Applicability, StatusRecord};

/// The instant every world is interpreted at, past every deadline the subject carries.
const EFFECTIVE: &str = "2026-01-28";

fn adopted() -> Adopted {
    provenance::adopted().expect("the arrangement holds")
}

/// Every (Base, Source) pair that would produce `world` by forking the world it extends.
///
/// The Target is **not** a free variable. A decision records which world it extends, so a
/// transfer that produced this one had that world as its Target — and a fork of any other Target
/// produces a different world, because a parent is part of an identity. Leaving the Target open
/// counts transfers that reach the same *selection*, which is a different question.
fn explanations(arrangement: &Adopted, world: usize) -> Vec<(usize, usize)> {
    let lineage = &arrangement.lineage;
    let archive = lineage.archive();
    let knowledge = arrangement.canon.history();

    let worlds: Vec<ThesisId> = lineage.decided().iter().map(|held| held.id()).collect();

    let target = lineage.decided()[world]
        .parent()
        .expect("the world was produced by a fork");

    let produced: BTreeSet<String> = lineage.decided()[world]
        .selection()
        .open()
        .map(|id| id.to_string())
        .collect();

    let mut found = Vec::new();

    for (at_base, base) in worlds.iter().enumerate() {
        for (at_source, source) in worlds.iter().enumerate() {
            let coherent = descends_from(archive, *source, *base).expect("ancestry walks")
                && descends_from(archive, target, *base).expect("ancestry walks");

            if !coherent {
                continue;
            }

            let report = synthesize(archive, knowledge, *base, *source, target)
                .expect("a coherent Base was checked first");

            if let StatusRecord::Applicable { candidate, .. } = Applicability::of(&report).status
                && candidate.open == produced
            {
                found.push((at_base, at_source));
            }
        }
    }

    found
}

/// Phase 1 — Ambiguity.
///
/// The arrangement has to offer more than one **non-degenerate** account of how a world came to
/// hold what it holds, or the necessity half of this experiment is over before it starts.
///
/// Degenerate means the Source is the world being explained, or descends from it: those are the
/// answer read backwards rather than a rival account of how it was reached.
///
/// What this phase establishes is narrower than it may look, and the distinction is the
/// experiment's: it shows that **search does not recover** which line an intention came from. It
/// does not show that anyone needs to know. Those are separate claims and Phase 2 takes the
/// second.
#[test]
fn phase_1_ambiguity() {
    let arrangement = adopted();
    let subject = &arrangement.subject;

    assert_eq!(arrangement.lineage.decided().len(), 5);

    // The two plans agree about the tooling and disagree about the expansion. Neither withdraws
    // anything, so the disagreement is in what they *introduce* — two substantive requests rather
    // than one request and one no-op.
    assert_eq!(
        arrangement
            .narrow()
            .selection()
            .open()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.tooling])
    );
    assert_eq!(
        arrangement
            .broad()
            .selection()
            .open()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.tooling, subject.expansion]),
        "the broad plan wants the expansion too"
    );
    assert_eq!(
        arrangement
            .receiving()
            .selection()
            .open()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.expansion, subject.grant]),
        "and the receiving line had decided on that expansion for its own reasons"
    );

    let adopting = arrangement.adopting();

    assert_eq!(
        adopting.parent(),
        &Some(arrangement.receiving().id()),
        "the transfer was carried into the receiving line"
    );
    assert_eq!(
        adopting.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            subject.funding,
            subject.tooling,
            subject.expansion,
            subject.grant
        ])
    );

    // As intentions over the ancestor the two plans differ. As transfers into the receiving line
    // they are one request: a resolved transfer drops what the Target already holds, because
    // introducing it again asks nothing of it.
    let ask = |source: ThesisId| {
        Applicability::of(
            &synthesize(
                arrangement.lineage.archive(),
                arrangement.canon.history(),
                arrangement.ancestor().id(),
                source,
                arrangement.receiving().id(),
            )
            .expect("the ancestor is a coherent Base"),
        )
    };

    let from_narrow = ask(arrangement.narrow().id());
    let from_broad = ask(arrangement.broad().id());

    assert_eq!(
        from_narrow.introduced,
        BTreeSet::from([subject.tooling.to_string()])
    );
    assert_eq!(
        from_broad.introduced,
        BTreeSet::from([subject.tooling.to_string(), subject.expansion.to_string()]),
        "as intentions over the ancestor, the two differ in what they ask for"
    );
    assert_eq!(
        from_narrow.status, from_broad.status,
        "and as transfers into the receiving line, they are one request"
    );

    // The two plans do not have the same standing. One is a world the account refuses.
    let readings = reading::all(
        arrangement.canon.history(),
        arrangement.lineage.decided(),
        subject.instance,
        &Date::parse(EFFECTIVE).expect("a real date"),
    )
    .expect("every world reads");

    assert_eq!(
        readings[2].conflicts,
        vec![ConflictRecord::OutOfBounds {
            instance: subject.instance.to_string(),
            level: -5.0,
        }],
        "40 − 15 − 30 is outside the account's bounds"
    );

    for (position, label) in [
        (1, "the narrow plan"),
        (3, "the receiving line"),
        (4, "the world the transfer produced"),
    ] {
        assert!(
            readings[position].conflicts.is_empty(),
            "{label} is feasible, found {:?}",
            readings[position].conflicts
        );
    }

    // The count, with the Target pinned by what the record says the decision extends.
    let found = explanations(&arrangement, 4);

    assert_eq!(
        found,
        [(0, 1), (0, 2), (0, 4), (3, 4)],
        "every Base and Source that would produce this world"
    );

    let non_degenerate: Vec<_> = found
        .iter()
        .filter(|(_, source)| {
            !descends_from(
                arrangement.lineage.archive(),
                arrangement.lineage.decided()[*source].id(),
                adopting.id(),
            )
            .expect("ancestry walks")
        })
        .copied()
        .collect();

    assert_eq!(
        non_degenerate,
        [(0, 1), (0, 2)],
        "two rival accounts survive the rule convergence proposed and did not test"
    );

    // One of which the account refuses and the other of which it does not. Whether that
    // difference reaches the world they both explain is Phase 2's question, and nothing here
    // answers it: what travelled is a commitment identity, and an identity carries no origin.
    assert_eq!(non_degenerate[0].1, 1, "the narrow plan explains it");
    assert_eq!(non_degenerate[1].1, 2, "and so does the refused one");
}

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-provenance-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

fn persist(repository: &Repository, arrangement: &Adopted) {
    repository
        .write_journal(&arrangement.journal)
        .expect("writable");
    repository
        .write_lineage(&arrangement.decisions)
        .expect("writable");
    repository
        .write_worlds(
            &arrangement
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("writable");
}

/// Phase 2 — Exhaust what the record already answers.
///
/// Every question a reader might ask about the adopted world, asked *of the repository* rather
/// than settled by argument. Four of the five have answers. The fifth is the experiment's, and
/// this phase establishes two things about it that no amount of recording could change.
#[test]
fn phase_2_exhaust() {
    let arrangement = adopted();
    let subject = &arrangement.subject;

    let repository = Repository::open(scratch("phase-2"));
    persist(&repository, &arrangement);

    let readings = reading::reconstruct(
        &repository,
        subject.instance,
        &Date::parse(EFFECTIVE).expect("a real date"),
    )
    .expect("the repository reconstructs");

    let adopting = &readings[4];
    let recorded = repository.read_lineage().expect("the lineage reads back");

    // What does this world select — derived, and the repository produces it.
    assert_eq!(
        adopting.open,
        BTreeSet::from([
            subject.funding.to_string(),
            subject.tooling.to_string(),
            subject.expansion.to_string(),
            subject.grant.to_string(),
        ])
    );

    // What did the decision ask for, which world did it extend, and when was it taken — recorded,
    // and each is read back rather than inferred.
    let taken = &recorded[4];

    let Decision::Fork {
        extends,
        omitted,
        introduced,
    } = &taken.decision
    else {
        panic!("the fifth decision is a fork, found {:?}", taken.decision);
    };

    assert!(omitted.is_empty(), "it withdrew nothing");
    assert_eq!(
        introduced,
        &BTreeSet::from([subject.tooling]),
        "and asked for the tooling"
    );
    assert_eq!(
        *extends,
        arrangement.receiving().id(),
        "over the receiving line"
    );
    assert_eq!(
        taken.after, arrangement.decisions[0].after,
        "at the point in the journal every decision here was taken at"
    );

    // Which line the intention came from — and the first reason no record of the world could say.
    //
    // A planner who never consulted anybody and decided the same fork produces a decision that is
    // *the same record*, field for field. A transfer applied is not a kind of decision; it is a
    // decision that happens to have been chosen with a report in hand.
    let independently = transfer::applied(
        arrangement.receiving().id(),
        match arrangement.carried.status() {
            ApplicabilityStatus::Applicable { transfer, .. } => transfer,
            other => panic!("the carried transfer is applicable, found {other:?}"),
        },
    );

    let deliberate = Decision::Fork {
        extends: arrangement.receiving().id(),
        omitted: BTreeSet::new(),
        introduced: BTreeSet::from([subject.tooling]),
    };

    assert_eq!(
        serde_json::to_string(&independently).expect("a decision serializes"),
        serde_json::to_string(&deliberate).expect("a decision serializes"),
        "a transfer carried and an independent decision are one record"
    );

    // And the second reason, which is about worlds rather than records: deciding it independently
    // produces the **same world**. Identity is derived from content, so two routes to one content
    // are not two worlds — there is no place on a world for where it came from to live.
    let mut alone = Lineage::new();
    for decision in [
        provenance::genesis(subject.funding),
        provenance::narrow(arrangement.ancestor().id(), subject.tooling),
        provenance::broad(
            arrangement.ancestor().id(),
            subject.tooling,
            subject.expansion,
        ),
        provenance::receiving(
            arrangement.ancestor().id(),
            subject.expansion,
            subject.grant,
        ),
        deliberate.clone(),
    ] {
        lineage::decide(arrangement.canon.history(), &mut alone, &decision)
            .expect("each decision applies");
    }

    assert_eq!(
        alone.decided()[4].id(),
        arrangement.adopting().id(),
        "the world a transfer produced and the world a planner produced are one world"
    );

    // So the standing asymmetry Phase 1 raised does not reach it either, and this is why rather
    // than an argument that it does not. The three commitments that make the broad plan
    // infeasible are *inside* the adopted world, and the adopted world is feasible.
    let refused: BTreeSet<_> = arrangement.broad().selection().resolved().collect();
    let accepted: BTreeSet<_> = arrangement.adopting().selection().resolved().collect();

    assert!(
        refused.is_subset(&accepted),
        "everything the refused plan selects, the adopted world selects too"
    );
    assert!(
        !readings[2].conflicts.is_empty(),
        "and that plan is refused"
    );
    assert!(
        adopting.conflicts.is_empty(),
        "while the world containing all of it is not"
    );

    // Which says what infeasibility is: a property of a selection, not of a member of one. So
    // nothing about the refused plan can travel with a commitment out of it, because the
    // commitment never carried it.

    // The relation runs one way only, and that is where anything provenance could be has to live.
    // A *different* record can produce the identical world: asking again for the expansion the
    // receiving line already holds is a request that changes nothing, so the fork lands on the
    // same selection under a different intention.
    let verbose = Decision::Fork {
        extends: arrangement.receiving().id(),
        omitted: BTreeSet::new(),
        introduced: BTreeSet::from([subject.tooling, subject.expansion]),
    };

    assert_ne!(
        serde_json::to_string(&verbose).expect("a decision serializes"),
        serde_json::to_string(&deliberate).expect("a decision serializes"),
        "two records"
    );

    let mut redundantly = Lineage::new();
    for decision in [
        provenance::genesis(subject.funding),
        provenance::receiving(
            arrangement.ancestor().id(),
            subject.expansion,
            subject.grant,
        ),
    ] {
        lineage::decide(arrangement.canon.history(), &mut redundantly, &decision)
            .expect("each decision applies");
    }
    lineage::decide(arrangement.canon.history(), &mut redundantly, &verbose)
        .expect("a redundant introduction is tolerated");

    assert_eq!(
        redundantly.decided()[2].id(),
        arrangement.adopting().id(),
        "and one world"
    );

    // So a record can say more than a world can hold. Whatever provenance is, it is an annotation
    // on a decision — there is no room for it on the thing the decision produced.
}

/// Pinning the Target is not bookkeeping, and this is what it changes.
///
/// A transfer whose Target is some other world can reach the same **selection** and cannot reach
/// the same **world**: a parent is part of an identity, so forking a different Target produces a
/// different Thesis holding exactly the same commitments.
///
/// The convergence experiment counted without pinning the Target, and its Observation 8 reports
/// three transfers producing one world where the count of transfers that produce that *world* is
/// smaller. Recorded here rather than corrected there.
#[test]
fn a_candidate_is_not_a_world() {
    let arrangement = adopted();

    let elsewhere = Applicability::of(
        &synthesize(
            arrangement.lineage.archive(),
            arrangement.canon.history(),
            arrangement.ancestor().id(),
            arrangement.adopting().id(),
            arrangement.narrow().id(),
        )
        .expect("the ancestor is a coherent Base"),
    );

    let StatusRecord::Applicable { candidate, .. } = &elsewhere.status else {
        panic!(
            "expected an applicable transfer, found {:?}",
            elsewhere.status
        );
    };

    assert_eq!(
        candidate.open,
        arrangement
            .adopting()
            .selection()
            .open()
            .map(|id| id.to_string())
            .collect::<BTreeSet<_>>(),
        "a transfer into another Target reaches the same selection"
    );

    // And the world it would build is not the same world, because the Target it extends is part
    // of what the identity is derived from. So a count that leaves the Target open counts
    // transfers that produce a selection, not transfers that produce a world.
    assert_ne!(
        arrangement.narrow().id(),
        arrangement.receiving().id(),
        "two different Targets"
    );
    assert!(
        !explanations(&arrangement, 4).contains(&(0, 3)),
        "and the receiving line is not among the accounts of the world it produced"
    );
}
