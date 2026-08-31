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

use ape::engine::thesis::ThesisId;
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

/// B — a designation as a fifth file, built here rather than in the application.
///
/// The row's rules say an experiment may not change the application, and the protocol's Phase 2 says
/// *B, built*. Both hold by building it where the phases that measure it live: if any of this earns
/// its way into `ape-cli`, that is a separate reviewed act after Phase 5, and the module that carries
/// it records the experiment — which is the protocol's fourth criterion and `cli/tests/pedigree.rs`'s
/// business.
///
/// It is the **bare pointer**, deliberately. P5 asks whether one can answer *what was the plan on the
/// twelfth*, and building the answer into the shape before the phase asks would make the phase a
/// formality.
mod fifth_file {
    use std::fs;
    use std::path::PathBuf;

    use ape::engine::thesis::ThesisId;
    use ape_cli::repository::Repository;

    pub const DESIGNATION: &str = "designation.json";

    /// What a record's designation file says, once the record has been asked to check it.
    ///
    /// Three states rather than two, and the third is the whole reason B is not A: **absent** is a
    /// record making no claim, which is custody's tolerance and the reason the repositories under
    /// `lab/agents` still read; **unresolved** is a claim the record can refuse, which is exactly
    /// what the admitting layer could not do in Phase 1.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Plan {
        NoClaim,
        Names(ThesisId),
        Unresolved(String),
    }

    /// Where the fifth file goes: beside the four a whole write puts down.
    ///
    /// `Repository::live()` is private and [`Repository::custody_path`] is not, so the live
    /// generation is reached through the accessor for the very file B is modelled on. A laboratory
    /// reaching past a private boundary would be measuring a shape the application cannot offer.
    fn path(repository: &Repository) -> PathBuf {
        repository
            .custody_path()
            .parent()
            .expect("a custody path is a file inside a generation")
            .join(DESIGNATION)
    }

    /// Write the record's claim about which of its worlds it means.
    ///
    /// Supplied rather than derived, which is the one place B departs from custody: `custody.json`
    /// is a function of the journal and the write computes it, so nothing can supply a wrong one.
    /// Nothing derives a plan.
    pub fn designate(repository: &Repository, plan: ThesisId) -> std::io::Result<()> {
        let encoded = serde_json::to_string_pretty(&plan).expect("an identity encodes");

        fs::write(path(repository), encoded)
    }

    /// Read the claim, and check it against `worlds.json`.
    ///
    /// The check is existence and nothing more, and that is a property of what a designation is
    /// rather than a shortcut. Custody's check can disagree with the record — two derivations of one
    /// fact, and a replay that produces different addresses says so. A plan is derived from nothing,
    /// so the only thing the record can contradict is *that the world is one of its own*.
    pub fn plan_of(repository: &Repository) -> Plan {
        let path = path(repository);

        if !path.exists() {
            return Plan::NoClaim;
        }

        let encoded = fs::read_to_string(&path).expect("the file is readable");
        let named: ThesisId = serde_json::from_str(&encoded).expect("it holds an identity");

        let worlds = repository.read_worlds().expect("worlds.json reads");

        if worlds.iter().any(|world| world.thesis == named.to_string()) {
            Plan::Names(named)
        } else {
            Plan::Unresolved(named.to_string())
        }
    }
}

/// Phase 2 — B holds a plan, and refuses the one thing A could not.
#[test]
fn phase_2_a_fifth_file_names_a_world_and_the_record_can_check_it() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-2");
    designation::found(&repository, &founded).expect("a whole write");

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::NoClaim,
        "a record with no designation file makes no such claim — custody's tolerance, and what \
         keeps every repository already in this workspace readable"
    );

    let plan = founded.designated(0);
    fifth_file::designate(&repository, plan).expect("the claim is written");

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Names(plan),
        "and once written it names a world this record holds"
    );
}

/// Phase 2 — a designation of a world the record does not hold is refused, by name.
///
/// This is the comparison the whole experiment turns on. Phase 1 aimed the same claim at sixty-four
/// zeroes and the record took it, wrote it whole, and read it back without a word. Here the claim
/// lives one layer over, next to `worlds.json` instead of inside the journal, and the same claim is
/// refused — with the identity in the refusal, so a reader is not sent to find out which.
#[test]
fn phase_2_a_designation_of_a_world_that_does_not_exist_is_refused() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-2-absent");
    designation::found(&repository, &founded).expect("a whole write");

    let absent = ThesisId::from([0u8; 32]);
    fifth_file::designate(&repository, absent).expect("nothing stops it being written");

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Unresolved(absent.to_string()),
        "the read refuses it, and names it"
    );
}

/// Phase 2 — and the plan moves without the journal moving, which is the whole of B's cheapness.
///
/// Three moves, and the record holds eighteen entries throughout. Against Phase 1, where the same
/// three moves left three journal entries the record could only count as two.
#[test]
fn phase_2_moving_the_plan_three_times_leaves_the_journal_where_it_was() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-2-moves");
    designation::found(&repository, &founded).expect("a whole write");

    for move_of in 0..MOVES {
        fifth_file::designate(&repository, founded.designated(move_of)).expect("the plan moves");

        assert_eq!(
            fifth_file::plan_of(&repository),
            fifth_file::Plan::Names(founded.designated(move_of)),
            "move {move_of} is where the record says it is"
        );
    }

    let reread = designation::read(&repository).expect("the record reads back");

    assert_eq!(
        reread.journal.len(),
        ENTRIES,
        "the journal never heard about any of it"
    );
    assert_eq!(
        designation::worlds(&reread.lineage).len(),
        WORLDS,
        "and neither did the worlds"
    );
}

/// Phase 2 — the shape's real cost: a supplied fifth file does not survive the next whole write.
///
/// `custody.json` is derived **by** the write, so it travels inside the all-or-nothing that puts the
/// four files in the generation nothing is reading yet. A designation is supplied, and a file written
/// beside the live generation is a file the next turn leaves behind — the pointer moves to the other
/// generation, which never had it.
///
/// So B's check is custody's shape and B's **lifecycle is not**. Whatever carries a designation has
/// to be an input to the write, which means [`ape_cli::repository::RepositoryInput`] grows a field.
/// That is measured here rather than argued, and it is the concrete thing this phase hands over.
#[test]
fn phase_2_a_designation_beside_the_generation_is_lost_by_the_next_write() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-2-lost");
    designation::found(&repository, &founded).expect("a whole write");

    let plan = founded.designated(0);
    fifth_file::designate(&repository, plan).expect("the claim is written");
    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Names(plan)
    );

    let mut working = designation::read(&repository).expect("the record reads back");
    designation::admit(
        &mut working,
        Admission::Role {
            label: "auditor".into(),
            recorded_at: "2026-01-11".into(),
        },
    )
    .expect("something unrelated is admitted");
    designation::write(&repository, &working).expect("and the record is put back, whole");

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::NoClaim,
        "the plan is gone, and the record says it never had one — which is the same silence a \
         record that genuinely never claimed anything gives"
    );
}

/// Phase 3 — C: two parties, two plans, and one file that can hold one of them.
///
/// The record already answers *who decided what*: `Taken::by` names the planner on two forks and the
/// steward on one. What it is asked here is whose **plan** the single designation is, and the
/// measurement is that nothing in the record answers — the two questions are not the same question,
/// and no field spans them.
#[test]
fn phase_3_one_designation_cannot_hold_two_parties_plans() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-3");
    designation::found(&repository, &founded).expect("a whole write");

    let planner_plan = founded.forks[0];
    let steward_plan = founded.forks[1];
    assert_ne!(planner_plan, steward_plan);

    fifth_file::designate(&repository, steward_plan).expect("one of them is written");

    let reread = designation::read(&repository).expect("the record reads back");
    let deciders: Vec<_> = reread.decisions.iter().map(|taken| taken.by).collect();

    assert_eq!(
        deciders,
        vec![
            None,
            Some(founded.subject.planner),
            Some(founded.subject.steward),
            Some(founded.subject.planner),
        ],
        "the record does say who decided each world"
    );

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Names(steward_plan),
        "and it says one world is the plan"
    );

    // The two facts do not compose into the third. Whose plan this is cannot be read off `by`: the
    // steward decided this world, and the planner decided two others — nothing says the planner is
    // not also content with the steward's, and nothing says it is.
    let claimed_by_steward: Vec<_> = reread
        .decisions
        .iter()
        .filter(|taken| taken.by == Some(founded.subject.steward))
        .collect();

    assert_eq!(
        claimed_by_steward.len(),
        1,
        "one decision names the steward, which is the strongest thing the record has"
    );
    assert!(
        claimed_by_steward[0].decision.extends().is_some(),
        "and it is a fork, so it says what was proposed and by whom — not what is meant now"
    );
}

/// Phase 3 — and the genesis claims nobody, which is what makes a house plan a separate question.
///
/// If a designation were per party, the record would hold one per party and a reader who is not a
/// party would get nothing. The arrangement has a decision that claims nobody precisely so that the
/// case is present rather than hypothetical: the shared ancestor is everyone's and no one's, and a
/// per-party file has no row for it.
#[test]
fn phase_3_a_per_party_file_has_no_row_for_a_decision_that_claims_nobody() {
    let founded = designation::founded().expect("the subject is admissible");

    let unclaimed: Vec<_> = founded
        .decisions
        .iter()
        .filter(|taken| taken.by.is_none())
        .collect();

    assert_eq!(unclaimed.len(), 1, "the genesis, and only the genesis");
    assert!(
        unclaimed[0].decision.extends().is_none(),
        "and it is the shared ancestor rather than a fork somebody forgot to sign"
    );

    let parties: std::collections::BTreeSet<_> = founded
        .decisions
        .iter()
        .filter_map(|taken| taken.by)
        .collect();

    assert_eq!(
        parties.len(),
        2,
        "two parties are answerable for three forks"
    );
    assert_eq!(
        founded.decisions.len() - unclaimed.len(),
        FORKS,
        "three of the four worlds are somebody's"
    );
    assert_eq!(
        founded.lineage.decided().len(),
        WORLDS,
        "and the fourth is nobody's, so a per-party file does not partition the record"
    );
}

/// Phase 4 — two parties, two plans, one file, and nothing says so.
///
/// P4 predicted the merge would not refuse two records that disagree about which world is principal.
/// It does not, and the reason is worse than agreement: the second designation **overwrites** the
/// first, in place, with no comparison anywhere. A refusal would at least have been a fact.
#[test]
fn phase_4_the_second_partys_plan_replaces_the_first_without_a_comparison() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-4");
    designation::found(&repository, &founded).expect("a whole write");

    let planner_plan = founded.forks[0];
    let steward_plan = founded.forks[1];

    fifth_file::designate(&repository, planner_plan)
        .expect("the planner says which world it means");
    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Names(planner_plan)
    );

    fifth_file::designate(&repository, steward_plan).expect("and so does the steward");

    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::Names(steward_plan),
        "one plan survives, and the record does not hold that there was ever another"
    );
}

/// Phase 4 — and converging drops both, because the merge does not know designations exist.
///
/// `converge` reads, appends, re-orders, rebuilds, and writes whole. Every one of those steps is
/// about the four files. A fifth beside them is not merged, not compared, and not carried: after a
/// merge the record makes no claim at all.
///
/// So P4 is **confirmed vacuously**, which is not the same as confirmed. The merge does not refuse
/// two designations for the reason a decision is not refused — it refuses nothing because it never
/// sees them.
#[test]
fn phase_4_converging_leaves_the_record_claiming_nothing() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-4-merge");
    designation::found(&repository, &founded).expect("a whole write");

    fifth_file::designate(&repository, founded.forks[0]).expect("a plan is claimed");

    let mut party = designation::read(&repository).expect("a party reads");
    designation::admit(
        &mut party,
        Admission::Role {
            label: "auditor".into(),
            recorded_at: "2026-01-11".into(),
        },
    )
    .expect("and admits something of its own");

    let merged = ape_cli::converge::converge(&repository, &party).expect("the merge goes through");

    assert_eq!(
        merged.journal.len(),
        ENTRIES + 1,
        "the knowledge merged, which is what converge is for"
    );
    assert_eq!(
        fifth_file::plan_of(&repository),
        fifth_file::Plan::NoClaim,
        "and the plan is gone — not refused, not arbitrated, absent"
    );
}

/// Phase 5 — a bare pointer cannot say what the plan was on the twelfth, and neither can anything else.
///
/// Two records: one whose plan went `W₁ → W₂ → W₁`, and one that designated `W₁` and never moved.
/// Every file they hold is compared, byte for byte.
///
/// P5 asked whether the answer is derivable from the lineage. It is not, and the arrangement is what
/// shows it rather than an argument: the lineage of the two records is the same lineage, because
/// designating is not deciding. The two records are **indistinguishable**, which is the shape
/// `17-imputation` ended on — an author and a relay were one record there, and a plan that moved and
/// one that never did are one record here.
#[test]
fn phase_5_a_plan_that_moved_and_a_plan_that_never_did_are_one_record() {
    let founded = designation::founded().expect("the subject is admissible");

    let moved = designation::scratch("phase-5-moved");
    designation::found(&moved, &founded).expect("a whole write");
    for move_of in 0..MOVES {
        fifth_file::designate(&moved, founded.designated(move_of)).expect("the plan moves");
    }

    let still = designation::scratch("phase-5-still");
    designation::found(&still, &founded).expect("a whole write");
    fifth_file::designate(&still, founded.designated(0)).expect("and this one never moves");

    assert_eq!(
        fifth_file::plan_of(&moved),
        fifth_file::plan_of(&still),
        "both records name the same world as the plan"
    );

    for file in [
        "journal.json",
        "lineage.json",
        "worlds.json",
        "custody.json",
        fifth_file::DESIGNATION,
    ] {
        // Two absent files compare equal, which is how this family of guard goes green measuring
        // nothing. Both sides are required to be there before they are required to agree.
        let (left, right) = (read_file(&moved, file), read_file(&still, file));
        assert!(
            left.is_some() && right.is_some(),
            "{file} is on disk in both records — if it is not, the comparison below is vacuous"
        );

        assert_eq!(
            left, right,
            "{file} is the same in a record whose plan moved three times and one whose never did"
        );
    }
}

/// Phase 5 — and the record cannot even say the plan moved, let alone when.
///
/// The weakest form of the question, asked so that the answer is not mistaken for one about
/// granularity: it is not that the record dates the moves badly. It holds no move.
#[test]
fn phase_5_nothing_in_the_record_counts_how_many_times_the_plan_moved() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-5-count");
    designation::found(&repository, &founded).expect("a whole write");

    for move_of in 0..MOVES {
        fifth_file::designate(&repository, founded.designated(move_of)).expect("the plan moves");
    }

    let reread = designation::read(&repository).expect("the record reads back");

    assert_eq!(reread.journal.len(), ENTRIES, "no entry records a move");
    assert_eq!(reread.decisions.len(), WORLDS, "no decision records one");
    assert_eq!(
        read_file(&repository, fifth_file::DESIGNATION)
            .expect("the file is there")
            .matches("\"")
            .count(),
        2,
        "and the file holds exactly one identity, so it cannot hold three"
    );
}

/// One of a repository's files as it stands on disk, or nothing where the record has none.
fn read_file(repository: &ape_cli::repository::Repository, named: &str) -> Option<String> {
    let generation = repository
        .custody_path()
        .parent()
        .expect("a custody path is a file inside a generation")
        .to_path_buf();

    std::fs::read_to_string(generation.join(named)).ok()
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
