//! Operations stands down the courier slot and puts the inventory purchase in its place.
//!
//! Two things happen here, and the crates keep them apart on purpose. The purchase is
//! **knowledge**: a commitment nobody had recorded yet, so it is admitted through the Canon and
//! appended to the journal. Standing the slot down is **intention**: nothing was observed, so no
//! Event is emitted — the world operations reasons about is forked, omitting the slot and
//! introducing the purchase in the same decision. Both new decisions are attributed to
//! operations, which is what `Taken::claimed` records and `reading::decided_by` reads back.
//!
//! The fork needs one thing before it can run. A fork inherits its parent's cut, and the world
//! on file was cut at 2026-01-06; a commitment recorded after that instant is not selectable
//! there (`CommitmentNotKnownAtCut`). So the line is `advance` and then `fork`, which is the
//! planning path the engine documents: recognizing a later day is not deciding anything, and
//! deciding is not recognizing.
//!
//! The purchase is built from the slot it replaces rather than from labels, because the two are
//! the same arrangement at a different size and deadline: same accountable party, same executors
//! and beneficiaries, same statement, same account. Nothing in the ontology says what a
//! commitment is *for*, so "courier slot" and "inventory purchase" are not distinguishable in the
//! record — see ANSWER.md.
//!
//! Written back through `converge`, which merges into the repository as it stands rather than
//! overwriting what it read, and rebuilds the whole thing in memory before writing a byte.

use std::collections::BTreeSet;
use std::error::Error;

use ape::engine::thesis::ThesisId;
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::converge;
use ape_cli::journal::{self, Admission};
use ape_cli::lineage::{self, Decision, Taken};
use ape_cli::reading::{self, Corroborated};
use ape_cli::repository::Repository;

/// The day operations acts. Later than everything recorded (the journal stops at 2026-01-05) and
/// later than the cut the world on file was taken at (2026-01-06), which is why an advance is
/// needed at all.
const TODAY: &str = "2026-01-07";

/// What the purchase asks for.
const PURCHASE_MAGNITUDE: f64 = 60.0;
const PURCHASE_DUE: &str = "2026-01-20";

/// What the slot being stood down asks for, used to find it rather than to build anything.
const SLOT_MAGNITUDE: f64 = 20.0;
const SLOT_DUE: &str = "2026-01-10";

fn main() -> Result<(), Box<dyn Error>> {
    let repository = Repository::open("repo");

    let Corroborated {
        mut canon,
        mut lineage,
        mut admitted,
        mut journal,
        mut decisions,
    } = reading::corroborated(&repository)?;

    println!("=== repo/ as it stands ===\n");
    describe_vocabulary(canon.history(), &admitted);
    describe_worlds(&lineage, &decisions, canon.history(), &admitted);

    let operations = agent_named(canon.history(), &admitted.agents, "operations")
        .ok_or("repo/ knows no agent labelled \"operations\"")?;
    let account = instance_named(canon.history(), &admitted.instances, "account")
        .ok_or("repo/ knows no resource instance labelled \"account\"")?;

    println!("acting for : operations {operations}");
    println!("reading of : account    {account}\n");

    // ---------------------------------------------------------------- the slot to stand down

    let held = lineage
        .decided()
        .last()
        .ok_or("repo/ holds no decided world")?;
    let held_id = held.id();

    let standing: Vec<CommitmentId> = held
        .selection()
        .open()
        .filter(|id| {
            canon
                .history()
                .commitment(*id)
                .is_some_and(|c| matches_slot(&c))
        })
        .collect();

    let substituted = held.selection().open().any(|id| {
        canon
            .history()
            .commitment(id)
            .is_some_and(|held| matches_purchase(&held))
    });

    let slot = match standing.as_slice() {
        [slot] => *slot,

        // The substitution is already in repo/, so this run has nothing to decide. Said rather
        // than refused, because a second run is how a reader checks the first one.
        [] if substituted => {
            println!("=== nothing to do ===\n");
            println!(
                "world {held_id} already stands the courier slot down and intends the purchase.\n\
                 Deciding it again would fork a world that changes nothing."
            );
            println!("\nworlds decided by operations:");
            for world in reading::decided_by(&repository, operations)? {
                println!("  {world}");
            }

            return Ok(());
        }

        other => {
            return Err(format!(
                "the world on file leaves {} open commitments of {SLOT_MAGNITUDE} due {SLOT_DUE}; \
                 operations named one, and nothing here may choose between them",
                other.len()
            )
            .into());
        }
    };

    println!("=== the slot operations is standing down ===\n");
    describe_commitment(canon.history(), slot);

    // ------------------------------------------------------- 1. record what operations intends

    let arrangement = canon
        .history()
        .commitment(slot)
        .ok_or("the slot left knowledge between two reads")?;

    let purchase = Admission::Commitment {
        accountable: arrangement.assignment().accountable(),
        executors: arrangement.assignment().executors().clone(),
        beneficiaries: arrangement.assignment().beneficiaries().clone(),
        statement: *arrangement.statement(),
        resource: *arrangement.resource(),
        committed_at: TODAY.to_owned(),
        due_date: PURCHASE_DUE.to_owned(),
        magnitude: Some(PURCHASE_MAGNITUDE),
        dependencies: BTreeSet::new(),
        recorded_at: TODAY.to_owned(),
    };

    journal.push(purchase);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let purchase = *admitted
        .commitments
        .last()
        .ok_or("the journal admitted no commitment")?;

    println!("=== recorded: what operations intends ===\n");
    println!(
        "journal entry {} of {} — {}\n",
        admitted.entries.len(),
        journal.len(),
        admitted
            .entries
            .last()
            .expect("the entry just admitted addresses itself")
    );
    describe_commitment(canon.history(), purchase);

    // ------------------------------------------------------ 2. take the decision as operations

    println!("=== decided, as operations ===\n");

    let recognize = Decision::Advance {
        extends: held_id,
        known_at: TODAY.to_owned(),
    };
    let imposed = lineage::decide(canon.history(), &mut lineage, &recognize)?;
    decisions.push(Taken::claimed(recognize, operations, &admitted)?);

    let recognized = lineage
        .decided()
        .last()
        .expect("the advancement was recorded")
        .id();

    println!("advance  extends {held_id}");
    println!("         known_at {TODAY}, so the purchase is knowledge the world can select");
    println!("         imposed by history: {}", imposed.len());
    println!("      -> world {recognized}\n");

    let substitute = Decision::Fork {
        extends: recognized,
        omitted: BTreeSet::from([slot]),
        introduced: BTreeSet::from([purchase]),
    };
    lineage::decide(canon.history(), &mut lineage, &substitute)?;
    decisions.push(Taken::claimed(substitute, operations, &admitted)?);

    let decided = lineage
        .decided()
        .last()
        .expect("the fork was recorded")
        .clone();

    println!("fork     extends {recognized}");
    println!("         omitted    {slot}  (the courier slot)");
    println!("         introduced {purchase}  (the inventory purchase)");
    println!("      -> world {}", decided.id());
    println!(
        "         frozen {:?}",
        decided
            .selection()
            .frozen()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
    );
    println!(
        "         open   {:?}\n",
        decided
            .selection()
            .open()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
    );

    // ------------------------------------------------------------------ put it back in repo/

    converge::converge(
        &repository,
        &Corroborated {
            canon,
            lineage,
            admitted,
            journal,
            decisions,
        },
    )?;

    println!("=== put back ===\n");
    println!("{}", repository.journal_path().display());
    println!("{}", repository.lineage_path().display());
    println!("{}", repository.worlds_path().display());
    println!(
        "\nrepo/ holds world {}: {}\n",
        decided.id(),
        converge::holds(&repository, decided.id())?
    );

    // ------------------------------------------- read it back, given nothing but the repository

    println!("=== read back from repo/, from nothing else ===\n");

    let mine = reading::decided_by(&repository, operations)?;
    println!("worlds decided by operations:");
    for world in &mine {
        println!("  {world}");
    }
    println!();

    for at in ["2026-01-07", "2026-01-20"] {
        let at = Date::parse(at)?;
        let readings = reading::reconstruct(&repository, account, &at)?;

        println!("--- every world, read of account at {} ---", at.to_iso());
        for reading in &readings {
            report(reading, decided.id(), &mine, slot, purchase);
        }
        println!();
    }

    Ok(())
}

/// Whether a commitment is the standing arrangement the brief describes.
///
/// Matched on what the brief gives — its size and its deadline — because nothing in the record
/// says what a commitment is for. Two open commitments of 20 due on the 10th would be
/// indistinguishable here, which is why the caller refuses rather than picks.
fn matches_slot(commitment: &ape::kernel::entities::Commitment) -> bool {
    commitment.action_value().as_value() == Some(SLOT_MAGNITUDE)
        && commitment.term().due_date().to_iso() == SLOT_DUE
}

/// Whether a commitment is the purchase this run would record.
///
/// The same weakness as [`matches_slot`], used for the same reason: it is how a second run
/// recognizes that the first one already happened.
fn matches_purchase(commitment: &ape::kernel::entities::Commitment) -> bool {
    commitment.action_value().as_value() == Some(PURCHASE_MAGNITUDE)
        && commitment.term().due_date().to_iso() == PURCHASE_DUE
}

fn report(
    reading: &ape_cli::reading::Reading,
    decided: ThesisId,
    mine: &BTreeSet<String>,
    slot: CommitmentId,
    purchase: CommitmentId,
) {
    let decided = reading.thesis == decided.to_string();
    let mine = mine.contains(&reading.thesis);

    println!(
        "world {}{}{}",
        reading.thesis,
        if mine { "  [operations]" } else { "" },
        if decided { "  <- the substitution" } else { "" }
    );
    println!(
        "  cut at {} on chain {}",
        reading.known_at,
        reading.event_head.as_deref().unwrap_or("(none)")
    );

    let selects = |id: CommitmentId| reading.conditions.contains_key(&id.to_string());

    println!(
        "  courier slot: {}    inventory purchase: {}",
        if selects(slot) {
            "intended"
        } else {
            "stood down"
        },
        if selects(purchase) {
            "intended"
        } else {
            "not intended"
        }
    );

    for (id, condition) in &reading.conditions {
        println!(
            "  {id}  {:?}/{:?}{}",
            condition.outcome,
            condition.timeliness,
            if reading.frozen.contains(id) {
                "  frozen"
            } else {
                "  open"
            }
        );
    }

    println!("  level of account: {}", reading.level);
    println!("  conflicts: {:?}", reading.conflicts);
}

// ------------------------------------------------------------------------------ who is who

fn agent_named(knowledge: &impl Knowledge, known: &[AgentId], label: &str) -> Option<AgentId> {
    known
        .iter()
        .find(|id| {
            knowledge
                .agent(**id)
                .is_some_and(|agent| agent.label().as_str() == label)
        })
        .copied()
}

fn instance_named(
    knowledge: &impl Knowledge,
    known: &[ResourceInstanceId],
    label: &str,
) -> Option<ResourceInstanceId> {
    known
        .iter()
        .find(|id| {
            knowledge
                .resource_instance(**id)
                .is_some_and(|instance| instance.label().as_str() == label)
        })
        .copied()
}

fn agent_label(knowledge: &impl Knowledge, id: AgentId) -> String {
    knowledge
        .agent(id)
        .map(|agent| agent.label().as_str().to_owned())
        .unwrap_or_else(|| format!("<unknown agent {id}>"))
}

/// Everything the journal established, resolved back to the labels it was written with.
fn describe_vocabulary(knowledge: &impl Knowledge, admitted: &journal::Replayed) {
    println!("roles");
    for id in &admitted.roles {
        if let Some(role) = knowledge.role(*id) {
            println!("  {} {}", role.label().as_str(), id);
        }
    }

    println!("\nagents");
    for id in &admitted.agents {
        if let Some(agent) = knowledge.agent(*id) {
            let eligibility: Vec<String> = knowledge
                .eligibilities_of(*id)
                .iter()
                .map(|assignment| {
                    let roles: Vec<String> = assignment
                        .roles()
                        .iter()
                        .filter_map(|role| knowledge.role(*role))
                        .map(|role| role.label().as_str().to_owned())
                        .collect();

                    format!("{:?} from {}", roles, assignment.effective_from().to_iso())
                })
                .collect();

            println!(
                "  {} {}  eligible: {}",
                agent.label().as_str(),
                id,
                eligibility.join(", ")
            );
        }
    }

    println!("\nresources");
    for id in &admitted.resources {
        if let Some(resource) = knowledge.resource(*id) {
            println!(
                "  {} {}  {:?}",
                resource.label().as_str(),
                id,
                resource.kind()
            );
        }
    }

    println!("\nresource instances");
    for id in &admitted.instances {
        if let Some(instance) = knowledge.resource_instance(*id) {
            println!("  {} {}", instance.label().as_str(), id);
        }
    }

    println!("\nactions");
    for id in &admitted.actions {
        if let Some(action) = knowledge.action(*id) {
            println!("  {} {}  {:?}", action.verb().as_str(), id, action.kind());
        }
    }

    println!("\nstatements");
    for id in &admitted.statements {
        if let Some(statement) = knowledge.statement(*id) {
            let roles = |set: &BTreeSet<ape::kernel::entities::RoleId>| -> Vec<String> {
                set.iter()
                    .filter_map(|role| knowledge.role(*role))
                    .map(|role| role.label().as_str().to_owned())
                    .collect()
            };
            let verb = knowledge
                .action(*statement.action())
                .map(|action| action.verb().as_str().to_owned())
                .unwrap_or_default();

            println!(
                "  {id}\n    {:?} {verb} for {:?}, fulfilled by {:?}, cancelled by {:?}",
                roles(statement.participants().actors()),
                roles(statement.participants().recipients()),
                statement
                    .settlement()
                    .fulfills()
                    .iter()
                    .map(|o| o.name())
                    .collect::<Vec<_>>(),
                statement
                    .settlement()
                    .cancels()
                    .iter()
                    .map(|o| o.name())
                    .collect::<Vec<_>>()
            );
        }
    }

    println!("\ncommitments");
    for id in &admitted.commitments {
        describe_commitment(knowledge, *id);
    }
}

fn describe_commitment(knowledge: &impl Knowledge, id: CommitmentId) {
    let Some(commitment) = knowledge.commitment(id) else {
        println!("  {id}  <absent from knowledge>");
        return;
    };

    let verb = knowledge
        .statement(*commitment.statement())
        .and_then(|statement| knowledge.action(*statement.action()))
        .map(|action| action.verb().as_str().to_owned())
        .unwrap_or_default();

    let instance = knowledge
        .resource_instance(*commitment.resource())
        .map(|instance| instance.label().as_str().to_owned())
        .unwrap_or_default();

    println!("  {id}");
    println!(
        "    {} {verb}s {} of {instance} to {:?}, due {} (committed {})",
        agent_label(knowledge, commitment.assignment().accountable()),
        commitment
            .action_value()
            .as_value()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "nothing quantified".to_owned()),
        commitment
            .assignment()
            .beneficiaries()
            .iter()
            .map(|agent| agent_label(knowledge, *agent))
            .collect::<Vec<_>>(),
        commitment.term().due_date().to_iso(),
        commitment.term().committed_at().to_iso()
    );

    println!();
}

/// One decision, with its identities as hex rather than as byte arrays.
fn describe_decision(decision: &Decision) {
    let ids = |set: &BTreeSet<CommitmentId>| -> Vec<String> {
        set.iter().map(|id| id.to_string()).collect()
    };

    match decision {
        Decision::Genesis {
            known_at,
            selection,
        } => println!(
            "  genesis  known_at {known_at}, selecting {:?}",
            ids(selection)
        ),

        Decision::Advance { extends, known_at } => {
            println!("  advance  extends {extends}, known_at {known_at}")
        }

        Decision::Fork {
            extends,
            omitted,
            introduced,
        } => println!(
            "  fork     extends {extends}\n           omitted {:?}\n           introduced {:?}",
            ids(omitted),
            ids(introduced)
        ),
    }
}

fn describe_worlds(
    lineage: &lineage::Lineage,
    decisions: &[Taken],
    knowledge: &impl Knowledge,
    admitted: &journal::Replayed,
) {
    println!("decisions on file");
    for taken in decisions {
        describe_decision(&taken.decision);
        println!(
            "    taken after entry {}, witnessing {} entries, by {}",
            taken.after,
            taken.witness.len(),
            taken
                .by
                .map(|by| format!("{} ({by})", agent_label(knowledge, by)))
                .unwrap_or_else(|| "nobody — the record claims no party".to_owned())
        );
    }

    println!("\nworlds on file");
    for thesis in lineage.decided() {
        println!(
            "  {}\n    cut at {} on chain {}\n    frozen {:?}\n    open   {:?}",
            thesis.id(),
            thesis.cut().known_at().to_iso(),
            thesis
                .cut()
                .event_head()
                .map(|head| head.to_string())
                .unwrap_or_else(|| "(none)".to_owned()),
            thesis
                .selection()
                .frozen()
                .map(|id| id.to_string())
                .collect::<Vec<_>>(),
            thesis
                .selection()
                .open()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        );
    }

    println!("\n{} journal entries admitted\n", admitted.entries.len());
}
