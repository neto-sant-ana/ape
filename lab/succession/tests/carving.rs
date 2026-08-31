//! Experiment 01 — the three carvings, guarded before any agent is asked about them.
//!
//! What is checked here is that the three are **comparable**: same record, same claims, same words,
//! and no carving holding a link that leaves it. A dangling link would not make a carving worse in
//! some measurable way — it would make the reading of that carving a reading of a different
//! artefact, and the comparison meaningless.
//!
//! Every number is a literal. They were produced by the generator and then written down; the
//! protocol's failure condition is about improving a carving **after its run**, and nothing has run.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ape_succession::articulation::carving::{self, Carving, Page};
use ape_succession::articulation::record::{self, Record};
use ape_succession::classification::Verdict;
use ape_succession::testimony::reconciliation;

fn opened() -> Record {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(record::SOURCE)
        .canonicalize()
        .expect("the record this experiment carves is in the workspace");

    Record::open(&source).expect("the record opens")
}

fn carved(carving: Carving) -> Vec<Page> {
    carving::carve(&opened(), reconciliation::CLAIMS, carving)
}

/// The shape of each carving, and it is what P4 is a prediction about.
#[test]
fn the_three_carvings_are_one_page_sixteen_pages_and_seven_pages() {
    let counted = |carving| carving::placement(&opened(), reconciliation::CLAIMS, carving);

    let (flat, entity, decision) = (
        counted(Carving::Flat),
        counted(Carving::PerEntity),
        counted(Carving::PerDecision),
    );

    assert_eq!(flat.pages, 1, "A is one document");
    assert_eq!(
        entity.pages, 16,
        "B — 4 agents, 4 commitments, 1 event, 5 theses, vocabulary, overflow"
    );
    assert_eq!(
        decision.pages, 7,
        "C — 5 decisions, vocabulary, overflow. The asymmetry P4 predicts is 16 against 7"
    );

    for at in [&flat, &entity, &decision] {
        assert_eq!(at.to_place, 27, "every carving is handed the same claims");
        assert_eq!(at.placed + at.overflowed, at.to_place, "and loses none");
    }
}

/// **A's overflow is zero because A is one document, and that is the null's advantage.**
///
/// Reported as a property of the carving rather than of the placement, because a reader comparing
/// the three would otherwise read *A placed everything* as A having done something.
#[test]
fn a_places_everything_by_having_nowhere_else_to_put_it() {
    let flat = carved(Carving::Flat);

    assert_eq!(flat.len(), 1);
    assert!(
        !flat.iter().any(|page| page.name == "overflow"),
        "A has no overflow page at all — it cannot, and that is the point"
    );

    let placement = carving::placement(&opened(), reconciliation::CLAIMS, Carving::Flat);
    assert_eq!(placement.overflowed, 0);
    assert_eq!(placement.placed, 27);
}

/// **B and C place the same nine claims, and distribute them differently.**
///
/// The finding this pair of numbers carries: the carvings do not differ in *reach* — the anchor rule
/// reaches what it reaches — they differ in **where the same claim ends up and how often**.
#[test]
fn b_and_c_reach_the_same_claims_and_c_repeats_them_five_times_over() {
    let with_claims = |carving| -> Vec<(String, usize)> {
        carved(carving)
            .into_iter()
            .filter(|page| page.name != "overflow")
            .filter_map(|page| {
                let count: usize = page
                    .frontmatter
                    .iter()
                    .find(|(key, _)| key == "claims")
                    .and_then(|(_, value)| value.parse().ok())
                    .unwrap_or(0);

                (count > 0).then_some((page.name, count))
            })
            .collect()
    };

    let entity = with_claims(Carving::PerEntity);
    let decision = with_claims(Carving::PerDecision);

    assert_eq!(entity.len(), 3, "B concentrates them on three pages");
    assert_eq!(decision.len(), 5, "C spreads them over five");

    let placements = |pages: &[(String, usize)]| pages.iter().map(|(_, at)| at).sum::<usize>();

    assert_eq!(placements(&entity), 12, "B writes a claim 12 times");
    assert_eq!(
        placements(&decision),
        23,
        "C writes the same nine claims 23 times, because a party's claims go on every decision it \
         took — the redundancy C pays for having the decision as its unit"
    );
}

/// Every wikilink lands on a page of the same carving.
///
/// The severe one. A carving whose links leave it is not a worse carving — it is a different
/// artefact, and every number measured against it would be measured against something this
/// experiment did not build. It has already caught one: B's agent pages linked to `decision-*`
/// pages, which exist only in C.
#[test]
fn no_carving_holds_a_link_that_leaves_it() {
    for carving in Carving::ALL {
        let pages = carved(carving);
        let names: BTreeSet<&str> = pages.iter().map(|page| page.name.as_str()).collect();

        let mut checked = 0;

        for page in &pages {
            for (key, value) in &page.frontmatter {
                for target in wikilinks(value) {
                    checked += 1;
                    assert!(
                        names.contains(target),
                        "{}/{}.md relates `{key}` to [[{target}]], which is not a page of this \
                         carving",
                        carving.directory(),
                        page.name
                    );
                }
            }
        }

        if carving == Carving::PerEntity {
            assert!(
                checked >= 10,
                "the scan read {checked} links in B; a scan finding none would pass this test \
                 without measuring anything"
            );
        }
    }
}

/// Every claim reaches every carving, verbatim, and none is quietly dropped.
///
/// *Same content, only the carving differs* is the protocol's premise and this is what makes it
/// checkable: the 27 claims appear in A, and in B and C between their pages and their overflow.
#[test]
fn the_same_twenty_seven_claims_appear_in_all_three_carvings() {
    let texts: Vec<&str> = reconciliation::CLAIMS
        .iter()
        .filter(|claim| !matches!(claim.verdict, Verdict::Housed(_)))
        .map(|claim| claim.text)
        .collect();

    assert_eq!(texts.len(), 27);

    for carving in Carving::ALL {
        let whole: String = carved(carving)
            .iter()
            .map(Page::rendered)
            .collect::<Vec<_>>()
            .join("\n");
        let flattened = whole.replace('\n', " ");

        for text in &texts {
            assert!(
                flattened.contains(&text.replace('\n', " ")),
                "{} does not hold, verbatim: {text:.60}…",
                carving.directory()
            );
        }
    }
}

/// The housed claims are in no carving, because they are the baseline rather than reasoning.
///
/// A carving that also wrote them down would be measuring whether a reader prefers to read the same
/// fact twice — and would make every carving beat the baseline by construction.
#[test]
fn no_carving_writes_down_a_claim_the_record_already_houses() {
    let housed: Vec<&str> = reconciliation::CLAIMS
        .iter()
        .filter(|claim| matches!(claim.verdict, Verdict::Housed(_)))
        .map(|claim| claim.text)
        .collect();

    assert_eq!(housed.len(), 19);

    for carving in Carving::ALL {
        let whole: String = carved(carving)
            .iter()
            .map(Page::rendered)
            .collect::<Vec<_>>()
            .join("\n")
            .replace('\n', " ");

        for text in &housed {
            assert!(
                !whole.contains(&text.replace('\n', " ")),
                "{} quotes a housed claim, which would beat the baseline by restating it",
                carving.directory()
            );
        }
    }
}

/// What is on disk is what the generator emits, so an agent reads what this suite measured.
#[test]
fn the_committed_carvings_are_what_the_generator_produces() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the crate is inside the workspace");
    let into = root.join("lab/succession/01-articulation/carvings");

    for carving in Carving::ALL {
        let directory = into.join(carving.directory());

        for page in carved(carving) {
            let on_disk = std::fs::read_to_string(directory.join(format!("{}.md", page.name)))
                .unwrap_or_else(|_| {
                    panic!(
                        "{}/{}.md is missing — re-run `cargo run -p ape-succession --bin carve`",
                        carving.directory(),
                        page.name
                    )
                });

            assert_eq!(
                on_disk,
                page.rendered(),
                "{}/{}.md on disk is not what the generator emits",
                carving.directory(),
                page.name
            );
        }
    }
}

/// The `[[targets]]` a frontmatter value names.
fn wikilinks(value: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = value;

    while let Some(open) = rest.find("[[") {
        let after = &rest[open + 2..];
        let Some(close) = after.find("]]") else { break };

        found.push(&after[..close]);
        rest = &after[close + 2..];
    }

    found
}
