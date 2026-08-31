//! Experiment 18 — Designation. Phases against `lab/frontier/docs/18-designation/00-protocol.md`.
//!
//! The question: a record's identity is derived from its content, and a reference is a thing that
//! changes. Can a record say which of its worlds it means — and where does that claim live?
//!
//! **Built** is settled before any phase runs, because the protocol's Phase 2 says *B, built* and
//! the row's rules say an experiment may not change the application. Both hold: a shape is built
//! **here**, in the laboratory, against the repository as it stands. Whether any of it becomes
//! application behaviour is a separate reviewed act, after Phase 5, and it is the obligation the
//! protocol's fourth criterion is about.

use ape::kernel::entities::ResourceId;

use ape_cli::journal::{self, Admission};
use ape_cli::reading::WorldRecord;

use ape_frontier::subject::designation::{
    self, BUDGET, DESIGNATED, DISTINCT, ENTRIES, FORKS, MOVES, PLANS, WORLDS,
};

/// Phase 0 — the arrangement is what the constants say, before anything is asked of it.
///
/// Every number here was written into the subject before this ran. A phase that derived them from
/// the arrangement it is describing would report that the arrangement equals itself.
#[test]
fn phase_0_the_record_holds_four_worlds_two_parties_and_eighteen_entries() {
    let founded = designation::founded().expect("the subject is admissible");

    assert_eq!(
        founded.subject.journal.len(),
        ENTRIES,
        "the founded journal is the one the subject describes"
    );
    assert_eq!(
        founded.lineage.decided().len(),
        WORLDS,
        "the shared ancestor and one fork per plan"
    );
    assert_eq!(founded.decisions.len(), WORLDS, "one decision per world");

    let claimed: Vec<Option<_>> = founded.decisions.iter().map(|taken| taken.by).collect();
    assert_eq!(
        claimed,
        vec![
            None,
            Some(founded.subject.planner),
            Some(founded.subject.steward),
            Some(founded.subject.planner),
        ],
        "a genesis that claims nobody, and three forks split between two parties"
    );

    let named: Vec<_> = (0..MOVES)
        .map(|move_of| founded.designated(move_of))
        .collect();
    assert_eq!(named[0], named[2], "the plan returns to where it started");
    assert_ne!(named[0], named[1], "and it went somewhere else in between");
    assert_eq!(
        named
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        DISTINCT,
        "three moves over two distinct worlds"
    );
}

/// Phase 0 — and the three forks are three worlds, which is what makes a designation a choice.
#[test]
fn phase_0_the_three_forks_are_distinct_and_share_one_ancestor() {
    let founded = designation::founded().expect("the subject is admissible");

    let ancestor = founded.shared().id();
    let worlds = designation::worlds(&founded.lineage);

    assert_eq!(worlds.len(), WORLDS);
    for (index, fork) in founded.forks.iter().enumerate() {
        assert_ne!(*fork, ancestor, "fork {index} is not its own parent");
        let record = worlds
            .iter()
            .find(|world| world.thesis == fork.to_string())
            .expect("every fork is witnessed in worlds.json");
        assert_eq!(
            record.thesis_parent.as_deref(),
            Some(ancestor.to_string().as_str()),
            "fork {index} extends the shared ancestor"
        );
    }

    let distinct: std::collections::BTreeSet<_> = founded.forks.iter().collect();
    assert_eq!(distinct.len(), FORKS, "three plans, three worlds");
    assert_eq!(PLANS.len(), FORKS);
    assert!(PLANS.iter().all(|plan| *plan < BUDGET));
}

/// Phase 1 — A, attempted rather than argued: put the designation in the journal.
///
/// The journal is a closed enum of the nine kernel entities and none of them is a claim about a
/// world, so an application that wants a designation in there has exactly one move available: put
/// the world's identity in a field that takes free text. That is the attempt, and it is the one an
/// application would actually reach for.
///
/// What is measured is the literal: whether the Canon refuses it, and — if it does not — what the
/// record then holds.
#[test]
fn phase_1_a_designation_in_the_journal_is_admitted_as_something_else() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-1");
    designation::found(&repository, &founded).expect("a whole write");

    let mut working = designation::read(&repository).expect("the record reads back");
    let plan = founded.designated(0);

    let attempt = Admission::Role {
        label: plan.to_string(),
        recorded_at: "2026-01-11".into(),
    };
    let outcome = designation::admit(&mut working, attempt);

    assert!(
        outcome.is_ok(),
        "the Canon does not refuse it: {:?}",
        outcome.err()
    );

    // The control, because `is_ok()` proves nothing unless this path can produce an `Err` at all:
    // the same admission aimed at an entity the record does not hold is refused, by name.
    let refused = designation::admit(
        &mut working,
        Admission::ResourceInstance {
            label: "ledger".into(),
            resource: absent_resource(),
            recorded_at: "2026-01-11".into(),
        },
    );
    assert!(
        refused.is_err(),
        "the admitting layer does refuse an unresolvable reference — it is references to \
         ADMITTED things that it checks"
    );

    let role = *working.admitted.roles.last().expect("a role was admitted");
    assert_eq!(
        working.admitted.roles.len(),
        3,
        "payer, payee, and the plan — which is now a role"
    );
    assert_ne!(
        role.to_string(),
        plan.to_string(),
        "and the entry's own identity is not the world's: it addresses a Role whose label happens \
         to be a world"
    );
}

/// Phase 1 — and nothing checks that the world it names is one this record holds.
///
/// The same attempt, aimed at an identity no lineage here ever produced. If the record refuses A
/// for naming a derived value, this is where it refuses.
#[test]
fn phase_1_the_journal_takes_a_designation_of_a_world_that_does_not_exist() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-1-absent");
    designation::found(&repository, &founded).expect("a whole write");

    let mut working = designation::read(&repository).expect("the record reads back");

    let absent = "0".repeat(64);
    assert!(
        !designation::worlds(&working.lineage)
            .iter()
            .any(|world| world.thesis == absent),
        "the arrangement holds no such world, which is the point of aiming at it"
    );

    let outcome = designation::admit(
        &mut working,
        Admission::Role {
            label: absent.clone(),
            recorded_at: "2026-01-11".into(),
        },
    );

    assert!(
        outcome.is_ok(),
        "nothing between the caller and the file weighs the target: {:?}",
        outcome.err()
    );

    designation::write(&repository, &working).expect("a whole write");
    let reread = designation::read(&repository).expect("and it reads back");

    assert!(
        reread
            .journal
            .iter()
            .any(|entry| matches!(entry, Admission::Role { label, .. } if *label == absent)),
        "corroboration passes it, so a reader gets a plan pointing at nothing"
    );
}

/// Phase 1 — a plan that moves is history that grew, and the record's claim about it does not.
///
/// The prediction written into this guard was that three moves leave three addresses, because the
/// recording instants differ where the designations do not. It measured **two**, and the reason is
/// the fact the whole project is built on: an entry's identity is derived from its content and no
/// identity carries a recording instant. The third move is `W₁` again, so it is the first move's
/// entry — byte for byte, address for address.
///
/// So the journal, which is a sequence, holds three; and everything the record checks itself
/// against, which is a set, holds two. `custody.json` is membership by construction, for the reason
/// [`ape_cli::lineage::Taken::witness`] gives — and a record that lost the return agrees with
/// itself about what it holds.
///
/// The numbers are the measurement and are asserted as literals rather than derived.
#[test]
fn phase_1_a_plan_that_returns_leaves_no_trace_in_what_the_record_claims_to_hold() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-1-cost");
    designation::found(&repository, &founded).expect("a whole write");

    let mut working = designation::read(&repository).expect("the record reads back");
    let before = journal::addresses(&working.journal).expect("the journal replays");

    for move_of in 0..MOVES {
        designation::admit(
            &mut working,
            Admission::Role {
                label: founded.designated(move_of).to_string(),
                recorded_at: format!("2026-01-1{}", move_of + 1),
            },
        )
        .expect("each move is admitted");
    }

    let after = journal::addresses(&working.journal).expect("the journal replays");

    assert_eq!(before.len(), ENTRIES);
    assert_eq!(
        after.len(),
        ENTRIES + MOVES,
        "a plan that moved three times left three entries"
    );

    let distinct: std::collections::BTreeSet<_> = after[ENTRIES..].iter().collect();
    let whole: std::collections::BTreeSet<_> = after.iter().collect();

    assert_eq!(
        distinct.len(),
        DISTINCT,
        "three moves, and the record can distinguish two"
    );
    assert_eq!(
        after[ENTRIES],
        after[ENTRIES + 2],
        "the return is the departure: same content, same address"
    );
    assert_eq!(
        whole.len(),
        ENTRIES + DISTINCT,
        "so custody, which is membership, is two longer where the journal is three"
    );

    let decided_after = ape_cli::lineage::Taken::now(
        designation::also(founded.forks[2], founded.subject.plans[2]),
        &working.admitted,
    )
    .expect("a decision can be taken after the plan returned");

    assert_eq!(
        decided_after.witness.len(),
        ENTRIES + DISTINCT,
        "and a decision taken afterwards witnesses two of the three moves"
    );
    assert_eq!(
        decided_after.after, after[ENTRIES],
        "with its coordinate addressing the entry where the plan FIRST was, not where it is"
    );
}

/// Phase 1 — the worlds already decided do not move, which is the one thing A does not break.
///
/// Worth measuring rather than assuming: a designation admitted into the journal is not an Event,
/// so it does not move the chain a cut resolves against, and every world witnessed before it reads
/// back identical. If A were disqualified by *this*, the finding would be about the cut instead.
#[test]
fn phase_1_admitting_a_designation_leaves_every_decided_world_where_it_was() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-1-worlds");
    designation::found(&repository, &founded).expect("a whole write");

    let before: Vec<WorldRecord> = designation::read(&repository)
        .map(|working| designation::worlds(&working.lineage))
        .expect("the record reads back");

    let mut working = designation::read(&repository).expect("the record reads back");
    designation::admit(
        &mut working,
        Admission::Role {
            label: founded.designated(0).to_string(),
            recorded_at: "2026-01-11".into(),
        },
    )
    .expect("admitted");
    designation::write(&repository, &working).expect("a whole write");

    let after = designation::read(&repository)
        .map(|working| designation::worlds(&working.lineage))
        .expect("and it reads back");

    assert_eq!(before.len(), WORLDS);
    for (was, is) in before.iter().zip(after.iter()) {
        assert_eq!(
            was.disagreement(is),
            None,
            "no coordinate of any world moved"
        );
    }
}

/// A resource identity no arrangement here admits, for the control in Phase 1.
fn absent_resource() -> ResourceId {
    ResourceId::from([0xEE; 32])
}

/// The order the plan names its worlds in, as a phase reads it.
#[test]
fn phase_0_the_plan_is_a_sequence_and_a_set_cannot_hold_it() {
    assert_eq!(DESIGNATED.len(), MOVES);

    let as_a_set: std::collections::BTreeSet<_> = DESIGNATED.iter().collect();
    assert_eq!(
        as_a_set.len(),
        DISTINCT,
        "the set the moves collapse into is smaller than the moves"
    );
    assert!(
        DISTINCT < MOVES,
        "so a record holding the set cannot answer what the plan was at a given move"
    );
}
