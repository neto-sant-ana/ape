//! Experiment 13 — Indexicality. Phases against `lab/frontier/docs/13-indexicality/00-protocol.md`.
//!
//! The question: can a decision name the knowledge it was taken against in a way that survives the
//! journal changing under it — and if it can, is what it then holds a claim the record it arrives
//! in can check?
//!
//! *Resolves correctly* is defined before any phase runs, and it is a comparison rather than a
//! judgement: a coordinate resolves correctly in a record when the knowledge it selects there is
//! the knowledge it selected in the record it was taken in. Knowledge is the entries admitted up to
//! it **and when each was learned**, which is [`indexicality::knowledge`]. It is deliberately not
//! the world produced: experiment 12 measured the two coming apart, and the arrangement keeps a
//! candidate that produces the right world from the wrong knowledge so the distinction cannot be
//! quietly dropped.

use std::collections::BTreeSet;

use ape_cli::error::{ConvergeError, JournalError, LineageError, ReadingError};
use ape_cli::journal::{Admission, EntryId};
use ape_cli::lineage::Taken;
use ape_cli::repository::Repository;
use ape_cli::{converge, reading};

use ape_frontier::subject::indexicality::{
    self, CANDIDATE_ENTRIES, CANDIDATES, ENTRIES, FOREIGN_ENTRIES, KINDS, MOVED_TO, STAGES, Stage,
    WHEN_EARLY, WHEN_LATE,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-indexicality-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    Repository::open(path)
}

/// An address that legitimately exists and that no record in the arrangement holds.
///
/// The harmless Commitment of the fifth candidate, found by difference rather than named by hand.
/// A mutation that moved a coordinate to *one receiver's own* address would be measuring that
/// receiver's luck: the point is a coordinate the journal does not offer, and it has to be one for
/// every receiver alike.
fn nobodys(arrangement: &indexicality::Arranged) -> EntryId {
    let held = |journal: &[Admission]| -> BTreeSet<EntryId> {
        indexicality::addresses(journal)
            .expect("the journal admits")
            .into_iter()
            .collect()
    };

    let mut orphaned = held(&arrangement.candidate("inserted-early").journal);

    for journal in [
        &arrangement.own.journal,
        &arrangement.sibling.journal,
        &arrangement.foreign.journal,
    ] {
        orphaned = orphaned.difference(&held(journal)).cloned().collect();
    }

    let mut found = orphaned.into_iter();
    let address = found
        .next()
        .expect("the insertion is one entry nobody holds");

    assert!(found.next().is_none(), "and it is the only one");

    address
}

/// Read a repository, expecting it to refuse, and hand back what it said.
///
/// A `Corroborated` does not implement `Debug`, and turning one into a string just to satisfy one
/// would be reporting a shape where a phase needs the refusal, named.
fn refusal(repository: &Repository) -> ReadingError {
    match reading::corroborated(repository) {
        Err(refusal) => refusal,
        Ok(_) => panic!("the record was expected to refuse and did not"),
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What the present pin determines
// ---------------------------------------------------------------------------------------------

/// The arrangement holds what it says it holds, and every literal is read once.
#[test]
fn the_arrangement_is_what_the_subject_says_it_is() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    assert_eq!(
        arrangement.own.journal.len(),
        ENTRIES,
        "the record's journal"
    );
    assert_eq!(arrangement.sibling.journal.len(), ENTRIES, "the sibling's");
    assert_eq!(
        arrangement.foreign.journal.len(),
        FOREIGN_ENTRIES,
        "the record founded apart"
    );

    for (candidate, (label, entries)) in arrangement
        .candidates
        .iter()
        .zip(CANDIDATES.into_iter().zip(CANDIDATE_ENTRIES))
    {
        assert_eq!(candidate.label, label, "the candidates are in order");
        assert_eq!(candidate.journal.len(), entries, "{label}");
    }
}

/// The pin a decision holds today: one address, and the set of addresses that stood with it.
///
/// Read here so that every later phase is weighing an addition against something measured rather
/// than against something assumed.
#[test]
fn the_present_pin_is_a_coordinate_and_a_set_of_addresses() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let taken = &arrangement.taken;

    let admitted = indexicality::addresses(&arrangement.own.journal).expect("the journal admits");

    assert_eq!(taken.witness.len(), ENTRIES, "every entry that stood");
    assert_eq!(
        taken.after,
        *admitted.last().expect("the journal is not empty"),
        "the coordinate is the entry the decision followed"
    );
    assert!(
        taken.witness.contains(&taken.after),
        "and the coordinate is inside its own witness"
    );
}

/// Each candidate answers the number the arrangement wrote down for it, before any pin is weighed.
///
/// The falsifiability condition of everything after: two candidates that answered alike would let a
/// phase report a pin determining an answer that was never in doubt.
#[test]
fn each_candidate_answers_the_number_the_arrangement_names() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    for candidate in &arrangement.candidates {
        let resolved = indexicality::resolve(
            &candidate.journal,
            &arrangement.taken.after,
            &arrangement.taken.decision,
            arrangement.instance,
        )
        .expect("every candidate resolves the decision");

        assert_eq!(resolved.answers, candidate.answers, "{}", candidate.label);
    }

    assert_ne!(WHEN_EARLY, WHEN_LATE, "the two answers are two answers");
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — What escapes it
// ---------------------------------------------------------------------------------------------

/// One witness, two journals, and the decision selects different knowledge in them. **I1 refuted.**
///
/// Both journals satisfy the witness the decision carries — the addresses are equal, entry for
/// entry — and the same coordinate reaches a settled fund in one and an open one in the other.
#[test]
fn one_witness_is_satisfied_by_two_journals_that_offer_different_knowledge() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let (early, late) = (
        arrangement.candidate("early"),
        arrangement.candidate("late"),
    );

    let pin = arrangement.pin(Stage::Witnessed).expect("the pin reads");

    for candidate in [early, late] {
        assert!(
            pin.satisfied_by(
                &candidate.journal,
                &arrangement.taken.decision,
                arrangement.instance
            ),
            "{} satisfies the witness the decision carries",
            candidate.label
        );
    }

    let (here, there) = (
        indexicality::knowledge(&early.journal, &arrangement.taken.after),
        indexicality::knowledge(&late.journal, &arrangement.taken.after),
    );

    assert_ne!(here, there, "and the knowledge they offer is not the same");
    assert_eq!(early.answers, WHEN_EARLY);
    assert_eq!(late.answers, WHEN_LATE);
}

/// And a record holding the second one is a repository that reads, answering the other number.
///
/// Which is what makes the refutation about the record rather than about a comparison in a test:
/// the decision is written into a repository whose journal it was not taken against, and every
/// guard the record has passes.
#[test]
fn a_record_holding_the_other_journal_reads_and_answers_differently() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let sibling = scratch("sibling-reads");

    indexicality::write_whole(&sibling, &arrangement.sibling).expect("a whole write");

    let readings = reading::reconstruct(&sibling, arrangement.instance, &indexicality::asked_at())
        .expect("the record reads");

    assert_eq!(readings.len(), 1, "one decision, one world");
    assert_eq!(readings[0].level, WHEN_LATE.0, "what the sibling settles");
    assert_eq!(
        readings[0].event_head, None,
        "because its cut recognizes no Event at all"
    );
}

/// No kind of entry carries its recording instant into its address, and every other field moves it.
///
/// **I2**, over the whole space rather than over the Event the arrangement turns on. The two sides
/// are one measurement: without the second, *the instant escapes* would be a claim about an address
/// that never moves.
#[test]
fn the_recording_instant_is_the_one_field_no_address_covers() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let journal = &arrangement.own.journal;

    let kinds = indexicality::kinds(journal).expect("every kind is derivable");
    assert_eq!(kinds.len(), KINDS, "every kind of entry a journal can hold");

    let address_at = |prefix: &[Admission]| {
        indexicality::addresses(prefix)
            .expect("the prefix admits")
            .last()
            .cloned()
            .expect("the prefix is not empty")
    };

    for kind in &kinds {
        let prefix = &journal[..=kind.at];
        let held = address_at(prefix);

        let mut moved = prefix.to_vec();
        moved[kind.at] = indexicality::dated(&prefix[kind.at], &indexicality::day(MOVED_TO));

        // The instrument first. Both halves of this guard read an address, and a mutation that
        // moved no instant would satisfy the half above without measuring anything — which is what
        // happened when the red pass moved it, and the only phase that noticed was another one.
        assert_ne!(
            moved[kind.at].recorded_at(),
            prefix[kind.at].recorded_at(),
            "{}: the instant was moved before the address was read",
            kind.label
        );

        assert_eq!(
            address_at(&moved),
            held,
            "{}: moving the recording instant left the address where it was",
            kind.label
        );

        let mut altered = prefix.to_vec();
        altered[kind.at] = kind.altered.clone();

        assert_ne!(
            address_at(&altered),
            held,
            "{}: and moving anything else moved it",
            kind.label
        );
    }
}

/// Order escapes the witness too, and changes nothing.
///
/// A witness is a set, so two admissible sequences of one membership satisfy it alike — and here
/// they also offer the same knowledge and produce the same world. Recorded as a measured negative:
/// what escapes the pin is not everything unpinned, and naming the one thing that matters is the
/// point of I2.
#[test]
fn order_escapes_the_witness_and_no_answer_moves_with_it() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let (written, swapped) = (
        arrangement.candidate("early"),
        arrangement.candidate("reordered-early"),
    );

    let addresses =
        |journal: &[Admission]| indexicality::addresses(journal).expect("the journal admits");

    assert_ne!(
        addresses(&written.journal),
        addresses(&swapped.journal),
        "the sequences differ"
    );
    assert_eq!(
        addresses(&written.journal)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        addresses(&swapped.journal)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        "and the memberships do not"
    );
    assert_eq!(
        indexicality::knowledge(&written.journal, &arrangement.taken.after),
        indexicality::knowledge(&swapped.journal, &arrangement.taken.after),
        "so the knowledge the coordinate selects is one body"
    );
    assert_eq!(swapped.answers, written.answers, "and the answer holds");
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — Completing the pin, in stages
// ---------------------------------------------------------------------------------------------

/// What each stage of the pin buys, attributed to that stage.
///
/// A stage determines the reference when every journal satisfying it offers one body of knowledge.
/// The table is the measurement: how many of the five candidates each stage admits, how many bodies
/// of knowledge those offer, and how many worlds they produce.
#[test]
fn each_stage_of_the_pin_is_measured_by_what_it_still_admits() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    for (stage, expected) in Stage::ALL.into_iter().zip(STAGES) {
        let pin = arrangement.pin(stage).expect("the pin reads");

        let admitted: Vec<_> = arrangement
            .candidates
            .iter()
            .filter(|candidate| {
                pin.satisfied_by(
                    &candidate.journal,
                    &arrangement.taken.decision,
                    arrangement.instance,
                )
            })
            .collect();

        let bodies: BTreeSet<_> = admitted
            .iter()
            .map(|candidate| {
                indexicality::knowledge(&candidate.journal, &arrangement.taken.after)
                    .expect("an admitted candidate resolves its coordinate")
            })
            .collect();

        let worlds: BTreeSet<_> = admitted
            .iter()
            .map(|candidate| {
                indexicality::resolve(
                    &candidate.journal,
                    &arrangement.taken.after,
                    &arrangement.taken.decision,
                    arrangement.instance,
                )
                .expect("an admitted candidate produces a world")
                .thesis
                .id()
                .to_string()
            })
            .collect();

        assert_eq!(
            (stage.label(), admitted.len(), bodies.len(), worlds.len()),
            expected,
            "{}: candidates admitted, bodies of knowledge, worlds",
            stage.label()
        );
    }
}

/// The stage that names the world admits a journal whose knowledge is not the decision's.
///
/// Taken on its own rather than on top of the witness, which is what separates *produces the same
/// world* from *resolves correctly*. Experiment 12's insertion is the witness for it, and it is the
/// reason the last row of the table above buys nothing.
#[test]
fn naming_the_world_a_decision_produced_does_not_name_its_knowledge() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let (early, inserted) = (
        arrangement.candidate("early"),
        arrangement.candidate("inserted-early"),
    );

    let world = |journal: &[Admission]| {
        indexicality::resolve(
            journal,
            &arrangement.taken.after,
            &arrangement.taken.decision,
            arrangement.instance,
        )
        .expect("the candidate resolves")
        .thesis
        .id()
        .to_string()
    };

    assert_eq!(
        world(&inserted.journal),
        world(&early.journal),
        "one world, by identity"
    );
    assert_ne!(
        indexicality::knowledge(&inserted.journal, &arrangement.taken.after),
        indexicality::knowledge(&early.journal, &arrangement.taken.after),
        "and two bodies of knowledge under it"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — Handing it to a stranger
// ---------------------------------------------------------------------------------------------

/// The record founded apart holds no address either journal holds. Verified, not arranged.
#[test]
fn the_record_founded_apart_shares_no_address_with_either_journal() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let held = |journal: &[Admission]| -> BTreeSet<EntryId> {
        indexicality::addresses(journal)
            .expect("the journal admits")
            .into_iter()
            .collect()
    };

    let stranger = held(&arrangement.foreign.journal);

    for journal in [&arrangement.own.journal, &arrangement.sibling.journal] {
        assert!(
            stranger.intersection(&held(journal)).next().is_none(),
            "the record founded apart holds nothing this journal holds"
        );
    }
}

/// A record that holds nothing cannot resolve the decision, and no stage of the pin changes that.
///
/// **I3 refuted for a receiver that holds no content.** A pin names addresses, and an address is
/// derived from content one way: a receiver that cannot write the entry cannot produce the entry
/// from its name. The completed pin is a description of a journal the receiver has to already be
/// able to write.
#[test]
fn a_record_that_holds_nothing_cannot_resolve_the_decision_at_any_stage() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    for stage in Stage::ALL {
        let pin = arrangement.pin(stage).expect("the pin reads");

        let described = pin
            .as_described(&arrangement.foreign.journal)
            .unwrap_or_else(|| arrangement.foreign.journal.clone());

        let outcome = indexicality::resolve(
            &described,
            &arrangement.taken.after,
            &arrangement.taken.decision,
            arrangement.foreign_instance,
        );

        assert!(
            matches!(
                outcome,
                Err(ape_cli::error::SubjectError::Journal(
                    JournalError::UnknownEntry(_)
                ))
            ),
            "{}: the journal holds no entry the coordinate names",
            stage.label()
        );
    }
}

/// The receiver that holds the content resolves the decision, and resolves it **wrongly**.
///
/// Every address matches, the witness is satisfied, nothing refuses, and the knowledge the
/// coordinate selects is not the knowledge it selected. This is the state the whole experiment is
/// about, and the pin as a `Taken` carries it does not exclude it.
#[test]
fn the_receiver_that_holds_the_content_resolves_it_and_resolves_it_wrongly() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let sibling = &arrangement.candidate("late").journal;

    let resolved = indexicality::resolve(
        sibling,
        &arrangement.taken.after,
        &arrangement.taken.decision,
        arrangement.instance,
    )
    .expect("the sibling resolves the coordinate");

    assert_eq!(resolved.answers, WHEN_LATE, "and answers the other number");
    assert_ne!(
        indexicality::knowledge(sibling, &arrangement.taken.after),
        indexicality::knowledge(&arrangement.own.journal, &arrangement.taken.after),
        "against knowledge that is not the decision's"
    );
}

/// Given the instants, the same receiver resolves it correctly — by rewriting its own past.
///
/// **I3 confirmed, and only for a receiver that already holds the content.** What the pin supplies
/// is the instants; what the receiver supplies is every entry. So the completed reference is
/// portable exactly as far as the content already is, which is experiment 09's boundary arriving
/// from the other side.
#[test]
fn given_the_instants_the_receiver_resolves_it_correctly_by_rewriting_its_own_past() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let sibling = &arrangement.candidate("late").journal;

    let pin = arrangement.pin(Stage::Dated).expect("the pin reads");
    let described = pin
        .as_described(sibling)
        .expect("a dated pin describes a journal");

    let resolved = indexicality::resolve(
        &described,
        &arrangement.taken.after,
        &arrangement.taken.decision,
        arrangement.instance,
    )
    .expect("the described journal resolves the coordinate");

    assert_eq!(resolved.answers, WHEN_EARLY, "the decision's own answer");
    assert_eq!(
        indexicality::knowledge(&described, &arrangement.taken.after),
        indexicality::knowledge(&arrangement.own.journal, &arrangement.taken.after),
        "and the knowledge it selects is the knowledge it selected"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — What the stranger can check
// ---------------------------------------------------------------------------------------------

/// What a receiver did with a part of the pin that had been moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Answered {
    /// It weighed the part against something it derived, and refused, naming what disagreed.
    Refused,
    /// It produced a world and had nothing to weigh the part against.
    Silent,
    /// It reached no entry of that name — which is also its answer to a pin that is entirely true.
    Unfound,
}

/// Every part of a completed pin, put to *what would the receiving record compare this against?*
///
/// **I4 and I5**, as a closed table over the space of what a pin holds: the two parts a [`Taken`]
/// carries today, the attribution beside them, and the two the laboratory added to complete it.
///
/// The receivers are ordered by how much they hold — the record the decision was taken in, a record
/// holding the same content and one different instant, and a record holding nothing — and the
/// columns are the finding. What a receiver can weigh is exactly what it holds.
///
/// **Two cells were predicted `Refused` and measured `Unfound`**, and the correction is the sharper
/// half of the phase. A moved coordinate is not weighed against anything: the replay walks to an
/// address and does not arrive, which is the same refusal, by the same name, that a record holding
/// nothing gives a pin that is entirely true. The refusal names an address and cannot say which
/// side of the pair is the wrong one — the error is itself indexical.
///
/// Laid out one row per line, which is what the table is for. A reflowed one reads as a list of
/// tuples and the column that carries the finding stops being a column.
#[rustfmt::skip]
const AUDIT: [(&str, Answered, Answered, Answered); 6] = [
    // The pin as written, so that the last column is readable as a constant rather than as a verdict.
    ("nothing moved", Answered::Silent, Answered::Silent, Answered::Unfound),
    ("the coordinate", Answered::Unfound, Answered::Unfound, Answered::Unfound),
    ("an entry out of the witness", Answered::Refused, Answered::Refused, Answered::Unfound),
    ("an entry into the witness", Answered::Refused, Answered::Refused, Answered::Unfound),
    ("the decider, to a party the journal does not hold", Answered::Refused, Answered::Refused, Answered::Unfound),
    ("the decider, to the other party of the record", Answered::Silent, Answered::Silent, Answered::Unfound),
];

#[test]
fn every_part_of_the_pin_is_put_to_what_the_receiver_would_compare_it_against() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let elsewhere = nobodys(&arrangement);
    let witnessed = arrangement
        .taken
        .witness
        .iter()
        .next()
        .cloned()
        .expect("the witness is not empty");

    let moved = |part: &str| -> Taken {
        let mut taken = arrangement.taken.clone();

        match part {
            "nothing moved" => {}
            "the coordinate" => taken.after = elsewhere.clone(),
            "an entry out of the witness" => {
                taken.witness.remove(&witnessed);
            }
            "an entry into the witness" => {
                taken.witness.insert(elsewhere.clone());
            }
            "the decider, to a party the journal does not hold" => {
                taken.by = Some(arrangement.foreign_parties[0])
            }
            "the decider, to the other party of the record" => {
                taken.by = Some(arrangement.parties[1])
            }
            other => panic!("the audit names a part the phase does not move: {other}"),
        }

        taken
    };

    for (part, own, sibling, foreign) in AUDIT {
        let taken = moved(part);

        // A repository is written whether or not it rebuilds, so that one reader classifies every
        // answer. Where the decisions do not produce worlds, the record it was handed keeps its
        // own — which is the state a receiver would actually be in, holding a decision and its own
        // history of what that decision produced.
        let answered = |journal: &[Admission], worlds: &indexicality::Files, named: &str| {
            let repository = scratch(&format!("audit-{named}"));
            let files = indexicality::kept(journal, std::slice::from_ref(&taken)).unwrap_or(
                indexicality::Files {
                    journal: journal.to_vec(),
                    lineage: vec![taken.clone()],
                    worlds: worlds.worlds.clone(),
                },
            );

            indexicality::write_whole(&repository, &files).expect("a whole write");

            match reading::corroborated(&repository) {
                Ok(_) => Answered::Silent,
                Err(ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(
                    _,
                )))) => Answered::Unfound,
                Err(ReadingError::Journal(JournalError::UnknownEntry(_))) => Answered::Unfound,
                Err(_) => Answered::Refused,
            }
        };

        assert_eq!(
            (
                part,
                answered(&arrangement.own.journal, &arrangement.own, "own"),
                answered(
                    &arrangement.sibling.journal,
                    &arrangement.sibling,
                    "sibling"
                ),
                answered(
                    &arrangement.foreign.journal,
                    &arrangement.foreign,
                    "foreign"
                ),
            ),
            (part, own, sibling, foreign),
            "{part}"
        );
    }
}

/// The record that holds nothing answers a true pin and every false one identically.
///
/// Which is the whole of I4 stated as a measurement rather than as a claim about a capability: the
/// last column of the audit is a constant, so nothing in it distinguishes anything. A record that
/// refuses every version of a decision has not checked one.
#[test]
fn a_record_holding_nothing_answers_a_true_pin_and_a_false_one_alike() {
    let answers: BTreeSet<_> = AUDIT.iter().map(|(_, _, _, foreign)| *foreign).collect();

    assert_eq!(
        answers,
        [Answered::Unfound].into_iter().collect(),
        "one answer for every part of the pin, true or moved"
    );
}

/// A refusal that names a missing entry cannot say which side of the pair is missing.
///
/// The two cells the audit predicted wrong, measured against each other. A record holding the right
/// journal and a false coordinate, and a record holding nothing and a true one, are refused by the
/// same guard with the same name — and each names an address, which is the half of the pair that
/// was **not** in doubt. A reader is told the coordinate; what it needs is which journal.
#[test]
fn a_refusal_that_names_a_missing_entry_cannot_say_which_side_is_missing() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let mut moved = arrangement.taken.clone();
    moved.after = nobodys(&arrangement);

    let wrong_pin = scratch("wrong-pin-right-journal");
    indexicality::write_whole(
        &wrong_pin,
        &indexicality::Files {
            journal: arrangement.own.journal.clone(),
            lineage: vec![moved.clone()],
            worlds: arrangement.own.worlds.clone(),
        },
    )
    .expect("a whole write");

    let wrong_journal = scratch("right-pin-wrong-journal");
    indexicality::write_whole(
        &wrong_journal,
        &indexicality::Files {
            journal: arrangement.foreign.journal.clone(),
            lineage: arrangement.own.lineage.clone(),
            worlds: arrangement.foreign.worlds.clone(),
        },
    )
    .expect("a whole write");

    let named = |repository: &Repository| match refusal(repository) {
        ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(entry))) => entry,
        other => panic!("an unexpected refusal — {other}"),
    };

    assert_eq!(named(&wrong_pin), moved.after, "the address the pin names");
    assert_eq!(
        named(&wrong_journal),
        arrangement.taken.after,
        "and the address a true pin names, refused identically"
    );
}

/// The one thing a record holding nothing can weigh is the pin against itself.
///
/// A coordinate outside its own witness describes no journal at all, and saying so needs no
/// journal. It separates **self-contradictory** from **false** and reaches nothing further, which
/// is the distance experiment 01 measured, arriving where a stranger can stand.
#[test]
fn what_a_record_holding_nothing_can_check_is_the_pin_contradicting_itself() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let honest = arrangement.pin(Stage::Witnessed).expect("the pin reads");
    assert!(!honest.contradicts_itself(), "the pin as written");

    let mut impossible = honest.clone();
    impossible
        .witness
        .as_mut()
        .expect("a witnessed pin carries one")
        .remove(&impossible.after);

    assert!(
        impossible.contradicts_itself(),
        "a coordinate outside its own witness"
    );

    let mut merely_false = honest.clone();
    merely_false.recorded = arrangement
        .pin(Stage::Dated)
        .expect("the pin reads")
        .recorded;

    assert!(
        !merely_false.contradicts_itself(),
        "and a pin whose instants are simply untrue is not distinguishable from one whose are not"
    );
}

/// A recording instant is weighed against a world, and only where it moves one.
///
/// **I5, with the addendum this experiment came for.** Every guard in the record weighs a stored
/// value against one derived from what is present, and a recording instant is derived from nothing
/// — so it is reachable only through a world that was derived from it, and only while that world is
/// still there to disagree.
#[test]
fn a_recording_instant_is_weighed_against_a_world_and_only_where_it_moves_one() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");
    let decisions = std::slice::from_ref(&arrangement.taken);

    // Moved, and no world moves: the record has nothing to say, and now claims a fact was learned
    // on a day it was not.
    let relearned = indexicality::founded(
        &indexicality::OWN_NAMES,
        indexicality::FUNDED,
        indexicality::RELEARNED_ON,
    )
    .expect("the base admits");
    let within = scratch("moved-within-the-cut");

    indexicality::write_whole(
        &within,
        &indexicality::kept(&relearned.journal, decisions).expect("a writer keeps this"),
    )
    .expect("a whole write");

    let readings = reading::reconstruct(&within, arrangement.instance, &indexicality::asked_at())
        .expect("the record reads");
    assert_eq!(readings[0].level, WHEN_EARLY.0, "and answers what it did");

    // Moved across the cut, with the world the record already recorded: caught, and caught as a
    // disagreement about a chain rather than about an instant.
    let across = scratch("moved-across-the-cut");
    indexicality::write_whole(
        &across,
        &indexicality::Files {
            journal: arrangement.sibling.journal.clone(),
            lineage: arrangement.own.lineage.clone(),
            worlds: arrangement.own.worlds.clone(),
        },
    )
    .expect("a whole write");

    assert!(
        matches!(
            refusal(&across),
            ReadingError::WorldDisagrees {
                coordinate: "the chain it recognizes",
                ..
            }
        ),
        "the worlds file is what notices"
    );

    // And rewriting that world — which is what any writer does — erases the only thing that
    // noticed. The record reads, and answers the other number.
    let rewritten = scratch("moved-and-rewritten");
    indexicality::write_whole(&rewritten, &arrangement.sibling).expect("a whole write");

    let readings =
        reading::reconstruct(&rewritten, arrangement.instance, &indexicality::asked_at())
            .expect("the record reads");
    assert_eq!(readings[0].level, WHEN_LATE.0, "nothing refuses");
}

// ---------------------------------------------------------------------------------------------
// Phase 5 — The three guards
// ---------------------------------------------------------------------------------------------

/// Which guard answered, where one did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weighed {
    /// `replay_through`: the journal offers no such entry, or offers it too late.
    Coordinate,
    /// `corroborate`: the prefix the journal offers is not the one the decision names.
    Witness,
    /// `reading::corroborated`: the worlds the decisions produce are not the worlds recorded.
    Worlds,
    /// Nothing refused, and the record answered.
    Nothing,
}

/// Every state this experiment produced, put to all three guards. Closed.
///
/// The number beside each state is what it answers where it answers at all. One row in this table
/// answers a number the decision's own record does not, with nothing refusing — and it is the row
/// that is a legitimate record of a legitimate reading, written by nobody in error.
#[rustfmt::skip]
const GUARDS: [(&str, Weighed, Option<i128>); 12] = [
    ("the record as written", Weighed::Nothing, Some(WHEN_EARLY.0)),
    ("the sibling, written whole", Weighed::Nothing, Some(WHEN_LATE.0)),
    ("the sibling's journal under the record's worlds", Weighed::Worlds, None),
    ("the instant moved within the cut", Weighed::Nothing, Some(WHEN_EARLY.0)),
    ("the coordinate moved elsewhere", Weighed::Coordinate, None),
    ("an entry out of the witness", Weighed::Witness, None),
    ("an entry into the witness", Weighed::Witness, None),
    ("the decider unknown to the journal", Weighed::Witness, None),
    ("the decider the record's other party", Weighed::Nothing, Some(WHEN_EARLY.0)),
    ("the journal reordered", Weighed::Nothing, Some(WHEN_EARLY.0)),
    ("the harmless insertion", Weighed::Witness, None),
    ("the record founded apart", Weighed::Coordinate, None),
];

#[test]
fn every_state_is_put_to_the_three_guards() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let elsewhere = nobodys(&arrangement);
    let witnessed = arrangement
        .taken
        .witness
        .iter()
        .next()
        .cloned()
        .expect("the witness is not empty");
    let relearned = indexicality::founded(
        &indexicality::OWN_NAMES,
        indexicality::FUNDED,
        indexicality::RELEARNED_ON,
    )
    .expect("the base admits")
    .journal;

    let with = |taken: Taken, journal: &[Admission]| -> indexicality::Files {
        indexicality::kept(journal, std::slice::from_ref(&taken)).unwrap_or(indexicality::Files {
            journal: journal.to_vec(),
            lineage: vec![taken],
            worlds: arrangement.own.worlds.clone(),
        })
    };

    let moved = |edit: &dyn Fn(&mut Taken)| -> Taken {
        let mut taken = arrangement.taken.clone();
        edit(&mut taken);
        taken
    };

    for (state, expected, answers) in GUARDS {
        let files = match state {
            "the record as written" => arrangement.own.journal.clone(),
            "the sibling, written whole" => arrangement.sibling.journal.clone(),
            "the instant moved within the cut" => relearned.clone(),
            "the journal reordered" => arrangement.candidate("reordered-early").journal.clone(),
            "the harmless insertion" => arrangement.candidate("inserted-early").journal.clone(),
            "the record founded apart" => arrangement.foreign.journal.clone(),
            _ => arrangement.own.journal.clone(),
        };

        let files = match state {
            "the sibling's journal under the record's worlds" => indexicality::Files {
                journal: arrangement.sibling.journal.clone(),
                lineage: arrangement.own.lineage.clone(),
                worlds: arrangement.own.worlds.clone(),
            },
            "the coordinate moved elsewhere" => {
                with(moved(&|taken| taken.after = elsewhere.clone()), &files)
            }
            "an entry out of the witness" => with(
                moved(&|taken| {
                    taken.witness.remove(&witnessed);
                }),
                &files,
            ),
            "an entry into the witness" => with(
                moved(&|taken| {
                    taken.witness.insert(elsewhere.clone());
                }),
                &files,
            ),
            "the decider unknown to the journal" => with(
                moved(&|taken| taken.by = Some(arrangement.foreign_parties[0])),
                &files,
            ),
            "the decider the record's other party" => with(
                moved(&|taken| taken.by = Some(arrangement.parties[1])),
                &files,
            ),
            _ => with(arrangement.taken.clone(), &files),
        };

        let repository = scratch(&format!("guard-{}", state.replace(' ', "-")));
        indexicality::write_whole(&repository, &files).expect("a whole write");

        let (weighed, level) = match reading::reconstruct(
            &repository,
            arrangement.instance,
            &indexicality::asked_at(),
        ) {
            Ok(readings) => (Weighed::Nothing, Some(readings[0].level)),
            Err(ReadingError::Journal(_)) => (Weighed::Coordinate, None),
            Err(ReadingError::Lineage(LineageError::Journal(_))) => (Weighed::Coordinate, None),
            Err(ReadingError::Lineage(_)) => (Weighed::Witness, None),
            Err(
                ReadingError::WorldDisagrees { .. } | ReadingError::LineageLengthDisagrees { .. },
            ) => (Weighed::Worlds, None),
            Err(other) => panic!("{state}: an unclassified refusal — {other}"),
        };

        assert_eq!(
            (state, weighed, level),
            (state, expected, answers),
            "{state}"
        );
    }
}

/// Exactly one state answers a number the decision's own record does not, and nothing refuses it.
///
/// Derived from the table rather than asserted beside it, so that a state added later is counted.
#[test]
fn one_state_answers_another_number_with_nothing_refusing() {
    let misleading: Vec<_> = GUARDS
        .iter()
        .filter(|(_, weighed, answers)| {
            *weighed == Weighed::Nothing && answers.is_some_and(|level| level != WHEN_EARLY.0)
        })
        .map(|(state, ..)| *state)
        .collect();

    assert_eq!(misleading, ["the sibling, written whole"]);
}

// ---------------------------------------------------------------------------------------------
// Phase 6 — What the application would need, and what it already refuses
// ---------------------------------------------------------------------------------------------

/// The one operation that takes another record's journal refuses this one, by name.
///
/// Which is why the misleading state above is not reachable: it is a repository somebody could
/// write, and no operation the application offers writes it. The refusal is experiment 11's, and it
/// was built for a merge rather than for a decision arriving alone.
#[test]
fn the_only_operation_that_takes_another_journal_refuses_the_one_that_misleads() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let ours = scratch("converge-ours");
    let theirs = scratch("converge-theirs");

    indexicality::write_whole(&ours, &arrangement.own).expect("a whole write");
    indexicality::write_whole(&theirs, &arrangement.sibling).expect("a whole write");

    let held = reading::corroborated(&theirs).expect("the sibling reads");

    assert!(
        matches!(
            converge::converge(&ours, &held),
            Err(ConvergeError::RecordedDifferently { .. })
        ),
        "the merge names the entry the two journals disagree about"
    );
}

/// And it refuses a reordering the record itself accepts.
///
/// Sound and not complete, which is the shape experiment 12 measured for the witness. A request
/// rather than a defect: the merge compares two sequences position by position, and the record's
/// own reader was measured above to accept the same journal reordered.
#[test]
fn and_it_refuses_a_reordering_the_records_own_reader_accepts() {
    let arrangement = indexicality::arranged().expect("the subject is admissible");

    let ours = scratch("converge-ordered");
    let theirs = scratch("converge-reordered");

    indexicality::write_whole(&ours, &arrangement.own).expect("a whole write");
    indexicality::write_whole(
        &theirs,
        &indexicality::kept(
            &arrangement.candidate("reordered-early").journal,
            std::slice::from_ref(&arrangement.taken),
        )
        .expect("a writer keeps this"),
    )
    .expect("a whole write");

    let held = reading::corroborated(&theirs).expect("the reordered record reads");

    assert!(
        matches!(
            converge::converge(&ours, &held),
            Err(ConvergeError::Diverged { position: 0, .. })
        ),
        "the merge disagrees at the first entry the two sequences order differently"
    );
}
