//! An audit of the house's account, conducted by walking the graph.
//!
//! The only handle taken from [`hindsight::build`] is the identity of the world the house is
//! in now. Everything else — the worlds it descends from, the commitments each selects, the
//! events each recognizes, the level each implies — is reached from that identity, through
//! the archive and the canonical history.
//!
//! Two rules govern what may be concluded, and both are the engine's rather than this
//! program's:
//!
//! - a Thesis is interpreted only under its own cut, so every verdict printed for a world is
//!   a verdict the knowledge of that world supports and no later knowledge enters it;
//! - the level is derived from movements, never stored, so the arithmetic is reproduced here
//!   line by line and checked against what the engine independently reports.
//!
//! What the program prints is the working. The conclusions drawn from it are in `ANSWER.md`.

mod hindsight;
mod world;

use std::collections::{BTreeMap, BTreeSet};

use ape::canon::{CanonicalHistory, CanonicalKnowledge, InMemoryHistory};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness, movement_of};
use ape::engine::thesis::{
    ForkInput, GenesisInput, InMemoryArchive, Interpretation, KnowledgeCut, Thesis, ThesisId,
    ThesisLookup,
};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{CommitmentId, Event, EventId};
use ape::kernel::value_objects::{ActionKind, Date, Effect, ResourceKind};

/// The three hypotheses feasibility offers, asked of every world.
const HYPOTHESES: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

/// What a commitment is called in this report, and the order the names were handed out in.
struct Ledger {
    labels: BTreeMap<CommitmentId, String>,
    order: Vec<CommitmentId>,
}

fn main() {
    let replay = hindsight::build();
    let history = replay.graph.canon.history();
    let archive = &replay.graph.archive;
    let current = replay.graph.current;

    banner("0. THE WAY IN");
    println!("the world the house is in now: {current}");
    println!(
        "\nthe canonical history's own head is {}, recorded through {}",
        option_id(history.head()),
        history
            .recorded_through()
            .map_or("nothing".to_owned(), |d| d.to_iso())
    );

    let worlds = lineage(archive, current);
    println!(
        "\nwalking `parent` from that identity reaches {} worlds before ending at a genesis.",
        worlds.len()
    );

    let ledger = name_commitments(history, &worlds);

    banner("1. WHAT WAS COMMITTED TO");
    report_commitments(history, &ledger);

    banner("2. WHAT WAS OBSERVED");
    report_chain(history, &ledger);

    banner("3. THE WORLDS, IN THE ORDER THEY WERE HELD");
    report_worlds(history, &ledger, &worlds);

    banner("4. THE FLOOR THE ACCOUNT ANSWERS TO");
    report_constraint(history, &ledger, &worlds);

    banner("5. WHAT EACH DECISION WAS DECIDED AGAINST");
    report_decisions(history, &ledger, &worlds);

    banner("6. WHAT THE CUT PREVENTS: THE SAME QUESTION ASKED WRONGLY");
    report_anachronism(history, &ledger, &worlds);

    banner("7. WHAT ELSE THE HOUSE COULD HAVE SELECTED");
    report_alternatives(history, &ledger, &worlds);

    banner("8. WHAT THE GRAPH DOES NOT CARRY");
    report_silences(history, &ledger, &worlds);
}

// ---------------------------------------------------------------- reaching the graph

/// The worlds from the genesis down to `current`, reached only by resolving `parent`.
fn lineage(archive: &InMemoryArchive, current: ThesisId) -> Vec<Thesis> {
    let mut chain = Vec::new();
    let mut cursor = Some(current);

    while let Some(id) = cursor {
        let thesis = archive
            .thesis(id)
            .expect("the archive refuses a child before its parent, so the walk resolves");

        cursor = *thesis.parent();
        chain.push(thesis);
    }

    chain.reverse();
    chain
}

/// Every commitment any world in the lineage selects, named in the order it was recorded.
fn name_commitments(history: &InMemoryHistory, worlds: &[Thesis]) -> Ledger {
    let mut reached: BTreeSet<CommitmentId> = BTreeSet::new();
    for world in worlds {
        reached.extend(world.selection().resolved());
    }

    let mut ordered: Vec<CommitmentId> = reached.into_iter().collect();
    ordered.sort_by_key(|id| {
        let record = record(history, *id);
        (
            *record.recorded_at(),
            *record.assertion().term().due_date(),
            id.to_string(),
        )
    });

    Ledger {
        labels: ordered
            .iter()
            .enumerate()
            .map(|(position, id)| (*id, format!("C{}", position + 1)))
            .collect(),
        order: ordered,
    }
}

fn agent_label(history: &InMemoryHistory, id: ape::kernel::entities::AgentId) -> String {
    history
        .agent(id)
        .map_or_else(|| short(id.to_string()), |a| a.label().as_str().to_owned())
}

fn record(
    history: &InMemoryHistory,
    id: CommitmentId,
) -> ape::canon::Canonical<ape::kernel::entities::Commitment> {
    history
        .canonical_commitment(id)
        .expect("a selected commitment is canonically admitted")
}

/// The chain a head recognizes, oldest first, walked back through `previous_event`.
fn chain(history: &InMemoryHistory, head: Option<EventId>) -> Vec<(Event, Date)> {
    let mut walked = Vec::new();
    let mut cursor = head;

    while let Some(id) = cursor {
        let record = history
            .canonical_event(id)
            .expect("a head names an event of the history");

        cursor = *record.assertion().previous_event();
        walked.push((record.assertion().clone(), *record.recorded_at()));
    }

    walked.reverse();
    walked
}

// ---------------------------------------------------------------- what is in the graph

fn report_commitments(history: &InMemoryHistory, ledger: &Ledger) {
    println!("every commitment reached by walking the selections of those worlds:\n");

    for id in &ledger.order {
        let label = ledger.name(*id);
        let record = record(history, *id);
        let commitment = record.assertion();

        let statement = history
            .statement(*commitment.statement())
            .expect("a commitment names an admitted statement");
        let action = history
            .action(*statement.action())
            .expect("a statement names an admitted action");
        let instance = history
            .resource_instance(*commitment.resource())
            .expect("a commitment names an admitted instance");

        let direction = match action.kind() {
            ActionKind::Quantifiable(Effect::Increase) => "+",
            ActionKind::Quantifiable(Effect::Decrease) => "-",
            ActionKind::Discrete => "?",
        };

        let magnitude = commitment
            .action_value()
            .as_value()
            .expect("a quantifiable action carries a magnitude");

        println!(
            "  {label}  {id}\n      {verb} {direction}{magnitude:.0} on '{instance}'\
             \n      committed_at {committed}  due {due}  recorded_at {recorded}\
             \n      accountable '{accountable}'  →  beneficiaries {beneficiaries}\
             \n      dependencies: {dependencies}",
            verb = action.verb().as_str(),
            instance = instance.label().as_str(),
            committed = commitment.term().committed_at().to_iso(),
            due = commitment.term().due_date().to_iso(),
            recorded = record.recorded_at().to_iso(),
            accountable = agent_label(history, commitment.assignment().accountable()),
            beneficiaries = commitment
                .assignment()
                .beneficiaries()
                .iter()
                .map(|a| agent_label(history, *a))
                .collect::<Vec<_>>()
                .join(", "),
            dependencies = if commitment.dependencies().is_empty() {
                "none".to_owned()
            } else {
                commitment
                    .dependencies()
                    .iter()
                    .map(|d| ledger.name(*d))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );
        println!();
    }

    println!(
        "the movement each contributes to the level, derived through the engine's own\n\
         `movement_of` rather than re-implemented here:\n"
    );
    for id in &ledger.order {
        let commitment = record(history, *id).assertion().clone();
        let movement = movement_of(history, &commitment)
            .expect("the commitment resolves")
            .expect("a quantifiable action moves a level");

        println!(
            "  {label}  {magnitude:>+5.0}  on instance {instance}",
            label = ledger.name(*id),
            magnitude = movement.magnitude(),
            instance = short(movement.instance().to_string()),
        );
    }
}

fn report_chain(history: &InMemoryHistory, ledger: &Ledger) {
    println!("the whole canonical event chain, oldest first:\n");

    for (event, recorded_at) in chain(history, history.head()) {
        println!(
            "  {id}\n      settles {commitment} with '{observation}'\
             \n      occurred_at {occurred}  recorded_at {recorded}\
             \n      previous_event {previous}",
            id = event.id(),
            commitment = ledger.name(*event.commitment_id()),
            observation = event.observation().name(),
            occurred = event.occurred_at().to_iso(),
            recorded = recorded_at.to_iso(),
            previous = option_id(*event.previous_event()),
        );
        println!();
    }

    println!(
        "both events were recorded on days the history can address. what the chain does\n\
         *not* order is the commitments: a commitment never enters the chain, so two\n\
         recorded on the same civil day cannot be put in order at all."
    );
}

fn report_worlds(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    for (position, world) in worlds.iter().enumerate() {
        println!("W{position}  {}", world.id());
        println!(
            "    cut: known_at {}  event_head {}",
            world.cut().known_at().to_iso(),
            option_id(world.cut().event_head())
        );

        if let Some(parent) = worlds.get(position.wrapping_sub(1)) {
            describe_edge(ledger, parent, world);
        } else {
            println!("    derivation: genesis — no parent");
        }

        println!(
            "    frozen: {}",
            ledger.names(world.selection().frozen().collect())
        );
        println!(
            "    open:   {}",
            ledger.names(world.selection().open().collect())
        );

        let interpretation = interpret(history, world);
        let conditions = interpretation
            .conditions_at(world.cut().known_at())
            .expect("a world is interpretable under its own cut");

        println!(
            "    conditions as of its own known_at ({}):",
            world.cut().known_at().to_iso()
        );
        for (id, condition) in conditions.conditions() {
            println!(
                "        {label:<4} {outcome:<10} {timeliness}",
                label = ledger.name(*id),
                outcome = format!("{:?}", condition.outcome()),
                timeliness = match condition.timeliness() {
                    Some(Timeliness::WithinDeadline) => "within deadline",
                    Some(Timeliness::Breached) => "BREACHED",
                    None => "(settled — no deadline applies)",
                },
            );
        }

        let settlements = recognized_settlements(history, world);
        let level = final_level(history, world, &settlements);

        println!(
            "    factual level — only what an event actually settled as fulfilled: {:+.0}",
            factual_level(history, world, &settlements)
        );
        println!("    level once every movement it still expects has landed:");
        print_arithmetic(history, ledger, world, &settlements);
        println!("        = {level:+.0}");

        for hypothesis in HYPOTHESES {
            let report = interpretation
                .feasibility_under(hypothesis)
                .expect("a world is interpretable under its own cut");

            println!(
                "    feasibility under {hypothesis:?}: {}",
                describe_conflicts(ledger, report.conflicts())
            );
        }

        // The audit's own arithmetic is worth nothing unless it moves with the engine's.
        let engine_level = interpretation
            .feasibility_under(Hypothesis::FinalState)
            .expect("a world is interpretable under its own cut")
            .conflicts()
            .iter()
            .find_map(|conflict| match conflict {
                Conflict::OutOfBounds { level, .. } => Some(*level),
                _ => None,
            });

        let allowed = within_bounds(history, world, level);
        let agreement = match engine_level {
            Some(reported) => !allowed && (reported - level).abs() < 1e-9,
            None => allowed,
        };

        println!(
            "    cross-check: this audit summed {level:+.0} ({}); the engine reported {} — {}",
            if allowed {
                "inside the bounds"
            } else {
                "outside the bounds"
            },
            match engine_level {
                Some(reported) => format!("a breach at {reported:+.0}"),
                None => "no breach".to_owned(),
            },
            if agreement { "agree" } else { "DISAGREE" }
        );

        println!();
    }
}

/// Which of the three derivations produced a world, read by comparing it with its parent.
///
/// The Thesis carries no record of the operation that made it; what it carries is a cut and a
/// selection, and only one derivation can move each.
fn describe_edge(ledger: &Ledger, parent: &Thesis, child: &Thesis) {
    let parent_open: BTreeSet<CommitmentId> = parent.selection().open().collect();
    let child_open: BTreeSet<CommitmentId> = child.selection().open().collect();

    if child.cut() == parent.cut() {
        println!(
            "    derivation: FORK — same cut, so a decision. introduced {}, omitted {}",
            ledger.names(child_open.difference(&parent_open).copied().collect()),
            ledger.names(parent_open.difference(&child_open).copied().collect()),
        );
        return;
    }

    let parent_frozen: BTreeSet<CommitmentId> = parent.selection().frozen().collect();
    let child_frozen: BTreeSet<CommitmentId> = child.selection().frozen().collect();
    let newly_frozen: BTreeSet<CommitmentId> =
        child_frozen.difference(&parent_frozen).copied().collect();

    let imposed: Vec<CommitmentId> = newly_frozen
        .iter()
        .filter(|id| !parent.selection().contains(**id))
        .copied()
        .collect();

    println!(
        "    derivation: ADVANCE — later cut, intention preserved. newly frozen {}, \
         of which imposed by history {}",
        ledger.names(newly_frozen),
        if imposed.is_empty() {
            "none".to_owned()
        } else {
            ledger.names(imposed.into_iter().collect())
        },
    );
}

fn report_constraint(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    let any = worlds
        .last()
        .expect("the lineage is non-empty")
        .selection()
        .resolved()
        .next()
        .expect("the current world selects something");

    let instance_id = *record(history, any).assertion().resource();
    let instance = history
        .resource_instance(instance_id)
        .expect("an admitted instance");
    let resource = history
        .resource(*instance.resource())
        .expect("an admitted resource");

    println!(
        "the instance every commitment in {} moves is '{}', of resource '{}'.",
        ledger.names(
            worlds
                .last()
                .expect("the lineage is non-empty")
                .selection()
                .resolved()
                .collect()
        ),
        instance.label().as_str(),
        resource.label().as_str()
    );

    let ResourceKind::Quantifiable(constraint) = resource.kind() else {
        println!("the resource is discrete — no level, no floor.");
        return;
    };

    println!("\nthe constraint is opaque: it exposes `check`, not its bound. probing it:\n");
    for probe in [-20.0_f64, -0.5, -0.0001, 0.0, 0.0001, 70.0, 100.0] {
        println!(
            "    check({probe:>9.4}) = {}",
            if constraint.check(probe) {
                "allowed"
            } else {
                "REFUSED"
            }
        );
    }
    println!(
        "\nthe refusals stop exactly below zero, so the floor is `level >= 0`. its debug form\n\
         confirms the reading: {constraint:?}"
    );
}

fn report_decisions(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    println!(
        "for each edge, the level its parent implied and the level it left, both derived\n\
         under each world's own cut:\n"
    );

    for position in 1..worlds.len() {
        let parent = &worlds[position - 1];
        let child = &worlds[position];

        let before = final_level(
            history,
            parent,
            &recognized_settlements(history, parent),
        );
        let after = final_level(history, child, &recognized_settlements(history, child));

        let kind = if child.cut() == parent.cut() {
            "fork (a decision)"
        } else {
            "advance (knowledge, not a decision)"
        };

        let clean_before = conflicts_free(history, parent);
        let clean_after = conflicts_free(history, child);

        println!(
            "  W{} → W{}  {kind}\n      open {} → {}\n      level {before:+.0} → {after:+.0}\
             \n      breaches the floor before? {} ; after? {}",
            position - 1,
            position,
            ledger.names(parent.selection().open().collect()),
            ledger.names(child.selection().open().collect()),
            if clean_before { "no" } else { "YES" },
            if clean_after { "no" } else { "YES" },
        );

        if !clean_after && clean_before {
            println!("      ← this edge is where the floor was first breached by a choice");
        }
        if !clean_after && !clean_before {
            println!("      ← already breached before this edge");
        }
        println!();
    }
}

/// The trap the exact-cut boundary closes, shown by walking into it deliberately.
fn report_anachronism(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    let genesis = &worlds[0];

    println!(
        "the genesis recognizes head {}.\nasking the canon today for the cut of the same \
         instant, {}, yields head {}.",
        option_id(genesis.cut().event_head()),
        genesis.cut().known_at().to_iso(),
        option_id(KnowledgeCut::at(history, *genesis.cut().known_at()).event_head()),
    );

    println!(
        "\nthose differ, so an event was recorded on {} *after* the genesis was taken.",
        genesis.cut().known_at().to_iso()
    );

    if let Some(head) = genesis.cut().event_head() {
        match KnowledgeCut::within(history, *genesis.cut().known_at(), head) {
            Ok(_) => println!("the same cut is still constructible today."),
            Err(error) => println!(
                "asking the canon to rebuild that exact cut today is refused: {error}\n\
                 the genesis therefore could not have been taken with the cancellation in\n\
                 hand. this is the only evidence in the graph that orders two things inside\n\
                 one civil day, and it works because a cut names a head."
            ),
        }
    }

    let settlements_now = settlements_from_chain(history, history.head());
    let settlements_then = recognized_settlements(history, genesis);

    println!(
        "\nsettlements the genesis recognizes: {}",
        describe_settlements(ledger, &settlements_then)
    );
    println!(
        "settlements the history knows now:   {}",
        describe_settlements(ledger, &settlements_now)
    );

    let honest = final_level(history, genesis, &settlements_then);
    let anachronistic = final_level(history, genesis, &settlements_now);

    println!(
        "\nthe same selection scored against the two: {honest:+.0} under its own cut, \
         {anachronistic:+.0} under today's.\nthe second number is the one an audit would \
         reach by asking 'what does that world mean now'; it is not\nwhat the house was \
         looking at, and the engine refuses to produce it — `Interpretation::of` takes the\n\
         chain from the Thesis, never from the caller."
    );
}

fn report_alternatives(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    let pool: Vec<CommitmentId> = ledger.labels.keys().copied().collect();

    println!(
        "the pool of commitments this audit can enumerate is exactly the ones some world in\n\
         the lineage selected: {}. the canon offers lookup by identity and no enumeration,\n\
         so a commitment the house admitted and never selected anywhere is unreachable from\n\
         here and cannot appear below.\n",
        ledger.names(pool.iter().copied().collect())
    );

    let genesis = &worlds[0];
    println!(
        "── at the genesis cut ({}, head {}) ──\n",
        genesis.cut().known_at().to_iso(),
        option_id(genesis.cut().event_head())
    );
    println!(
        "a genesis selection is a proposal: the cut adds whatever it made unavoidable, so\n\
         two proposals differing only by a frozen commitment denote the same world.\n"
    );

    let available: Vec<CommitmentId> = pool
        .iter()
        .copied()
        .filter(|id| {
            record(history, *id)
                .recorded_at()
                .up_to(genesis.cut().known_at())
        })
        .collect();

    for subset in subsets(&available) {
        let selection: BTreeSet<CommitmentId> = subset.iter().copied().collect();

        match Thesis::genesis(
            history,
            GenesisInput {
                cut: genesis.cut().clone(),
                selection: selection.clone(),
            },
        ) {
            Ok(thesis) => {
                let settlements = recognized_settlements(history, &thesis);
                println!(
                    "  select {:<12} → level {:+.0}  {}{}",
                    ledger.names(selection.clone()),
                    final_level(history, &thesis, &settlements),
                    verdict(history, ledger, &thesis),
                    if thesis.id() == genesis.id() {
                        "   ← what the house did"
                    } else {
                        ""
                    }
                );
            }
            Err(error) => println!(
                "  select {:<12} → refused: {error}",
                ledger.names(selection)
            ),
        }
    }

    let last_fork = worlds.len() - 1;
    let parent = &worlds[last_fork - 1];
    println!(
        "\n── at the last decision, forking W{} ({}, head {}) ──\n",
        last_fork - 1,
        parent.cut().known_at().to_iso(),
        option_id(parent.cut().event_head())
    );

    let revisable: Vec<CommitmentId> = pool
        .iter()
        .copied()
        .filter(|id| !parent.selection().is_frozen(*id))
        .filter(|id| record(history, *id).recorded_at().up_to(parent.cut().known_at()))
        .collect();

    let parent_open: BTreeSet<CommitmentId> = parent.selection().open().collect();

    for subset in subsets(&revisable) {
        let target: BTreeSet<CommitmentId> = subset.iter().copied().collect();

        let input = ForkInput {
            omitted: parent_open.difference(&target).copied().collect(),
            introduced: target.difference(&parent_open).copied().collect(),
        };

        match parent.fork(history, input) {
            Ok(thesis) => {
                let settlements = recognized_settlements(history, &thesis);
                println!(
                    "  open {:<12} → level {:+.0}  {}{}",
                    ledger.names(target.clone()),
                    final_level(history, &thesis, &settlements),
                    verdict(history, ledger, &thesis),
                    if thesis.id() == worlds[last_fork].id() {
                        "   ← what the house did"
                    } else {
                        ""
                    }
                );
            }
            Err(error) => println!("  open {:<12} → refused: {error}", ledger.names(target)),
        }
    }
}

fn report_silences(history: &InMemoryHistory, ledger: &Ledger, worlds: &[Thesis]) {
    println!("questions this graph was interrogated for and did not answer:\n");

    let cancel = chain(history, history.head())
        .into_iter()
        .find(|(event, _)| event.observation().name() == "Cancelled");

    match cancel {
        Some((event, recorded)) => {
            println!("  · why the cancellation happened. the event carries an observation name");
            println!(
                "    ('{}'), an occurred_at ({}) and a recorded_at ({}). there is no field",
                event.observation().name(),
                event.occurred_at().to_iso(),
                recorded.to_iso()
            );
            println!("    for a reason and none for who observed it.");
        }
        None => println!("  · no cancellation is present in the chain."),
    }

    println!("\n  · who took each decision. a Thesis carries parent, cut and selection — the");
    println!("    fields its identity is derived from. no author, no timestamp of its own.");

    println!("\n  · when each decision was physically taken. the cut is *declared* knowledge,");
    println!("    not attested provenance; the layer says so itself. the only ordering");
    println!("    evidence recovered above is indirect: a cut naming a head the instant no");
    println!("    longer addresses.");

    println!("\n  · whether anyone ever ran feasibility. nothing records that a projection was");
    println!("    taken; projections are derived and never stored.");

    let same_day: Vec<String> = {
        let mut by_day: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for id in worlds
            .iter()
            .flat_map(|world| world.selection().resolved())
            .collect::<BTreeSet<CommitmentId>>()
        {
            by_day
                .entry(record(history, id).recorded_at().to_iso())
                .or_default()
                .push(ledger.name(id));
        }
        by_day
            .into_iter()
            .filter(|(_, named)| named.len() > 1)
            .map(|(day, named)| format!("{day} ({})", named.join(" and ")))
            .collect()
    };

    println!("\n  · the order of commitments recorded on one civil day. recording time is a");
    println!("    date, and a commitment never enters the event chain, so there is nothing");
    println!(
        "    finer to read. days carrying more than one selected commitment: {}",
        if same_day.is_empty() {
            "none".to_owned()
        } else {
            same_day.join(", ")
        }
    );
}

// ---------------------------------------------------------------- deriving the level

/// The settlements a world's own cut recognizes, resolved the way Hermeneia resolves them:
/// each event of the recognized chain against the settlement its commitment's statement names.
fn recognized_settlements(
    history: &InMemoryHistory,
    world: &Thesis,
) -> BTreeMap<CommitmentId, Outcome> {
    settlements_from_chain(history, world.cut().event_head())
}

fn settlements_from_chain(
    history: &InMemoryHistory,
    head: Option<EventId>,
) -> BTreeMap<CommitmentId, Outcome> {
    let mut settled = BTreeMap::new();

    for (event, _) in chain(history, head) {
        let commitment = history
            .commitment(*event.commitment_id())
            .expect("an event settles an admitted commitment");
        let statement = history
            .statement(*commitment.statement())
            .expect("a commitment names an admitted statement");

        let outcome = if statement.settlement().can_settle(event.observation()) {
            Outcome::Fulfilled
        } else if statement.settlement().can_cancel(event.observation()) {
            Outcome::Cancelled
        } else {
            continue;
        };

        settled.insert(*event.commitment_id(), outcome);
    }

    settled
}

/// The level once every movement the world still expects has landed: fulfilled movements are
/// facts, unsettled ones are the hypothesis, cancelled ones never happened.
fn final_level(
    history: &InMemoryHistory,
    world: &Thesis,
    settlements: &BTreeMap<CommitmentId, Outcome>,
) -> f64 {
    world
        .selection()
        .resolved()
        .filter(|id| settlements.get(id) != Some(&Outcome::Cancelled))
        .map(|id| magnitude(history, id))
        .sum()
}

/// What actually moved the resource: only a fulfilled commitment contributes a fact.
fn factual_level(
    history: &InMemoryHistory,
    world: &Thesis,
    settlements: &BTreeMap<CommitmentId, Outcome>,
) -> f64 {
    world
        .selection()
        .resolved()
        .filter(|id| settlements.get(id) == Some(&Outcome::Fulfilled))
        .map(|id| magnitude(history, id))
        .sum()
}

fn print_arithmetic(
    history: &InMemoryHistory,
    ledger: &Ledger,
    world: &Thesis,
    settlements: &BTreeMap<CommitmentId, Outcome>,
) {
    let mut running = 0.0;

    for id in world.selection().resolved() {
        let outcome = settlements.get(&id).cloned().unwrap_or(Outcome::Unsettled);
        let contribution = if outcome == Outcome::Cancelled {
            0.0
        } else {
            magnitude(history, id)
        };

        running += contribution;

        println!(
            "        {label:<4} {outcome:<10} {contribution:>+8.0}   running {running:+.0}",
            label = ledger.name(id),
            outcome = format!("{outcome:?}"),
        );
    }
}

/// Whether `level` satisfies the constraint the world's own resource declares.
fn within_bounds(history: &InMemoryHistory, world: &Thesis, level: f64) -> bool {
    let Some(any) = world.selection().resolved().next() else {
        return true;
    };

    let instance = history
        .resource_instance(*record(history, any).assertion().resource())
        .expect("an admitted instance");
    let resource = history
        .resource(*instance.resource())
        .expect("an admitted resource");

    match resource.kind() {
        ResourceKind::Quantifiable(constraint) => constraint.check(level),
        ResourceKind::Discrete => true,
    }
}

fn magnitude(history: &InMemoryHistory, id: CommitmentId) -> f64 {
    let commitment = record(history, id).assertion().clone();

    movement_of(history, &commitment)
        .expect("the commitment resolves")
        .map_or(0.0, |movement| movement.magnitude())
}

// ---------------------------------------------------------------- asking the engine

fn interpret(history: &InMemoryHistory, world: &Thesis) -> Interpretation {
    Interpretation::of(world, history).expect("a world is interpretable under its own cut")
}

fn verdict(history: &InMemoryHistory, ledger: &Ledger, world: &Thesis) -> String {
    let interpretation = interpret(history, world);

    HYPOTHESES
        .iter()
        .map(|hypothesis| {
            let report = interpretation
                .feasibility_under(*hypothesis)
                .expect("a world is interpretable under its own cut");

            format!(
                "{hypothesis:?}: {}",
                describe_conflicts(ledger, report.conflicts())
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn conflicts_free(history: &InMemoryHistory, world: &Thesis) -> bool {
    let interpretation = interpret(history, world);

    HYPOTHESES.iter().all(|hypothesis| {
        interpretation
            .feasibility_under(*hypothesis)
            .expect("a world is interpretable under its own cut")
            .conflicts()
            .is_empty()
    })
}

fn describe_conflicts(ledger: &Ledger, conflicts: &[Conflict]) -> String {
    if conflicts.is_empty() {
        return "no conflict found".to_owned();
    }

    conflicts
        .iter()
        .map(|conflict| match conflict {
            Conflict::OutOfBounds { instance, level } => format!(
                "OutOfBounds on {} at level {level:+.0}",
                short(instance.to_string())
            ),
            Conflict::Unrealizable(id) => format!("Unrealizable({})", ledger.name(*id)),
            Conflict::PunctualDependencyViolation {
                dependency,
                dependent,
            } => format!(
                "PunctualDependencyViolation({} before {})",
                ledger.name(*dependency),
                ledger.name(*dependent)
            ),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn describe_settlements(ledger: &Ledger, settlements: &BTreeMap<CommitmentId, Outcome>) -> String {
    if settlements.is_empty() {
        return "none".to_owned();
    }

    settlements
        .iter()
        .map(|(id, outcome)| format!("{} {outcome:?}", ledger.name(*id)))
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------- presentation

impl Ledger {
    fn name(&self, id: CommitmentId) -> String {
        self.labels
            .get(&id)
            .cloned()
            .unwrap_or_else(|| short(id.to_string()))
    }

    fn names(&self, ids: BTreeSet<CommitmentId>) -> String {
        if ids.is_empty() {
            return "{}".to_owned();
        }

        let mut named: Vec<String> = ids.into_iter().map(|id| self.name(id)).collect();
        named.sort();

        format!("{{{}}}", named.join(", "))
    }
}

fn subsets(pool: &[CommitmentId]) -> Vec<Vec<CommitmentId>> {
    (0..(1u32 << pool.len()))
        .map(|mask| {
            pool.iter()
                .enumerate()
                .filter(|(slot, _)| mask & (1 << slot) != 0)
                .map(|(_, id)| *id)
                .collect()
        })
        .collect()
}

fn short(id: String) -> String {
    id.chars().take(8).collect()
}

fn option_id<T: std::fmt::Display>(id: Option<T>) -> String {
    id.map_or("none".to_owned(), |id| short(id.to_string()))
}

fn banner(title: &str) {
    println!("\n{}", "=".repeat(78));
    println!("{title}");
    println!("{}\n", "=".repeat(78));
}
