//! Experiment 11 — Veracity. Phases against `lab/frontier/docs/11-veracity/00-protocol.md`.
//!
//! The question: is there a state this record can reach **by accident** that passes every check it
//! has, is nobody's forgery, and answers a question falsely?
//!
//! *False* is defined before any phase runs, and the definition is a comparison rather than a
//! judgement: a record answers falsely when it answers a question differently from the record the
//! same events would have produced, and nothing in it refuses. The anchor is the arrangement, which
//! constructs the events and therefore holds the faithful record.
//!
//! Where two writers acted there is no single faithful record. The faithful record is then each
//! writer's own, and the question is whether a state answers something **neither** would — which is
//! how experiment 08 read its closest case, and this suite inherits it.

use std::collections::BTreeSet;

use ape::canon::CanonError;

use ape_cli::error::{ConvergeError, JournalError, LineageError, ReadingError};
use ape_cli::journal::{Admission, EntryId};
use ape_cli::repository::Repository;
use ape_cli::{converge, reading};

use ape_frontier::subject::veracity::{
    self, ALONE, BASE_ENTRIES, EARLIER_WORLDS, FAITHFUL, FAITHFUL_ENTRIES, FAITHFUL_WORLDS, MERGED,
    RECORDED_ON, SIDE_ENTRIES,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-veracity-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    Repository::open(path)
}

/// Read a repository, expecting it to refuse, and hand back what it said.
///
/// A `Corroborated` does not implement `Debug`, and it must not be turned into a string here just
/// to satisfy one — what a phase needs is the refusal, named.
fn refusal(repository: &Repository) -> ReadingError {
    match reading::corroborated(repository) {
        Err(refusal) => refusal,
        Ok(_) => panic!("the record was expected to refuse and did not"),
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — The faithful record, and what it answers
// ---------------------------------------------------------------------------------------------

/// The anchor: one record, written whole, read once, and every answer compared to a literal.
#[test]
fn the_faithful_record_answers_what_the_arrangement_says_it_does() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("faithful");

    assert_eq!(
        arrangement.base.journal.len(),
        BASE_ENTRIES,
        "what both parties read"
    );
    assert_eq!(
        arrangement.earlier.journal.len(),
        SIDE_ENTRIES,
        "generation one"
    );
    assert_eq!(
        arrangement.faithful.journal.len(),
        FAITHFUL_ENTRIES,
        "generation two"
    );
    assert_eq!(arrangement.earlier.worlds.len(), EARLIER_WORLDS);
    assert_eq!(arrangement.faithful.worlds.len(), FAITHFUL_WORLDS);

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let answers =
        veracity::read_answers(&repository, arrangement.instance).expect("the record reads");

    assert_eq!(
        answers,
        FAITHFUL.to_vec(),
        "every world of the faithful line, settled and intended"
    );
}

/// The two generations answer differently, which is what makes an interrupted write able to fail.
///
/// A phase that measured an interruption over two generations answering the same number would
/// report *nothing false* about an arrangement that could not have produced anything false.
#[test]
fn the_two_generations_answer_differently() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let earlier = scratch("earlier");
    let later = scratch("later");

    veracity::write_whole(&earlier, &arrangement.earlier).expect("a whole write");
    veracity::write_whole(&later, &arrangement.faithful).expect("a whole write");

    let one = veracity::read_answers(&earlier, arrangement.instance).expect("reads");
    let other = veracity::read_answers(&later, arrangement.instance).expect("reads");

    assert_ne!(
        one.last(),
        other.last(),
        "the tip of one generation and the tip of the next"
    );
}

/// Each party's own record answers what the arrangement says, and the two disagree.
///
/// The disagreement is the recording instant and nothing else: the two journals are equal entry for
/// entry, and the entry they disagree about is the same entry by address.
#[test]
fn each_party_alone_answers_what_the_arrangement_says() {
    let arrangement = veracity::arranged().expect("the subject is admissible");

    let mut journals = Vec::new();

    for (side, expected) in arrangement.sides.iter().zip(ALONE) {
        let repository = scratch(side.label);

        veracity::found(&repository, &arrangement).expect("the base is written");

        let mut working = veracity::read(&repository).expect("a party reads the base");
        veracity::observe(&mut working, side).expect("a party observes and decides");

        veracity::write_whole(&repository, &veracity::files(&working)).expect("a whole write");

        let answers = veracity::read_answers(&repository, arrangement.instance).expect("reads");

        assert_eq!(
            answers,
            vec![expected],
            "{} alone, settled and intended",
            side.label
        );
        assert_eq!(working.journal.len(), SIDE_ENTRIES, "{}", side.label);

        journals.push(working.admitted.entries.clone());
    }

    assert_eq!(
        journals[0], journals[1],
        "the two journals are the same journal by address"
    );
    assert_ne!(
        RECORDED_ON[0], RECORDED_ON[1],
        "and differ in the one value no address contains"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — Calibration: what a false record looks like
//
// Not a finding. An instrument. Every state below is produced by a hand editing a file, which is
// the authenticity candidate's instrument and this experiment's excluded one. It exists so that
// the search knows what its target looks like.
// ---------------------------------------------------------------------------------------------

/// Experiment 01's first tamper, against the record as it now stands: repoint the coordinate.
///
/// It was silent then. The witness arrived four experiments later, for an unrelated reason, and
/// this is the first time anybody has put that tamper back to the record.
#[test]
fn calibration_a_repointed_coordinate_is_refused_by_the_witness() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("repointed");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let mut lineage = repository.read_lineage().expect("the lineage reads");
    let journal = repository.read_journal().expect("the journal reads");

    let elsewhere = addresses(&journal)
        .into_iter()
        .find(|entry| *entry != lineage[0].after)
        .expect("the journal holds another address");

    lineage[0].after = elsewhere;
    repository.write_lineage(&lineage).expect("the hand writes");

    let refusal = refusal(&repository);

    assert!(
        matches!(
            refusal,
            ReadingError::Lineage(
                LineageError::UnwitnessedKnowledge { .. }
                    | LineageError::WitnessedKnowledgeAbsent { .. }
            )
        ),
        "the witness is what refuses it: {refusal}"
    );
}

/// Experiment 01's second tamper: swap two adjacent entries, so the coordinate is carried by order.
#[test]
fn calibration_a_swapped_pair_is_refused() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("swapped");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let mut journal = repository.read_journal().expect("the journal reads");
    let event = position_of_event(&journal);

    journal.swap(event - 1, event);
    repository.write_journal(&journal).expect("the hand writes");

    let refusal = refusal(&repository);

    assert!(
        matches!(refusal, ReadingError::Lineage(_) | ReadingError::Journal(_)),
        "a reordered journal is refused: {refusal}"
    );
}

/// The third exclusion, produced rather than asserted: a record arranged differently.
///
/// Two commitments admitted at the same instant, swapped. Nothing in either identity is a function
/// of where it sits, and the prefix a decision witnesses is a set — so the record is a different
/// file that answers every question the same way. That is not falsity, and the definition says so
/// before the search begins; the swap that **is** refused, one test up, moves an Event.
#[test]
fn a_differently_arranged_record_answers_the_same() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("rearranged");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let untouched = std::fs::read_to_string(repository.journal_path()).expect("the file reads");
    let mut journal = repository.read_journal().expect("the journal reads");

    journal.swap(BASE_ENTRIES - 2, BASE_ENTRIES - 1);
    repository.write_journal(&journal).expect("the hand writes");

    assert_ne!(
        std::fs::read_to_string(repository.journal_path()).expect("the file reads"),
        untouched,
        "the file is not the file that was there"
    );
    assert_eq!(
        veracity::read_answers(&repository, arrangement.instance).expect("the record reads"),
        FAITHFUL.to_vec(),
        "and every answer is what it was"
    );
}

/// The tamper V3 names: move a recording instant, and every address in the record stays intact.
///
/// Two things are measured, and the second is the one nobody had written down. The addresses do not
/// move, so the witness has nothing to disagree with — and the record refuses anyway, because the
/// one derived value the instant decides is `event_head`, and that is written down in `worlds.json`.
#[test]
fn calibration_a_moved_recording_instant_leaves_every_address_intact() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("restamped");

    veracity::write_whole(&repository, &arrangement.earlier).expect("a whole write");

    let mut journal = repository.read_journal().expect("the journal reads");
    let event = position_of_event(&journal);
    let before = addresses(&journal);

    journal[event] = restamped(&journal[event], RECORDED_ON[1]);

    assert_eq!(
        addresses(&journal),
        before,
        "a recording instant belongs to no identity"
    );

    repository.write_journal(&journal).expect("the hand writes");

    let refusal = refusal(&repository);

    assert!(
        matches!(
            refusal,
            ReadingError::WorldDisagrees {
                coordinate: "the chain it recognizes",
                ..
            }
        ),
        "the worlds file is what refuses it, not the witness: {refusal}"
    );
}

/// And a recording instant cannot move freely: the watermark bounds it from both sides.
///
/// Measured because it narrows the space every later phase searches. An instant that overtakes a
/// later entry's makes the **journal** inadmissible, before any decision is weighed — so a false
/// record built on a moved instant has to keep the sequence monotonic, which is a constraint the
/// hypothesis did not know it had.
#[test]
fn calibration_the_watermark_bounds_where_an_instant_can_move() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("overtaken");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let mut journal = repository.read_journal().expect("the journal reads");
    let event = position_of_event(&journal);

    journal[event] = restamped(&journal[event], RECORDED_ON[1]);
    repository.write_journal(&journal).expect("the hand writes");

    let refusal = refusal(&repository);

    assert!(
        matches!(
            refusal,
            ReadingError::Journal(JournalError::Canon(CanonError::RecordedOutOfOrder { .. }))
                | ReadingError::Lineage(LineageError::Journal(JournalError::Canon(
                    CanonError::RecordedOutOfOrder { .. }
                )))
        ),
        "admitting is what refuses, before any decision is weighed: {refusal}"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — The audit: what the record checks, and what it does not
// ---------------------------------------------------------------------------------------------

/// What weighs one field of a repository, and there are five answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weighed {
    /// The field is in the identity admitting produced, so moving it moves the entry's address —
    /// which the witness holds a copy of, and which every later reference resolves against.
    Address,
    /// The witness: a second representation of the same fact, compared on every read.
    Witness,
    /// The worlds file: written down as a derivation and recompared against what the decisions
    /// produce.
    Worlds,
    /// The journal replayed to the coordinate, which is the other half of the witness comparison.
    Replay,
    /// That the party existed at the coordinate, and nothing about whether it is the right one.
    Existence,
}

/// Every field the three files hold, and what weighs it.
///
/// Declared rather than derived, because *what weighs a field* is a claim about the application and
/// not a property of its bytes. What is derived is the **coverage**: the keys below are compared
/// against the keys the files actually hold, so a field added to any record breaks this before it
/// can be quietly unclassified.
const AUDIT: &[(&str, &str, Weighed)] = &[
    ("journal", "admits", Weighed::Address),
    ("journal", "label", Weighed::Address),
    ("journal", "agent", Weighed::Address),
    ("journal", "roles", Weighed::Address),
    ("journal", "effective_from", Weighed::Address),
    ("journal", "kind", Weighed::Address),
    ("journal", "resource", Weighed::Address),
    ("journal", "verb", Weighed::Address),
    ("journal", "actors", Weighed::Address),
    ("journal", "recipients", Weighed::Address),
    ("journal", "action", Weighed::Address),
    ("journal", "fulfills", Weighed::Address),
    ("journal", "cancels", Weighed::Address),
    ("journal", "accountable", Weighed::Address),
    ("journal", "executors", Weighed::Address),
    ("journal", "beneficiaries", Weighed::Address),
    ("journal", "statement", Weighed::Address),
    ("journal", "committed_at", Weighed::Address),
    ("journal", "due_date", Weighed::Address),
    ("journal", "magnitude", Weighed::Address),
    ("journal", "dependencies", Weighed::Address),
    ("journal", "commitment", Weighed::Address),
    ("journal", "observation", Weighed::Address),
    ("journal", "occurred_at", Weighed::Address),
    // The one field of an admission that belongs to no identity.
    ("journal", "recorded_at", Weighed::Worlds),
    ("lineage", "decides", Weighed::Worlds),
    ("lineage", "known_at", Weighed::Worlds),
    ("lineage", "selection", Weighed::Worlds),
    ("lineage", "extends", Weighed::Worlds),
    ("lineage", "omitted", Weighed::Worlds),
    ("lineage", "introduced", Weighed::Worlds),
    ("lineage", "after", Weighed::Witness),
    ("lineage", "witness", Weighed::Replay),
    ("lineage", "by", Weighed::Existence),
    ("worlds", "thesis", Weighed::Replay),
    ("worlds", "thesis_parent", Weighed::Replay),
    ("worlds", "known_at", Weighed::Replay),
    ("worlds", "event_head", Weighed::Replay),
    ("worlds", "frozen", Weighed::Replay),
    ("worlds", "open", Weighed::Replay),
];

/// Every key the three files actually hold, so that a field cannot be added without being audited.
fn keys(repository: &Repository) -> BTreeSet<(String, String)> {
    let files = [
        ("journal", repository.journal_path()),
        ("lineage", repository.lineage_path()),
        ("worlds", repository.worlds_path()),
    ];

    let mut held = BTreeSet::new();

    for (file, path) in files {
        let encoded = std::fs::read_to_string(&path).expect("the file reads");
        let records: Vec<serde_json::Value> =
            serde_json::from_str(&encoded).expect("the file is a sequence of records");

        for record in records {
            for key in record.as_object().expect("a record is an object").keys() {
                held.insert((file.to_owned(), key.clone()));
            }
        }
    }

    held
}

/// The audit covers every field the subject's own files hold, and claims none they do not.
///
/// The subject is built to reach every variant of an admission, so the derived side of this is the
/// whole vocabulary rather than a sample — which is what the assertion on the count is for.
#[test]
fn the_audit_covers_every_field_the_record_holds() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("audited");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let held = keys(&repository);
    let claimed: BTreeSet<(String, String)> = AUDIT
        .iter()
        .map(|(file, field, _)| ((*file).to_owned(), (*field).to_owned()))
        .collect();

    assert!(
        held.len() >= 30,
        "the sweep reached {} fields, which is too few to be the vocabulary",
        held.len()
    );

    let unaudited: Vec<_> = held.difference(&claimed).collect();
    assert!(
        unaudited.is_empty(),
        "the record holds fields the audit does not classify: {unaudited:?}"
    );

    // `by` is optional and absent from a decision that claims nobody, so the audit is allowed to
    // classify a field this particular record does not carry. Nothing else may be.
    let unheld: Vec<_> = claimed
        .difference(&held)
        .filter(|(_, field)| field != "by")
        .collect();
    assert!(
        unheld.is_empty(),
        "the audit classifies fields the record does not hold: {unheld:?}"
    );
}

/// Two of the claims the audit names are weighed less than the hypothesis said, and one more.
///
/// V1 predicted exactly two fields nothing checks, and named `after` and every `recorded_at`. The
/// table above says otherwise on both counts, and each half is measured rather than argued: `after`
/// **is** checked now, `recorded_at` is checked only through a derivation it does not always move,
/// and `by` is a third — an identity that resolves and could be the wrong one, which is the exact
/// standing `after` had before the witness existed.
#[test]
fn the_coordinate_is_checked_and_the_decider_is_not() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("attributed");

    veracity::write_whole(&repository, &arrangement.faithful).expect("a whole write");

    let mut lineage = repository.read_lineage().expect("the lineage reads");
    let ledger = arrangement.sides[0].agent;
    let counterparty = arrangement.sides[1].agent;

    assert_eq!(lineage[0].by, Some(ledger), "the faithful line's writer");

    // The one field a hand can change with nothing to disagree with it: the other party existed at
    // the coordinate, so the claim resolves, and no derivation is a function of who decided.
    lineage[0].by = Some(counterparty);
    repository.write_lineage(&lineage).expect("the hand writes");

    let answers =
        veracity::read_answers(&repository, arrangement.instance).expect("the record still reads");

    assert_eq!(
        answers,
        FAITHFUL.to_vec(),
        "and every answer is what it was"
    );
    assert_eq!(
        reading::decided_by(&repository, counterparty)
            .expect("reads")
            .len(),
        1,
        "while the record now attributes a world to a party that did not decide it"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — The search, one generator at a time
// ---------------------------------------------------------------------------------------------

/// The merge `converge` performs, minus the comparison that decides whether to perform it.
///
/// Written here rather than called, so that what this experiment measured stays measurable after
/// the application changes. It is the same three steps — take the longer journal, hold every
/// decision once, linearize by where each was taken and then by its own content — and the one
/// thing it does not do is ask whether the two journals are the same journal.
///
/// Its agreement with the application's own merge is asserted rather than assumed, by the phase
/// below, and it was asserted before anything was repaired.
fn merge_without_comparing(
    repository: &Repository,
    arrived: &ape_cli::reading::Corroborated,
    held: &ape_cli::reading::Corroborated,
) {
    let journal = if held.admitted.entries.len() >= arrived.admitted.entries.len() {
        held.journal.clone()
    } else {
        arrived.journal.clone()
    };

    let entries = addresses(&journal);
    let mut pending: Vec<ape_cli::lineage::Taken> = Vec::new();

    for side in [arrived, held] {
        for taken in &side.decisions {
            if !pending.contains(taken) {
                pending.push(taken.clone());
            }
        }
    }

    assert!(
        pending
            .iter()
            .all(|taken| taken.decision.extends().is_none()),
        "this arrangement's decisions are all genesis, so the sort is the whole linearization"
    );

    let at = |entry: &EntryId| entries.iter().position(|held| held == entry);
    pending.sort_by(|one, other| (at(&one.after), one).cmp(&(at(&other.after), other)));

    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
    let (lineage, _) = ape_cli::lineage::rebuild(&mut canon, &journal, &pending)
        .expect("the merged record rebuilds");

    let worlds: Vec<_> = lineage
        .decided()
        .iter()
        .map(ape_cli::reading::WorldRecord::of)
        .collect();

    repository
        .write_whole(ape_cli::repository::RepositoryInput {
            journal: &journal,
            lineage: &pending,
            worlds: &worlds,
        })
        .expect("a whole write");
}

/// Both parties read one base, observe, and put back — `last` converging second.
///
/// `through` is how the second party puts back, which is the only thing that differs between the
/// phase that measures the state and the phase that measures what the application does about it.
fn merged(
    arrangement: &veracity::Arranged,
    last: usize,
    named: &str,
    through: impl Fn(&Repository, &ape_cli::reading::Corroborated),
) -> Repository {
    let repository = scratch(&format!("{named}-{last}"));
    let (first, second) = (1 - last, last);

    veracity::found(&repository, arrangement).expect("the base is written");

    // Both parties read the base before either writes, which is what makes them two writers
    // rather than one writer twice.
    let mut held = [
        veracity::read(&repository).expect("a party reads"),
        veracity::read(&repository).expect("a party reads"),
    ];

    for side in [first, second] {
        veracity::observe(&mut held[side], &arrangement.sides[side]).expect("observes and decides");
    }

    converge::converge(&repository, &held[first]).expect("the first party puts back");
    through(&repository, &held[second]);

    repository
}

/// The merge as the laboratory performs it, which is the state the search is about.
fn merged_by_the_laboratory(arrangement: &veracity::Arranged, last: usize) -> Repository {
    merged(arrangement, last, "merged", |repository, held| {
        let arrived = reading::corroborated(repository).expect("the record reads");

        merge_without_comparing(repository, &arrived, held);
    })
}

/// The decisions a repository holds, in the order the merge linearized them, by the party claiming.
fn claimed_by(repository: &Repository, arrangement: &veracity::Arranged) -> Vec<usize> {
    repository
        .read_lineage()
        .expect("the lineage reads")
        .iter()
        .map(|taken| {
            arrangement
                .sides
                .iter()
                .position(|side| Some(side.agent) == taken.by)
                .expect("every decision here claims one of the two parties")
        })
        .collect()
}

/// A merge keeps both decisions, passes every guard, and answers a world neither party decided.
///
/// The journals are the same journal by address, so `converge` finds nothing to refuse; the witness
/// is a set of addresses, so it finds nothing either; and the merge writes its own `worlds.json`,
/// so the third guard weighs a derivation against itself. What survives is the converging party's
/// **recording instant**, imposed on a decision taken under the other one.
#[test]
fn a_merge_re_derives_the_other_party_s_world_and_nothing_refuses() {
    let arrangement = veracity::arranged().expect("the subject is admissible");

    for last in [0usize, 1] {
        let repository = merged_by_the_laboratory(&arrangement, last);
        let by = claimed_by(&repository, &arrangement);

        assert_eq!(by.len(), 2, "both parties' decisions survive the merge");

        let held: Vec<(i128, i128)> = by.iter().map(|side| MERGED[last][*side]).collect();

        assert_eq!(
            says(&repository, &arrangement),
            Says::Answers(held),
            "the merge {} converged last answers",
            arrangement.sides[last].label
        );

        let moved = 1 - last;

        assert_ne!(
            MERGED[last][moved], ALONE[moved],
            "{}'s own world moved",
            arrangement.sides[moved].label
        );
        assert!(
            !ALONE.contains(&MERGED[last][moved]),
            "and it moved to a pair neither party's own record holds"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 8 — Part B: the repair, which closes the state that was found and nothing else
// ---------------------------------------------------------------------------------------------

/// The application's own merge now refuses the state, by a name that says what disagrees.
///
/// The repair is one comparison, and its scope is settled by the addressing rather than by
/// judgement: every other field of an admission is in the identity, so two journals that agree by
/// address can differ in the recording instant and in nothing else. Comparing it therefore closes
/// exactly the state that was found.
///
/// What the repair replaces survives. Every merge that was correct before is correct now — the two
/// journals agree entry for entry *and* instant for instant in every earlier experiment's
/// arrangement — and what stops going through is the one case that produced a wrong answer.
///
/// This phase is also what keeps the laboratory's instrument honest. Before the repair it asserted
/// that `converge` reached the same state the instrument reaches, and it did, for both orderings;
/// the numbers in [`MERGED`] are the application's, measured, and the phase above still reports
/// them because the instrument is the merge minus the comparison this adds.
#[test]
fn the_application_s_own_merge_refuses_a_disagreement_about_an_instant() {
    let arrangement = veracity::arranged().expect("the subject is admissible");

    for last in [0usize, 1] {
        let refused = std::cell::RefCell::new(None);

        let repository = merged(&arrangement, last, "converged", |repository, held| {
            *refused.borrow_mut() = converge::converge(repository, held).err();
        });

        let refusal = refused.into_inner().expect("the merge is refused");

        assert!(
            matches!(refusal, ConvergeError::RecordedDifferently { position, .. }
                if position == SIDE_ENTRIES - 1),
            "at the entry the two parties recorded differently: {refusal}"
        );
        assert_eq!(
            says(&repository, &arrangement),
            Says::Answers(vec![ALONE[1 - last]]),
            "and a refused merge leaves the repository exactly as it was"
        );
    }
}

/// And the repair refuses nothing a merge used to do.
///
/// A party that admits nothing and only decides, and a party that appends knowledge the other does
/// not have, both still converge — the two shapes every earlier experiment's merges are made of.
#[test]
fn the_repair_leaves_every_merge_that_agreed_about_its_instants() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("still-merges");

    veracity::found(&repository, &arrangement).expect("the base is written");

    let mut one = veracity::read(&repository).expect("a party reads");
    let mut other = veracity::read(&repository).expect("a party reads");

    veracity::observe(&mut one, &arrangement.sides[0]).expect("one party observes and decides");

    // The other party admits nothing at all and decides against the base it read, which is the
    // shape a stale reading takes when the two journals stand in extension.
    let taken = ape_cli::lineage::Taken::claimed(
        veracity::founding([arrangement.fund].into()),
        arrangement.sides[1].agent,
        &other.admitted,
    )
    .expect("a decision against what this party read");
    ape_cli::lineage::decide(other.canon.history(), &mut other.lineage, &taken.decision)
        .expect("decidable");
    other.decisions.push(taken);

    converge::converge(&repository, &one).expect("the appending party puts back");
    converge::converge(&repository, &other).expect("and so does the one that only decided");

    assert_eq!(
        repository.read_lineage().expect("the lineage reads").len(),
        2,
        "both lines are there"
    );
}

/// What a state answers, or what it said instead.
///
/// One value rather than two assertions, so that a row of the table cannot be reported as a refusal
/// and as an answer at the same time.
#[derive(Debug, PartialEq)]
enum Says {
    Answers(Vec<(i128, i128)>),
    Refuses(String),
}

fn says(repository: &Repository, arrangement: &veracity::Arranged) -> Says {
    match veracity::read_answers(repository, arrangement.instance) {
        Ok(answers) => Says::Answers(answers),
        Err(refusal) => Says::Refuses(refusal.to_string()),
    }
}

/// The interrupted-write generator: a whole write that never turns its pointer.
///
/// The state it leaves is the previous generation, entire. That is **incomplete** and the definition
/// excludes it: every answer is true of the record that is there.
#[test]
fn an_interrupted_write_leaves_the_previous_generation_whole() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("interrupted");

    veracity::write_whole(&repository, &arrangement.earlier).expect("a whole write");

    let prepared = repository
        .prepare(veracity::input(&arrangement.faithful))
        .expect("the first half of a write");

    assert_eq!(
        says(&repository, &arrangement),
        Says::Answers(FAITHFUL[..EARLIER_WORLDS].to_vec()),
        "a reader reads the generation that is still live"
    );

    prepared.turn().expect("and the second half publishes it");

    assert_eq!(
        says(&repository, &arrangement),
        Says::Answers(FAITHFUL.to_vec()),
        "only the turn changes what a reader reads"
    );
}

/// Which of the eight mixtures answer at all, as bits over [`veracity::File::ALL`].
///
/// `0` is the older generation whole and `7` the newer one; `1` is the newer **journal** over the
/// older lineage and worlds, which is the state experiment 07 named — a record that knows more than
/// anything in it has decided about. It answers the older generation, and every answer is true of
/// what is there.
const MIXTURES_THAT_ANSWER: [u8; 3] = [0, 1, 7];

/// The mixture generator: one file of one generation over another, at the grain a repository writes.
///
/// All eight combinations, enumerated rather than sampled. The two whole ones are each generation;
/// the six between them are what a second writer aimed at somebody else's live generation reaches.
#[test]
fn every_mixture_of_two_generations_is_refused_or_incomplete() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let whole = [
        Says::Answers(FAITHFUL[..EARLIER_WORLDS].to_vec()),
        Says::Answers(FAITHFUL.to_vec()),
    ];

    let mut answered = Vec::new();

    for taken in 0u8..8 {
        let repository = scratch(&format!("mixture-{taken}"));

        veracity::write_whole(&repository, &arrangement.earlier).expect("a whole write");

        for (bit, file) in veracity::File::ALL.into_iter().enumerate() {
            if taken & (1 << bit) != 0 {
                veracity::put(&repository, &arrangement.faithful, file).expect("one file");
            }
        }

        // A consumer breaking, recorded rather than absorbed. Experiment 16 gave the whole write a
        // fourth file — the addresses the journal comes to — and a mixture assembled by single-file
        // writes leaves the older generation's claim over the newer generation's journal, so every
        // mixture that takes the journal is now refused by it. This phase enumerates eight mixtures
        // of THREE files; the fourth is not in that space, and classifying with it present would be
        // measuring 16's repair instead of what a mixture of two generations says.
        std::fs::remove_file(repository.custody_path()).expect("a whole write put one there");

        let said = says(&repository, &arrangement);

        if let Says::Answers(_) = &said {
            assert!(
                whole.contains(&said),
                "mixture {taken} answers something no generation answers: {said:?}"
            );

            answered.push(taken);
        }
    }

    assert_eq!(
        answered, MIXTURES_THAT_ANSWER,
        "which mixtures answer, by the files taken from the newer generation"
    );
}

/// The readmission generator: a journal that admits an address it already holds.
///
/// Two shapes, and they differ in whether anything was learned in between. Neither is false: the
/// first answers what the record without it answers, and the second is refused by name.
#[test]
fn a_readmission_answers_the_same_or_is_refused_by_name() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("readmitted");

    let mut journal = arrangement.faithful.journal.clone();
    let last = journal.last().cloned().expect("the journal is not empty");
    journal.push(last);

    veracity::write_whole(
        &repository,
        &veracity::Files {
            journal,
            lineage: arrangement.faithful.lineage.clone(),
            worlds: arrangement.faithful.worlds.clone(),
        },
    )
    .expect("a whole write");

    assert_eq!(
        says(&repository, &arrangement),
        Says::Answers(FAITHFUL.to_vec()),
        "a readmission with nothing learned in between changes no answer"
    );

    // And the shape experiment 10 named. It needs a decision taken after the *second* occurrence
    // with something genuinely new between the two — a readmission of what the witness already
    // holds changes no set, which is why the case above is silent rather than ambiguous.
    let ambiguous = scratch("ambiguous");
    let mut working = reading::corroborated(&repository).expect("the record reads");

    working.journal.push(arrangement.spare.clone());
    working.journal.push(
        working
            .journal
            .get(FAITHFUL_ENTRIES - 1)
            .cloned()
            .expect("the entry the last decision was taken after"),
    );
    ape_cli::journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)
        .expect("the readmission admits");

    let after = working
        .admitted
        .entries
        .last()
        .cloned()
        .expect("the readmitted address");
    let mut decisions = working.decisions.clone();
    let mut extended = decisions.last().cloned().expect("a decision to extend");

    extended.after = after;
    extended.witness = working.admitted.entries.iter().cloned().collect();
    decisions.push(extended);

    veracity::write_whole(
        &ambiguous,
        &veracity::Files {
            journal: working.journal.clone(),
            lineage: decisions,
            worlds: arrangement.faithful.worlds.clone(),
        },
    )
    .expect("a whole write");

    let refusal = refusal(&ambiguous);

    assert!(
        matches!(
            refusal,
            ReadingError::Lineage(LineageError::ReadmittedEntryIsAmbiguous { .. })
        ),
        "and one with knowledge in between is refused by name: {refusal}"
    );
}

/// The interleaving generator, without a merge: two writers that both prepare before either turns.
///
/// The turn compares against what it wrote, so the writer whose files were overwritten is refused
/// rather than told it succeeded. Nothing false is reachable here, and the reason is a refusal.
#[test]
fn two_writers_that_do_not_merge_lose_a_turn_rather_than_an_answer() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("interleaved");

    let one = repository
        .prepare(veracity::input(&arrangement.earlier))
        .expect("the first writer prepares");
    let other = repository
        .prepare(veracity::input(&arrangement.faithful))
        .expect("the second writer prepares the same generation");

    let refused = one.turn().expect_err("the overwritten writer is refused");
    assert!(
        matches!(refused, ape_cli::error::RepositoryError::Contended { .. }),
        "and it is told so by name: {refused}"
    );

    other
        .turn()
        .expect("the writer whose files are there publishes");

    assert_eq!(
        says(&repository, &arrangement),
        Says::Answers(FAITHFUL.to_vec()),
        "and what a reader reads is one writer's whole state"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 5 — Compositions
// ---------------------------------------------------------------------------------------------

/// A merge over a journal that a stopped write left, which is the composition neither 07 nor 08 ran.
///
/// The interruption leaves the previous generation live, so what the merging party reads is that
/// generation — and the composition adds nothing to the merge on its own: the same disagreement
/// about a recording instant is what decides the outcome, and the interruption only chooses which
/// journal is on the other side of it.
#[test]
fn a_merge_over_an_interrupted_write_reaches_the_same_state() {
    let arrangement = veracity::arranged().expect("the subject is admissible");
    let repository = scratch("composed");

    veracity::found(&repository, &arrangement).expect("the base is written");

    let mut held = [
        veracity::read(&repository).expect("a party reads"),
        veracity::read(&repository).expect("a party reads"),
    ];

    for side in [0, 1] {
        veracity::observe(&mut held[side], &arrangement.sides[side]).expect("observes and decides");
    }

    converge::converge(&repository, &held[0]).expect("the first party puts back");

    // A write that stops before its turn, between the two parties.
    let _abandoned = repository
        .prepare(veracity::input(&arrangement.faithful))
        .expect("a write that never turns");

    let refusal = converge::converge(&repository, &held[1])
        .err()
        .expect("the repair refuses the composition too");

    assert!(
        matches!(refusal, ConvergeError::RecordedDifferently { .. }),
        "for the same reason and at the same entry: {refusal}"
    );

    let arrived = reading::corroborated(&repository).expect("the record reads");
    merge_without_comparing(&repository, &arrived, &held[1]);

    let by = claimed_by(&repository, &arrangement);
    let expected: Vec<(i128, i128)> = by.iter().map(|side| MERGED[1][*side]).collect();

    assert_eq!(
        says(&repository, &arrangement),
        Says::Answers(expected),
        "and the state it refuses is the one a single merge reaches — \
         the abandoned generation adds nothing"
    );
}

// ---------------------------------------------------------------------------------------------
// Helpers shared by the phases
// ---------------------------------------------------------------------------------------------

/// Every address a journal produces, by replaying it into a canon of its own.
fn addresses(journal: &[Admission]) -> Vec<EntryId> {
    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());

    ape_cli::journal::replay(&mut canon, journal)
        .expect("the journal admits")
        .entries
}

/// Where the one Event sits in a journal.
fn position_of_event(journal: &[Admission]) -> usize {
    journal
        .iter()
        .position(|entry| matches!(entry, Admission::Event { .. }))
        .expect("the subject holds an Event")
}

/// The same admission, recorded on another day.
fn restamped(entry: &Admission, on: u8) -> Admission {
    let mut moved = entry.clone();

    if let Admission::Event { recorded_at, .. } = &mut moved {
        *recorded_at = format!("2026-01-{on:02}");
    }

    moved
}
