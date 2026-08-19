//! Stage what a reader of the merge is given: one repository holding both lines, and the report.
//!
//! ```text
//! staged <path>
//! ```
//!
//! Writes `<path>/repo` and `<path>/report.json`.
//!
//! The report is written to a file because an `ApplicabilityReport` derives no serialization and is
//! not an entity — a process that wants one asks for one. So what crosses into the briefing is the
//! application's record of what Synthesis said, which is what an application would have to hand a
//! reader anyway.
//!
//! Only the direction that applies is written. The refused one is defined behaviour readable from the
//! two cuts, and a briefing carrying a refusal would be handing over a different question.

use ape_agents::coordination;
use ape_cli::transfer;

fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("a path to stage the briefing under");

    let repository = coordination::copied(coordination::FINANCE, format!("{root}/repo"))
        .expect("the earlier-dated party's repository is copyable");

    let merged = coordination::merge(&repository);

    let shared = coordination::shared();
    let base = coordination::base_of(&merged);

    let operations =
        coordination::tip_of(&merged, shared.operations).expect("operations' world is here");
    let finance = coordination::tip_of(&merged, shared.finance).expect("and finance's");

    let report = transfer::reconstruct(&repository, base, finance, operations)
        .expect("a report is derivable");

    let rendered = serde_json::to_string_pretty(&report).expect("the record serializes");

    std::fs::write(format!("{root}/report.json"), format!("{rendered}\n"))
        .expect("the report is writable");

    println!("root        {root}");
    println!("base        {base}");
    println!("operations  {operations}");
    println!("finance     {finance}");
    println!("decisions   {}", merged.decisions.len());
    println!("entries     {}", merged.journal.len());
}
