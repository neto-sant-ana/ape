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
use ape::kernel::entities::{AgentId, ResourceId};

use ape_cli::journal::{self, Admission, EntryId};
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

/// Phase 6 — the discriminator, and it collapses from three to two.
///
/// The protocol asked whether *which world is live* is a fact about the house, about a record, or
/// about a party. The first two are not two answers, and this is what shows it: two repositories
/// founded from one subject are **byte-identical in every file**. A record carries no name, no
/// identity of its own, and nothing above it — so a claim *about this record* and a claim *about the
/// house* have the same subject and the same silence, because there is nothing a record could say to
/// tell itself from another.
///
/// What survives is one axis: a designation is either **qualified by a party or it is not**, which is
/// exactly the axis [`ape_cli::lineage::Taken::by`] already has and for the reason recorded there — a
/// record reasoning alone has no party to name.
#[test]
fn phase_6_two_records_founded_alike_are_one_record_so_house_and_record_are_one_home() {
    let founded = designation::founded().expect("the subject is admissible");

    let here = designation::scratch("phase-6-here");
    let there = designation::scratch("phase-6-there");
    designation::found(&here, &founded).expect("a whole write");
    designation::found(&there, &founded).expect("and another, elsewhere");

    assert_ne!(here.root(), there.root(), "two directories, not one");

    for file in [
        "journal.json",
        "lineage.json",
        "worlds.json",
        "custody.json",
    ] {
        let (left, right) = (read_file(&here, file), read_file(&there, file));
        assert!(
            left.is_some() && right.is_some(),
            "{file} is on disk in both, or the comparison below is vacuous"
        );
        assert_eq!(left, right, "{file} does not distinguish the two records");
    }
}

/// C, built: the log the five phases converged on, and it is `Taken`'s shape.
///
/// Three fields, and each answers something a phase measured missing.
///
/// ```text
/// plan     which world      checked against `worlds.json`   — Phase 2
/// after    where in the     checked against the journal     — Phase 5, and it is the
///          journal it moved                                   project's own way of
///                                                              asking about time
/// by       whose plan       checked against the replay      — Phase 3, optional for
///                                                              the reason `Taken::by` is
/// ```
///
/// **A sequence, and ordered by position rather than by `after`.** Phase 1 is what forces it: an
/// `EntryId` is content-derived, so two moves with no admission between them carry the same
/// coordinate and a set would lose one. `after` orders the log against knowledge; the file's own
/// order is what separates two moves at one coordinate.
///
/// This is not `at`. A recording instant is the one value nothing derives — `13-indexicality`'s
/// finding — so a designation carrying a date would be a claim no receiver can weigh, which is the
/// class `17-imputation` closed. What the log answers is *what was the plan when the record knew
/// this much*, and that is checkable.
mod log {
    use std::fs;
    use std::path::PathBuf;

    use ape::engine::thesis::ThesisId;
    use ape::kernel::entities::AgentId;
    use ape_cli::journal::EntryId;
    use ape_cli::repository::Repository;

    pub const DESIGNATIONS: &str = "designations.json";

    #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct Designated {
        pub plan: ThesisId,
        pub after: EntryId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub by: Option<AgentId>,
    }

    /// What the record's log is, once it has been checked.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Log {
        NoClaim,
        Held(Vec<Designated>),
        Unresolved(&'static str, String),
    }

    fn path(repository: &Repository) -> PathBuf {
        repository
            .custody_path()
            .parent()
            .expect("a custody path is a file inside a generation")
            .join(DESIGNATIONS)
    }

    pub fn write(repository: &Repository, held: &[Designated]) -> std::io::Result<()> {
        let encoded = serde_json::to_string_pretty(held).expect("a log encodes");

        fs::write(path(repository), encoded)
    }

    /// Read the log, and check every claim in it against the four files.
    ///
    /// The first unresolved claim is returned rather than a count, and it says **which** of the three
    /// references failed. A reader told only that a log is bad has to go and find out how.
    pub fn read(repository: &Repository) -> Log {
        let path = path(repository);

        if !path.exists() {
            return Log::NoClaim;
        }

        let encoded = fs::read_to_string(&path).expect("the file is readable");
        let held: Vec<Designated> = serde_json::from_str(&encoded).expect("it holds a log");

        let worlds = repository.read_worlds().expect("worlds.json reads");
        let journal = repository.read_journal().expect("journal.json reads");

        let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
        let replayed = ape_cli::journal::replay(&mut canon, &journal)
            .expect("the journal this record wrote replays");

        for entry in &held {
            if !worlds
                .iter()
                .any(|world| world.thesis == entry.plan.to_string())
            {
                return Log::Unresolved("plan", entry.plan.to_string());
            }
            if !replayed.entries.contains(&entry.after) {
                return Log::Unresolved("after", entry.after.to_string());
            }
            if let Some(party) = entry.by {
                if !replayed.agents.contains(&party) {
                    return Log::Unresolved("by", party.to_string());
                }
            }
        }

        Log::Held(held)
    }

    /// What the plan was when the record held everything up to `after`, or nothing before the first.
    ///
    /// The last entry at or before that coordinate, by the log's own order. Which is the question P5
    /// asked, in the coordinate the record can check rather than in a date it cannot.
    pub fn plan_at(
        held: &[Designated],
        after: &EntryId,
        addresses: &[EntryId],
    ) -> Option<ThesisId> {
        let asked = addresses.iter().position(|entry| entry == after)?;

        held.iter()
            .filter(|entry| {
                addresses
                    .iter()
                    .position(|address| *address == entry.after)
                    .is_some_and(|at| at <= asked)
            })
            .next_back()
            .map(|entry| entry.plan)
    }
}

/// Phase 6 — the log answers what a bare pointer could not, and refuses what A could not.
#[test]
fn phase_6_the_log_says_what_the_plan_was_at_each_coordinate() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-log");
    designation::found(&repository, &founded).expect("a whole write");

    let mut working = designation::read(&repository).expect("the record reads back");
    let mut held = Vec::new();

    // Each move is separated by an admission, so the three moves sit at three coordinates. Two moves
    // at one coordinate is the case the file's own order carries, and the next guard is about it.
    for move_of in 0..MOVES {
        designation::admit(
            &mut working,
            Admission::Role {
                label: format!("witness-{move_of}"),
                recorded_at: "2026-01-11".into(),
            },
        )
        .expect("something is admitted between moves");

        held.push(log::Designated {
            plan: founded.designated(move_of),
            after: working
                .admitted
                .entries
                .last()
                .cloned()
                .expect("the replay reached an entry"),
            by: Some(founded.subject.planner),
        });
    }

    designation::write(&repository, &working).expect("the record is put back, whole");
    log::write(&repository, &held).expect("and the log beside it");

    let log::Log::Held(read_back) = log::read(&repository) else {
        panic!("the log resolves against the record it sits beside");
    };
    assert_eq!(read_back, held, "three moves, and the record holds three");

    let addresses = journal::addresses(&working.journal).expect("the journal replays");
    for move_of in 0..MOVES {
        assert_eq!(
            log::plan_at(&read_back, &held[move_of].after, &addresses),
            Some(founded.designated(move_of)),
            "at move {move_of} the plan was where the log says it was"
        );
    }

    assert_eq!(
        log::plan_at(&read_back, &addresses[0], &addresses),
        None,
        "and before the first move the record had no plan, which is not the same as having one"
    );
}

/// Phase 6 — the log carries the party, so two parties hold two plans in one record.
///
/// Phase 4 measured the bare pointer's answer to this: the second write overwrote the first with no
/// comparison anywhere. Here both are present, both are attributed, and neither is arbitrated —
/// which is `converge`'s own treatment of two decisions, one file over.
#[test]
fn phase_6_two_parties_hold_two_plans_and_neither_is_wrong() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-parties");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    let held = vec![
        log::Designated {
            plan: founded.forks[0],
            after: at.clone(),
            by: Some(founded.subject.planner),
        },
        log::Designated {
            plan: founded.forks[1],
            after: at.clone(),
            by: Some(founded.subject.steward),
        },
        log::Designated {
            plan: founded.shared().id(),
            after: at,
            by: None,
        },
    ];
    log::write(&repository, &held).expect("the log is written");

    let log::Log::Held(read_back) = log::read(&repository) else {
        panic!("every claim in the log resolves");
    };

    let of = |party: Option<AgentId>| -> Vec<ThesisId> {
        read_back
            .iter()
            .filter(|entry| entry.by == party)
            .map(|entry| entry.plan)
            .collect()
    };

    assert_eq!(of(Some(founded.subject.planner)), vec![founded.forks[0]]);
    assert_eq!(of(Some(founded.subject.steward)), vec![founded.forks[1]]);
    assert_eq!(
        of(None),
        vec![founded.shared().id()],
        "and the unqualified row is the record's own — the answer for a reader who is no party, \
         and the row Phase 3 measured a per-party file has no place for"
    );
}

/// Phase 6 — a log naming a world the record does not hold is refused, and so is a bad coordinate.
///
/// Three references, three ways to be wrong, and the refusal says which. Phase 2's file could only
/// check one of them because it only had one.
#[test]
fn phase_6_the_log_refuses_each_of_its_references_by_name() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-refusals");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    let absent_world = ThesisId::from([0u8; 32]);
    log::write(
        &repository,
        &[log::Designated {
            plan: absent_world,
            after: at.clone(),
            by: None,
        }],
    )
    .expect("nothing stops it being written");
    assert_eq!(
        log::read(&repository),
        log::Log::Unresolved("plan", absent_world.to_string()),
        "the world is refused, and named"
    );

    let absent_entry = EntryId::of(ape::kernel::entities::CommitmentId::from([0xEEu8; 32]));
    log::write(
        &repository,
        &[log::Designated {
            plan: founded.forks[0],
            after: absent_entry.clone(),
            by: None,
        }],
    )
    .expect("nor this");
    assert_eq!(
        log::read(&repository),
        log::Log::Unresolved("after", absent_entry.to_string()),
        "and so is the coordinate, separately, so a reader is not left guessing which reference failed"
    );

    let absent_party = AgentId::from([0x11u8; 32]);
    log::write(
        &repository,
        &[log::Designated {
            plan: founded.forks[0],
            after: at,
            by: Some(absent_party),
        }],
    )
    .expect("nor this");
    assert_eq!(
        log::read(&repository),
        log::Log::Unresolved("by", absent_party.to_string()),
        "and the party, which is `Taken::by`'s check and the reason an identity is used rather \
         than a label"
    );
}

/// Phase 6 — two moves at one coordinate, which is what makes the log a sequence and not a set.
///
/// Phase 1's finding, arriving as a constraint on the remedy: an `EntryId` is content-derived, so
/// two moves with no admission between them carry the **same** `after`. Only the file's own order
/// separates them, and the last one is the plan.
#[test]
fn phase_6_two_moves_at_one_coordinate_are_separated_by_the_logs_own_order() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-order");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    // Three DISTINCT worlds rather than `DESIGNATED`, and the red pass is what found it: with the
    // return in place the first and last entries name the same world, so a `plan_at` that read the
    // log backwards would have satisfied this guard. What is being measured here is order, and the
    // arrangement's return is the one thing that hides it.
    let held: Vec<_> = (0..MOVES)
        .map(|move_of| log::Designated {
            plan: founded.forks[move_of],
            after: at.clone(),
            by: None,
        })
        .collect();
    log::write(&repository, &held).expect("the log is written");

    let log::Log::Held(read_back) = log::read(&repository) else {
        panic!("every claim resolves");
    };

    assert_eq!(
        read_back.len(),
        MOVES,
        "three moves at one coordinate, kept"
    );

    let coordinates: std::collections::BTreeSet<_> =
        read_back.iter().map(|entry| entry.after.clone()).collect();
    assert_eq!(
        coordinates.len(),
        1,
        "and one coordinate between them, so nothing derived can tell them apart"
    );

    let addresses = journal::addresses(&working.journal).expect("the journal replays");
    assert_eq!(
        log::plan_at(&read_back, &at, &addresses),
        Some(founded.forks[MOVES - 1]),
        "the plan is the last one written, which the file's order is the only thing to say"
    );
    assert_ne!(
        founded.forks[0],
        founded.forks[MOVES - 1],
        "and first and last are different worlds, or the line above measures nothing"
    );
}

/// Phase 6 — and the concluded repositories still read, which is the protocol's second criterion.
///
/// A log is additive and optional, so a record that has none makes no claim — the same tolerance
/// `custody.json` has, and for the same reason: four repositories under `lab/agents/04-multiagent`
/// were written by parties nobody can re-run.
///
/// **The file is removed after the write**, which is custody's `unclaimed()` move: a whole write now
/// puts a log down, so a phase measuring what a record *without* one does must not be handed the
/// thing it exists to say was missing. What it produces is the shape every repository written before
/// this experiment has.
#[test]
fn phase_6_a_record_with_no_log_makes_no_claim() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-silent");
    designation::found(&repository, &founded).expect("a whole write");

    assert_eq!(
        log::read(&repository),
        log::Log::Held(Vec::new()),
        "a whole write puts an empty log down, which says the plan never moved"
    );

    std::fs::remove_file(repository.designations_path()).expect("the record loses its log");

    assert_eq!(
        log::read(&repository),
        log::Log::NoClaim,
        "and without the file the record says nothing at all, which is a different sentence"
    );

    let reread = designation::read(&repository).expect("and the record reads back unchanged");
    assert_eq!(reread.journal.len(), ENTRIES);
    assert_eq!(designation::worlds(&reread.lineage).len(), WORLDS);
    assert!(
        reread.designations.is_empty(),
        "the application reads an absent log as no plan, which is what every caller asks"
    );
}

/// Phase 6 — the prototype and what was built off it are the same file.
///
/// The laboratory's log was written before `ape-cli` had one, and it is kept: it is the measurement,
/// and `lab/README.md`'s rule is that a concluded arrangement is pinned rather than frozen. What is
/// worth asserting is that the two did not drift — the application's `Designated` and this
/// prototype's read each other's bytes, which is the only thing that makes the phases above still
/// evidence about the thing that shipped.
#[test]
fn phase_6_the_prototype_and_the_application_read_the_same_file() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-6-wire");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    // Written by the prototype, read by the application.
    log::write(
        &repository,
        &[log::Designated {
            plan: founded.forks[0],
            after: at.clone(),
            by: Some(founded.subject.planner),
        }],
    )
    .expect("the prototype writes");

    let read_by_application = designation::read(&repository).expect("the application reads it");
    assert_eq!(read_by_application.designations.len(), 1);
    assert_eq!(read_by_application.designations[0].plan, founded.forks[0]);
    assert_eq!(
        read_by_application.designations[0].by,
        Some(founded.subject.planner)
    );

    // And the other way round.
    designation::write(&repository, &read_by_application).expect("the application writes");
    assert_eq!(
        log::read(&repository),
        log::Log::Held(vec![log::Designated {
            plan: founded.forks[0],
            after: at,
            by: Some(founded.subject.planner),
        }]),
        "the prototype reads back what the application wrote"
    );
}

/// Phase 7 — what a merge does to two logs, measured before anything is built that does it.
///
/// Not in the protocol's procedure, and here because building forced the question: a whole write
/// takes the log as an input, so `converge` has to supply one and no phase had said which. The row's
/// rule is that an experiment may not build what it did not measure, so this measures it.
///
/// The rule the merge is weighed against is `converge`'s own, one file over: **two decisions cannot
/// contradict one another**, so a second party's line is a branch rather than a competing version
/// and the union is a lineage in the same sense either party's was. Phase 6 established the same of
/// designations — two parties hold two plans and neither is wrong.
fn merged(arrived: &[log::Designated], held: &[log::Designated]) -> Vec<log::Designated> {
    let mut merged = arrived.to_vec();

    merged.extend(
        held.iter()
            .filter(|entry| !arrived.contains(entry))
            .cloned(),
    );

    merged
}

/// Phase 7 — the union keeps both parties, and keeps each party's own order.
#[test]
fn phase_7_a_union_keeps_both_lines_and_each_lines_order() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-7");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    let row = |plan, by| log::Designated {
        plan,
        after: at.clone(),
        by: Some(by),
    };

    // The planner's plan moved, so its own two rows are at one coordinate and only order tells them
    // apart. That is the case a merge can destroy, which is why it is the one being merged.
    let planner = founded.subject.planner;
    let steward = founded.subject.steward;
    let arrived = vec![
        row(founded.forks[0], planner),
        row(founded.forks[2], planner),
    ];
    let held = vec![row(founded.forks[1], steward)];

    let merged = merged(&arrived, &held);
    log::write(&repository, &merged).expect("the merged log is written");

    let log::Log::Held(read_back) = log::read(&repository) else {
        panic!("every claim in the merged log resolves");
    };

    let of = |party: AgentId| -> Vec<ThesisId> {
        read_back
            .iter()
            .filter(|entry| entry.by == Some(party))
            .map(|entry| entry.plan)
            .collect()
    };

    assert_eq!(
        of(planner),
        vec![founded.forks[0], founded.forks[2]],
        "the planner's two rows survive, in the planner's own order"
    );
    assert_eq!(
        of(steward),
        vec![founded.forks[1]],
        "and the steward's line is not lost, which is what Phase 4 measured a merge doing"
    );
    assert_eq!(read_back.len(), arrived.len() + held.len());
}

/// Phase 7 — a row already there is not added twice, so converging twice is not two plans.
///
/// A designation's fields are its whole content, so two parties that agree hold the **same row** and
/// the union collapses them — which is the journal's rule for entries, arriving here for the same
/// reason and without anything having arranged it.
#[test]
fn phase_7_two_parties_that_agree_hold_one_row_and_not_two() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-7-agree");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    let same = log::Designated {
        plan: founded.forks[0],
        after: at,
        by: Some(founded.subject.planner),
    };

    assert_eq!(
        merged(&[same.clone()], &[same.clone()]),
        vec![same.clone()],
        "one row, because a designation is its content"
    );
    assert_eq!(
        merged(&merged(&[same.clone()], &[same.clone()]), &[same.clone()]),
        vec![same],
        "and converging again does not grow the log"
    );
}

/// Phase 7 — the limit, measured rather than left for somebody to find.
///
/// Two **unqualified** rows from two parties at one coordinate cannot be ordered by anything the
/// records carry: neither has a party to be read under, and the coordinate is the same. The union
/// puts the arrived one first, and that is the merge's arrival order surviving into the result —
/// which is the thing `converge`'s own docstring says must not happen for decisions.
///
/// It does not reach a decision, a world or a number. It reaches *which unattributed plan a reader
/// sees last*, in a record where two parties both declined to say whose plan it was. Recorded as a
/// limit of the shape, and queued.
#[test]
fn phase_7_two_unqualified_rows_at_one_coordinate_are_ordered_by_arrival() {
    let founded = designation::founded().expect("the subject is admissible");
    let repository = designation::scratch("phase-7-limit");
    designation::found(&repository, &founded).expect("a whole write");

    let working = designation::read(&repository).expect("the record reads back");
    let at = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the replay reached an entry");

    let anonymous = |plan| log::Designated {
        plan,
        after: at.clone(),
        by: None,
    };
    let (here, there) = (anonymous(founded.forks[0]), anonymous(founded.forks[1]));

    assert_eq!(
        merged(&[here.clone()], &[there.clone()]),
        vec![here.clone(), there.clone()],
        "whoever arrived first is first"
    );
    assert_eq!(
        merged(&[there.clone()], &[here.clone()]),
        vec![there, here],
        "and the other way round gives the other answer — the order is the merge's, not the record's"
    );
}

/// Part B — the two defects the phases measured, gone from the application that shipped them.
///
/// Kept apart from the phases and named for what they guard rather than for a phase, because they
/// are about `ape-cli` and not about the question. Every one of them goes through the application's
/// own read and write, with nothing of the laboratory's prototype in the path — which is the whole
/// reason they exist. The phases above prove the *shape*; a shape proven only against a substitute
/// leaves the code that actually runs with no red behind it.
mod built {
    use super::*;

    use ape_cli::designation::Designated;
    use ape_cli::error::{DesignationError, ReadingError, SubjectError};

    /// The refusal a bad log produces, three error types down, so a guard can match one variant.
    fn refusal(repository: &ape_cli::repository::Repository) -> Option<DesignationError> {
        match designation::read(repository) {
            Err(SubjectError::Reading(ReadingError::Designation(why))) => Some(why),
            _ => None,
        }
    }

    fn claim(plan: ThesisId, after: EntryId, by: Option<AgentId>) -> Designated {
        Designated { plan, after, by }
    }

    /// A whole write carries the log, and the next turn does not lose it.
    ///
    /// Phase 2's finding, closed: a fifth file written *beside* the live generation was left behind
    /// by the next turn, and the record then read as one that never claimed anything.
    #[test]
    fn a_whole_write_carries_the_log_across_the_turn() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-turn");
        designation::found(&repository, &founded).expect("a whole write");

        let mut working = designation::read(&repository).expect("the record reads back");
        let at = working
            .admitted
            .entries
            .last()
            .cloned()
            .expect("the replay reached an entry");

        working.designations = vec![claim(founded.forks[0], at, Some(founded.subject.planner))];
        designation::write(&repository, &working).expect("a whole write");

        // Something unrelated, so the pointer turns to the other generation — which is exactly what
        // erased the plan before.
        let mut again = designation::read(&repository).expect("the record reads back");
        designation::admit(
            &mut again,
            Admission::Role {
                label: "auditor".into(),
                recorded_at: "2026-01-11".into(),
            },
        )
        .expect("admitted");
        designation::write(&repository, &again).expect("a whole write");

        let reread = designation::read(&repository).expect("and again");

        assert_eq!(reread.journal.len(), ENTRIES + 1, "the knowledge grew");
        assert_eq!(
            reread.designations, again.designations,
            "and the plan crossed the turn with it"
        );
        assert_eq!(reread.designations.len(), 1);
    }

    /// Converging keeps both parties' plans, where before it left the record claiming nothing.
    #[test]
    fn converging_keeps_both_parties_plans() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-merge");
        designation::found(&repository, &founded).expect("a whole write");

        let mut arrived = designation::read(&repository).expect("the record reads back");
        let at = arrived
            .admitted
            .entries
            .last()
            .cloned()
            .expect("the replay reached an entry");

        arrived.designations = vec![claim(
            founded.forks[0],
            at.clone(),
            Some(founded.subject.planner),
        )];
        designation::write(&repository, &arrived).expect("the planner puts its plan back");

        let mut party = designation::read(&repository).expect("the steward reads");
        party.designations = vec![claim(founded.forks[1], at, Some(founded.subject.steward))];
        designation::admit(
            &mut party,
            Admission::Role {
                label: "auditor".into(),
                recorded_at: "2026-01-11".into(),
            },
        )
        .expect("and admits something of its own");

        let merged =
            ape_cli::converge::converge(&repository, &party).expect("the merge goes through");

        let plans: Vec<_> = merged
            .designations
            .iter()
            .map(|held| (held.plan, held.by))
            .collect();

        assert_eq!(
            plans,
            vec![
                (founded.forks[0], Some(founded.subject.planner)),
                (founded.forks[1], Some(founded.subject.steward)),
            ],
            "both lines survive the merge, in the arrived-then-held order the union gives"
        );
        assert_eq!(
            designation::read(&repository)
                .expect("and the record on disk says the same")
                .designations,
            merged.designations
        );
    }

    /// Converging twice does not grow the log, because a designation is its content.
    #[test]
    fn converging_twice_does_not_grow_the_log() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-idempotent");
        designation::found(&repository, &founded).expect("a whole write");

        let mut party = designation::read(&repository).expect("the record reads back");
        let at = party
            .admitted
            .entries
            .last()
            .cloned()
            .expect("the replay reached an entry");
        party.designations = vec![claim(founded.forks[0], at, Some(founded.subject.planner))];

        let once = ape_cli::converge::converge(&repository, &party).expect("once");
        let twice = ape_cli::converge::converge(&repository, &party).expect("and again");

        assert_eq!(once.designations.len(), 1);
        assert_eq!(twice.designations, once.designations);
    }

    /// The application's own read refuses each of the three references, by name.
    ///
    /// Through `reading::corroborated`, with `write_designations` putting the bad log down the way a
    /// record edited from outside would — which is what that single-file writer is public for.
    #[test]
    fn the_read_refuses_each_reference_and_says_which() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-refusals");
        designation::found(&repository, &founded).expect("a whole write");

        let working = designation::read(&repository).expect("the record reads back");
        let at = working
            .admitted
            .entries
            .last()
            .cloned()
            .expect("the replay reached an entry");

        let absent_world = ThesisId::from([0u8; 32]);
        repository
            .write_designations(&[claim(absent_world, at.clone(), None)])
            .expect("edited from outside");
        assert!(
            matches!(
                refusal(&repository),
                Some(DesignationError::PlanIsNotAWorldOfThisRecord { position: 0, .. })
            ),
            "a plan that is no world of this record is refused: {:?}",
            refusal(&repository)
        );

        let absent_entry = EntryId::of(ape::kernel::entities::CommitmentId::from([0xEEu8; 32]));
        repository
            .write_designations(&[claim(founded.forks[0], absent_entry, None)])
            .expect("edited from outside");
        assert!(
            matches!(
                refusal(&repository),
                Some(DesignationError::CoordinateIsNotInTheJournal { position: 0, .. })
            ),
            "and so is a coordinate the journal does not offer"
        );

        let absent_party = AgentId::from([0x11u8; 32]);
        repository
            .write_designations(&[
                claim(founded.forks[0], at.clone(), None),
                claim(founded.forks[1], at, Some(absent_party)),
            ])
            .expect("edited from outside");
        assert!(
            matches!(
                refusal(&repository),
                Some(DesignationError::PartyWasNeverAdmitted { position: 1, .. })
            ),
            "and a party never admitted — at position 1, because a log is a sequence and the value \
             alone would leave a reader searching"
        );
    }

    /// And a good log reads, which is the control the three refusals above are worth nothing without.
    #[test]
    fn a_log_this_record_can_support_reads() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-control");
        designation::found(&repository, &founded).expect("a whole write");

        let working = designation::read(&repository).expect("the record reads back");
        let at = working
            .admitted
            .entries
            .last()
            .cloned()
            .expect("the replay reached an entry");

        repository
            .write_designations(&[
                claim(founded.forks[0], at.clone(), Some(founded.subject.planner)),
                claim(founded.forks[1], at, None),
            ])
            .expect("edited from outside");

        let reread = designation::read(&repository).expect("and it reads");
        assert_eq!(reread.designations.len(), 2);
        assert_eq!(reread.designations[1].by, None, "the unqualified row reads");
    }

    /// What the plan was at a coordinate, through the application's own walk of the log.
    #[test]
    fn the_application_says_what_the_plan_was_at_each_coordinate() {
        let founded = designation::founded().expect("the subject is admissible");
        let repository = designation::scratch("built-history");
        designation::found(&repository, &founded).expect("a whole write");

        let mut working = designation::read(&repository).expect("the record reads back");
        let mut at = Vec::new();

        for move_of in 0..MOVES {
            designation::admit(
                &mut working,
                Admission::Role {
                    label: format!("witness-{move_of}"),
                    recorded_at: "2026-01-11".into(),
                },
            )
            .expect("something is admitted between moves");

            let coordinate = working
                .admitted
                .entries
                .last()
                .cloned()
                .expect("the replay reached an entry");
            working.designations.push(claim(
                founded.designated(move_of),
                coordinate.clone(),
                Some(founded.subject.planner),
            ));
            at.push(coordinate);
        }

        designation::write(&repository, &working).expect("a whole write");
        let reread = designation::read(&repository).expect("the record reads back");

        for move_of in 0..MOVES {
            assert_eq!(
                ape_cli::designation::plan_at(&reread.designations, &at[move_of], &reread.admitted),
                Some((founded.designated(move_of), Some(founded.subject.planner))),
                "at move {move_of}"
            );
        }

        assert_eq!(
            ape_cli::designation::plan_at(
                &reread.designations,
                &reread.admitted.entries[0],
                &reread.admitted
            ),
            None,
            "and before the first move the record had no plan"
        );
    }
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
