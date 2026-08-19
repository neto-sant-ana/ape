//! The pedigree guard: nothing in the application is here without an experiment that earned it.
//!
//! The laboratory discovers obligations and the application implements the ones already earned. That
//! division is only worth having if it is checkable, because otherwise the application becomes the
//! new informal source of truth — a behaviour that is here because it seemed right, with the
//! experiment that would have justified it never written.
//!
//! So every module of the application declares what earned it, and **silence is not an answer**: a
//! module with nothing to cite says so, with a reason. What the guard mechanises is the form and the
//! resolution, never whether the attribution is apt — that one is a reading, and it belongs to
//! whoever reviews the diff.
//!
//! # Why the verdict and not just the document
//!
//! A citation that names only a document passes while saying nothing. Experiment 04 is **Refuted**
//! in both halves and its Part B was reverted, so `04-provenance` names a real document whose answer
//! is that the thing does not hold. A claim has to name the verdict it rests on, and the verdict is
//! read out of the result rather than kept in a list here — a second copy of seven verdicts would be
//! a second place for them to diverge.
//!
//! # What keeps it from passing over nothing
//!
//! This family of guard fails by scanning nothing and reporting agreement, so the counts are
//! asserted against literals, and [`the_checker_refuses_a_confirmed_claim_against_a_refuted_result`]
//! is a permanent red-for-the-right-reason: it feeds the checker the trap and requires a refusal.
//!
//! Paths resolve through `CARGO_MANIFEST_DIR`, so the guard survives the application moving.

use std::path::{Path, PathBuf};

/// The declaration every module of the application carries.
const EARNED_BY: &str = "//! Earned by:";

/// What a module may say instead of citing an experiment.
const NOTHING: &str = "nothing —";

/// The modules of the application. Asserted as a literal because a scan that found none would
/// otherwise report that every module it did not find is properly attributed.
const MODULES: usize = 12;

/// Citations across all of them, for the same reason.
const CITATIONS: usize = 20;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The application's modules: the crate's own sources, without the laboratory's.
///
/// `subject/` is a fixture per experiment and `docs/` is the experimental record — both are the
/// laboratory, and neither is something the application implements.
fn modules() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(root().join("src"))
        .expect("the application's sources are there")
        .map(|entry| entry.expect("readable").path())
        .filter(|path| path.extension().is_some_and(|kind| kind == "rs"))
        .collect();

    found.sort();
    found
}

/// One claim: the experiment's directory, and the verdict the module says it rests on.
#[derive(Debug, PartialEq)]
struct Claim {
    experiment: String,
    verdict: String,
}

/// Read a module's declaration, or say what is missing.
///
/// The message names the module and the form, because a guard that answers "the pedigree is
/// inconsistent" hands back the work of finding out which file and what to write.
fn declared(module: &Path) -> Result<Vec<Claim>, String> {
    let source = std::fs::read_to_string(module).expect("readable");
    let name = module.file_name().expect("a file").to_string_lossy();

    let at = source
        .lines()
        .position(|line| line.trim().starts_with(EARNED_BY))
        .ok_or_else(|| {
            format!(
                "{name} declares no pedigree; add `{EARNED_BY} NN-experiment (Verdict)` to its \
                 module docstring, or `{EARNED_BY} {NOTHING} <why>` if nothing earned it"
            )
        })?;

    // A declaration too long for one line continues on the next `//!` lines, so the house's width
    // is not a constraint on how many experiments a module may cite. It has to be the last thing in
    // the docstring, and prose after it fails as an unparsable claim rather than being swallowed.
    let line = source
        .lines()
        .skip(at)
        .map_while(|line| line.trim().strip_prefix("//!"))
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim_start_matches("Earned by:")
        .trim()
        .to_owned();

    if line.starts_with(NOTHING) {
        return Ok(Vec::new());
    }

    line.split(',')
        .map(|claim| {
            let claim = claim.trim();
            let (experiment, verdict) = claim
                .split_once(" (")
                .ok_or_else(|| format!("{name}: {claim:?} is not `NN-experiment (Verdict)`"))?;

            Ok(Claim {
                experiment: experiment.to_owned(),
                verdict: verdict
                    .strip_suffix(')')
                    .ok_or_else(|| format!("{name}: {claim:?} does not close its verdict"))?
                    .to_owned(),
            })
        })
        .collect()
}

/// The verdicts a result document leads with, read out of the document.
///
/// The verdict line is the first non-blank line after `# Result`, and every verdict on it is bold.
/// Only that line is read: a bold span anywhere else in the document says nothing about the answer,
/// and matching against the whole file is how this kind of check starts passing for the wrong reason.
fn verdicts(experiment: &str) -> Result<Vec<String>, String> {
    // The record is the laboratory's, and the application cites it. This is the application's one
    // deliberate reach outside itself, and it is a test reading a file rather than the library
    // depending on anything: nothing in `src/` knows the laboratory exists, and a `use` of it does
    // not compile.
    let result = root()
        .join("../lab/frontier/docs")
        .join(experiment)
        .join("99-result.md");

    let document = std::fs::read_to_string(&result).map_err(|_| {
        format!(
            "{experiment} names no result document at {}",
            result.display()
        )
    })?;

    let line = document
        .lines()
        .skip_while(|line| line.trim() != "# Result")
        .skip(1)
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| format!("{experiment}'s result states no verdict"))?;

    let bold: Vec<String> = line
        .split("**")
        .skip(1)
        .step_by(2)
        .map(|span| span.trim().to_owned())
        .collect();

    if bold.is_empty() {
        return Err(format!(
            "{experiment}'s verdict line holds no verdict in bold: {line:?}"
        ));
    }

    Ok(bold)
}

/// Weigh one claim against the document it names.
fn earned(claim: &Claim) -> Result<(), String> {
    let stated = verdicts(&claim.experiment)?;

    if stated.iter().any(|verdict| verdict == &claim.verdict) {
        return Ok(());
    }

    Err(format!(
        "claims {:?} of {}, whose result says {:?}",
        claim.verdict, claim.experiment, stated
    ))
}

/// Every module of the application declares what earned it, and every claim resolves.
#[test]
fn nothing_is_in_the_application_without_an_experiment_that_earned_it() {
    let modules = modules();

    assert_eq!(
        modules.len(),
        MODULES,
        "the scan found {} modules; if the application gained or lost one, MODULES moves with it",
        modules.len()
    );

    let mut refused = Vec::new();
    let mut citations = 0;

    for module in &modules {
        let name = module
            .file_name()
            .expect("a file")
            .to_string_lossy()
            .into_owned();

        match declared(module) {
            Err(missing) => refused.push(missing),
            Ok(claims) => {
                citations += claims.len();

                for claim in claims {
                    if let Err(unearned) = earned(&claim) {
                        refused.push(format!("{name} {unearned}"));
                    }
                }
            }
        }
    }

    assert!(
        refused.is_empty(),
        "the application claims what no experiment earned:\n  {}",
        refused.join("\n  ")
    );

    assert_eq!(
        citations, CITATIONS,
        "the scan resolved {citations} citations; a scan that resolved none would otherwise agree \
         with everything"
    );
}

/// The trap, permanently: a module may not claim a verdict its experiment did not reach.
///
/// This is the guard's own red. Experiment 04 is refuted in both halves, so a citation naming it is
/// a real document — which is exactly why naming the document cannot be enough.
#[test]
fn the_checker_refuses_a_confirmed_claim_against_a_refuted_result() {
    let trap = Claim {
        experiment: "04-provenance".to_owned(),
        verdict: "Confirmed".to_owned(),
    };

    let refusal = earned(&trap).expect_err("a confirmed claim against a refuted result is refused");

    assert!(
        refusal.contains("Refuted"),
        "the refusal has to say what the result actually says, and it said {refusal:?}"
    );

    assert!(
        earned(&Claim {
            experiment: "04-provenance".to_owned(),
            verdict: "Refuted".to_owned(),
        })
        .is_ok(),
        "and the same document is citable for what it does say"
    );
}
