//! A probe, not a deliverable: it asks the record the questions the report does not answer.
//!
//! Everything it prints is derived from `repo/` through the same public path `ape-cli` uses,
//! so a number here is a number the record produces rather than one this file computed.

use std::collections::BTreeSet;

use ape::engine::hermeneia::Hypothesis;
use ape::engine::synthesis::synthesize;
use ape::engine::thesis::{ForkInput, Interpretation, ThesisArchive, ThesisId, ThesisLookup};
use ape::kernel::axiom::Knowledge;
use ape::kernel::value_objects::Date;

use ape_cli::journal::Admission;
use ape_cli::reading;
use ape_cli::repository::Repository;

fn id(hex: &str) -> [u8; 32] {
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
        .collect();
    bytes.try_into().unwrap()
}

fn main() {
    let repository = Repository::open("repo");
    let corroborated = reading::corroborated(&repository).expect("the record rebuilds");

    let journal = repository.read_journal().expect("journal");
    let admitted = &corroborated.admitted;

    println!("== names ==");
    let mut roles = admitted.roles.iter();
    let mut agents = admitted.agents.iter();
    let mut resources = admitted.resources.iter();
    let mut instances = admitted.instances.iter();
    let mut actions = admitted.actions.iter();
    let mut commitments = admitted.commitments.iter();

    for entry in &journal {
        match entry {
            Admission::Role { label, .. } => {
                println!("role       {label:<14} {}", roles.next().unwrap())
            }
            Admission::Agent { label, .. } => {
                println!("agent      {label:<14} {}", agents.next().unwrap())
            }
            Admission::Resource { label, kind, .. } => println!(
                "resource   {label:<14} {} {kind:?}",
                resources.next().unwrap()
            ),
            Admission::ResourceInstance { label, .. } => {
                println!("instance   {label:<14} {}", instances.next().unwrap())
            }
            Admission::Action { verb, kind, .. } => {
                println!("action     {verb:<14} {} {kind:?}", actions.next().unwrap())
            }
            Admission::Commitment {
                magnitude,
                due_date,
                recorded_at,
                accountable,
                executors,
                ..
            } => println!(
                "commitment {:<14} {} magnitude={:?} due={due_date} recorded={recorded_at} accountable={accountable} executors={executors:?}",
                "", commitments.next().unwrap(), magnitude
            ),
            _ => {}
        }
    }

    println!();
    println!("== who decided what (the `by` claim) ==");
    for agent in &admitted.agents {
        let decided = reading::decided_by(&repository, *agent).expect("readable");
        if !decided.is_empty() {
            println!("{agent} -> {decided:?}");
        }
    }

    println!();
    println!("== worlds ==");
    for thesis in corroborated.lineage.decided() {
        println!(
            "{} parent={:?} known_at={} frozen={:?} open={:?}",
            thesis.id(),
            thesis.parent().map(|p| p.to_string()),
            thesis.cut().known_at().to_iso(),
            thesis
                .selection()
                .frozen()
                .map(|c| c.to_string())
                .collect::<Vec<_>>(),
            thesis
                .selection()
                .open()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        );
    }

    let base = ThesisId::from(id(
        "bbee1243429cb970f46358d411a0d4e8e10e829f2e3300ad5b417558c691a98a",
    ));
    let source = ThesisId::from(id(
        "791528450d627c4523dbfa073d231ef0884c5e78c4d564be6f65059b9875f737",
    ));
    let target = ThesisId::from(id(
        "1cd9afbbe16aba8e1455be1f48e4a01b95c99e3b18ac0a4c263ea25f2d415cec",
    ));

    println!();
    println!("== the report, asked again ==");
    let again = ape_cli::transfer::reconstruct(&repository, base, source, target)
        .expect("the report reproduces");
    println!("{}", serde_json::to_string_pretty(&again).unwrap());

    println!();
    println!("== the same question with the other base choices ==");
    for (name, b) in [("source-as-base", source), ("target-as-base", target)] {
        match ape_cli::transfer::reconstruct(&repository, b, source, target) {
            Ok(report) => println!("{name}: {}", serde_json::to_string(&report.status).unwrap()),
            Err(reason) => println!("{name}: refused: {reason}"),
        }
    }

    let archive = corroborated.lineage.archive();
    let history = corroborated.canon.history();

    println!();
    println!("== the candidate, actually built, and interpreted ==");
    let report = synthesize(archive, history, base, source, target).expect("report");
    let transfer = match report.status() {
        ape::engine::synthesis::ApplicabilityStatus::Applicable { transfer, .. } => transfer,
        other => panic!("not applicable: {other:?}"),
    };

    let target_thesis = archive.thesis(target).expect("target");
    let candidate = target_thesis
        .fork(
            history,
            ForkInput {
                omitted: transfer.remove().collect::<BTreeSet<_>>(),
                introduced: transfer.introduce().collect::<BTreeSet<_>>(),
            },
        )
        .expect("the fork the report authorizes");

    let instance = *admitted.instances.first().expect("account");

    for thesis in [&target_thesis, &candidate] {
        let interpretation = Interpretation::of(thesis, history).expect("interpretable");

        println!();
        println!(
            "thesis {} open={:?}",
            thesis.id(),
            thesis
                .selection()
                .open()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        );

        for hypothesis in [
            Hypothesis::FinalState,
            Hypothesis::OnDueDateNet,
            Hypothesis::OnDueDateInAnyOrder,
        ] {
            let feasibility = interpretation
                .feasibility_under(hypothesis)
                .expect("feasibility");
            println!("  {hypothesis:?}: {:?}", feasibility.conflicts());
        }

        for date in ["2026-01-07", "2026-01-10", "2026-01-14", "2026-01-20"] {
            let at = Date::parse(date).expect("date");
            let projected = interpretation.conditions_at(&at).expect("projection");
            let level = ape_cli::level::settled(history, &projected, instance).expect("level");
            println!("  settled level at {date}: {level}");
        }
    }

    println!();
    println!("== does the fork the report authorizes land on the candidate it names? ==");
    if let ape::engine::synthesis::ApplicabilityStatus::Applicable {
        candidate: named, ..
    } = report.status()
    {
        println!(
            "frozen agrees: {}   open agrees: {}",
            named.frozen().collect::<BTreeSet<_>>()
                == candidate.selection().frozen().collect::<BTreeSet<_>>(),
            named.open().collect::<BTreeSet<_>>()
                == candidate.selection().open().collect::<BTreeSet<_>>()
        );
    }
    println!("the world operations would land in: {}", candidate.id());

    println!();
    println!("== the reverse question: operations' intention in finance's line ==");
    match ape_cli::transfer::reconstruct(&repository, base, target, source) {
        Ok(reverse) => println!("{}", serde_json::to_string_pretty(&reverse).unwrap()),
        Err(reason) => println!("refused: {reason}"),
    }

    println!();
    println!("== the same verdict on a neighbouring world, one decision away ==");
    let sibling = archive
        .thesis(ThesisId::from(id(
            "ddef201121401c9c658d8a087d214cda57287ee630e217c2ea5bf0fd2b5ed5f4",
        )))
        .expect("operations' advance");

    let kept_both = sibling
        .fork(
            history,
            ForkInput {
                omitted: BTreeSet::new(),
                introduced: [
                    ape::kernel::entities::CommitmentId::from(id(
                        "0df2ea53f9055f2344ad3d4b2bff16f161ce92414dab3919dd94e602ccee063e",
                    )),
                ]
                .into_iter()
                .collect(),
            },
        )
        .expect("a fork that adds the 60 without dropping the 20");

    let mut hypothetical = ape_cli::archive::ResidentArchive::default();
    for thesis in corroborated.lineage.decided() {
        hypothetical.put_thesis(thesis.clone()).expect("known world");
    }
    hypothetical.put_thesis(kept_both.clone()).expect("the new one");

    let on_sibling =
        synthesize(&hypothetical, history, base, source, kept_both.id()).expect("a report");
    println!(
        "world {} open={:?}",
        kept_both.id(),
        kept_both
            .selection()
            .open()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
    );
    println!(
        "  status: {}",
        serde_json::to_string(&ape_cli::transfer::Applicability::of(&on_sibling).status).unwrap()
    );

    let interpretation = Interpretation::of(&kept_both, history).expect("interpretable");
    println!(
        "  its own feasibility (FinalState): {:?}",
        interpretation
            .feasibility_under(Hypothesis::FinalState)
            .expect("feasibility")
            .conflicts()
    );

    let taken = kept_both
        .fork(
            history,
            ForkInput {
                omitted: BTreeSet::new(),
                introduced: [ape::kernel::entities::CommitmentId::from(id(
                    "06b94c41827b09c3aeb817b8be12954c877ec94102a8f690adeb0c0b5bb5c9c7",
                ))]
                .into_iter()
                .collect(),
            },
        )
        .expect("the transfer the report calls applicable");

    let interpretation = Interpretation::of(&taken, history).expect("interpretable");
    println!(
        "  after taking the transfer it calls applicable: {:?}",
        interpretation
            .feasibility_under(Hypothesis::FinalState)
            .expect("feasibility")
            .conflicts()
    );

    println!();
    println!("== can the record check whose line the Source is? ==");
    let probe = Repository::open("probe-attribution");
    match reading::corroborated(&probe) {
        Ok(_) => println!("a copy whose `by` says `market` instead of `finance` rebuilds fine"),
        Err(reason) => println!("refused: {reason}"),
    }
    for (name, agent) in [
        ("market", "0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace"),
        ("finance", "108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b"),
    ] {
        let decided = reading::decided_by(
            &probe,
            ape::kernel::entities::AgentId::from(id(agent)),
        )
        .expect("readable");
        println!("  the copy says {name} decided {decided:?}");
    }

    match reading::corroborated(&Repository::open("probe-unknown-decider")) {
        Ok(_) => println!("  and a `by` naming something that is not an agent also rebuilds"),
        Err(reason) => println!("  what the check does buy: {reason}"),
    }

    println!();
    println!("== movements, so the arithmetic is not guessed ==");
    for commitment in &admitted.commitments {
        let record = history.commitment(*commitment).expect("known");
        let movement = ape::engine::hermeneia::movement_of(history, &record).expect("movement");
        println!(
            "{commitment} -> {:?}",
            movement.map(|m| (m.instance().to_string(), m.magnitude()))
        );
    }
}
