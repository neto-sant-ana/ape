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

use ape_cli::lineage::{self, Adoption, Decision, Lineage, Taken};
use ape_cli::reading::{self, ConflictRecord, OutcomeRecord, WorldRecord};
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

/// Phase 3 — The consequence of not knowing.
///
/// A line of thinking is discredited by the only means this repository admits: an Event voids the
/// one commitment that ever travelled out of it. Then the question is asked as an operator would
/// ask it — *which worlds does this reach?* — and answered from the repository.
///
/// This is necessity's last chance. If a reader can find every affected world without knowing
/// where anything came from, the necessity half is refuted.
#[test]
fn phase_3_consequence() {
    let arrangement = provenance::discredited().expect("the arrangement holds");
    let subject = &arrangement.subject;

    let repository = Repository::open(scratch("phase-3"));
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

    let readings = reading::reconstruct(
        &repository,
        subject.instance,
        &Date::parse(EFFECTIVE).expect("a real date"),
    )
    .expect("the repository reconstructs");

    assert_eq!(readings.len(), 6);

    // The fact arrived, and the world that adopted the tooling recognizes it once advanced. The
    // commitment is frozen now — history settled it — and its condition says what happened.
    let recognizing = &readings[5];

    assert_eq!(recognizing.known_at, "2026-01-15");
    assert!(
        recognizing.frozen.contains(&subject.tooling.to_string()),
        "a settled commitment is no longer anyone's to revise"
    );
    assert_eq!(
        recognizing.conditions[&subject.tooling.to_string()].outcome,
        OutcomeRecord::Cancelled,
        "the travelled intention is void"
    );

    // The question an operator actually asks. It is about the commitment, and the repository
    // answers it: every world that selects the tooling is reached by what happened to it.
    let reached: Vec<usize> = readings
        .iter()
        .enumerate()
        .filter(|(_, world)| {
            world.open.contains(&subject.tooling.to_string())
                || world.frozen.contains(&subject.tooling.to_string())
        })
        .map(|(position, _)| position)
        .collect();

    assert_eq!(
        reached,
        [1, 2, 4, 5],
        "the narrow plan, the broad plan, the world that adopted it, and its advancement"
    );

    // The question provenance would answer names a different set, and a smaller one. Only the
    // adopted world took anything from the broad plan; the narrow plan holds the same tooling and
    // donated nothing, and it is reached all the same.
    let donated_to = [4, 5];

    assert!(
        donated_to.iter().all(|world| reached.contains(world)),
        "everything provenance would name is already in the answer"
    );
    assert_ne!(
        reached, donated_to,
        "and the answer is larger than anything provenance would name"
    );
    assert!(
        reached.contains(&1) && !donated_to.contains(&1),
        "the narrow plan is affected and was never a donor"
    );

    // So provenance would not merely add nothing here — it would answer a question nobody asked,
    // and under-report the one they did. What reaches a world is what that world selects, and
    // selection is derived.
}

/// Phase 4 — Record the claim.
///
/// Part A ended refuted, and Part B runs anyway to price what it declined. What it builds does not
/// stay.
///
/// # Why the shape is `stated` and not `computed`
///
/// The protocol left two live. A decision that records only the question and **derives** what it
/// introduced is rejected on principle rather than on taste: if the record holds no content, then
/// whatever Synthesis answers *is* the content, and there is no second representation for a reader
/// to disagree with. It is unfalsifiable by construction, which is the one thing the corroboration
/// experiment's rule forbids outright.
///
/// So the fork stays as it is and the claim is an annotation beside it — which is also the only
/// place Phase 2 left available.
#[test]
fn phase_4_record() {
    let arrangement = provenance::attributed().expect("the arrangement holds");
    let subject = &arrangement.subject;

    let repository = Repository::open(scratch("phase-4"));
    persist(&repository, &arrangement);

    // What it added, named field by field. Only the decision chosen from a transfer carries it: a
    // planner with their own reasons has nothing to say here, and saying nothing is not the same
    // as claiming to have come from nowhere.
    let written: Vec<serde_json::Value> = serde_json::from_str(
        &std::fs::read_to_string(repository.lineage_path()).expect("the file is there"),
    )
    .expect("the lineage is a list");

    let fields = |at: usize| -> BTreeSet<String> {
        written[at]
            .as_object()
            .expect("a decision is an object")
            .keys()
            .cloned()
            .collect()
    };

    let fork = BTreeSet::from([
        "decides".to_owned(),
        "extends".into(),
        "omitted".into(),
        "introduced".into(),
        "after".into(),
        "witness".into(),
    ]);

    for at in [1, 2, 3] {
        assert_eq!(fields(at), fork, "decision {at} claims nothing");
    }

    let mut adopting = fork.clone();
    adopting.insert("from".into());

    assert_eq!(
        fields(4),
        adopting,
        "and the adopted one carries the question"
    );
    assert_eq!(
        written[4]["from"]["base"],
        arrangement.ancestor().id().to_string()
    );
    assert_eq!(
        written[4]["from"]["source"],
        arrangement.broad().id().to_string(),
        "which names the plan the intention actually came from"
    );

    // The first of the corroboration experiment's two questions, answered honestly.
    //
    // *What becomes impossible if this is not preserved?* Nothing. Part A measured that in three
    // phases: the world reproduces, no reader's question depends on it, and an operator asking
    // what a voided commitment reaches gets a better answer without it.
    let unclaimed = provenance::adopted().expect("the arrangement holds");
    let plain = Repository::open(scratch("phase-4-unclaimed"));
    persist(&plain, &unclaimed);

    assert_eq!(
        reading::reconstruct(
            &plain,
            subject.instance,
            &Date::parse(EFFECTIVE).expect("a real date")
        )
        .expect("the unclaimed repository reconstructs")[4]
            .thesis,
        reading::reconstruct(
            &repository,
            subject.instance,
            &Date::parse(EFFECTIVE).expect("a real date")
        )
        .expect("the claimed repository reconstructs")[4]
            .thesis,
        "the same world either way, which is what makes the claim optional"
    );

    // The second question. *What compares it, on every read?* The claim names a Base and a Source;
    // the decision names the Target and the change. So the question is asked again, inside
    // reconstruction, and a Source whose transfer is not this change is refused by name.
    let mut lying = arrangement.decisions.clone();
    lying[4] = Taken::adopting(
        lying[4].decision.clone(),
        &subject.admitted,
        Adoption {
            base: arrangement.ancestor().id(),
            source: arrangement.receiving().id(),
        },
    )
    .expect("a claim is writable");

    let refused = Repository::open(scratch("phase-4-detectable"));
    persist(&refused, &arrangement);
    refused.write_lineage(&lying).expect("writable");

    let complaint = reading::corroborated(&refused)
        .err()
        .expect("a claim naming a world nothing could have been taken from is refused")
        .to_string();

    assert!(
        complaint.contains(&arrangement.receiving().id().to_string())
            && complaint.contains("cannot be applied here"),
        "and the refusal names the world it could not have come from: {complaint}"
    );

    // Note which branch that was. Claiming the receiving line is refused because the transfer from
    // it is **empty** — it already holds everything it decided — so the claim is impossible rather
    // than merely wrong.
    //
    // The other branch, a claim that is possible and predicts a different change, is not exercised
    // here and Phase 5 is why: in this arrangement there is no such claim.
}

/// Phase 5 — Refuse a false claim.
///
/// A claim nothing checks is a story. So every claim the record could carry is written, one at a
/// time, and the read is asked what it makes of each — the whole space rather than a sample,
/// because the question is how much the check discriminates and a sample cannot answer that.
#[test]
fn phase_5_refuse() {
    let arrangement = provenance::attributed().expect("the arrangement holds");
    let truth = arrangement.broad().id();

    let worlds: Vec<_> = arrangement
        .lineage
        .decided()
        .iter()
        .map(|world| world.id())
        .collect();

    // Every Base and Source a claim could name, against the Target the decision already fixes.
    let mut verdicts: Vec<(usize, usize, &str)> = Vec::new();

    for (at_base, base) in worlds.iter().enumerate() {
        for (at_source, source) in worlds.iter().enumerate() {
            let mut claimed = arrangement.decisions.clone();
            claimed[4] = Taken::adopting(
                claimed[4].decision.clone(),
                &arrangement.subject.admitted,
                Adoption {
                    base: *base,
                    source: *source,
                },
            )
            .expect("a claim is writable");

            let repository = Repository::open(scratch(&format!("phase-5-{at_base}-{at_source}")));
            persist(&repository, &arrangement);
            repository.write_lineage(&claimed).expect("writable");

            verdicts.push((
                at_base,
                at_source,
                match reading::corroborated(&repository) {
                    Ok(_) => "accepted",
                    Err(refusal) if refusal.to_string().contains("not a common ancestor") => {
                        "incoherent"
                    }
                    Err(refusal) if refusal.to_string().contains("cannot be applied here") => {
                        "impossible"
                    }
                    Err(refusal) if refusal.to_string().contains("asks for something else") => {
                        "contradicted"
                    }
                    // A claim naming the world the decision is about, or anything descending from
                    // it. At the moment the check runs that world has not been decided, so it is
                    // not in the archive to be named — which excludes every degenerate account
                    // for free, by when things happen rather than by a rule anyone wrote.
                    Err(refusal) if refusal.to_string().contains("absent from the archive") => {
                        "unwritable"
                    }
                    Err(other) => panic!("an unclassified refusal: {other}"),
                },
            ));
        }
    }

    let accepted: Vec<_> = verdicts
        .iter()
        .filter(|(_, _, verdict)| *verdict == "accepted")
        .map(|(base, source, _)| (*base, *source))
        .collect();

    // Two claims are accepted, and only one of them is true.
    assert_eq!(
        accepted,
        [(0, 1), (0, 2)],
        "the narrow plan and the broad plan, over the ancestor"
    );
    assert!(
        accepted.iter().any(|(_, source)| worlds[*source] == truth),
        "the true claim is among them"
    );
    assert!(
        accepted.len() > 1,
        "and so is a claim that is false, with nothing to tell them apart"
    );

    // Which is exactly Phase 1's two non-degenerate accounts, and the degenerate ones needed no
    // rule to exclude: naming the world a decision is about is unwritable, because that world does
    // not exist when the claim is read.
    let found = explanations(&arrangement, 4);
    let non_degenerate: Vec<_> = found
        .iter()
        .filter(|(_, source)| {
            !descends_from(
                arrangement.lineage.archive(),
                worlds[*source],
                arrangement.adopting().id(),
            )
            .expect("ancestry walks")
        })
        .copied()
        .collect();

    assert_eq!(
        accepted, non_degenerate,
        "every rival account is an acceptable claim, and no more"
    );
    assert!(
        verdicts
            .iter()
            .any(|(_, _, verdict)| *verdict == "unwritable"),
        "and the degenerate accounts are refused without anyone having written a rule"
    );

    // So the possibility half is refuted, and precisely: a false claim naming a rival account
    // passes, and there is no repository state that would catch it. Nothing was built wrong — the
    // check does everything a check could do here, and the space it cannot enter is exactly the
    // space provenance was for.
    assert!(
        !verdicts
            .iter()
            .any(|(_, _, verdict)| *verdict == "contradicted"),
        "no claim in this arrangement is contradicted, because every possible one asks the same"
    );
}

/// Phase 6 — What the claim couples.
///
/// Named by measurement rather than described. A claim names a world the claiming world has no
/// ancestry to, and re-asking the question on every read means that world must still be there.
#[test]
fn phase_6_couple() {
    let arrangement = provenance::attributed().expect("the arrangement holds");
    let subject = &arrangement.subject;

    // The adopted world does not descend from the plan it took its intention from. They are on
    // separate branches; that is what made the transfer a transfer.
    assert!(
        !descends_from(
            arrangement.lineage.archive(),
            arrangement.adopting().id(),
            arrangement.broad().id(),
        )
        .expect("ancestry walks"),
        "nothing relates the adopted world to the plan it adopted from"
    );

    // A lineage without that plan. Every world the adopted one needs is still decided — its
    // ancestry is untouched — and the discarded branch is simply not written down.
    let pruned: Vec<_> = [0, 1, 3, 4]
        .into_iter()
        .map(|at| arrangement.decisions[at].clone())
        .collect();

    let unclaimed = Repository::open(scratch("phase-6-unclaimed"));
    unclaimed
        .write_journal(&arrangement.journal)
        .expect("writable");
    unclaimed
        .write_lineage(
            &pruned
                .iter()
                .cloned()
                .map(|mut taken| {
                    taken.from = None;
                    taken
                })
                .collect::<Vec<_>>(),
        )
        .expect("writable");

    let mut rebuilt = Vec::new();
    {
        let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
        let lineage = lineage::rebuild(
            &mut canon,
            &arrangement.journal,
            &unclaimed.read_lineage().expect("reads back"),
        )
        .expect("a pruned lineage still produces its worlds");
        rebuilt.extend(lineage.decided().iter().map(|world| world.id()));
    }
    unclaimed
        .write_worlds(
            &[0, 1, 3, 4]
                .into_iter()
                .map(|at| WorldRecord::of(&arrangement.lineage.decided()[at]))
                .collect::<Vec<_>>(),
        )
        .expect("writable");

    assert_eq!(
        rebuilt[3],
        arrangement.adopting().id(),
        "the adopted world is the same world without the plan it came from"
    );
    assert!(
        reading::reconstruct(
            &unclaimed,
            subject.instance,
            &Date::parse(EFFECTIVE).expect("a real date")
        )
        .is_ok(),
        "and the pruned repository reads"
    );

    // The same pruning, with the claim kept. Now it does not read: the question cannot be asked
    // again, because the world it asks about is gone.
    let claimed = Repository::open(scratch("phase-6-claimed"));
    claimed
        .write_journal(&arrangement.journal)
        .expect("writable");
    claimed.write_lineage(&pruned).expect("writable");
    claimed
        .write_worlds(
            &[0, 1, 3, 4]
                .into_iter()
                .map(|at| WorldRecord::of(&arrangement.lineage.decided()[at]))
                .collect::<Vec<_>>(),
        )
        .expect("writable");

    let complaint = reading::corroborated(&claimed)
        .err()
        .expect("a claim about a world that is gone cannot be checked")
        .to_string();

    assert!(
        complaint.contains("absent from the archive"),
        "and the refusal says the world is not there: {complaint}"
    );

    // So this is what it couples, exactly: **a line of thinking cannot be discarded once anything
    // claims to have adopted from it.** Not because the worlds need it — they do not, measured
    // above — but because a claim re-asked on every read keeps a world alive that nothing else
    // refers to.
    //
    // Four protocols have left abandoned siblings unmodelled. This is the first measurement of what
    // provenance would cost that question, and it costs it everything.
}

/// Phase 7 — Terminate, rebuild, compare.
#[test]
fn phase_7_terminate() {
    let arrangement = provenance::attributed().expect("the arrangement holds");
    let subject = &arrangement.subject;

    let repository = Repository::open(scratch("phase-7"));
    persist(&repository, &arrangement);

    let living = reading::reconstruct(
        &repository,
        subject.instance,
        &Date::parse(EFFECTIVE).expect("a real date"),
    )
    .expect("the repository reconstructs");

    let dead = std::path::Path::new(env!("CARGO_BIN_EXE_ape-cli"));
    let survived = std::process::Command::new(dead)
        .arg(repository.root())
        .arg(subject.instance.to_string())
        .arg(EFFECTIVE)
        .output()
        .expect("the binary runs");

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let rebuilt: Vec<reading::Reading> =
        serde_json::from_slice(&survived.stdout).expect("a lineage came back");

    assert_eq!(rebuilt, living, "whole, across a process boundary");

    // And against literals written before the run, which equality between two derivations cannot
    // do. The claim changed no world, so these are the same five the unclaimed arrangement gives.
    assert_eq!(rebuilt.len(), 5);
    assert_eq!(rebuilt[4].known_at, "2026-01-10");
    assert_eq!(rebuilt[4].open.len(), 4);
    assert!(rebuilt[4].conflicts.is_empty());
    assert_eq!(
        rebuilt[2].conflicts,
        vec![ConflictRecord::OutOfBounds {
            instance: subject.instance.to_string(),
            level: -5.0,
        }]
    );
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
