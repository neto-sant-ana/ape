//! Write the two records that parted, where a party will read them.
//!
//! ```text
//! parted <path>
//! ```
//!
//! Puts `mine/` and `theirs/` under the path given, each written whole, and prints what the two
//! records come to and what the application says when they are put together. That print is the
//! experiment's Phase 0: it is the state the agent will meet, recorded before the agent exists, so
//! that the protocol's account of what it meets is a measurement rather than a description.
//!
//! Nothing here is handed to the agent. The briefing gets the two directories and the crates to read
//! them with.

use ape_agents::reconciliation;
use ape_cli::converge;
use ape_cli::error::ConvergeError;
use ape_cli::reading;
use ape_cli::repository::Repository;

fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("a path to write the two records under");

    let operations = reconciliation::operations();
    let finance = reconciliation::finance();

    let mine = Repository::open(format!("{root}/mine"));
    let theirs = Repository::open(format!("{root}/theirs"));

    operations.write(&mine).expect("a whole write");
    finance.write(&theirs).expect("a whole write");

    println!(
        "base entries shared by both  {}",
        reconciliation::base_entries()
    );
    println!(
        "mine    {} entries, {} decisions",
        operations.journal().len(),
        operations.decisions().len()
    );
    println!(
        "theirs  {} entries, {} decisions",
        finance.journal().len(),
        finance.decisions().len()
    );

    // What the application says when the two are put together, which is what the agent will meet.
    let held = reading::corroborated(&theirs).expect("the other record reads");

    match converge::converge(&mine, &held) {
        Ok(_) => println!("\nconverge  merged, which this arrangement is not supposed to allow"),
        Err(ConvergeError::Diverged {
            position, shared, ..
        }) => println!("\nconverge  Diverged at {position}, {shared} entries in common"),
        Err(other) => println!("\nconverge  {other}"),
    }
}
