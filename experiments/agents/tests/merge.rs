//! Phases 2.5 to 4 — putting two parties' lines in one repository, and asking Synthesis about them.
//!
//! Everything here is derived from what the two parties **wrote**. The recorded repositories in
//! `04-multiagent/run-a/repo` and `04-multiagent/run-b-prime/repo` are read as data: the admissions
//! each party appended and the decisions each party took come out of those files, so nothing below
//! is the experimenter's account of what either party chose.
//!
//! The suite refuses to run against a world it does not recognize. Every identity it needs is
//! located structurally — the genesis' world, the last world a party claimed — and the base it finds
//! is checked against the base the harness builds. A repository from some other arrangement fails at
//! that check rather than being measured as if it were this one.
//!
//! # Nothing here is a finding, and the docstrings say whose it is
//!
//! Every property this suite asserts is already written down, in the two CLI experiments that
//! measured it. Asserting them is how the phases that follow get an arrangement worth asking a
//! question in, and an experiment that read defined behaviour as friction would go looking for a
//! repair nobody needs — which the convergence experiment says in as many words, about this exact
//! asymmetry.
//!
//! So each test names the rule it is exercising. What is left over after all of them is one sentence
//! about how two of those rules compose, and it is handed over rather than claimed here.

use std::path::{Path, PathBuf};

use ape::engine::hermeneia::Hypothesis;
use ape::engine::synthesis::{ApplicabilityStatus, ResolvedTransfer, synthesize};
use ape::engine::thesis::{Interpretation, ThesisId, ThesisLookup, descends_from};
use ape::kernel::entities::AgentId;

use ape_cli::converge;
use ape_cli::error::{ConvergeError, JournalError};
use ape_cli::journal::{self, Admission};
use ape_cli::lineage::{self, Taken};
use ape_cli::reading::{self, Corroborated};
use ape_cli::repository::Repository;
use ape_cli::transfer::{self, ConflictRecord, StatusRecord};

use ape_agents::coordination;
use ape_agents::policy::{self, Verdict};

const OPERATIONS: &str = "run-a";
const FINANCE: &str = "run-b-prime";

/// A repository as one of the parties left it.
fn recorded(run: &str) -> Repository {
    Repository::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("04-multiagent")
            .join(run)
            .join("repo"),
    )
}

/// A writable copy of one, since converging writes.
fn working(run: &str, name: &str) -> Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-agents-04-merge-{}", std::process::id()))
        .join(name);

    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("a scratch directory");

    for file in ["journal.json", "lineage.json", "worlds.json"] {
        std::fs::copy(
            recorded(run).root().join(file),
            PathBuf::from(&path).join(file),
        )
        .expect("the recorded repository is readable");
    }

    Repository::open(path)
}

/// The world the genesis produced, checked to be the world this experiment founded.
fn base_of(read: &Corroborated) -> ThesisId {
    let found = read
        .decisions
        .iter()
        .zip(read.lineage.decided())
        .find(|(taken, _)| taken.decision.extends().is_none())
        .map(|(_, world)| world.id())
        .expect("a repository holds a genesis");

    assert_eq!(
        found,
        coordination::shared().base,
        "this repository is not the shared world the experiment founded"
    );

    found
}

/// The last world a party claimed, where it claimed any.
fn tip_of(read: &Corroborated, party: AgentId) -> Option<ThesisId> {
    read.decisions
        .iter()
        .zip(read.lineage.decided())
        .filter(|(taken, _)| taken.by == Some(party))
        .map(|(_, world)| world.id())
        .next_back()
}

/// What a party appended to the journal it read, and every decision it claimed, in file order.
fn contributed(run: &str, party: AgentId) -> (Vec<Admission>, Vec<Taken>) {
    let repository = recorded(run);

    let journal = repository.read_journal().expect("readable");
    let founded = coordination::shared().journal.len();

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
/// This is the repair the refusal names, applied verbatim: the knowledge is admitted again, and the
/// decisions are re-witnessed at the coordinate the merged journal gives. Nothing about either is
/// authored here — both come out of the party's own files.
fn re_admit(
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

/// Phase 2.5 — the two lines exist, and neither is on the other's.
#[test]
fn the_two_parties_produced_two_lines_from_one_base() {
    let shared = coordination::shared();

    let held =
        reading::corroborated(&recorded(OPERATIONS)).expect("run A's repository reconstructs");
    let other =
        reading::corroborated(&recorded(FINANCE)).expect("run B′'s repository reconstructs");

    assert_eq!(
        base_of(&held),
        base_of(&other),
        "both parties decided against the same world"
    );

    let operations = tip_of(&held, shared.operations).expect("operations claimed a world");
    let finance = tip_of(&other, shared.finance).expect("finance claimed a world");

    assert_ne!(
        operations, finance,
        "and they did not arrive at the same one"
    );

    assert!(
        tip_of(&held, shared.finance).is_none(),
        "neither party's repository holds the other's decisions"
    );
    assert!(tip_of(&other, shared.operations).is_none());

    assert!(
        descends_from(held.lineage.archive(), operations, shared.base).expect("walkable"),
        "operations' line extends the base"
    );
    assert!(
        descends_from(other.lineage.archive(), finance, shared.base).expect("walkable"),
        "and so does finance's"
    );

    assert_eq!(
        recorded(OPERATIONS).read_lineage().expect("readable").len(),
        3,
        "the base, an advance and a fork"
    );
    assert_eq!(
        recorded(FINANCE).read_lineage().expect("readable").len(),
        2,
        "the base and a fork — this party needed no advance, having recorded at the base's instant"
    );
}

/// Phase 2.5 — a party writing what it holds onto a journal that moved is refused, and by name.
///
/// Not a defect. Both parties admitted knowledge against the same journal, so neither journal is a
/// prefix of the other, and a repository whose earlier entries moved would make a standing decision
/// disagree with it. The CLI's coordination experiment measured this refusal and named the repair:
/// the party reads again and admits again, because knowledge is not revisable.
#[test]
fn a_second_party_cannot_write_what_it_read_the_base_as() {
    let target = working(OPERATIONS, "refused");
    let before = bytes(&target);

    let held = reading::corroborated(&recorded(FINANCE)).expect("run B′'s repository reconstructs");

    match converge::converge(&target, &held) {
        Err(ConvergeError::Diverged { position, .. }) => assert_eq!(
            position,
            coordination::shared().journal.len(),
            "at the first entry the two parties do not share"
        ),
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => {
            panic!("two parties admitted different knowledge against one journal, and it wrote")
        }
    }

    assert_eq!(
        bytes(&target),
        before,
        "and a refusal leaves the repository exactly as it was"
    );
}

/// Phase 2.5 — the repair runs in one direction only, and both halves of why are already written.
///
/// Two documented rules, composed:
///
/// * the repair for a diverged party is *read again and admit again*, because knowledge is not
///   revisable and learning the same thing on top of more history is the same fact — the
///   coordination experiment's, stated without qualification;
/// * a Canon refuses an admission whose recording instant precedes its `recorded_through`, which is
///   in the port's own contract and has conformance tests behind it.
///
/// Put together, the repair has an **order**: the party whose knowledge is dated earlier has to land
/// first. Both directions are exercised here rather than only the one that works, because the one
/// that fails is the one the repair prescribes, and a suite that quietly picked the working order
/// would be hiding the qualification rather than recording it.
///
/// The qualification is the only thing this suite turns up that is not already a sentence somewhere,
/// and it is a small one: it is visible here only because the two parties chose different recording
/// instants, each for a reason it stated, and the coordination subject's two parties learned on the
/// same day. It belongs to the branch that owns that sentence.
///
/// It is also the same fact the phase after this one turns on. One choice of instant decides both
/// which party may converge second and which direction a transfer can carry — the second of those
/// being defined behaviour the convergence experiment gives in closed form.
#[test]
fn the_repair_runs_only_in_the_order_the_dates_impose() {
    let shared = coordination::shared();

    let (later, later_decisions) = contributed(OPERATIONS, shared.operations);
    let (earlier, earlier_decisions) = contributed(FINANCE, shared.finance);

    let onto_the_later = working(OPERATIONS, "earlier-onto-later");
    let mut held = reading::corroborated(&onto_the_later).expect("reconstructs");

    match re_admit(&mut held, earlier, &earlier_decisions, shared.finance) {
        Err(JournalError::Canon(reason)) => assert!(
            format!("{reason:?}").contains("RecordedOutOfOrder"),
            "refused for the recording instant, and it says so: {reason:?}"
        ),
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(()) => panic!("knowledge dated before what the journal holds was admitted onto it"),
    }

    let onto_the_earlier = working(FINANCE, "later-onto-earlier");
    let mut held = reading::corroborated(&onto_the_earlier).expect("reconstructs");

    re_admit(&mut held, later, &later_decisions, shared.operations)
        .expect("the later party's knowledge admits onto the earlier party's");

    converge::converge(&onto_the_earlier, &held).expect("and converges");

    assert_eq!(
        onto_the_earlier.read_lineage().expect("readable").len(),
        4,
        "the base, finance's fork, and operations' advance and fork"
    );
}

/// Phase 2.5 — one repository holding both lines.
#[test]
fn one_repository_holds_both_lines() {
    let shared = coordination::shared();
    let (repository, merged) = merged("repaired");

    assert_eq!(
        merged.decisions.len(),
        4,
        "the base, finance's one, and operations' two"
    );

    let operations = tip_of(&merged, shared.operations).expect("operations' world is here");
    let finance = tip_of(&merged, shared.finance).expect("and finance's");
    let archive = merged.lineage.archive();

    assert!(
        !descends_from(archive, operations, finance).expect("walkable"),
        "neither party's world is on the other's line"
    );
    assert!(
        !descends_from(archive, finance, operations).expect("walkable"),
        "in either direction"
    );

    for party in [shared.operations, shared.finance] {
        assert!(
            !reading::decided_by(&repository, party)
                .expect("readable")
                .is_empty(),
            "each party's decisions survived the other's write"
        );
    }
}

/// Phase 3 — the report in each direction, which the convergence experiment already settled.
///
/// Source and Target are roles rather than sides: a difference is measured between the Base and the
/// Source, and the Target arrives afterwards, so swapping them asks another question rather than
/// reversing one. Which direction can carry an intention wholesale is **defined behaviour, readable
/// in advance from the two cuts** and not something an application discovers by trying:
///
/// ```text
/// Source.known_at  ≥  recorded_at  >  Target.known_at
/// ```
///
/// This asserts it because criterion 3 requires the asymmetry between the two reports to be
/// explained, and the explanation is that rule with the parties' own instants in it — operations
/// recorded on the 7th and advanced its cut to reach it, finance recorded at the instant the base
/// already recognized. Nothing here is measured that the engine had not already fixed.
#[test]
fn synthesis_answers_one_direction_and_refuses_the_other() {
    let shared = coordination::shared();
    let (repository, merged) = merged("both-directions");

    let operations = tip_of(&merged, shared.operations).expect("operations' world is here");
    let finance = tip_of(&merged, shared.finance).expect("and finance's");

    let into_operations = transfer::reconstruct(&repository, shared.base, finance, operations)
        .expect("a report is derivable");

    match &into_operations.status {
        StatusRecord::Applicable { transfer, .. } => {
            assert!(transfer.remove.is_empty(), "finance dropped nothing");
            assert_eq!(
                transfer.introduce.len(),
                1,
                "and asks operations to take on the one thing it added"
            );
        }
        other => panic!("finance's line into operations' was expected to apply: {other:?}"),
    }

    let into_finance = transfer::reconstruct(&repository, shared.base, operations, finance)
        .expect("a report is derivable");

    match &into_finance.status {
        StatusRecord::Conflicted { conflicts, .. } => assert!(
            conflicts.iter().any(|conflict| matches!(
                conflict,
                ConflictRecord::HistoricalUnavailability { .. }
            )),
            "and it is refused for the instant, not for the money: {conflicts:?}"
        ),
        other => panic!("operations' line into finance's was expected to be refused: {other:?}"),
    }

    assert!(
        into_operations.omitted.is_empty(),
        "what finance decided relative to the base omits nothing"
    );
    assert_eq!(
        into_finance.omitted.len(),
        1,
        "and what operations decided stands one thing down"
    );
}

/// Phase 4 — apply the direction that applies, then interpret the world it produced.
///
/// A candidate is deliberately not a world, so feasibility is asked of the Thesis the transfer
/// produces once a party takes it up — which is an ordinary fork, taken through the same path as
/// every other decision either party took.
#[test]
fn applying_the_transfer_yields_a_world_that_holds_both_intentions() {
    let shared = coordination::shared();
    let (repository, mut merged) = merged("applied");

    let operations = tip_of(&merged, shared.operations).expect("operations' world is here");
    let finance = tip_of(&merged, shared.finance).expect("and finance's");

    let report = synthesize(
        merged.lineage.archive(),
        merged.canon.history(),
        shared.base,
        finance,
        operations,
    )
    .expect("a report is derivable");

    let ApplicabilityStatus::Applicable { transfer, .. } = report.status() else {
        panic!("this direction applies, and the phase before this one asserts it");
    };

    let adopted = adopt(&mut merged, operations, transfer, shared.operations);

    converge::converge(&repository, &merged).expect("the adoption converges");

    let world = merged
        .lineage
        .archive()
        .thesis(adopted)
        .expect("the archive holds the world just decided");

    assert_eq!(
        world.selection().len(),
        3,
        "the settled receipt, operations' purchase and finance's payable"
    );

    let interpretation = Interpretation::of(&world, merged.canon.history()).expect("interpretable");

    for hypothesis in [
        Hypothesis::FinalState,
        Hypothesis::OnDueDateNet,
        Hypothesis::OnDueDateInAnyOrder,
    ] {
        assert_eq!(
            policy::rule(&interpretation, hypothesis).expect("feasibility is derivable"),
            Verdict::MayProceed,
            "100 received, 60 and 30 intended out — the account holds it under {hypothesis:?}"
        );
    }
}

/// Phase 3 — a report is refused outright while a party's world is in nobody's archive.
///
/// Recorded because it is the reason the repair comes first rather than being an alternative to it.
#[test]
fn a_report_cannot_be_asked_before_the_other_party_converges() {
    let shared = coordination::shared();

    let target = working(OPERATIONS, "unconverged");
    let read = reading::corroborated(&target).expect("reconstructs");
    let elsewhere = reading::corroborated(&recorded(FINANCE)).expect("reconstructs");

    let operations = tip_of(&read, shared.operations).expect("operations' world is here");
    let theirs = tip_of(&elsewhere, shared.finance).expect("finance's is elsewhere");

    match transfer::reconstruct(&target, shared.base, theirs, operations) {
        Err(reason) => assert!(
            format!("{reason:?}").contains("UnknownThesis"),
            "the Source is nowhere, and the refusal says so: {reason:?}"
        ),
        Ok(report) => panic!("a world in no archive was reported on: {report:?}"),
    }
}

/// One repository holding both lines, in the only order the dates admit.
fn merged(name: &str) -> (Repository, Corroborated) {
    let shared = coordination::shared();

    let repository = working(FINANCE, name);
    let mut held = reading::corroborated(&repository).expect("reconstructs");

    let (admissions, claimed) = contributed(OPERATIONS, shared.operations);

    re_admit(&mut held, admissions, &claimed, shared.operations)
        .expect("the later party's knowledge admits onto the earlier party's");

    converge::converge(&repository, &held).expect("both lines converge");

    let merged = reading::corroborated(&repository).expect("the merged repository reconstructs");

    (repository, merged)
}

/// Take a transfer up as the Target's party, which is a fork and nothing new.
fn adopt(
    held: &mut Corroborated,
    target: ThesisId,
    transfer: &ResolvedTransfer,
    by: AgentId,
) -> ThesisId {
    let decision = transfer::applied(target, transfer);

    held.decisions.push(
        Taken::claimed(decision.clone(), by, &held.admitted).expect("something was admitted first"),
    );
    lineage::decide(held.canon.history(), &mut held.lineage, &decision)
        .expect("adopting is an ordinary fork");

    held.lineage
        .decided()
        .last()
        .expect("a decision produces a world")
        .id()
}

fn bytes(repository: &Repository) -> Vec<Vec<u8>> {
    ["journal.json", "lineage.json", "worlds.json"]
        .iter()
        .map(|file| std::fs::read(repository.root().join(file)).expect("readable"))
        .collect()
}
