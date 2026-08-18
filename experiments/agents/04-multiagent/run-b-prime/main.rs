//! Finance takes on the storage the market invoiced the house for: 30, payable by the 14th.
//!
//! # What this does, and in what order
//!
//! 1. Opens `repo/` and rebuilds it — the journal and the lineage together, which is the only
//!    reading that resolves each standing decision against the knowledge it was taken against.
//! 2. Resolves the vocabulary the house already admitted (who is who, which statement says
//!    "a spender pays a counterparty", which instance the cash moves out of). Nothing is
//!    invented: every identity used below is one the journal produced.
//! 3. **Records the intention** as one more admission to the journal — a Commitment for 30 of
//!    cash out of `account`, due 2026-01-14, with the house accountable and finance executing.
//! 4. **Takes the decision as finance** — a `Fork` of the world the house was reasoning about,
//!    introducing that commitment under the cut already recognized, and claimed by finance.
//! 5. Puts both back through `converge`, which appends knowledge, merges intention, and refuses
//!    to write anything that does not rebuild.
//! 6. Reads the repository again from disk, having been given nothing the first process computed.
//!
//! # Why a fork and not an advance
//!
//! `advance` moves the cut and deliberately adopts nothing: a commitment admitted between two
//! cuts and left unsettled does not enter a selection. So an intention only reaches a world
//! through `fork`. The commitment is therefore recorded at the instant the tip already
//! recognizes (2026-01-06), which is what makes it selectable under the parent's cut and keeps
//! this one decision rather than an advance-then-fork pair.
//!
//! # Why the house is accountable and finance executes
//!
//! The invoice was addressed to the house, and finance is the party taking the paying on.
//! `Assignment` separates the two, and that separation is the closest the kernel comes to
//! saying "finance acts for the house" — see `ANSWER.md`, because it is *not* the same thing.

use std::collections::BTreeSet;
use std::error::Error;

use ape::kernel::entities::{AgentId, ResourceInstanceId, RoleId, StatementId};
use ape::kernel::value_objects::Date;

use ape_cli::converge;
use ape_cli::journal::{self, Admission, Replayed};
use ape_cli::lineage::{self, Decision, Taken};
use ape_cli::reading::{self, WorldRecord};
use ape_cli::repository::Repository;

const REPOSITORY: &str = "repo";

/// The latest instant anything in the repository stands at, and so the instant finance acts at.
///
/// Nothing in the brief names a day. The house's tip recognizes 2026-01-06 and the recording
/// watermark has reached 2026-01-05, so this is the earliest instant that is both admissible and
/// already recognized by the world being extended.
const RECORDED_AT: &str = "2026-01-06";
const DUE_DATE: &str = "2026-01-14";
const STORAGE: f64 = 30.0;

fn main() -> Result<(), Box<dyn Error>> {
    let repository = Repository::open(REPOSITORY);

    let mut held = reading::corroborated(&repository)?;
    let vocabulary = resolve(&held.journal, &held.admitted)?;

    let entries_before = held.admitted.entries.len();
    let worlds_before = held.lineage.decided().len();

    let tip = held
        .lineage
        .decided()
        .last()
        .ok_or("the repository holds no world to extend")?
        .clone();

    println!("== what the house already knows ==\n");
    println!("{} journal entries, {worlds_before} world(s) decided", entries_before);
    println!("agents:");
    for (label, id) in &vocabulary.agents {
        println!("  {label:<12} {}", short(&id.to_string()));
    }
    println!("roles:");
    for (label, id) in &vocabulary.roles {
        println!("  {label:<12} {}", short(&id.to_string()));
    }
    println!(
        "the statement finance acts under: actors {{spender}} -> recipients {{counterparty}}, \
         action \"spend\" (a decrease of cash)\n  {}",
        vocabulary.owed
    );
    println!("the instance it moves: account\n  {}", vocabulary.account);
    println!("\nthe world being extended:");
    print_world(&WorldRecord::of(&tip));

    println!("\n== 1. what finance records ==\n");

    let intention = Admission::Commitment {
        accountable: vocabulary.house,
        executors: BTreeSet::from([vocabulary.finance]),
        beneficiaries: BTreeSet::from([vocabulary.market]),
        statement: vocabulary.owed,
        resource: vocabulary.account,
        committed_at: RECORDED_AT.to_owned(),
        due_date: DUE_DATE.to_owned(),
        magnitude: Some(STORAGE),
        dependencies: BTreeSet::new(),
        recorded_at: RECORDED_AT.to_owned(),
    };

    held.journal.push(intention);
    journal::replay_remaining(&mut held.canon, &held.journal, &mut held.admitted)?;

    let storage = *held
        .admitted
        .commitments
        .last()
        .ok_or("the commitment was not admitted")?;
    let entry = held
        .admitted
        .entries
        .last()
        .ok_or("the admission produced no entry")?
        .clone();

    println!(
        "commitment {storage}\n  the house is accountable, finance executes, the market \
         benefits;\n  {STORAGE} of cash leaves `account`; committed {RECORDED_AT}, due {DUE_DATE}."
    );
    println!("journal entry {entry}");
    println!(
        "the journal grew from {entries_before} to {} entries",
        held.admitted.entries.len()
    );

    println!("\n== 2. what finance decides ==\n");

    let decision = Decision::Fork {
        extends: tip.id(),
        omitted: BTreeSet::new(),
        introduced: BTreeSet::from([storage]),
    };

    let taken = Taken::claimed(decision.clone(), vocabulary.finance, &held.admitted)?;
    let imposed = lineage::decide(held.canon.history(), &mut held.lineage, &decision)?;
    held.decisions.push(taken.clone());

    let decided = held
        .lineage
        .decided()
        .last()
        .ok_or("the fork produced no world")?
        .clone();

    println!(
        "fork of {}\n  introducing {storage}, omitting nothing, under the cut already recognized",
        tip.id()
    );
    println!("taken after entry {}, with {} entries standing", taken.after, taken.witness.len());
    println!(
        "claimed by {} ({})",
        taken
            .by
            .map(|id| id.to_string())
            .unwrap_or_else(|| "nobody".to_owned()),
        label_of(&vocabulary.agents, taken.by)
    );
    println!("history imposed {} commitment(s) — a fork imposes none", imposed.len());
    println!("\nthe world finance decided:");
    print_world(&WorldRecord::of(&decided));

    println!("\n== 3. put back, so that repo/ holds it ==\n");

    let put_back = converge::converge(&repository, &held)?;

    println!(
        "journal: {entries_before} -> {} entries",
        put_back.journal.len()
    );
    println!("lineage: {worlds_before} -> {} decisions", put_back.decisions.len());
    println!("worlds:  {worlds_before} -> {} recorded", put_back.lineage.decided().len());
    println!(
        "the repository holds the world finance decided: {}",
        converge::holds(&repository, decided.id())?
    );
    println!("worlds the repository says finance decided:");
    for world in reading::decided_by(&repository, vocabulary.finance)? {
        println!("  {world}");
    }

    println!("\n== 4. read back from disk, given nothing else ==\n");

    let effective_at = Date::parse(DUE_DATE)?;

    for reading in reading::reconstruct(&repository, vocabulary.account, &effective_at)? {
        println!("world {}", reading.thesis);
        println!("  parent      {:?}", reading.thesis_parent);
        println!("  known_at    {}", reading.known_at);
        println!("  event_head  {:?}", reading.event_head);
        println!("  read at     {}", reading.effective_at);
        println!("  frozen      {:?}", reading.frozen);
        println!("  open        {:?}", reading.open);
        println!("  level       {}", reading.level);
        println!("  conflicts   {:?}", reading.conflicts);
        println!("  conditions:");
        for (commitment, condition) in &reading.conditions {
            println!(
                "    {commitment}\n      {:?}, {:?}, pending deps {}, unfulfillable deps {}",
                condition.outcome,
                condition.timeliness,
                condition.pending_dependencies,
                condition.unfulfillable_dependencies
            );
        }
    }

    Ok(())
}

/// The identities the house's journal produced, under the labels it admitted them by.
///
/// Replay hands identities back in admission order and carries no labels, so the nth admission
/// of a family is the nth identity of that family. That correspondence is the whole of how a
/// name in a brief becomes an identity here, and it is checked rather than assumed: a family
/// whose label count and identity count disagree is refused.
struct Vocabulary {
    agents: Vec<(String, AgentId)>,
    roles: Vec<(String, RoleId)>,
    house: AgentId,
    market: AgentId,
    finance: AgentId,
    /// The statement whose actors are spenders and whose action decreases cash.
    owed: StatementId,
    account: ResourceInstanceId,
}

fn resolve(journal: &[Admission], admitted: &Replayed) -> Result<Vocabulary, String> {
    let mut agent_labels = Vec::new();
    let mut role_labels = Vec::new();
    let mut action_verbs = Vec::new();
    let mut instance_labels = Vec::new();
    let mut statement_actions = Vec::new();
    let mut statement_actors = Vec::new();

    for record in journal {
        match record {
            Admission::Agent { label, .. } => agent_labels.push(label.clone()),
            Admission::Role { label, .. } => role_labels.push(label.clone()),
            Admission::Action { verb, .. } => action_verbs.push(verb.clone()),
            Admission::ResourceInstance { label, .. } => instance_labels.push(label.clone()),
            Admission::Statement { action, actors, .. } => {
                statement_actions.push(*action);
                statement_actors.push(actors.clone());
            }
            _ => {}
        }
    }

    let agents = paired(agent_labels, &admitted.agents, "agent")?;
    let roles = paired(role_labels, &admitted.roles, "role")?;
    let actions = paired(action_verbs, &admitted.actions, "action")?;
    let instances = paired(instance_labels, &admitted.instances, "resource instance")?;

    let spender = by_label(&roles, "spender", "role")?;
    let spend = by_label(&actions, "spend", "action")?;

    if statement_actions.len() != admitted.statements.len() {
        return Err("the journal's statements and the replay's disagree in number".to_owned());
    }

    let at = statement_actions
        .iter()
        .position(|action| *action == spend)
        .ok_or("no statement is stated in terms of the `spend` action")?;

    if statement_actors[at] != BTreeSet::from([spender]) {
        return Err("the `spend` statement's actors are not exactly {spender}".to_owned());
    }

    Ok(Vocabulary {
        house: by_label(&agents, "house", "agent")?,
        market: by_label(&agents, "market", "agent")?,
        finance: by_label(&agents, "finance", "agent")?,
        owed: admitted.statements[at],
        account: by_label(&instances, "account", "resource instance")?,
        agents,
        roles,
    })
}

/// Labels beside the identities admitting them produced, refusing two lists of unequal length.
///
/// The refusal is the point: a `zip` over lists that have drifted apart truncates in silence,
/// and every name in this program would then resolve to the wrong identity while still
/// resolving to *an* identity.
fn paired<Id: Copy>(
    labels: Vec<String>,
    ids: &[Id],
    family: &str,
) -> Result<Vec<(String, Id)>, String> {
    if labels.len() != ids.len() {
        return Err(format!(
            "the journal admits {} {family}(s) and the replay produced {}",
            labels.len(),
            ids.len()
        ));
    }

    Ok(labels.into_iter().zip(ids.iter().copied()).collect())
}

fn by_label<Id: Copy>(pairs: &[(String, Id)], label: &str, family: &str) -> Result<Id, String> {
    pairs
        .iter()
        .find(|(name, _)| name == label)
        .map(|(_, id)| *id)
        .ok_or_else(|| format!("the journal admits no {family} labelled {label:?}"))
}

fn label_of(agents: &[(String, AgentId)], id: Option<AgentId>) -> String {
    id.and_then(|id| {
        agents
            .iter()
            .find(|(_, known)| *known == id)
            .map(|(label, _)| label.clone())
    })
    .unwrap_or_else(|| "unclaimed".to_owned())
}

fn print_world(record: &WorldRecord) {
    println!("  thesis      {}", record.thesis);
    println!("  parent      {:?}", record.thesis_parent);
    println!("  known_at    {}", record.known_at);
    println!("  event_head  {:?}", record.event_head);
    println!("  frozen      {:?}", record.frozen);
    println!("  open        {:?}", record.open);
}

fn short(id: &str) -> String {
    id.chars().take(12).collect()
}
