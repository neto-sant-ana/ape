//! Emits Phase 4's sample: the claim texts of one testimony, with every verdict stripped.
//!
//! The second classifier is asked question 1 only — *could this be read off the record?* — and it
//! must not see what this reading answered. So the sample is **generated from the classification**
//! rather than transcribed out of it: a list copied by hand is the failure this experiment spent
//! eight testimonies measuring, and it would be a poor place to commit it.
//!
//! ```text
//! cargo run -p ape-succession --bin sample -- Reconciliation
//! ```
//!
//! The order is the order the claims appear in the testimony, which leaks nothing: grouping them by
//! verdict would.

use ape_succession::corpus::Run;
use ape_succession::testimony::classified;

fn main() {
    let wanted = std::env::args().nth(1).unwrap_or_default();

    let Some(run) = Run::ALL
        .into_iter()
        .find(|run| format!("{run:?}") == wanted)
    else {
        eprintln!("name one of {:?}", Run::ALL.map(|run| format!("{run:?}")));
        std::process::exit(1);
    };

    for (number, claim) in classified()
        .iter()
        .filter(|claim| claim.run == run)
        .enumerate()
    {
        println!("## {}\n\n{}\n", number + 1, claim.text);
    }
}
