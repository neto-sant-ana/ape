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
const CITATIONS: usize = 36;

/// Concluded experiments across both rows, so a sweep that read none cannot report agreement.
const RESULTS: usize = 23;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every module of the application: every `.rs` file under `src/`, at any depth.
///
/// It recurses, and that is the whole of the fix it once needed. The first version read one directory,
/// which was **right by coincidence** — the two things it had to leave out were `subject/` and `docs/`,
/// and both were directories, so the extension filter dropped them without meaning to. Once the
/// bifurcation moved them to the laboratory the coincidence stopped being load-bearing and the gap it
/// was hiding stayed: a module in a folder would not have been asked to declare anything.
fn modules() -> Vec<PathBuf> {
    let mut found = Vec::new();

    walk_sources(&root().join("src"), &mut found);
    found.sort();

    found
}

fn walk_sources(directory: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("the application's sources are there") {
        let path = entry.expect("readable").path();

        if path.is_dir() {
            walk_sources(&path, found);
        } else if path.extension().is_some_and(|kind| kind == "rs") {
            found.push(path);
        }
    }
}

/// One claim: the experiment's directory, and the verdict the module says it rests on.
#[derive(Debug, PartialEq)]
struct Claim {
    experiment: String,
    verdict: String,
}

/// A module's name as a reader would have to type it, relative to `src/`.
///
/// The path and not the file name, because every folder submodule is called `mod.rs` — a report that
/// named the file would name three of them identically and locate none.
fn named(module: &Path) -> String {
    module
        .strip_prefix(root().join("src"))
        .unwrap_or(module)
        .display()
        .to_string()
}

/// Read a module's declaration, or say what is missing.
///
/// The message names the module and the form, because a guard that answers "the pedigree is
/// inconsistent" hands back the work of finding out which file and what to write.
fn declared(module: &Path) -> Result<Vec<Claim>, String> {
    let source = std::fs::read_to_string(module).expect("readable");
    let name = named(module);

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

/// Where each row of the laboratory keeps one directory per concluded experiment.
///
/// Two, because the application may cite either, and the paths are asymmetric because the layout is:
/// the frontier row keeps its documents under a `docs/` beside its crate, and the agents row keeps
/// them at its top level.
const ROWS: [&str; 2] = ["frontier/docs", "agents"];

/// The result document one experiment name resolves to, searching every row.
///
/// It resolved against the frontier row alone until the agents row's stand-by was written down, and
/// the guard's own claim is what made that a defect rather than a limitation: *nothing in the
/// application is here without an experiment that earned it* cannot be checked by something that
/// reads half the laboratory. Nothing cited the agents row, so nothing was wrong — which is the
/// state a guard is least able to report.
///
/// **A name in two rows is refused rather than resolved.** Picking one silently is how a citation
/// starts naming a document nobody meant; the rows are disjoint today and
/// [`the_rows_hold_no_experiment_name_in_common`] is what says so.
fn result_of(experiment: &str) -> Result<PathBuf, String> {
    // The record is the laboratory's, and the application cites it. This is the application's one
    // deliberate reach outside itself, and it is a test reading a file rather than the library
    // depending on anything: nothing in `src/` knows the laboratory exists, and a `use` of it does
    // not compile.
    let found: Vec<PathBuf> = ROWS
        .iter()
        .map(|row| {
            root()
                .join("../lab")
                .join(row)
                .join(experiment)
                .join("99-result.md")
        })
        .filter(|path| path.is_file())
        .collect();

    match found.as_slice() {
        [only] => Ok(only.clone()),
        [] => Err(format!(
            "{experiment} names no result document in any row of the laboratory ({})",
            ROWS.join(", ")
        )),
        several => Err(format!(
            "{experiment} names a result in {} rows, so a citation of it names no document in \
             particular",
            several.len()
        )),
    }
}

/// The verdicts a result document leads with, read out of the document.
///
/// The verdict line is the first non-blank line after `# Result`, and every verdict on it is bold.
/// Only that line is read: a bold span anywhere else in the document says nothing about the answer,
/// and matching against the whole file is how this kind of check starts passing for the wrong reason.
///
/// **One form, and both rows write it.** The agents row wrote three others — a fenced block for
/// three experiments, the verdict inside the heading for the fourth — which is one truth in three
/// representations, and this is the reader that would have caught the divergence had it been looking.
/// The four documents were normalized rather than the reader taught the shapes: a reader that accepts
/// four forms ratifies them, and [`every_result_document_states_a_verdict`] is what keeps the number
/// at one.
fn verdicts(experiment: &str) -> Result<Vec<String>, String> {
    let document = std::fs::read_to_string(result_of(experiment)?).expect("readable");

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

/// Every concluded experiment of one row: the directories holding a result document.
fn experiments(row: &str) -> Vec<String> {
    let mut found = Vec::new();

    for entry in std::fs::read_dir(root().join("../lab").join(row))
        .expect("a row of the laboratory is there")
    {
        let path = entry.expect("readable").path();

        if path.join("99-result.md").is_file() {
            found.push(
                path.file_name()
                    .expect("a directory has a name")
                    .display()
                    .to_string(),
            );
        }
    }

    found.sort();

    found
}

/// Every result document in the laboratory states its verdict in the one form the guard reads.
///
/// The sweep that was missing, and the reason the blocker was invisible: the reader looked at one
/// row, so the other row's three other shapes were nobody's error. A result whose verdict this
/// cannot read is a result no citation can rest on, which is a defect in the document rather than in
/// the reader — and it is caught here rather than when somebody first tries to cite it.
#[test]
fn every_result_document_states_a_verdict() {
    let mut swept = 0;
    let mut refused = Vec::new();

    for row in ROWS {
        for experiment in experiments(row) {
            swept += 1;

            if let Err(unstated) = verdicts(&experiment) {
                refused.push(unstated);
            }
        }
    }

    assert!(
        refused.is_empty(),
        "results the guard cannot read a verdict out of:\n  {}",
        refused.join("\n  ")
    );

    assert_eq!(
        swept, RESULTS,
        "the sweep read {swept} results; a sweep that read none would otherwise agree with \
         everything"
    );
}

/// No experiment name resolves to two documents, which is what lets a citation name only the name.
///
/// Prevention rather than measurement, and the honest reading of it: the numbers repeat across the
/// rows — both hold an `04` — and the names do not, so a citation is unambiguous by the naming
/// convention rather than by anything enforcing it. This is what turns a convention into a refusal,
/// and it is the only thing standing between `result_of`'s ambiguity arm and being unreachable
/// forever.
#[test]
fn the_rows_hold_no_experiment_name_in_common() {
    let [frontier, agents] = ROWS.map(experiments);

    let shared: Vec<&String> = frontier
        .iter()
        .filter(|named| agents.contains(named))
        .collect();

    assert!(
        shared.is_empty(),
        "these names resolve to a result in both rows, so a citation of them names neither: \
         {shared:?}"
    );
    assert!(
        !frontier.is_empty() && !agents.is_empty(),
        "both rows have to hold results for the comparison to have compared anything"
    );
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
        let name = named(module);

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
