//! Finance takes on the market's storage invoice: 30, payable by 2026-01-14.
//!
//! Two things happen here, in the two layers the crates keep apart.
//!
//! **What finance intends becomes knowledge.** The invoice is an obligation of the house
//! towards the market, so it is admitted to canonical history as one Commitment on the
//! existing vocabulary — the `spend` statement the house's other payables already use, over
//! the `account` instance of `cash`. Nothing new is admitted besides the commitment itself:
//! the house already knows every role, agent, action and statement this needs, and inventing a
//! second way to say "the house pays the market" would be a second version of the same fact.
//!
//! **What finance decides becomes a world.** A commitment admitted to history is not yet part
//! of anybody's plan — `advance` recognizes later knowledge and never adds an intention — so
//! the payable enters the plan by a `fork` of the world currently at the tip, introducing it
//! under the cut that world already recognizes. That decision is written down as *finance's*,
//! which is the one place the repository can carry a party at all.
//!
//! Nothing is hard-coded by identity. Every party, resource and statement is found by the
//! label the records give it, so this program says what it means rather than what the fixture
//! happened to hash to — and it would fail loudly, rather than act on the wrong agent, if the
//! records named someone else.
//!
//! The write-back goes through `converge`, which re-reads the repository, appends to the
//! journal it finds there, merges the decisions, rebuilds the whole thing in memory, and only
//! then writes. Reading the result back afterwards is done through `reconstruct`, which is
//! given the repository and nothing else — no value computed above reaches it.

use std::collections::BTreeSet;

use ape::canon::CanonicalHistory;
use ape::engine::hermeneia::movement_of;
use ape::engine::thesis::{Thesis, ThesisId};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, RoleId, StatementId};
use ape::kernel::value_objects::Date;

use ape_cli::converge;
use ape_cli::journal::{self, Admission, ResourceKindRecord};
use ape_cli::lineage::{self, Decision, Taken};
use ape_cli::reading::{self, Corroborated, Reading};
use ape_cli::repository::Repository;

const REPOSITORY: &str = "repo";

/// What the market invoiced, and by when.
const STORAGE: f64 = 30.0;
const PAYABLE_BY: &str = "2026-01-14";

/// The instant finance records at: the most recent one the house's knowledge already holds.
///
/// Recording later would have meant inventing a date the records say nothing about, and would
/// have cost an `advance` before the fork — a decision about *what could be known* that
/// finance has no observation to justify.
const RECORDED_AT: &str = "2026-01-07";

/// When the worlds are read at the end: the day the invoice falls due.
const READ_AT: &str = "2026-01-14";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository = Repository::open(REPOSITORY);

    let mut held = reading::corroborated(&repository)?;
    let cast = Cast::read(&held)?;

    heading("what the house's records already say");
    cast.describe(&held);
    describe_worlds(&held, &cast);

    let tip = tip_of(&held)?;
    println!(
        "\nthe world at the tip is {}, and it is what finance decides against.",
        short(&tip.to_string())
    );

    heading("1. what finance records");
    let payable = match already_recorded(&held, &cast) {
        Some(payable) => {
            println!(
                "the journal already holds the invoice, as {} — nothing to record.",
                short(&payable.to_string())
            );
            payable
        }
        None => record_the_invoice(&mut held, &cast)?,
    };

    heading("2. what finance decides");
    let (world, decided_now) = match world_selecting(&held, payable) {
        Some(world) => {
            println!(
                "the payable is already part of world {} — nothing to decide.",
                short(&world.to_string())
            );
            (world, false)
        }
        None => (decide_as_finance(&mut held, &cast, tip, payable)?, true),
    };
    describe_intention(&held, &cast, world)?;

    heading("3. putting it back");
    match decided_now {
        false => println!(
            "{}/ already holds it; nothing to write.",
            repository.root().display()
        ),
        true => {
            let put_back = converge::converge(&repository, &held)?;
            println!(
                "{} admissions and {} decisions written to {}/",
                put_back.journal.len(),
                put_back.decisions.len(),
                repository.root().display()
            );
            for path in [
                repository.journal_path(),
                repository.lineage_path(),
                repository.worlds_path(),
            ] {
                println!("  {}", path.display());
            }
        }
    }

    heading("4. read again, from the repository alone");
    let readings = reading::reconstruct(&repository, cast.account, &Date::parse(READ_AT)?)?;
    println!(
        "{} worlds rebuilt and corroborated against what was recorded, read at {READ_AT}:\n",
        readings.len()
    );
    for reading in &readings {
        cast.describe_reading(reading);
    }

    println!(
        "worlds the repository says finance decided: {}",
        joined(
            reading::decided_by(&repository, cast.finance)?
                .iter()
                .map(|id| short(id))
        )
    );
    println!(
        "worlds the repository says operations decided: {}",
        joined(
            reading::decided_by(&repository, cast.operations)?
                .iter()
                .map(|id| short(id))
        )
    );

    Ok(())
}

/// Admit the invoice as one Commitment, and hand back the identity it got.
///
/// The house is accountable because the market invoiced the house; finance is the executor
/// because taking the payment on is what finance was asked to do, and the eligibility the
/// house recorded for finance on 2026-01-03 in the `spender` role is what makes that
/// admissible. Nothing else about who did the recording is representable — see `ANSWER.md`.
fn record_the_invoice(
    held: &mut Corroborated,
    cast: &Cast,
) -> Result<CommitmentId, Box<dyn std::error::Error>> {
    held.journal.push(Admission::Commitment {
        accountable: cast.house,
        executors: BTreeSet::from([cast.finance]),
        beneficiaries: BTreeSet::from([cast.market]),
        statement: cast.payment,
        resource: cast.account,
        committed_at: RECORDED_AT.to_owned(),
        due_date: PAYABLE_BY.to_owned(),
        magnitude: Some(STORAGE),
        dependencies: BTreeSet::new(),
        recorded_at: RECORDED_AT.to_owned(),
    });

    journal::replay_remaining(&mut held.canon, &held.journal, &mut held.admitted)?;

    let payable = *held
        .admitted
        .commitments
        .last()
        .ok_or("the journal admitted no commitment")?;

    println!(
        "admitted a commitment {}: {} spends {STORAGE} from {} to {}, committed {RECORDED_AT}, \
         due {PAYABLE_BY}, executed by {}.",
        short(&payable.to_string()),
        cast.label_of(cast.house),
        cast.label_of(cast.account),
        cast.label_of(cast.market),
        cast.label_of(cast.finance),
    );
    println!(
        "the journal now holds {} admissions, and history is recorded through {}.",
        held.admitted.entries.len(),
        held.canon
            .history()
            .recorded_through()
            .map(|date| date.to_iso())
            .unwrap_or_else(|| "nothing".to_owned()),
    );

    Ok(payable)
}

/// Fork the tip to introduce the payable, and record that finance is who decided it.
fn decide_as_finance(
    held: &mut Corroborated,
    cast: &Cast,
    tip: ThesisId,
    payable: CommitmentId,
) -> Result<ThesisId, Box<dyn std::error::Error>> {
    let taken = Taken::claimed(
        Decision::Fork {
            extends: tip,
            omitted: BTreeSet::new(),
            introduced: BTreeSet::from([payable]),
        },
        cast.finance,
        &held.admitted,
    )?;

    lineage::decide(held.canon.history(), &mut held.lineage, &taken.decision)?;

    println!(
        "forked {} introducing {}, omitting nothing, claimed by {}.",
        short(&tip.to_string()),
        short(&payable.to_string()),
        cast.label_of(cast.finance),
    );
    println!(
        "taken after journal entry {}, witnessed by {} entries.",
        short(&taken.after.to_string()),
        taken.witness.len(),
    );

    held.decisions.push(taken);

    Ok(held
        .lineage
        .decided()
        .last()
        .ok_or("the fork decided no world")?
        .id())
}

/// The invoice, if a previous run already admitted it.
///
/// Identified by what the brief fixes — the amount, the day it falls due, and finance among the
/// parties that execute it — because that is what a second run would be about to record again.
fn already_recorded(held: &Corroborated, cast: &Cast) -> Option<CommitmentId> {
    let history = held.canon.history();
    let payable_by = Date::parse(PAYABLE_BY).ok()?;

    held.admitted.commitments.iter().copied().find(|id| {
        history.commitment(*id).is_some_and(|commitment| {
            commitment.action_value().as_value() == Some(STORAGE)
                && *commitment.term().due_date() == payable_by
                && commitment.assignment().executors().contains(&cast.finance)
        })
    })
}

/// The world a decision already put `payable` in, if any.
fn world_selecting(held: &Corroborated, payable: CommitmentId) -> Option<ThesisId> {
    held.lineage
        .decided()
        .iter()
        .find(|world| world.selection().contains(payable))
        .map(Thesis::id)
}

/// What the new world intends, and what the account would hold if all of it landed.
///
/// The sum is printed because no stored number answers for it: `Reading::level` counts only
/// what a projection reports as fulfilled, and feasibility reports conflicts rather than
/// levels. The arithmetic is the engine's — `movement_of` says what one commitment moves and
/// in which direction — and only the summing is done here.
fn describe_intention(
    held: &Corroborated,
    cast: &Cast,
    world: ThesisId,
) -> Result<(), Box<dyn std::error::Error>> {
    let world = held
        .lineage
        .decided()
        .iter()
        .find(|decided| decided.id() == world)
        .ok_or("the lineage decided no such world")?;

    println!(
        "\nthe world finance is planning in is {}, at the cut {} it inherits.",
        short(&world.id().to_string()),
        world.cut().known_at().to_iso(),
    );

    let history = held.canon.history();
    let mut projected = 0.0;

    for (half, ids) in [
        ("frozen", world.selection().frozen().collect::<Vec<_>>()),
        ("open  ", world.selection().open().collect::<Vec<_>>()),
    ] {
        for id in ids {
            let commitment = history
                .commitment(id)
                .ok_or("the world selects a commitment history does not hold")?;

            let movement = movement_of(history, &commitment)?
                .map(|movement| movement.magnitude())
                .unwrap_or(0.0);

            projected += movement;

            println!(
                "  {half}  {}  {movement:+}  due {}",
                short(&id.to_string()),
                commitment.term().due_date().to_iso(),
            );
        }
    }

    match cast.bounds {
        Some((lower, upper)) => println!(
            "  once every movement lands, {} holds {projected}, within the bounds [{lower}, {upper}] \
             its resource declares.",
            cast.label_of(cast.account),
        ),
        None => println!("  once every movement lands, {} holds {projected}.", cast.label_of(cast.account)),
    }

    Ok(())
}

/// Every party, resource and statement this program needs, found by the label it carries.
///
/// It exists so that nothing below reads an identity out of the fixture. A label is what the
/// brief speaks in — "finance", "the market", "the house" — and resolving the two against each
/// other once, loudly, is what makes the rest of the program answerable to the brief.
struct Cast {
    house: AgentId,
    market: AgentId,
    operations: AgentId,
    finance: AgentId,
    spender: RoleId,
    counterparty: RoleId,
    account: ResourceInstanceId,
    /// The statement the house's payables are stated in: a spender spends, a counterparty receives.
    payment: StatementId,
    /// The bounds the `cash` resource declares, read from the journal because a `Constraint`
    /// cannot be read back off an admitted `Resource`.
    bounds: Option<(f64, f64)>,
    named: Vec<(String, String)>,
}

impl Cast {
    fn read(held: &Corroborated) -> Result<Self, String> {
        let history = held.canon.history();
        let admitted = &held.admitted;

        let agent = |label: &str| {
            named(&admitted.agents, label, |id| {
                history.agent(id).map(|agent| agent.label().as_str().to_owned())
            })
        };

        let role = |label: &str| {
            named(&admitted.roles, label, |id| {
                history.role(id).map(|role| role.label().as_str().to_owned())
            })
        };

        let account = named(&admitted.instances, "account", |id| {
            history
                .resource_instance(id)
                .map(|instance| instance.label().as_str().to_owned())
        })?;

        let payment = *admitted
            .statements
            .iter()
            .find(|id| {
                history
                    .statement(**id)
                    .and_then(|statement| history.action(*statement.action()))
                    .is_some_and(|action| action.verb().as_str() == "spend")
            })
            .ok_or("no statement the house knows realizes a spend")?;

        let mut cast = Self {
            house: agent("house")?,
            market: agent("market")?,
            operations: agent("operations")?,
            finance: agent("finance")?,
            spender: role("spender")?,
            counterparty: role("counterparty")?,
            account,
            payment,
            bounds: bounds_of(&held.journal, "cash"),
            named: Vec::new(),
        };

        cast.named = vec![
            (cast.house.to_string(), "house".to_owned()),
            (cast.market.to_string(), "market".to_owned()),
            (cast.operations.to_string(), "operations".to_owned()),
            (cast.finance.to_string(), "finance".to_owned()),
            (cast.spender.to_string(), "spender".to_owned()),
            (cast.counterparty.to_string(), "counterparty".to_owned()),
            (cast.account.to_string(), "account".to_owned()),
        ];

        Ok(cast)
    }

    /// The label the records give an identity, with the head of the identity itself alongside.
    fn label_of(&self, id: impl std::fmt::Display) -> String {
        let id = id.to_string();

        match self.named.iter().find(|(known, _)| *known == id) {
            Some((_, label)) => format!("{label}({})", short(&id)),
            None => short(&id),
        }
    }

    fn describe(&self, held: &Corroborated) {
        let history = held.canon.history();

        println!("agents:  {}", joined([
            self.label_of(self.house),
            self.label_of(self.market),
            self.label_of(self.operations),
            self.label_of(self.finance),
        ]));
        println!("roles:   {}", joined([
            self.label_of(self.spender),
            self.label_of(self.counterparty),
        ]));

        match self.bounds {
            Some((lower, upper)) => println!(
                "cash:    quantifiable within [{lower}, {upper}], one instance: {}",
                self.label_of(self.account)
            ),
            None => println!("cash:    one instance: {}", self.label_of(self.account)),
        }

        for id in &held.admitted.statements {
            let Some(statement) = history.statement(*id) else {
                continue;
            };
            let Some(action) = history.action(*statement.action()) else {
                continue;
            };

            println!(
                "stmt:    {} — {} {} for {}{}",
                short(&id.to_string()),
                joined(statement.participants().actors().iter().map(|r| self.label_of(*r))),
                action.verb().as_str(),
                joined(statement.participants().recipients().iter().map(|r| self.label_of(*r))),
                if *id == self.payment { "   ← what a payable is stated in" } else { "" },
            );
        }

        println!("\nwhat the house has committed to, and who answers for it:");
        for id in &held.admitted.commitments {
            let Some(commitment) = history.commitment(*id) else {
                continue;
            };

            let movement = movement_of(history, &commitment)
                .ok()
                .flatten()
                .map(|movement| movement.magnitude())
                .unwrap_or(0.0);

            println!(
                "  {}  {movement:+}  due {}  accountable {}  executors {}  beneficiaries {}{}",
                short(&id.to_string()),
                commitment.term().due_date().to_iso(),
                self.label_of(commitment.assignment().accountable()),
                joined(commitment.assignment().executors().iter().map(|a| self.label_of(*a))),
                joined(commitment.assignment().beneficiaries().iter().map(|a| self.label_of(*a))),
                match history.event_of(*id) {
                    Some(event) => format!("  [{} on {}]", event.observation().name(), event.occurred_at().to_iso()),
                    None => String::new(),
                },
            );
        }
    }

    fn describe_reading(&self, reading: &Reading) {
        println!(
            "  world {}  known_at {}  frozen {}  open {}",
            short(&reading.thesis),
            reading.known_at,
            joined(reading.frozen.iter().map(|id| short(id))),
            joined(reading.open.iter().map(|id| short(id))),
        );

        for (id, condition) in &reading.conditions {
            println!(
                "    {}  {:?}  {}",
                short(id),
                condition.outcome,
                match &condition.timeliness {
                    Some(timeliness) => format!("{timeliness:?}"),
                    None => "-".to_owned(),
                },
            );
        }

        println!(
            "    settled level of {}: {}   conflicts under FinalState: {}\n",
            self.label_of(self.account),
            reading.level,
            if reading.conflicts.is_empty() {
                "none".to_owned()
            } else {
                format!("{:?}", reading.conflicts)
            },
        );
    }
}

/// The worlds the repository already holds, and which party each decision claims.
fn describe_worlds(held: &Corroborated, cast: &Cast) {
    println!("\nworlds already decided, oldest first:");

    for (taken, world) in held.decisions.iter().zip(held.lineage.decided()) {
        println!(
            "  {}  {:12}  known_at {}  open {}  decided by {}",
            short(&world.id().to_string()),
            match taken.decision {
                Decision::Genesis { .. } => "genesis",
                Decision::Advance { .. } => "advance",
                Decision::Fork { .. } => "fork",
            },
            world.cut().known_at().to_iso(),
            joined(world.selection().open().map(|id| short(&id.to_string()))),
            match taken.by {
                Some(party) => cast.label_of(party),
                None => "nobody".to_owned(),
            },
        );
    }
}

/// The world no other world descends from, which is the one a new decision extends.
fn tip_of(held: &Corroborated) -> Result<ThesisId, String> {
    let decided = held.lineage.decided();

    let tips: Vec<ThesisId> = decided
        .iter()
        .map(Thesis::id)
        .filter(|id| !decided.iter().any(|other| *other.parent() == Some(*id)))
        .collect();

    match tips.as_slice() {
        [only] => Ok(*only),
        [] => Err("the lineage decides no world".to_owned()),
        many => Err(format!(
            "the lineage has {} tips, and nothing here says which one finance is planning from",
            many.len()
        )),
    }
}

fn bounds_of(journal: &[Admission], resource: &str) -> Option<(f64, f64)> {
    journal.iter().find_map(|entry| match entry {
        Admission::Resource {
            label,
            kind: ResourceKindRecord::Between { lower, upper },
            ..
        } if label == resource => Some((*lower, *upper)),
        _ => None,
    })
}

/// The one id in `ids` whose label is `label`, or a refusal naming what was looked for.
fn named<I: Copy>(
    ids: &[I],
    label: &str,
    label_of: impl Fn(I) -> Option<String>,
) -> Result<I, String> {
    ids.iter()
        .copied()
        .find(|id| label_of(*id).as_deref() == Some(label))
        .ok_or_else(|| format!("the house's records name no {label:?}"))
}

/// The head of an identity: enough to follow it through the output, too little to mistake for it.
fn short(id: &str) -> String {
    id.chars().take(8).collect()
}

fn joined(values: impl IntoIterator<Item = impl std::fmt::Display>) -> String {
    let joined: Vec<String> = values.into_iter().map(|value| value.to_string()).collect();

    match joined.is_empty() {
        true => "-".to_owned(),
        false => joined.join(", "),
    }
}

fn heading(title: &str) {
    println!("\n=== {title} ===\n");
}
