//! Phase 4's number, computed rather than counted.
//!
//! The row's hazard is a classifier who wants a particular answer, and this is the instrument built
//! to detect it — so the one thing that must not happen here is the agreement being tallied by hand
//! and reported as a finding.

use ape_succession::classification::Verdict;
use ape_succession::corpus::Run;
use ape_succession::phase4::{DIFFICULT, ESTABLISHED, disagreements, housed_here};
use ape_succession::testimony::classified;

#[test]
fn the_second_classifier_answered_every_claim_and_no_others() {
    let mine = housed_here();

    assert_eq!(
        mine.len(),
        46,
        "the sample was 46 claims; this reading holds {} for that run",
        mine.len()
    );

    let stray: Vec<&usize> = ESTABLISHED.iter().filter(|at| **at > mine.len()).collect();

    assert!(
        stray.is_empty(),
        "the second classifier answered claims that are not in the sample: {stray:?}"
    );
}

#[test]
fn the_two_readings_agree_on_most_of_the_sample_and_the_number_is_reported() {
    let mine = housed_here();
    let apart = disagreements();

    let agreed = mine.len() - apart.len();

    println!("\nPHASE 4 — two readings of 46 claims\n");
    println!(
        "  agreement   {agreed} of {}   ({:.0}%)",
        mine.len(),
        100.0 * agreed as f64 / mine.len() as f64
    );
    println!(
        "  this reading   {} housed",
        mine.iter().filter(|housed| **housed).count()
    );
    println!("  second reading {} established", ESTABLISHED.len());

    let texts: Vec<&'static str> = classified()
        .iter()
        .filter(|claim| claim.run == Run::Reconciliation)
        .map(|claim| claim.text)
        .collect();

    let verdicts: Vec<Verdict> = classified()
        .iter()
        .filter(|claim| claim.run == Run::Reconciliation)
        .map(|claim| claim.verdict)
        .collect();

    println!("\n  where they part:\n");

    for at in &apart {
        let flagged = DIFFICULT.contains(at);
        let short: String = texts[at - 1].chars().take(78).collect();

        println!(
            "  {at:>2}  here {:<28} there {}{}",
            format!("{:?}", verdicts[at - 1]),
            if ESTABLISHED.contains(at) {
                "ESTABLISHED"
            } else {
                "not"
            },
            match flagged {
                true => "   [it flagged this one as difficult]",
                false => "",
            }
        );
        println!("      {short}…");
    }

    // A floor rather than a target. Below half the two readings are measuring different things and
    // nothing in `99-result.md` stands; above it, the disagreements are the finding and are named
    // above rather than averaged away.
    assert!(
        agreed * 2 > mine.len(),
        "the two readings agree on {agreed} of {} — below half, they are not reading the same \
         question, and this experiment's counts do not stand",
        mine.len()
    );
}

/// The check `02-the-third-verdict.md` promised, and **it fired**: one of five exposition claims came
/// back established.
///
/// Claim 33, and the diagnosis is not the category's. The testimony reads *`transfer::applied` says
/// the same thing about its own case ("the record says which commitments were introduced and never
/// that another line of thinking is why")* — and this reading stored **only the inner quotation**,
/// dropping the frame that makes it the crate quoting itself. The second classifier judged a bare
/// claim about the record and was right to call it established.
///
/// So the disagreement is an extraction defect, not `Exposition` absorbing what the record holds. The
/// text is left exactly as the classifier judged it, because amending a claim after somebody has
/// answered it is not a correction, it is a different sample.
///
/// **Pinned rather than passing**: a new absorption fails, and this one stays named.
#[test]
fn the_one_exposition_claim_that_came_back_established_is_the_one_already_diagnosed() {
    let absorbed: Vec<usize> = classified()
        .iter()
        .filter(|claim| claim.run == Run::Reconciliation)
        .enumerate()
        .filter(|(_, claim)| claim.verdict == Verdict::Exposition)
        .map(|(index, _)| index + 1)
        .filter(|at| ESTABLISHED.contains(at))
        .collect();

    assert_eq!(
        absorbed,
        vec![33],
        "an exposition claim other than the one diagnosed came back established, which is the third \
         verdict absorbing claims the record holds"
    );
}
