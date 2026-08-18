//! Write the shared world where a party will read it.
//!
//! ```text
//! found <path> [floor]
//! ```
//!
//! A briefing hands an agent files rather than a fixture, which is the whole reason this exists:
//! every run before this one gave the agent the harness's own source, and a repository is the first
//! substrate that lets the agent be given *data* and nothing else.
//!
//! The optional floor is the second half of the experiment. It changes the world under the same
//! choices and is printed back, so that a record of a run says which world it was.

use ape_agents::coordination;
use ape_agents::world;
use ape_cli::journal::ResourceKindRecord;
use ape_cli::repository::Repository;

fn main() {
    let mut arguments = std::env::args().skip(1);

    let root = arguments.next().expect("a path to write the repository to");

    let cash = match arguments.next() {
        None => world::cash(),
        Some(floor) => ResourceKindRecord::Between {
            lower: floor.parse().expect("a floor as a number"),
            upper: 1000.0,
        },
    };

    let shared = coordination::under(cash);
    let repository = Repository::open(&root);

    shared.write(&repository).expect("writable");

    println!("root      {root}");
    println!("base      {}", shared.base);
    println!("standing  {}", shared.standing);
    println!("operations {}", shared.operations);
    println!("finance   {}", shared.finance);
    println!("entries   {}", shared.journal.len());
}
