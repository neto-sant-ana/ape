//! Experiment 01 — Articulation. The instrument, measured before any carving is generated.
//!
//! What is guarded here is the **anchor rule**, because everything downstream is a function of it:
//! a carving can only place a claim where the rule says the claim attaches, so a rule that reaches
//! nothing produces three carvings that differ nowhere and a result that means nothing.
//!
//! The numbers are literals. They were measured before the rule was fixed — which is the one time
//! that is allowed, and the protocol's failure condition is about tuning a carving *after seeing its
//! run*, not about choosing an instrument in the open.

use std::path::{Path, PathBuf};

use ape_succession::articulation::{anchor, record};
use ape_succession::classification::Verdict;
use ape_succession::corpus::Run;
use ape_succession::testimony::reconciliation;

/// The record the protocol names, from the repository root.
fn source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(record::SOURCE)
        .canonicalize()
        .expect("the record this experiment carves is in the workspace")
}

fn opened() -> record::Run {
    record::Run::open(&source()).expect("the run opens")
}

/// **The source is `run-a`, and `run-a` is three arms.**
///
/// Run 1 carved `run-a/mine` alone and lost four housed claims to it — every claim about *the two
/// journals* was unreachable because only one had been carved. The numbers here are the testimony's
/// own arithmetic, and they are what makes those claims derivable at all.
#[test]
fn the_source_is_three_arms_and_they_stand_in_the_relation_the_testimony_describes() {
    let run = opened();

    let shape: Vec<(&str, usize, usize, usize)> = run
        .arms
        .iter()
        .map(|arm| {
            (
                arm.arm,
                arm.journal.len(),
                arm.lineage.len(),
                arm.worlds.len(),
            )
        })
        .collect();

    assert_eq!(
        shape,
        vec![
            ("operations", 20, 3, 3),
            ("finance", 20, 3, 3),
            ("merged", 21, 5, 5),
        ]
    );

    for arm in &run.arms {
        assert_eq!(
            arm.custody.len(),
            arm.journal.len(),
            "{}: one custody address per entry",
            arm.arm
        );
    }

    let (operations, finance, merged) = (&run.arms[0], &run.arms[1], &run.arms[2]);

    let prefix = operations
        .custody
        .iter()
        .zip(&finance.custody)
        .take_while(|(here, there)| here == there)
        .count();

    assert_eq!(
        prefix, 19,
        "the two parties' journals are identical for nineteen entries and part at the twentieth"
    );
    assert_eq!(
        merged.custody[..20],
        operations.custody[..],
        "and the merge is operations' twenty, unchanged and in order"
    );
    assert!(
        finance.custody.contains(&merged.custody[20]),
        "followed by the one entry only finance had"
    );

    for arm in &run.arms {
        assert!(
            !source().join("mine/a").join("designations.json").exists(),
            "no arm holds a designation log, so no carving gets a plan to carve: {}",
            arm.arm
        );
    }
}

/// The claim set is `00-testimony`'s Reconciliation run, and the baseline is its housed claims.
#[test]
fn the_question_set_is_forty_six_claims_of_which_nineteen_are_the_baseline() {
    let claims = reconciliation::CLAIMS;

    assert!(claims.iter().all(|claim| claim.run == Run::Reconciliation));
    assert_eq!(claims.len(), 46, "the question set");

    let housed = claims
        .iter()
        .filter(|claim| matches!(claim.verdict, Verdict::Housed(_)))
        .count();

    assert_eq!(
        housed, 19,
        "the baseline this laboratory measured — the independent classifier said 17, and a carving \
         landing inside that range is no result"
    );
    assert_eq!(
        claims.len() - housed,
        27,
        "the claims a carving has to place"
    );
}

/// **The measurement the rule was chosen against, and the protocol calls it a result.**
///
/// Of the 27 claims a carving has to place, how many attach to anything the record names. The two
/// derivations are reported apart, because the first alone would have decided the experiment:
/// reaching one claim of twenty-seven, it would have put 26 in the overflow of **both** B and C, and
/// two carvings differing by one claim answer nothing.
#[test]
fn nine_of_the_twenty_seven_claims_anchor_and_eighteen_anchor_to_nothing() {
    let run = opened();

    let unplaced: Vec<_> = reconciliation::CLAIMS
        .iter()
        .filter(|claim| !matches!(claim.verdict, Verdict::Housed(_)))
        .map(|claim| anchor::of(claim.text, &run))
        .collect();

    assert_eq!(unplaced.len(), 27);

    let by_identity = unplaced
        .iter()
        .filter(|at| !at.identities.is_empty())
        .count();
    let by_party = unplaced.iter().filter(|at| !at.parties.is_empty()).count();
    let anchored = unplaced.iter().filter(|at| !at.is_empty()).count();

    assert_eq!(
        by_identity, 1,
        "identities alone reach one claim, which is why the rule reads labels too"
    );
    assert_eq!(by_party, 9, "party labels reach nine");
    assert_eq!(anchored, 9, "and the one identity claim is among the nine");
    assert_eq!(
        unplaced.len() - anchored,
        18,
        "so eighteen of twenty-seven anchor to nothing, in every carving alike"
    );
}

/// The housed claims anchor more than the unhoused ones, which says the rule reads the record.
///
/// A control rather than a finding: the 19 housed claims are the ones a reader can already get off
/// the files, so they are the ones most likely to quote the files. If the rule reached those no
/// better than it reaches the rest, it would be matching noise.
#[test]
fn the_rule_reaches_the_housed_claims_far_more_often_than_the_rest() {
    let run = opened();

    let reach = |housed: bool| {
        reconciliation::CLAIMS
            .iter()
            .filter(|claim| matches!(claim.verdict, Verdict::Housed(_)) == housed)
            .filter(|claim| !anchor::of(claim.text, &run).is_empty())
            .count()
    };

    let (of_housed, of_rest) = (reach(true), reach(false));

    assert_eq!(of_housed, 14, "of 19 housed");
    assert_eq!(of_rest, 9, "of 27 that are not");
    assert!(
        of_housed * 27 > of_rest * 19,
        "the rule reaches the claims that quote the record more often than the ones that do not: \
         {of_housed}/19 against {of_rest}/27"
    );
}

/// Every identity a claim names resolves, and resolves to exactly one thing.
///
/// The rule discards an ambiguous prefix rather than guessing, so this is the assertion that it
/// never had to: the corpus abbreviates to eight hex digits and the record holds 26 identities.
#[test]
fn every_identity_the_corpus_names_resolves_uniquely_in_this_record() {
    let run = opened();
    let identities = run.identities();

    assert_eq!(identities.len(), 26, "distinct identities in the record");

    let named: usize = reconciliation::CLAIMS
        .iter()
        .map(|claim| anchor::of(claim.text, &run).identities.len())
        .sum();

    assert_eq!(
        named, 10,
        "ten resolutions across the question set, none of them ambiguous — more than the eight \
         claims that carry them, because two claims name two identities each"
    );

    assert!(
        anchor::of("a claim naming `0000000000` and nothing else", &run)
            .identities
            .is_empty(),
        "and a prefix the record does not hold anchors to nothing rather than to the nearest thing"
    );
}
