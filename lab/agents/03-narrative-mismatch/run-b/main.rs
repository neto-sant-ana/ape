//! A walk from the identity of the current world back to its beginning, printing what each
//! world along the way selected and what interpreting it says.
//!
//! Nothing here is the answer; it is the instrument the answer is read off of.

mod hindsight;
mod world;

use ape::canon::{CanonicalHistory, CanonicalKnowledge};
use ape::engine::hermeneia::{Hypothesis, Outcome, Timeliness, movement_of};
use ape::engine::thesis::{ForkInput, Interpretation, Thesis, ThesisId, ThesisLookup};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Date, Term};

fn short(bytes: &[u8; 32]) -> String {
    bytes[..4].iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let replay = hindsight::build();
    let canon = &replay.graph.canon;
    let history = canon.history();
    let archive = &replay.graph.archive;

    // Walk from the identity outward: the lineage is resolved by following `parent`.
    let mut lineage: Vec<Thesis> = Vec::new();
    let mut cursor: Option<ThesisId> = Some(replay.graph.current);
    while let Some(id) = cursor {
        let thesis = archive.thesis(id).expect("the archive holds the lineage");
        cursor = *thesis.parent();
        lineage.push(thesis);
    }
    lineage.reverse();

    println!("worlds in the lineage: {}", lineage.len());
    println!("distinct worlds recorded: {}", replay.worlds.len());
    println!("history head: {:?}", history.head().map(|e| short(e.as_ref())));
    println!("recorded through: {:?}", history.recorded_through().map(|d| d.to_iso()));
    println!();

    let describe = |id: CommitmentId| -> String {
        let c = history.commitment(id).expect("a selected commitment is known");
        let m = movement_of(history, &c)
            .expect("a movement is derivable")
            .expect("cash is quantifiable");
        let rec = history
            .canonical_commitment(id)
            .expect("record")
            .recorded_at()
            .to_iso();
        let ev = history.event_of(id);
        let settled = match ev {
            None => "unsettled".to_string(),
            Some(e) => format!(
                "{} on {}",
                e.observation().name().to_string(),
                e.occurred_at().to_iso()
            ),
        };
        format!(
            "{} {:+.0} committed {} due {} recorded {} [{}]",
            short(id.as_ref()),
            m.magnitude(),
            c.term().committed_at().to_iso(),
            c.term().due_date().to_iso(),
            rec,
            settled
        )
    };

    for (i, thesis) in lineage.iter().enumerate() {
        println!("=== world {} — {}", i, short(thesis.id().as_ref()));
        println!(
            "    parent: {:?}",
            thesis.parent().map(|p| short(p.as_ref()))
        );
        println!(
            "    cut: known_at {} head {:?}",
            thesis.cut().known_at().to_iso(),
            thesis.cut().event_head().map(|e| short(e.as_ref()))
        );
        println!("    frozen:");
        for id in thesis.selection().frozen() {
            println!("      {}", describe(id));
        }
        println!("    open:");
        for id in thesis.selection().open() {
            println!("      {}", describe(id));
        }

        let interp = Interpretation::of(thesis, history).expect("a thesis interprets");

        for at in [thesis.cut().known_at(), &world::today(), &hindsight::january(20)] {
            let conds = interp.conditions_at(at).expect("conditions");
            let mut line = format!("    conditions at {}: ", at.to_iso());
            for (id, c) in conds.conditions() {
                line.push_str(&format!(
                    "{}={:?}{} ",
                    short(id.as_ref()),
                    c.outcome(),
                    match c.timeliness() {
                        Some(Timeliness::Breached) => "/Breached",
                        Some(Timeliness::WithinDeadline) => "/WithinDeadline",
                        None => "",
                    }
                ));
            }
            println!("{}", line);
        }

        for h in [
            Hypothesis::FinalState,
            Hypothesis::OnDueDateNet,
            Hypothesis::OnDueDateInAnyOrder,
        ] {
            let report = interp.feasibility_under(h).expect("feasibility");
            println!("    {:?}: {:?}", h, report.conflicts());
        }

        // The levels the selection implies, computed here only to say them in prose.
        let mut settled = 0.0;
        let mut all = 0.0;
        for id in thesis.selection().resolved() {
            let c = history.commitment(id).expect("known");
            let m = movement_of(history, &c).expect("derivable").expect("cash");
            let ev = history.event_of(id);
            let fulfilled = ev
                .as_ref()
                .map(|e| e.observation().name() == "Settled")
                .unwrap_or(false);
            let cancelled = ev
                .as_ref()
                .map(|e| e.observation().name() == "Cancelled")
                .unwrap_or(false);
            if fulfilled {
                settled += m.magnitude();
            }
            if !cancelled {
                all += m.magnitude();
            }
        }
        println!("    level settled-only: {:+.0} ; if every live intention lands: {:+.0}", settled, all);
        println!();
    }

    // Did any world gain a commitment nobody chose? Frozen(child) − Selection(parent) is what
    // an advance would have imposed; a fork's additions are decisions by construction.
    println!("--- what each edge changed");
    for pair in lineage.windows(2) {
        let (parent, child) = (&pair[0], &pair[1]);
        let moved_cut = parent.cut() != child.cut();
        let imposed: Vec<String> = child
            .selection()
            .frozen()
            .filter(|id| !parent.selection().contains(*id))
            .map(|id| short(id.as_ref()))
            .collect();
        let introduced: Vec<String> = child
            .selection()
            .open()
            .filter(|id| !parent.selection().contains(*id))
            .map(|id| short(id.as_ref()))
            .collect();
        println!(
            "    {} -> {}: {} ; imposed by history {:?} ; introduced by decision {:?}",
            short(parent.id().as_ref()),
            short(child.id().as_ref()),
            if moved_cut { "cut moved" } else { "intention changed" },
            imposed,
            introduced
        );
    }
    println!();

    println!("--- every commitment the canon holds, in the order it was recorded");
    for id in replay.intentions.iter().copied() {
        println!("    {}", describe(id));
    }

    counterfactuals(replay);
}

/// What the engine says about the ways out, asked the same way the plan itself was asked.
fn counterfactuals(replay: hindsight::Replay) {
    let mut canon = replay.graph.canon;
    let archive = replay.graph.archive;
    let current = archive
        .thesis(replay.graph.current)
        .expect("the current world is archived");

    let thirty = replay.intentions[1];
    let ninety = replay.intentions[2];

    let verdict = |label: &str, thesis: &Thesis, canon: &ape::canon::Canon<ape::canon::InMemoryHistory>| {
        let interp = Interpretation::of(thesis, canon.history()).expect("interprets");
        for h in [Hypothesis::FinalState, Hypothesis::OnDueDateNet, Hypothesis::OnDueDateInAnyOrder] {
            let r = interp.feasibility_under(h).expect("feasibility");
            println!(
                "    {label} under {h:?}: {}",
                if r.conflicts().is_empty() {
                    "no conflict".to_string()
                } else {
                    format!("{:?}", r.conflicts())
                }
            );
        }
    };

    println!("\n--- counterfactuals from the current world");

    let without_ninety = current
        .fork(
            canon.history(),
            ForkInput {
                omitted: [ninety].into(),
                introduced: [].into(),
            },
        )
        .expect("the 90 is open, so it may be dropped");
    verdict("drop the 90", &without_ninety, &canon);

    let without_thirty = current
        .fork(
            canon.history(),
            ForkInput {
                omitted: [thirty].into(),
                introduced: [].into(),
            },
        )
        .expect("the 30 is open, so it may be dropped");
    verdict("drop the 30", &without_thirty, &canon);

    // The opening tells us the inbound statement and who stands on each side of it.
    let opening = canon
        .history()
        .commitment(
            current
                .selection()
                .frozen()
                .find(|id| {
                    let c = canon.history().commitment(*id).expect("known");
                    movement_of(canon.history(), &c)
                        .expect("derivable")
                        .expect("cash")
                        .magnitude()
                        > 0.0
                })
                .expect("one inbound commitment is frozen"),
        )
        .expect("known");

    let inbound_twenty = canon
        .admit_commitment(
            CommitmentInput {
                assignment: opening.assignment().clone(),
                statement: *opening.statement(),
                resource: *opening.resource(),
                term: Term::new(hindsight::january(12), hindsight::january(19)).expect("term"),
                action_value: ActionValue::value(20.0).expect("magnitude"),
                dependencies: [].into(),
            },
            hindsight::january(12),
        )
        .expect("an inbound intention is admissible");

    let with_inflow = current
        .fork(
            canon.history(),
            ForkInput {
                omitted: [].into(),
                introduced: [inbound_twenty].into(),
            },
        )
        .expect("recorded no later than the cut, so it is introducible");
    verdict("add an inbound 20 due Jan 19", &with_inflow, &canon);

    let smaller = canon
        .admit_commitment(
            CommitmentInput {
                assignment: canon
                    .history()
                    .commitment(ninety)
                    .expect("known")
                    .assignment()
                    .clone(),
                statement: *canon.history().commitment(ninety).expect("known").statement(),
                resource: *canon.history().commitment(ninety).expect("known").resource(),
                term: Term::new(hindsight::january(12), hindsight::january(20)).expect("term"),
                action_value: ActionValue::value(70.0).expect("magnitude"),
                dependencies: [].into(),
            },
            hindsight::january(12),
        )
        .expect("a smaller outbound intention is admissible");

    let resized = current
        .fork(
            canon.history(),
            ForkInput {
                omitted: [ninety].into(),
                introduced: [smaller].into(),
            },
        )
        .expect("swap the 90 for a 70");
    verdict("swap the 90 for a 70", &resized, &canon);
}

#[allow(dead_code)]
fn iso(d: &Date) -> String {
    d.to_iso()
}
