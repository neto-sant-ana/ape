//! Experiment 17 — Imputation. Phases against `lab/frontier/docs/17-imputation/00-protocol.md`.
//!
//! The question: may a decision say where its intention came from — and is that a claim the record
//! can hold, given that its subject is another record's past?
//!
//! *Came from* is settled before any phase runs: **a decision's intention came from a source when the
//! source is nameable by something the receiving record holds, and naming it asserts nothing the
//! receiving record cannot state truthfully.** Not *who decided* (`by`, which means the party that
//! took it), not *what it extends* (an ancestor in this lineage), and not *what it agrees with*.

use std::collections::{BTreeMap, BTreeSet};

use ape_cli::error::{LineageError, ReadingError, SubjectError};
use ape_cli::lineage::Decision;
use ape_cli::reading;

use ape_frontier::subject::imputation::{
    self, HERE_DECIDES, HERE_DECISIONS, HERE_ENTRIES, Imputed, LACKING, ORIGIN_ENTRIES, RETAKEN,
    RETAKES_AT, SHOWN_ON, TAKEN_DECISIONS, TAKEN_ENTRIES, THERE_DECIDES, THERE_DECISIONS,
    THERE_ENTRIES,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> ape_cli::repository::Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-imputation-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    ape_cli::repository::Repository::open(path)
}

/// What a reader is handed: the four files of a written record, by name.
///
/// Read off the disk rather than encoded in memory, because two records being *equal* is a question
/// about a `PartialEq` somebody derived and this one is about what a reader gets.
fn on_disk(named: &str, files: &imputation::Files) -> BTreeMap<String, String> {
    let repository = scratch(named);
    imputation::write_whole(&repository, files).expect("a whole write");

    [
        repository.journal_path(),
        repository.lineage_path(),
        repository.worlds_path(),
        repository.custody_path(),
    ]
    .into_iter()
    .map(|path| {
        (
            path.file_name()
                .expect("a file has a name")
                .to_string_lossy()
                .to_string(),
            std::fs::read_to_string(&path).expect("the file is there"),
        )
    })
    .collect()
}

/// Which of the four files two records disagree about.
fn differing(one: &BTreeMap<String, String>, other: &BTreeMap<String, String>) -> Vec<String> {
    one.iter()
        .filter(|(name, content)| other.get(*name) != Some(content))
        .map(|(name, _)| name.clone())
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What the record says about origin today
// ---------------------------------------------------------------------------------------------

/// The arrangement holds what it says it holds, and every literal is read once.
#[test]
fn the_arrangement_is_what_the_subject_says_it_is() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    for (label, files, entries, decisions) in [
        (
            "origin",
            &arrangement.origin,
            ORIGIN_ENTRIES,
            THERE_DECISIONS,
        ),
        (
            "authored",
            &arrangement.authored,
            THERE_ENTRIES,
            THERE_DECISIONS,
        ),
        (
            "relayed honestly",
            &arrangement.relayed_honestly,
            THERE_ENTRIES,
            THERE_DECISIONS,
        ),
        ("here", &arrangement.here, HERE_ENTRIES, HERE_DECISIONS),
        (
            "having taken",
            &arrangement.taken,
            TAKEN_ENTRIES,
            TAKEN_DECISIONS,
        ),
    ] {
        assert_eq!(files.journal.len(), entries, "{label}: the journal");
        assert_eq!(files.lineage.len(), decisions, "{label}: the lineage");
        assert_eq!(files.worlds.len(), decisions, "{label}: the worlds");
    }

    assert_eq!(
        imputation::lacking(&arrangement.here.journal, &arrangement.authored.journal)
            .expect("both journals admit")
            .len(),
        LACKING,
        "what the receiving record lacks"
    );

    assert_eq!(
        imputation::answers(&arrangement.here, arrangement.account).expect("the record reads"),
        vec![HERE_DECIDES],
    );
    assert_eq!(
        imputation::answers(&arrangement.authored, arrangement.account).expect("the record reads"),
        THERE_DECIDES.to_vec(),
    );
    assert_eq!(
        imputation::answers(&arrangement.taken, arrangement.account).expect("the record reads"),
        [vec![HERE_DECIDES], RETAKEN.to_vec()].concat(),
    );
}

/// The record says three things that touch origin, and not one of them is where an intention came
/// from.
///
/// Phase 0, read once and as a closed list, so that what a second relation would add is weighed
/// against something measured rather than against an impression.
#[test]
fn the_record_says_three_things_about_origin_and_none_is_where_an_intention_came_from() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let taken = &arrangement.taken;

    // `by` — the party that took it. Every decision this record holds claims nobody, including the
    // two it took from another record, which is what experiment 15 measured a candidate writes.
    assert!(
        taken.lineage.iter().all(|held| held.by.is_none()),
        "every decision here claims nobody"
    );

    // `extends` — an ancestor in THIS lineage. The retaken fork names the world this record produced
    // for the one the original extended, so the only ancestry written down is its own.
    let extended: Vec<Option<String>> = taken
        .lineage
        .iter()
        .map(|held| held.decision.extends().map(|id| id.to_string()))
        .collect();

    assert_eq!(
        extended,
        vec![None, None, Some(taken.worlds[1].thesis.clone())],
        "the fork extends the world this record produced for the one the source extended"
    );
    assert_ne!(
        extended[2].as_deref(),
        Some(arrangement.source_world.to_string().as_str()),
        "and in particular it is not the source's world"
    );

    // `worlds.json` — what this record's own decisions produced, and nothing else.
    let recorded: BTreeSet<&str> = taken
        .worlds
        .iter()
        .map(|world| world.thesis.as_str())
        .collect();

    assert!(
        !recorded.contains(arrangement.source_world.to_string().as_str()),
        "the worlds file names no world this record did not produce"
    );
    assert_eq!(recorded.len(), TAKEN_DECISIONS);
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — Naming the source party
// ---------------------------------------------------------------------------------------------

/// **P1.** The source party resolves against the receiving record, and only after the crossing.
///
/// The control is the whole of criterion 2. `merchant` is a party of the base and resolves before
/// anything crosses; `analyst` is admitted by no base and resolves only once the record has learned
/// it. So what the phase measures is **learning**, not the vocabulary the two records always shared.
#[test]
fn the_source_party_resolves_and_only_because_the_record_learned_it() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let before = imputation::rebuilt(&arrangement.here).expect("the record rebuilds");
    let after = imputation::rebuilt(&arrangement.taken).expect("the record rebuilds");

    assert!(
        imputation::resolves(&before, Imputed::Party(arrangement.merchant)),
        "a party of the base resolves before anything crossed"
    );
    assert!(
        !imputation::resolves(&before, Imputed::Party(arrangement.analyst)),
        "and the source's party does not"
    );
    assert!(
        imputation::resolves(&after, Imputed::Party(arrangement.analyst)),
        "having taken, it does"
    );
}

/// And it resolves in a record that learned the material and took no intention at all.
///
/// Which is where *resolves* comes apart from *checks*, reached from the side nobody looks at: the
/// name resolving is a fact about the journal. A record that was shown another's files and declined
/// every one of its intentions can name the party just as well as one that took them all.
#[test]
fn it_resolves_in_a_record_that_took_no_intention_at_all() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let learned = imputation::learning(&arrangement.here, &arrangement.authored, SHOWN_ON)
        .expect("the record learns");

    assert_eq!(
        learned.journal.len(),
        TAKEN_ENTRIES,
        "it learned everything"
    );
    assert_eq!(
        learned.lineage.len(),
        HERE_DECISIONS,
        "and took no intention"
    );

    assert!(
        imputation::resolves(
            &imputation::rebuilt(&learned).expect("the record rebuilds"),
            Imputed::Party(arrangement.analyst)
        ),
        "the party resolves anyway, because resolving is a fact about the journal"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — Naming the source world
// ---------------------------------------------------------------------------------------------

/// **P2.** The source world does not resolve, and the one guard that resolves a world by name refuses
/// it.
///
/// A world is produced and never admitted, so it is in no journal and reaches no archive but the one
/// that built it. `extends` is the record's only way of naming a world, and it says so by name.
#[test]
fn the_source_world_does_not_resolve_and_extending_it_is_refused() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let after = imputation::rebuilt(&arrangement.taken).expect("the record rebuilds");

    assert!(
        !imputation::resolves(&after, Imputed::World(arrangement.source_world)),
        "the source's world is not in this record's archive"
    );

    let refusal = imputation::extending(&arrangement.taken, arrangement.source_world)
        .expect_err("a world this record never produced has nothing to resolve against");

    assert!(
        matches!(
            refusal,
            SubjectError::Lineage(LineageError::ExtendsUnknownWorld { thesis })
                if thesis == arrangement.source_world
        ),
        "and the refusal names it: {refusal}"
    );

    // The control: a world this record DID produce is extendable, so the refusal above is about the
    // world named and not about the operation.
    let ours = after
        .lineage
        .decided()
        .last()
        .expect("the record decided")
        .id();

    assert!(
        imputation::extending(&arrangement.taken, ours).is_ok(),
        "a world this record produced extends"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — What any of it is weighed against
// ---------------------------------------------------------------------------------------------

/// **P3, and it is refuted in its literal form.** The record checks one thing about a party, and the
/// thing it checks is not nothing.
///
/// `attributed` weighs the party against the knowledge that stood **when the decision was taken**.
/// So a decision taken before the crossing cannot claim the source's party — and that is a class of
/// false provenance the record **rules out**: *you cannot have got this from somebody you had not met
/// yet*. What it cannot do is choose among the parties the record had met.
///
/// Four rows, closed. The one that refuses is the pre-crossing decision; the three that pass include
/// two that are false.
#[rustfmt::skip]
const CLAIMED: [(&str, bool); 4] = [
    ("the source's party, on a decision taken before the crossing", false),
    ("the source's party, on the decisions taken after it",         true),
    ("a party of the base, on a decision taken before it",          true),
    ("a party of the base, on the decisions taken after it",        true),
];

#[test]
fn the_record_rules_out_a_party_it_had_not_met_and_chooses_among_the_rest() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let truthful = imputation::answers(&arrangement.taken, arrangement.account)
        .expect("the record as written reads");

    for (state, accepted) in CLAIMED {
        let party = match state.starts_with("the source's party") {
            true => arrangement.analyst,
            false => arrangement.merchant,
        };
        let first = match state.contains("before") {
            true => 0,
            false => HERE_DECISIONS,
        };

        let claiming = imputation::kept(
            &arrangement.taken.journal,
            &imputation::claiming(&arrangement.taken.lineage, party, first),
        );

        assert_eq!((state, claiming.is_ok()), (state, accepted), "{state}");

        let Ok(files) = claiming else {
            continue;
        };

        let repository = scratch(&format!("claimed-{}", state.replace([' ', ','], "-")));
        imputation::write_whole(&repository, &files).expect("a whole write");

        assert!(
            reading::corroborated(&repository).is_ok(),
            "{state}: and the written record reads"
        );
        assert_eq!(
            imputation::answers(&files, arrangement.account).expect("the record reads"),
            truthful,
            "{state}: every world answers what it answered"
        );
    }
}

/// And the refusal names the party, which is what makes the row above a measurement.
///
/// The check is derived from the journal prefix — the party is or is not among the agents admitted by
/// the coordinate — so experiment 13's premise stands exactly where it said it would: **what a record
/// checks, it derives.** The checkable half of a provenance claim is the derivable half.
#[test]
fn a_party_the_record_had_not_met_is_refused_by_name() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let refusal = imputation::kept(
        &arrangement.here.journal,
        &imputation::claiming(&arrangement.here.lineage, arrangement.analyst, 0),
    )
    .expect_err("the record has not learned that party");

    assert!(
        matches!(
            refusal,
            SubjectError::Lineage(LineageError::DeciderNotKnown { agent })
                if agent == arrangement.analyst
        ),
        "and it names the party: {refusal}"
    );
}

/// **P4.** Nothing the record derives moves when the claim changes. Closed, over all four files.
///
/// The answer to *what is it weighed against* is **nothing**, reported as an answer. Three of the four
/// files a whole write puts on disk are byte-identical between a record claiming nobody and one
/// claiming a party; the fourth is the file the claim is written in.
#[test]
fn nothing_the_record_derives_moves_when_the_claim_changes() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let claiming = imputation::kept(
        &arrangement.taken.journal,
        &imputation::claiming(
            &arrangement.taken.lineage,
            arrangement.analyst,
            HERE_DECISIONS,
        ),
    )
    .expect("the record rebuilds");

    let differs = differing(
        &on_disk("derives-silent", &arrangement.taken),
        &on_disk("derives-claiming", &claiming),
    );

    assert_eq!(
        differs,
        vec!["lineage.json".to_owned()],
        "only the file the claim is written in"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — Whether the writer is in a position to know
// ---------------------------------------------------------------------------------------------

/// **P5.** A record that authored the intention and one that was shown it are the same record.
///
/// Not a question about code: it asks what it would take for a writer to be **wrong**, and the
/// arrangement cannot answer that by being honest — it was built honest. So it builds a relay that
/// holds exactly what the author holds and authored none of it, and compares what a reader is handed.
///
/// Byte-identical, in all four files. The relay learned four entries at the instant it was shown them
/// and retook both intentions in its own frame, and nothing survives of the crossing.
#[test]
fn a_record_that_authored_the_intention_and_one_shown_it_are_the_same_record() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    assert_eq!(
        differing(
            &on_disk("authored", &arrangement.authored),
            &on_disk("relayed-claiming", &arrangement.relayed_claiming),
        ),
        Vec::<String>::new(),
        "an author and a relay that claims the source are one record"
    );
}

/// And a relay that claims nobody differs in exactly the field that already means something else.
///
/// So the record has one lever for saying anything about origin, and it is `by`. Using it truthfully
/// — a candidate claims nobody, which experiment 15 measured — leaves a record indistinguishable from
/// one that reasoned alone; using it the other way makes the relay indistinguishable from the author.
/// Neither state says where the intention came from, and there is no third.
#[test]
fn a_relay_that_claims_nobody_differs_in_exactly_the_field_that_means_something_else() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    assert_eq!(
        differing(
            &on_disk("authored-vs-honest", &arrangement.authored),
            &on_disk("relayed-honestly", &arrangement.relayed_honestly),
        ),
        vec!["lineage.json".to_owned()],
        "the journals, the worlds and the custody claim agree"
    );

    let authored: Vec<_> = arrangement
        .authored
        .lineage
        .iter()
        .map(|held| held.by)
        .collect();
    let relayed: Vec<_> = arrangement
        .relayed_honestly
        .lineage
        .iter()
        .map(|held| held.by)
        .collect();

    assert_eq!(authored, vec![Some(arrangement.analyst); THERE_DECISIONS]);
    assert_eq!(relayed, vec![None; THERE_DECISIONS]);

    // And the difference is only that field: strip it from both and the two lineages coincide.
    assert_eq!(
        imputation::claiming(&arrangement.authored.lineage, arrangement.analyst, 0),
        imputation::claiming(
            &arrangement.relayed_honestly.lineage,
            arrangement.analyst,
            0
        ),
        "everything else about the two decisions is the same decision"
    );
}

/// What it would take for the writer to be wrong, as the state it would be wrong about.
///
/// The relay holds `analyst`'s party and did not get the intention from `analyst` — it got it from a
/// record that had it from `analyst`. Both parties resolve here, and nothing distinguishes them: the
/// writer is in a position to know **who showed it**, and a relation reading *where it came from* is
/// a different sentence.
#[test]
fn the_writer_knows_one_hop_and_both_parties_resolve() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    // A receiver shown the relay rather than the author. What it holds is what it would have held
    // either way, because the two are the same record.
    let onward = imputation::shown(
        &arrangement.here,
        &arrangement.relayed_claiming,
        imputation::Shown {
            learned_at: SHOWN_ON,
            known_at: RETAKES_AT,
            by: None,
        },
    )
    .expect("the record takes");

    assert_eq!(
        differing(
            &on_disk("onward", &onward),
            &on_disk("taken-directly", &arrangement.taken),
        ),
        Vec::<String>::new(),
        "taking from the relay and taking from the author produce one record"
    );

    let rebuilt = imputation::rebuilt(&onward).expect("the record rebuilds");

    assert!(
        imputation::resolves(&rebuilt, Imputed::Party(arrangement.analyst)),
        "the party two hops away resolves"
    );
    assert!(
        imputation::resolves(&rebuilt, Imputed::Party(arrangement.merchant)),
        "and so does one that was never a source at all"
    );
}

/// And a record that took another's intentions is a record that never met anybody.
///
/// Experiment 09's question asked of intentions rather than of knowledge, and it is what settles
/// whether a reader of a record that took is **misled** or merely uninformed. A record that admitted
/// the same four entries itself, and decided the same three times, is byte-identical — built along a
/// path with no crossing and no translation in it.
///
/// So there is no state a reader is misled by: the reader is uninformed, and would be uninformed of a
/// record that had nothing to tell. What a reader could conclude from either — *this record reasoned
/// alone* — is a conclusion neither record supports.
#[test]
fn a_record_that_took_and_a_record_that_never_met_anybody_are_one_record() {
    let arrangement = imputation::arranged().expect("the subject is admissible");
    let alone = imputation::reasoning_alone().expect("a record reasons its way there");

    assert_eq!(
        differing(
            &on_disk("reasoning-alone", &alone),
            &on_disk("taken-for-alone", &arrangement.taken),
        ),
        Vec::<String>::new(),
        "having taken and having never met are the same four files"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 5 — The three guards
// ---------------------------------------------------------------------------------------------

/// Which guard answered, where one did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weighed {
    Coordinate,
    Witness,
    Worlds,
    Custody,
    Nothing,
}

/// Every state this experiment produced, put to all the guards. Closed.
const GUARDS: [(&str, Weighed); 7] = [
    ("origin", Weighed::Nothing),
    ("authored", Weighed::Nothing),
    ("relayed honestly", Weighed::Nothing),
    ("relayed claiming the source", Weighed::Nothing),
    ("here", Weighed::Nothing),
    ("having taken", Weighed::Nothing),
    (
        "having taken, claiming a party of the base",
        Weighed::Nothing,
    ),
];

#[test]
fn every_state_is_put_to_the_guards() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let of_the_base = imputation::kept(
        &arrangement.taken.journal,
        &imputation::claiming(&arrangement.taken.lineage, arrangement.merchant, 0),
    )
    .expect("the record rebuilds");

    for (state, expected) in GUARDS {
        let repository = scratch(&format!("guard-{}", state.replace([' ', ','], "-")));

        let files = match state {
            "origin" => &arrangement.origin,
            "authored" => &arrangement.authored,
            "relayed honestly" => &arrangement.relayed_honestly,
            "relayed claiming the source" => &arrangement.relayed_claiming,
            "here" => &arrangement.here,
            "having taken" => &arrangement.taken,
            "having taken, claiming a party of the base" => &of_the_base,
            other => panic!("the table names a state the phase does not build: {other}"),
        };

        imputation::write_whole(&repository, files).expect("a whole write");

        let weighed =
            match reading::reconstruct(&repository, arrangement.account, &imputation::asked_at()) {
                Ok(_) => Weighed::Nothing,
                Err(ReadingError::Journal(_)) => Weighed::Coordinate,
                Err(ReadingError::Lineage(LineageError::Journal(_))) => Weighed::Coordinate,
                Err(ReadingError::Lineage(_)) => Weighed::Witness,
                Err(
                    ReadingError::UnheldKnowledge { .. } | ReadingError::HeldKnowledgeAbsent { .. },
                ) => Weighed::Custody,
                Err(_) => Weighed::Worlds,
            };

        assert_eq!((state, weighed), (state, expected), "{state}");
    }
}

/// Nothing here reaches the coordinate, the witness or the custody claim, and that is the point.
///
/// A relation about origin is not knowledge, so no guard whose subject is knowledge can bear on it.
/// The phase above reports `Nothing` seven times; this says which seven guards did not fire and why
/// the silence is structural rather than a gap in the arrangement.
#[test]
fn the_relation_is_about_no_part_of_what_the_guards_are_about() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let claiming = imputation::kept(
        &arrangement.taken.journal,
        &imputation::claiming(&arrangement.taken.lineage, arrangement.merchant, 0),
    )
    .expect("the record rebuilds");

    // The coordinate and the witness are functions of the journal, which the claim does not touch.
    assert_eq!(
        imputation::addresses(&claiming.journal).expect("the journal admits"),
        imputation::addresses(&arrangement.taken.journal).expect("the journal admits"),
    );

    for (one, other) in claiming.lineage.iter().zip(&arrangement.taken.lineage) {
        assert_eq!(one.after, other.after, "the coordinate");
        assert_eq!(one.witness, other.witness, "the witness");
        assert_eq!(one.decision, other.decision, "the intention");
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 6 — Where the relation would have to live, measured rather than proposed
// ---------------------------------------------------------------------------------------------

/// The record has one field a party can go in, and it is spoken for.
///
/// Which closes the shape question without proposing anything: a second relation cannot reuse `by`,
/// and every other place a decision writes is an identity the record resolves — a coordinate, a
/// witness, an ancestor. A relation about another record's past resolves against nothing here, and
/// Phase 2 measured the one candidate that would have.
#[test]
fn the_only_place_a_party_goes_is_the_one_that_means_who_took_it() {
    let arrangement = imputation::arranged().expect("the subject is admissible");

    let held = &arrangement.taken.lineage[0];

    // Everything a `Taken` writes down, and what each of them is. Closed: a field added to the type
    // turns this red at the destructuring rather than being silently left out of the reading.
    let ape_cli::lineage::Taken {
        decision,
        after,
        witness,
        by,
    } = held;

    assert!(matches!(decision, Decision::Genesis { .. }), "an intention");
    assert!(!after.to_string().is_empty(), "where the decision stood");
    assert!(!witness.is_empty(), "the prefix it stood on");
    assert_eq!(
        *by, None,
        "and the one party it names, which is the decider"
    );
}
