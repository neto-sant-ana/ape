//! Phase 3 — what turns the classification from a reading into a measurement.
//!
//! A classification is prose about prose, and this row's stated hazard is that the person doing it
//! wants a particular answer. None of these guards checks whether a judgement is *apt* — that is a
//! reading, and it belongs to whoever reviews the diff, and to the second classifier in Phase 4.
//!
//! What they check is everything around the judgement that can be wrong without anybody noticing:
//!
//! - **The corpus is whole.** Every `ANSWER.md` the laboratory holds is a run this crate names, read
//!   from the filesystem rather than listed, so a ninth testimony cannot arrive unclassified in
//!   silence.
//! - **The quotations are real.** Every claim's text is found in the file it is attributed to. This
//!   is the one that matters most: a classification of words nobody said would satisfy every other
//!   guard here.
//! - **The carriers exist.** A housed claim naming a kernel entity is checked against `core/`'s
//!   sources, so *housed by `Committment`* fails rather than reassuring.
//! - **Coverage is stated.** An unfinished Phase 2 says which testimonies are still unread instead of
//!   reporting agreement over the part that is done.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ape_succession::classification::{Carrier, Kind, Standing, Verdict};
use ape_succession::corpus::Run;
use ape_succession::testimony::{classified, unread};

fn laboratory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Markdown formatting removed and whitespace collapsed, so a quotation matches across a line wrap.
///
/// The comparison has to survive four things that are formatting rather than content: the corpus is
/// hard-wrapped, so a sentence spans lines; the classification stores it as one line; a claim quoted
/// out of a table carries the cell pipes with it; and one quoted out of a blockquote carries a `>` at
/// the start of every line. Stripping those keeps the words and their order, which is what *verbatim*
/// is being asked to mean here.
///
/// **The blockquote marker is stripped per line rather than everywhere**, and the distinction is not
/// pedantry: `cash >= 0` appears in the corpus, and a filter that removed every `>` would quietly
/// turn it into `cash = 0` in the message a reader is shown when a quotation fails.
fn flattened(text: &str) -> String {
    let unquoted: Vec<&str> = text
        .lines()
        .map(|line| line.trim_start().trim_start_matches('>').trim_start())
        .collect();

    let kept: String = unquoted
        .join(" ")
        .chars()
        .filter(|glyph| !matches!(glyph, '*' | '`' | '|' | '#'))
        .collect();

    kept.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every `ANSWER.md` the laboratory holds, found rather than listed.
fn testimonies() -> Vec<String> {
    let agents = laboratory().join("agents");
    let mut found = Vec::new();

    walk(&agents, &agents, &mut found);
    found.sort();

    found
}

fn walk(directory: &Path, from: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_dir() {
            walk(&path, from, found);
        } else if path.file_name().is_some_and(|name| name == "ANSWER.md") {
            found.push(
                path.strip_prefix(from)
                    .expect("under the agents row")
                    .display()
                    .to_string(),
            );
        }
    }
}

#[test]
fn every_testimony_the_laboratory_holds_is_one_this_crate_names() {
    let held = testimonies();
    let named: BTreeSet<&str> = Run::ALL.into_iter().map(|run| run.path()).collect();

    assert!(
        held.len() >= 8,
        "the scan found {held:?}; a scan that found none would agree with everything"
    );

    let unnamed: Vec<&String> = held
        .iter()
        .filter(|path| !named.contains(path.as_str()))
        .collect();

    assert!(
        unnamed.is_empty(),
        "the laboratory holds testimonies this crate does not name, so they are unclassified and \
         nothing says so: {unnamed:?}"
    );
}

#[test]
fn every_run_this_crate_names_is_a_file_that_exists() {
    let missing: Vec<&str> = Run::ALL
        .into_iter()
        .map(|run| run.path())
        .filter(|path| !laboratory().join("agents").join(path).is_file())
        .collect();

    assert!(
        missing.is_empty(),
        "these runs name no testimony in the laboratory: {missing:?}"
    );
}

/// Every claim's text is found in the testimony it is attributed to.
///
/// The severe one. A judgement about words the agent did not write satisfies every other guard in
/// this file, and it is the failure a reader of the classification alone cannot detect — which is
/// exactly the failure this row was opened knowing it was exposed to.
///
/// An elision is written `[…]` and each side is required separately, in order, so that abridging a
/// long sentence cannot become quoting two unrelated halves as one claim.
#[test]
fn every_claim_is_quoted_from_the_testimony_it_names() {
    let mut unfound = Vec::new();

    for run in Run::ALL {
        let source = laboratory().join("agents").join(run.path());

        let Ok(prose) = std::fs::read_to_string(&source) else {
            continue;
        };

        let flat = flattened(&prose);

        for claim in classified().iter().filter(|claim| claim.run == run) {
            let mut from = 0;

            for fragment in claim.text.split("[…]") {
                let wanted = flattened(fragment);

                if wanted.is_empty() {
                    continue;
                }

                match flat[from..].find(&wanted) {
                    Some(at) => from += at + wanted.len(),
                    None => {
                        unfound.push(format!("{:?} in {}: {wanted:?}", run, run.path()));
                        break;
                    }
                }
            }
        }
    }

    assert!(
        unfound.is_empty(),
        "these claims are not found in the testimony they are attributed to, in this order:\n  {}",
        unfound.join("\n  ")
    );
}

/// A housed claim naming a kernel entity names one the kernel has.
///
/// Derived from `core/src/kernel/entities/` rather than held as a list, because a list of nine names
/// beside the nine names is the duplication this laboratory keeps paying for. A misspelling here
/// would otherwise read as a confident answer.
#[test]
fn every_entity_a_housed_claim_names_is_one_the_kernel_has() {
    let kernel = kernel_entities();

    assert!(
        kernel.len() >= 9,
        "the scan read {kernel:?}; a scan that read none would agree with every carrier"
    );

    let orphans: Vec<&str> = classified()
        .iter()
        .filter_map(|claim| match claim.verdict {
            Verdict::Housed(Carrier::Entity(named)) => Some(named),
            _ => None,
        })
        .filter(|named| !kernel.contains(*named))
        .collect();

    assert!(
        orphans.is_empty(),
        "these carriers are not entities the kernel has, so the claim is housed by nothing: \
         {orphans:?} — the kernel has {kernel:?}"
    );
}

fn kernel_entities() -> BTreeSet<String> {
    let entities = laboratory()
        .join("../core/src/kernel/entities")
        .canonicalize()
        .expect("the kernel is where the engine keeps it");

    let mut found = BTreeSet::new();

    for entry in std::fs::read_dir(&entities).expect("the kernel's entities are readable") {
        let path = entry.expect("readable").path();

        if path.extension().is_none_or(|kind| kind != "rs") {
            continue;
        }

        for line in std::fs::read_to_string(&path)
            .expect("a kernel source is readable")
            .lines()
        {
            if let Some(named) = line.trim().strip_prefix("pub struct ") {
                found.insert(
                    named
                        .split(['{', '(', '<', ' ', ';'])
                        .next()
                        .unwrap_or_default()
                        .to_owned(),
                );
            }
        }
    }

    found
}

/// Every unhoused claim carries words rather than a stub, so that a judgement can be argued with.
///
/// **A floor against the degenerate case, and not a measure of disputability** — the honest limit is
/// worth stating, because the name promises more than the assertion delivers. What actually lets a
/// reader weigh a judgement is [`every_claim_is_quoted_from_the_testimony_it_names`], which locates
/// it, and Phase 4, which puts somebody else on the same question.
///
/// It was a character count until `02-hindsight`, where a genuinely verbatim table row —
/// `| C3 | +70 | no conflict |` — tripped it at nineteen characters. A terse row is terse because the
/// agent wrote it that way, and the reader who can find it can dispute it; the threshold was
/// measuring line width. Recorded rather than quietly retuned, because *the guard went red and the
/// number moved* is exactly the shape of a guard being fitted to the data.
#[test]
fn every_unhoused_claim_carries_its_words() {
    let bare: Vec<&'static str> = classified()
        .iter()
        .filter(|claim| matches!(claim.verdict, Verdict::Unhoused(_)))
        .filter(|claim| flattened(claim.text).split_whitespace().count() < 4)
        .map(|claim| claim.text)
        .collect();

    assert!(
        bare.is_empty(),
        "these unhoused claims carry too little text to be disputed: {bare:?}"
    );
}

/// Every standing that cites a document cites one that exists.
///
/// A want marked *tracked* against a path nobody can open is worse than one marked untracked: it
/// says the laboratory has the item and sends the reader nowhere. Checked from the repository root
/// because a standing may cite the engine's own documentation, which is where a *by design* ruling
/// lives.
#[test]
fn every_standing_that_cites_a_document_cites_one_that_exists() {
    let root = laboratory().join("..");

    let missing: Vec<&str> = classified()
        .iter()
        .filter_map(|claim| claim.standing)
        .filter_map(|standing| standing.cited())
        .filter(|path| !root.join(path).is_file())
        .collect();

    assert!(
        missing.is_empty(),
        "these standings cite documents that do not exist, so a reader is sent nowhere: {missing:?}"
    );
}

/// A claim carries a standing exactly when it asks the record for something.
///
/// The constructors already make this true — there is no way to build a want without one — so this
/// is the assertion that they still do, and the thing it would catch is a sixth constructor written
/// later that forgets.
#[test]
fn a_standing_is_present_exactly_for_the_two_kinds_that_ask() {
    let mut wrong = Vec::new();

    for claim in classified() {
        let asks = matches!(
            claim.verdict,
            Verdict::Unhoused(Some(Kind::Want)) | Verdict::Unhoused(Some(Kind::Loss))
        );

        if asks != claim.standing.is_some() {
            wrong.push(format!(
                "{:?} asks={asks} standing={:?}: {:?}",
                claim.run, claim.standing, claim.text
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "a want or a loss carries where it stands, and nothing else does:\n  {}",
        wrong.join("\n  ")
    );
}

/// What Phase 2 has read, and what it has not, reported rather than left to be inferred.
///
/// Not an assertion about completeness — Phase 2 lands run by run on purpose. It fails only if
/// nothing has been classified at all, which is the state where every other guard here passes by
/// having nothing to check.
#[test]
fn the_classification_says_which_testimonies_it_has_not_read() {
    let claims = classified();

    assert!(
        !claims.is_empty(),
        "nothing is classified, so every other guard in this file passed over an empty set"
    );

    let outstanding = unread();

    println!(
        "classified {} claims across {} runs",
        claims.len(),
        8 - outstanding.len()
    );
    println!("still unread: {outstanding:?}");

    let counted = |wanted: Kind| {
        claims
            .iter()
            .filter(|claim| claim.verdict == Verdict::Unhoused(Some(wanted)))
            .count()
    };

    println!(
        "housed {}  exposition {}  unhoused {}  unclassified {}",
        claims
            .iter()
            .filter(|claim| matches!(claim.verdict, Verdict::Housed(_)))
            .count(),
        claims
            .iter()
            .filter(|claim| claim.verdict == Verdict::Exposition)
            .count(),
        claims
            .iter()
            .filter(|claim| matches!(claim.verdict, Verdict::Unhoused(Some(_))))
            .count(),
        claims
            .iter()
            .filter(|claim| claim.verdict == Verdict::Unhoused(None))
            .count(),
    );

    for kind in [
        Kind::RoadNotTaken,
        Kind::Want,
        Kind::Qualification,
        Kind::Loss,
        Kind::MethodLimit,
    ] {
        println!("  {kind:?} {}", counted(kind));
    }

    println!("\nper run — the rate P2 is about, which is unclassified over everything not housed:");

    for run in Run::ALL {
        let mine: Vec<_> = claims.iter().filter(|claim| claim.run == run).collect();

        if mine.is_empty() {
            continue;
        }

        let orphans = mine
            .iter()
            .filter(|claim| claim.verdict == Verdict::Unhoused(None))
            .count();
        let asked = mine
            .iter()
            .filter(|claim| matches!(claim.verdict, Verdict::Unhoused(_)))
            .count();

        println!(
            "  {:<18} {orphans:>2} of {asked:>2}   ({} claims, {} exposition)",
            format!("{run:?}"),
            mine.len(),
            mine.iter()
                .filter(|claim| claim.verdict == Verdict::Exposition)
                .count(),
        );
    }

    println!("\nwhere the wants and the losses stand:");

    let standings: Vec<Standing> = claims.iter().filter_map(|claim| claim.standing).collect();
    let how_many = |label: &str, matching: fn(&Standing) -> bool| {
        println!(
            "  {label:<10} {}",
            standings.iter().filter(|it| matching(it)).count()
        );
    };

    how_many("tracked", |it| matches!(it, Standing::Tracked(_)));
    how_many("recorded", |it| matches!(it, Standing::Recorded(_)));
    how_many("met", |it| matches!(it, Standing::Met(_)));
    how_many("by design", |it| matches!(it, Standing::ByDesign(_)));
    how_many("untracked", |it| matches!(it, Standing::Untracked));
}
