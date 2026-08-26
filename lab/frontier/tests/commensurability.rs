//! Experiment 12 — Commensurability. Phases against
//! `lab/frontier/docs/12-commensurability/00-protocol.md`.
//!
//! The question: can a record say that an insertion cannot have changed what a decision decided —
//! and is the agreement of two records actionable on that basis, without either of them promising
//! less about its own history?
//!
//! *Unchanged* is defined before any phase runs, and it is identity rather than answer: a decision
//! is unchanged by an insertion when the world it produces against the extended journal is the same
//! world **by identity** as the world it produced against the journal it was taken against.

use ape::canon::Canon;
use ape::engine::thesis::ThesisId;

use ape_cli::error::{ConvergeError, LineageError, ReadingError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, EntryId};
use ape_cli::lineage::{self, Taken};
use ape_cli::repository::Repository;
use ape_cli::{converge, reading};

use ape_frontier::subject::commensurability::{
    self, BASE_ENTRIES, CHANGED, OWN, SHARED, SIDE_ENTRIES, SIDE_WORLDS, UNION_ENTRIES,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-commensurability-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    Repository::open(path)
}

/// Rebuild one journal against one lineage, and say what came back.
///
/// The laboratory's own rebuild rather than a repository's, because every phase here weighs a
/// journal the arrangement assembled against decisions two records took — which is not a state any
/// operation the application has can put on disk.
fn rebuilt(
    journal: &[Admission],
    decisions: &[Taken],
) -> Result<(Canon<ResidentHistory>, Vec<ThesisId>), LineageError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, _) = lineage::rebuild(&mut canon, journal, decisions)?;
    let decided = lineage.decided().iter().map(|world| world.id()).collect();

    Ok((canon, decided))
}

/// What every world a rebuild produced answers, oldest first.
fn answers(
    journal: &[Admission],
    decisions: &[Taken],
    arrangement: &commensurability::Arranged,
) -> Vec<(i128, i128)> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, _) =
        lineage::rebuild(&mut canon, journal, decisions).expect("the record rebuilds");

    lineage
        .decided()
        .iter()
        .map(|thesis| {
            commensurability::answers(canon.history(), thesis, arrangement.instance)
                .expect("the world reads")
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — The two records, and what each answers
// ---------------------------------------------------------------------------------------------

/// Two records founded apart admitted the same base, entry for entry, with nothing between them.
///
/// The collision experiment's twinning, reproduced because every later phase rests on it: what makes
/// the two records comparable at all is that they said the same thing, not that anything copied.
#[test]
fn the_two_records_share_a_base_by_content_and_a_world_by_identity() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let [one, other] = &arrangement.sides;

    assert_eq!(&one.entries[..BASE_ENTRIES], arrangement.shared.as_slice());
    assert_eq!(
        &other.entries[..BASE_ENTRIES],
        arrangement.shared.as_slice(),
        "the two bases are the same base by address"
    );

    assert_eq!(one.entries.len(), SIDE_ENTRIES[0]);
    assert_eq!(other.entries.len(), SIDE_ENTRIES[1]);
    assert_ne!(
        one.beyond(BASE_ENTRIES),
        other.beyond(BASE_ENTRIES),
        "and they diverge at the entry each of them added"
    );

    for side in &arrangement.sides {
        let (_, decided) = rebuilt(&side.files.journal, &side.files.lineage)
            .expect("each record rebuilds on its own");

        assert_eq!(decided.len(), SIDE_WORLDS, "{}", side.label);
        assert_eq!(
            decided[0], arrangement.agreed,
            "{} decided the shared world",
            side.label
        );
    }
}

/// Each record alone answers what the arrangement says, and the four worlds are three answers.
#[test]
fn each_record_alone_answers_what_the_arrangement_says() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");

    for (side, expected) in arrangement.sides.iter().zip(OWN) {
        let held = answers(&side.files.journal, &side.files.lineage, &arrangement);

        assert_eq!(
            held,
            vec![SHARED, expected],
            "{}: the shared world, then its own",
            side.label
        );
    }
}

/// The union of the two journals is itself an admissible journal.
///
/// The collision experiment established it and it is carried forward rather than found: what the
/// refusal below is about is the **lineages**, not the knowledge.
#[test]
fn the_union_of_the_two_journals_admits() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let union = arrangement.union();

    assert_eq!(union.len(), UNION_ENTRIES);

    let mut canon = Canon::new(ResidentHistory::new());
    let replayed = journal::replay(&mut canon, &union).expect("the union admits");

    assert_eq!(
        replayed.entries.len(),
        UNION_ENTRIES,
        "every entry of it produced an address"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — The three refusals, reproduced by value
// ---------------------------------------------------------------------------------------------

/// Request 1's block: the only operation there is refuses all-or-nothing, at what they share.
#[test]
fn the_meeting_is_refused_at_the_first_entry_the_two_do_not_share() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let [there, here] = ["there", "here"].map(scratch);

    for (repository, side) in [&there, &here].into_iter().zip(&arrangement.sides) {
        commensurability::write_whole(repository, &side.files).expect("a record is there");
    }

    // What the second record holds, read the way a party holds it — and put back to the first,
    // which is the only operation the application has for two records meeting.
    let held = reading::corroborated(&here).expect("the second record reads");
    let refusal = converge::converge(&there, &held).expect_err_or_panic();

    assert!(
        matches!(refusal, ConvergeError::Diverged { position, .. } if position == BASE_ENTRIES),
        "and the position of the refusal is the size of what they have in common: {refusal}"
    );
    assert_eq!(
        answers(
            &arrangement.sides[0].files.journal,
            &arrangement.sides[0].files.lineage,
            &arrangement
        ),
        vec![SHARED, OWN[0]],
        "and a refused meeting leaves the first record exactly as it was"
    );
}

/// Request 1's sharp form: the union with both lineages is refused, and by the witness.
///
/// This is the state the whole experiment is about. The journal admits, every decision resolves its
/// coordinate, and one of the two records is refused because entries it never saw stand in front of
/// the entry it was taken after.
#[test]
fn the_union_with_both_lineages_is_refused_by_the_witness() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let union = arrangement.union();
    let decisions = arrangement.lineages(&union);

    let refusal = rebuilt(&union, &decisions).expect_err_or_panic();

    let entry = match &refusal {
        LineageError::UnwitnessedKnowledge { entry } => entry.clone(),
        other => panic!("the witness is what refuses it, and it said: {other}"),
    };

    assert!(
        arrangement.sides[0].beyond(BASE_ENTRIES).contains(&entry),
        "the entry it names belongs to the other record: {entry}"
    );
}

/// Request 3's block: agreement is representable only as repetition.
#[test]
fn the_agreed_world_is_held_twice_because_two_records_of_one_decision_are_two_facts() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let union = arrangement.union();
    let decisions = arrangement.lineages(&union);

    let agreed: Vec<&Taken> = decisions
        .iter()
        .filter(|taken| taken.after == arrangement.shared[BASE_ENTRIES - 1])
        .collect();

    assert_eq!(
        agreed.len(),
        2,
        "the same decision, taken at the same coordinate, claimed by two parties"
    );
    assert_eq!(
        agreed[0].decision, agreed[1].decision,
        "and the decision is the same decision"
    );
    assert_ne!(agreed[0].by, agreed[1].by, "only the party differs");
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — The claim, derived
// ---------------------------------------------------------------------------------------------

/// The worlds a lineage produces against a journal, with the witness not consulted.
///
/// The counterfactual the definition names — *the world it produces against the extended journal* —
/// and skipping the witness is the whole of what makes it one: the witness is what refuses the
/// extended journal, so asking what a decision would produce is asking what stands behind that
/// refusal. Everything else is the application's own path, in the application's own order.
///
/// It needs nothing the record does not hold: a journal, a coordinate, and a decision.
fn produced_without_the_witness(
    journal: &[Admission],
    decisions: &[Taken],
) -> Result<Vec<ThesisId>, LineageError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut admitted = journal::Replayed::default();
    let mut lineage = lineage::Lineage::new();

    for taken in decisions {
        journal::replay_through(&mut canon, journal, &mut admitted, &taken.after)?;
        lineage::decide(canon.history(), &mut lineage, &taken.decision)?;
    }

    Ok(lineage.decided().iter().map(|world| world.id()).collect())
}

/// *This insertion cannot have changed what I decided* is derivable, and it tells the two apart.
///
/// Two insertions, both belonging to the other record, both in front of this record's coordinate,
/// both refused by the witness today. The derivation answers differently for them, which is M1 and
/// M2 together: it decides, and it decides in both directions.
#[test]
fn the_claim_is_derivable_and_tells_the_two_insertions_apart() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let receiving = &arrangement.sides[1];

    let alone = produced_without_the_witness(&receiving.files.journal, &receiving.files.lineage)
        .expect("the record produces its own worlds");

    let with_the_commitment = produced_without_the_witness(
        &arrangement.union_without_the_event(),
        &receiving.files.lineage,
    )
    .expect("and produces them against a journal holding one more commitment");

    let with_the_event =
        produced_without_the_witness(&arrangement.union(), &receiving.files.lineage)
            .expect("and against one holding an Event as well");

    assert_eq!(
        with_the_commitment, alone,
        "a Commitment nothing selects cannot have changed what this record decided"
    );
    assert_ne!(
        with_the_event, alone,
        "and an Event, which nothing selects either, can"
    );
    assert_eq!(
        with_the_event[0], alone[0],
        "the shared world is untouched — the coordinate it names is in front of both insertions"
    );
}

/// And what changes is a world nobody decided, by a number.
///
/// The Event settles a commitment this record never selected, so the commitment enters the frozen
/// past of a world that does not hold it, and the level moves. That is the hazard experiment 10
/// handed over and did not reach.
#[test]
fn the_event_drags_a_commitment_into_a_world_that_never_selected_it() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let receiving = &arrangement.sides[1];

    let held = answers(
        &receiving.files.journal,
        &receiving.files.lineage,
        &arrangement,
    );
    assert_eq!(held, vec![SHARED, OWN[1]], "what this record answers alone");

    let mut canon = Canon::new(ResidentHistory::new());
    let mut admitted = journal::Replayed::default();
    let mut lineage = lineage::Lineage::new();
    let union = arrangement.union();

    for taken in &receiving.files.lineage {
        journal::replay_through(&mut canon, &union, &mut admitted, &taken.after)
            .expect("the coordinate resolves");
        lineage::decide(canon.history(), &mut lineage, &taken.decision)
            .expect("the decision applies");
    }

    let changed = lineage.decided().last().expect("this record's own world");

    assert_eq!(
        commensurability::answers(canon.history(), changed, arrangement.instance)
            .expect("it reads"),
        CHANGED,
        "and what it answers with the Event in front of its coordinate"
    );
    assert!(
        changed
            .selection()
            .frozen()
            .any(|id| id == arrangement.plans[0]),
        "the other record's plan is frozen into a world that never selected it"
    );
}

/// Whether one admission names an address anywhere in what it holds.
///
/// Over the encoded record rather than over the variants, because what is wanted is *any* reference
/// however the field is spelled, and an address is 64 hex characters — long enough that a match is
/// a reference and not a coincidence, which the sanity assertion below is what says.
fn references(admission: &Admission, address: &EntryId) -> bool {
    let encoded = serde_json::to_string(admission).expect("an admission encodes");
    let address = address.to_string();

    assert_eq!(address.len(), 64, "an address is a sha256 in hex");

    encoded.contains(&address)
}

/// Nothing in the receiving record references either insertion — so no closure over it can see one.
///
/// This is what decides which derivation answers 09's question, and it says the assumed one does
/// not. Experiment 10 established that what a decision is **about** is derivable, and three results
/// have carried the reading that recording it would make a partial meeting statable. It would not:
///
/// an insertion is, by construction, an entry the receiving record never admitted — so nothing in
/// that record resolves against it, and no reach closure over any of its worlds contains it. A
/// derivation over what a record depends on therefore answers *harmless* for **every** insertion,
/// including the one that is not. That is not a check.
///
/// The positive control is what keeps this from being a scan that found nothing: the fund **is**
/// referenced, by the Event that settles it, and the same sweep finds it.
#[test]
fn no_closure_over_the_receiving_record_can_see_an_insertion() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let receiving = &arrangement.sides[1];
    let inserted = arrangement.sides[0].beyond(BASE_ENTRIES);

    assert_eq!(inserted.len(), 2, "a commitment and an Event");

    for address in inserted {
        let naming: Vec<usize> = receiving
            .files
            .journal
            .iter()
            .enumerate()
            .filter(|(_, admission)| references(admission, address))
            .map(|(position, _)| position)
            .collect();

        assert!(
            naming.is_empty(),
            "an entry of the receiving record names an insertion at {naming:?}, which cannot happen"
        );
    }

    // And the sweep is able to find a reference when there is one.
    let fund = EntryId::of(arrangement.fund);
    let naming = receiving
        .files
        .journal
        .iter()
        .filter(|admission| references(admission, &fund))
        .count();

    assert_eq!(
        naming, 1,
        "the Event that settles the fund names it, and the same sweep sees that"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — What admits, and Phase 5 — the two boundaries
// ---------------------------------------------------------------------------------------------

/// Which decisions of a lineage produce, against `journal`, the world their record recorded.
///
/// The partial meeting stated as a question rather than as an operation: it selects nothing and
/// merges nothing, and what it returns is the subset an operation *could* carry. Naming the
/// operation is Part B's, and the collision experiment excluded it from the request on purpose.
fn unchanged_by(journal: &[Admission], side: &commensurability::Side) -> Vec<usize> {
    let recorded = produced_without_the_witness(&side.files.journal, &side.files.lineage)
        .expect("a record produces its own worlds");

    (0..side.files.lineage.len())
        .filter(|position| {
            produced_without_the_witness(journal, &side.files.lineage[..=*position])
                .is_ok_and(|produced| produced.last() == recorded.get(*position))
        })
        .collect()
}

/// A partial meeting is statable, and it carries three of the four decisions.
///
/// M3. The union is one journal, and each decision either produces the world its record recorded
/// against it or does not. Three do. What is left over is one decision — which is the answer to
/// *put together the part we agree about, and tell me what is left*.
#[test]
fn a_partial_meeting_carries_every_decision_the_union_leaves_unchanged() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let union = arrangement.union();

    let carried: Vec<(&str, Vec<usize>)> = arrangement
        .sides
        .iter()
        .map(|side| (side.label, unchanged_by(&union, side)))
        .collect();

    assert_eq!(
        carried,
        vec![("one", vec![0, 1]), ("other", vec![0])],
        "both of the first record's worlds and only the shared one of the second's"
    );
}

/// And which record loses a world is decided by recording order, not by either record.
///
/// The asymmetry is the price, and it is not a preference: a journal is a sequence, recording is
/// monotonic across admission, and the entries that sort **first** are the ones in front of the
/// other record's coordinate. The record whose knowledge is dated later is the one whose own world
/// the meeting cannot carry — which is the same field experiment 11 measured a merge adopting.
#[test]
fn the_record_whose_knowledge_is_dated_later_is_the_one_that_loses_a_world() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let union = arrangement.union();
    let entries = commensurability::addresses(&union);

    let [one, other] = &arrangement.sides;
    let first = entries
        .iter()
        .position(|entry| one.beyond(BASE_ENTRIES).contains(entry))
        .expect("the first record's entries are in the union");
    let second = entries
        .iter()
        .position(|entry| other.beyond(BASE_ENTRIES).contains(entry))
        .expect("the second record's entries are in the union");

    assert!(first < second, "the first record's entries sort first");
    assert_eq!(
        unchanged_by(&union, one).len(),
        one.files.lineage.len(),
        "so it keeps every world"
    );
    assert!(
        unchanged_by(&union, other).len() < other.files.lineage.len(),
        "and the other does not"
    );
}

/// The witness is sound about worlds and refuses more than it has to.
///
/// Phase 5, and the boundary that decides the answer. Two insertions, both refused by the witness;
/// re-derivation says one of them changes nothing. So the witness never admits a changed world —
/// which is what makes it safe — and refuses unchanged ones, which is what makes 09's request
/// legitimate rather than a wish for a looser check.
#[test]
fn the_witness_never_admits_a_changed_world_and_refuses_unchanged_ones() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let receiving = &arrangement.sides[1];

    let table = [
        (
            "the commitment alone",
            arrangement.union_without_the_event(),
        ),
        ("the commitment and the Event", arrangement.union()),
    ];

    for (named, journal) in table {
        let refused = rebuilt(&journal, &receiving.files.lineage).is_err();

        assert!(
            refused,
            "the witness refuses {named}, whether or not it mattered"
        );
    }

    assert_eq!(
        unchanged_by(&arrangement.union_without_the_event(), receiving).len(),
        receiving.files.lineage.len(),
        "and re-derivation carries both of this record's worlds past the commitment alone"
    );
}

/// The partial meeting computes, and its result is not a repository.
///
/// The answer to the whole question, and it is a refusal rather than a count. A carried decision's
/// `witness` says *these entries stood when I was taken*, and after a meeting the journal holds more
/// than that in front of its coordinate — so the field is not unverifiable, as it was after the
/// merge experiment 10 measured. It is **false**.
///
/// Which leaves an application two moves and no third: keep the witness and the record refuses
/// itself, or rewrite it and the record says a prefix stood that did not. The second is the one
/// thing this laboratory has refused since experiment 02.
#[test]
fn a_partial_meeting_produces_a_record_that_refuses_itself() {
    let arrangement = commensurability::arranged().expect("the subject is admissible");
    let receiving = &arrangement.sides[1];
    let journal = arrangement.union_without_the_event();

    assert_eq!(
        unchanged_by(&journal, receiving).len(),
        receiving.files.lineage.len(),
        "every world this record decided survives the insertion, by identity"
    );

    let repository = scratch("carried");
    let worlds = {
        let mut canon = Canon::new(ResidentHistory::new());
        let (lineage, _) = lineage::rebuild(
            &mut canon,
            &receiving.files.journal,
            &receiving.files.lineage,
        )
        .expect("the record rebuilds on its own");

        commensurability::worlds(&lineage)
    };

    commensurability::write_whole(
        &repository,
        &commensurability::Files {
            journal,
            lineage: receiving.files.lineage.clone(),
            worlds,
        },
    )
    .expect("the three files are written");

    let refusal = reading::corroborated(&repository).expect_err_or_panic();

    assert!(
        matches!(
            refusal,
            ReadingError::Lineage(LineageError::UnwitnessedKnowledge { .. })
        ),
        "and reading it back is refused by the witness, whose claim is now false: {refusal}"
    );
}

/// A convenience for the one refusal a `Corroborated` cannot be `expect_err`'d out of.
trait ExpectErrOrPanic<E> {
    fn expect_err_or_panic(self) -> E;
}

impl<T, E> ExpectErrOrPanic<E> for Result<T, E> {
    fn expect_err_or_panic(self) -> E {
        match self {
            Err(refusal) => refusal,
            Ok(_) => panic!("this was expected to be refused and was not"),
        }
    }
}
