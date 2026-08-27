//! Reconciling operations' record with finance's, so that `mine/` ends up holding one record.
//!
//! Two parties took the same state away, learned separately and decided separately. Neither
//! journal extends the other: they share nineteen entries and each holds a twentieth the other
//! never saw. That is why [`ape_cli::converge::converge`] refuses the pair outright — its
//! comparison is over *sequence*, and a divergence is not an extension. The refusal is measured
//! here rather than assumed, because it is the fact that decides the shape of everything after it.
//!
//! # What this does, and the one thing it does not
//!
//! **Knowledge is carried whole.** An `EntryId` is derived from what admitting produced, so
//! finance's entry is the same knowledge in either record and can simply be admitted. The Canon
//! refuses an admission dated before its recording watermark, so the merged journal is operations'
//! twenty (through 2026-01-07) followed by finance's one (2026-01-08) — the order the instants
//! impose, not a preference.
//!
//! **Intention is carried by re-deciding, not by copying.** A `Taken` is a decision *plus* the
//! prefix it stood on, and `lineage::rebuild` demands the two agree exactly. Finance's records
//! claim a prefix without operations' commitment in it; in the merged journal that prefix no longer
//! exists at any position, so finance's two records cannot be replayed as written — in either
//! journal order, symmetrically. What is carried across is the `Decision` verbatim, re-witnessed
//! against the merged prefix. That is the recovery `converge` prescribes to a refused party, and
//! the whole of what experiment 15 says is left to do.
//!
//! **The claim that is dropped.** Finance's record asserted that finance decided against a prefix
//! that did *not* hold operations' commitment. Nothing in the merged record can say that, and this
//! does not try: the retaken witness is a true claim about the merged record's history, and
//! finance's original claim about its own history is not carried. It is stated in the output rather
//! than approximated.
//!
//! **Nothing is arbitrated.** The two forks disagree about one commitment, and a lineage is a tree,
//! so both survive as branches. Synthesis is asked what one intention would come to in the other's
//! world and the answer is printed — as a reading, never decided. Deciding it would be this process
//! inventing an intention neither party held.

use std::collections::BTreeSet;
use std::error::Error;

use ape::canon::Canon;
use ape::engine::synthesis::{ApplicabilityConflict, ApplicabilityStatus, synthesize};
use ape::engine::thesis::Thesis;
use ape::kernel::entities::AgentId;
use ape_cli::converge::converge;
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, EntryId, Replayed};
use ape_cli::lineage::{self, Decision, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::Repository;

/// The two records, overridable so that the guard below can be shown to fail.
///
/// The check that decides whether anything is written is *every world either record claimed comes
/// back identically*. A guard nobody has seen refuse is a hypothesis, and refusing it takes a
/// record whose worlds do not reproduce — which cannot be made by editing `theirs/`, because
/// `theirs/` is not ours to write. So the roots are arguments, and the refusal is demonstrated
/// against copies.
fn roots() -> (String, String) {
    let mut given = std::env::args().skip(1);

    (
        given.next().unwrap_or_else(|| "mine".to_owned()),
        given.next().unwrap_or_else(|| "theirs".to_owned()),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mine, theirs) = roots();

    println!("reconciling {mine}/ (operations) with {theirs}/ (finance)");

    let ours = Repository::open(&mine);
    let finance = Repository::open(&theirs);

    let held = reading::corroborated(&ours)?;
    let handed = reading::corroborated(&finance)?;

    rule("THE TWO RECORDS, AS EACH RECONSTRUCTS ON ITS OWN");
    describe("operations (mine/)", &held);
    describe("finance   (theirs/)", &handed);

    let who = parties(&held.journal, &held.admitted);
    rule("WHO IS WHO");
    for (agent, label) in &who {
        println!("  {}  {label}", brief(&agent.to_string()));
    }

    rule("WHAT THE TWO RECORDS HAVE, AND HAVE NOT, GOT IN COMMON");
    knowledge(&held, &handed);
    intention(&held, &handed, &who);

    rule("PUTTING FINANCE'S RECORD BACK AS IT STANDS: MEASURED, NOT ASSUMED");
    match converge(&ours, &handed) {
        Ok(_) => println!("  it was accepted — which contradicts everything below; stopping."),
        Err(refusal) => {
            println!("  converge(mine, theirs) refuses:");
            println!("    {refusal}");
            println!(
                "\n  Sequence, not membership. Neither journal extends the other, so there is no\n  \
                 order in which one can be appended to the other. The refusal wrote nothing."
            );
        }
    }

    rule("ONE JOURNAL: OPERATIONS' KNOWLEDGE, THEN FINANCE'S");
    let journal = merged_journal(&held, &handed);
    let mut aside = Canon::new(ResidentHistory::new());
    let standing = journal::replay(&mut aside, &journal)?;

    println!("  {} entries, recorded in this order:", journal.len());
    for (position, entry) in journal.iter().enumerate().skip(held.journal.len() - 1) {
        println!(
            "    [{position:>2}] {}  recorded_at {}",
            brief(&standing.entries[position].to_string()),
            entry.recorded_at()
        );
    }
    println!(
        "\n  Operations' 2026-01-07 entry has to land before finance's 2026-01-08 one: a Canon\n  \
         refuses an admission dated before its watermark, so the party that learned earlier goes first."
    );

    rule("WHY FINANCE'S DECISIONS CANNOT BE REPLAYED AS WRITTEN");
    unreplayable(&handed, &standing);

    rule("AND THE OTHER ORDER IS NOT A WAY OUT");
    reversed(&held, &handed, &standing);

    rule("RE-DECIDING FINANCE'S INTENTIONS AGAINST THE MERGED PREFIX");
    let retaken = retake(&held, &handed, &standing)?;
    println!(
        "  {} of finance's {} decisions are not already operations': retaking those.\n",
        retaken.len(),
        handed.decisions.len()
    );

    let mut decisions = held.decisions.clone();
    decisions.extend(retaken);

    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, admitted) = lineage::rebuild(&mut canon, &journal, &decisions)?;

    let recorded = ours.read_worlds()?;
    let theirs_worlds = finance.read_worlds()?;

    println!("  Every world either record claimed, weighed against what the merge produces:\n");
    let produced: Vec<String> = lineage
        .decided()
        .iter()
        .map(|thesis| thesis.id().to_string())
        .collect();

    let mut lost = Vec::new();
    for (owner, claimed) in [("operations", &recorded), ("finance", &theirs_worlds)] {
        for world in claimed {
            println!(
                "    {owner:<10} {}  {}",
                brief(&world.thesis),
                match produced.contains(&world.thesis) {
                    true => "produced again, identically",
                    false => "NOT produced by the merge",
                }
            );
        }

        lost.extend(unreproduced(&produced, claimed));
    }

    if !lost.is_empty() {
        println!(
            "\n  {} world(s) the merge does not reproduce. Writing a record that claims them\n  \
             would be writing a claim nothing here can produce. Stopping without writing.",
            lost.len()
        );
        return Ok(());
    }

    println!("\n  Nothing was lost by re-deciding: in this arrangement the extra entry is a");
    println!("  Commitment, so it moves no Event head, and an advance absorbs only what its cut");
    println!("  froze. Both of finance's worlds come back with the identities finance recorded.");

    rule("WRITING THE MERGED RECORD INTO mine/");
    let merged = Corroborated {
        canon,
        lineage,
        admitted,
        journal,
        decisions,
    };
    converge(&ours, &merged)?;
    println!("  converge(mine, merged) accepted and wrote the whole record.");

    rule("mine/, RE-READ FROM DISK BY A READER THAT WAS TOLD NOTHING");
    let now = reading::corroborated(&ours)?;
    describe("operations (mine/)", &now);
    for (taken, world) in now.decisions.iter().zip(now.lineage.decided()) {
        println!(
            "    {}  parent {:<14} by {}",
            brief(&world.id().to_string()),
            world
                .parent()
                .map(|id| brief(&id.to_string()).to_owned())
                .unwrap_or_else(|| "-".to_owned()),
            taken
                .by
                .and_then(|agent| named(&who, agent))
                .unwrap_or("(nobody)")
        );
    }

    rule("WHAT THE TWO INTENTIONS DO TO EACH OTHER — READ, NOT DECIDED");
    applicability(&now, &recorded, &theirs_worlds)?;

    Ok(())
}

/// The one journal both records' knowledge fits into: operations' entries, then finance's own.
///
/// Membership decides what is missing and the recording instants decide where it goes. Finance's
/// entry is appended rather than merged in place because it is the later-recorded one; the opposite
/// order is not a choice this makes but one the Canon's watermark forbids.
fn merged_journal(held: &Corroborated, handed: &Corroborated) -> Vec<Admission> {
    let ours: BTreeSet<&EntryId> = held.admitted.entries.iter().collect();

    let mut journal = held.journal.clone();

    for (entry, admission) in handed.admitted.entries.iter().zip(&handed.journal) {
        if !ours.contains(entry) {
            journal.push(admission.clone());
        }
    }

    journal
}

/// Finance's decisions, taken again against the prefix that now stands.
///
/// The `Decision` is carried verbatim — it is finance's intention and nothing about the merge
/// touches it. What is rebuilt is the coordinate and the witness, which are claims about a journal
/// that finance never held. Attributed to finance, because finance is the party whose decision it
/// is, and the record has no field for the party that re-applied it.
///
/// A decision operations already holds is not retaken, and the comparison is over the `Decision`
/// rather than the whole `Taken`: what is being carried is the intention, and two records of one
/// intention taken at different coordinates would produce one world twice.
fn retake(
    held: &Corroborated,
    handed: &Corroborated,
    standing: &Replayed,
) -> Result<Vec<Taken>, Box<dyn Error>> {
    handed
        .decisions
        .iter()
        .filter(|taken| {
            !held
                .decisions
                .iter()
                .any(|ours| ours.decision == taken.decision)
        })
        .map(|taken| match taken.by {
            Some(by) => Taken::claimed(taken.decision.clone(), by, standing),
            None => Taken::now(taken.decision.clone(), standing),
        })
        .map(|taken| taken.map_err(Into::into))
        .collect()
}

/// Name the entries that make finance's own records unreplayable here, one decision at a time.
///
/// The comparison is [`lineage`]'s: a witness must equal the prefix exactly, in both directions.
/// Printing which entry each record fails on is the difference between *the merge refused* and
/// *this record claims a prefix that no position in the merged journal offers*.
fn unreplayable(handed: &Corroborated, standing: &Replayed) {
    for taken in &handed.decisions {
        let Some(position) = standing.entries.iter().position(|held| held == &taken.after) else {
            continue;
        };

        let prefix: BTreeSet<&EntryId> = standing.entries[..=position].iter().collect();
        let witnessed: BTreeSet<&EntryId> = taken.witness.iter().collect();

        let unexpected: Vec<&&EntryId> = prefix.difference(&witnessed).collect();
        let missing: Vec<&&EntryId> = witnessed.difference(&prefix).collect();

        print!(
            "  {} after {} ",
            kind(&taken.decision),
            brief(&taken.after.to_string())
        );

        match (unexpected.as_slice(), missing.as_slice()) {
            ([], []) => println!("replays as written."),
            (extra, absent) => {
                println!("cannot replay as written:");
                for entry in extra {
                    println!(
                        "    the merged prefix holds {}, which its witness does not name",
                        brief(&entry.to_string())
                    );
                }
                for entry in absent {
                    println!(
                        "    its witness names {}, which the merged prefix has not reached",
                        brief(&entry.to_string())
                    );
                }
            }
        }
    }

}

/// Show that choosing the other journal order moves the problem rather than solving it.
///
/// Two measurements, not one assertion. The set comparison says operations' records would fail the
/// way finance's do; the replay says the Canon will not accept that journal at all. Only the first
/// is about witnesses, and only the second is the reason the choice was never open.
fn reversed(held: &Corroborated, handed: &Corroborated, standing: &Replayed) {
    println!("  If finance's entry went first, operations' own records would fail the same way:\n");

    let whole: BTreeSet<&EntryId> = standing.entries.iter().collect();

    let ours_last = held.admitted.entries.last();

    for taken in &held.decisions {
        if ours_last == Some(&taken.after) {
            let witnessed: BTreeSet<&EntryId> = taken.witness.iter().collect();

            for entry in whole.difference(&witnessed) {
                println!(
                    "    operations' {} would find {} in its prefix, unnamed by its witness",
                    kind(&taken.decision),
                    brief(&entry.to_string())
                );
            }
        }
    }

    println!(
        "\n  An address is derived from content, so the same twenty-one entries are the same\n  \
         twenty-one addresses in any order — whichever entry is last, the decisions taken after\n  \
         it stand on a prefix holding the other party's, which their witnesses do not name."
    );

    let mut journal = held.journal[..held.journal.len() - 1].to_vec();
    journal.push(handed.journal[handed.journal.len() - 1].clone());
    journal.push(held.journal[held.journal.len() - 1].clone());

    let mut aside = Canon::new(ResidentHistory::new());

    match journal::replay(&mut aside, &journal) {
        Ok(_) => println!("\n  And the Canon accepts that journal, so the order was a choice."),
        Err(refusal) => println!(
            "\n  And the order was never a choice in the first place — the Canon refuses that\n  \
             journal outright:\n    {refusal}\n\n  \
             Recording is monotonic. The party that learned earlier lands first, so the party\n  \
             that retakes is finance, by arithmetic rather than by preference."
        ),
    }
}

/// What the two records know in common, and what each knows alone.
fn knowledge(held: &Corroborated, handed: &Corroborated) {
    let (ours, theirs) = (&held.admitted.entries, &handed.admitted.entries);

    let common = ours
        .iter()
        .zip(theirs)
        .take_while(|(one, other)| one == other)
        .count();

    let theirs_set: BTreeSet<&EntryId> = theirs.iter().collect();
    let ours_set: BTreeSet<&EntryId> = ours.iter().collect();

    println!("  knowledge");
    println!("    entries: operations {}, finance {}", ours.len(), theirs.len());
    println!("    identical prefix: {common} entries");
    println!(
        "    held in common at all: {} entries",
        ours.iter().filter(|entry| theirs_set.contains(*entry)).count()
    );

    for (owner, only) in [
        ("operations", ours_set.difference(&theirs_set).collect::<Vec<_>>()),
        ("finance", theirs_set.difference(&ours_set).collect::<Vec<_>>()),
    ] {
        for entry in only {
            println!("    only {owner:<10} knows {}", brief(&entry.to_string()));
        }
    }
}

/// What the two records intend in common, and what each intends alone.
fn intention(held: &Corroborated, handed: &Corroborated, who: &[(AgentId, String)]) {
    let ours: Vec<String> = held.lineage.decided().iter().map(identity).collect();
    let theirs: Vec<String> = handed.lineage.decided().iter().map(identity).collect();

    println!("\n  intention");
    println!("    worlds: operations {}, finance {}", ours.len(), theirs.len());

    for world in ours.iter().filter(|world| theirs.contains(world)) {
        println!("    both decided       {}", brief(world));
    }

    for (owner, side, other) in [
        ("operations", &ours, &theirs),
        ("finance", &theirs, &ours),
    ] {
        for world in side.iter().filter(|world| !other.contains(world)) {
            println!("    only {owner:<10} decided {}", brief(world));
        }
    }

    for (owner, side) in [("operations", held), ("finance", handed)] {
        let deciders: BTreeSet<&str> = side
            .decisions
            .iter()
            .filter_map(|taken| taken.by)
            .filter_map(|agent| named(who, agent))
            .collect();

        println!(
            "    {owner}'s decisions claim: {}",
            if deciders.is_empty() {
                "(nobody)".to_owned()
            } else {
                deciders.into_iter().collect::<Vec<_>>().join(", ")
            }
        );
    }
}

/// Ask Synthesis what finance's intention would come to inside operations' world, and print it.
///
/// Read-only and decided by nobody. It exists to say what the two branches do to each other in
/// terms the record itself produces, so that leaving them as two branches is a result rather than
/// an omission.
fn applicability(
    now: &Corroborated,
    ours: &[WorldRecord],
    theirs: &[WorldRecord],
) -> Result<(), Box<dyn Error>> {
    let (Some(base), Some(target), Some(source)) = (
        ours.first().map(|world| world.thesis.clone()),
        ours.last().map(|world| world.thesis.clone()),
        theirs.last().map(|world| world.thesis.clone()),
    ) else {
        println!("  one of the three worlds is missing; asking nothing.");
        return Ok(());
    };

    let find = |wanted: &str| {
        now.lineage
            .decided()
            .iter()
            .find(|thesis| identity(thesis) == wanted)
            .map(Thesis::id)
    };

    let (Some(base), Some(source), Some(target)) = (find(&base), find(&source), find(&target))
    else {
        println!("  the merged record does not hold all three worlds; asking nothing.");
        return Ok(());
    };

    println!("  base   {}  (the world both parties left from)", brief(&base.to_string()));
    println!("  source {}  (finance's tip)", brief(&source.to_string()));
    println!("  target {}  (operations' tip)", brief(&target.to_string()));

    let report = synthesize(now.lineage.archive(), now.canon.history(), base, source, target)?;

    println!("\n  what finance decided, relative to the base:");
    for omitted in report.difference().omitted() {
        println!("    omitted    {}", brief(&omitted.to_string()));
    }
    for introduced in report.difference().introduced() {
        println!("    introduced {}", brief(&introduced.to_string()));
    }

    match report.status() {
        ApplicabilityStatus::Applicable { transfer, .. } => {
            println!("\n  applicable to operations' world. What it would still take:");
            for remove in transfer.remove() {
                println!("    remove    {}", brief(&remove.to_string()));
            }
            for introduce in transfer.introduce() {
                println!("    introduce {}", brief(&introduce.to_string()));
            }
        }
        ApplicabilityStatus::AlreadyApplied => {
            println!("\n  already applied: operations' world satisfies it.");
        }
        ApplicabilityStatus::Conflicted { conflicts, .. } => {
            println!("\n  conflicted — it would break {} rule(s):", conflicts.len());
            for conflict in conflicts {
                println!("    {}", broken(conflict));
            }
        }
    }

    println!(
        "\n  Not decided, and the refusal says why in a way worth reading. Operations' tip\n  \
         recognizes history only up to the instant operations last advanced to; finance's\n  \
         commitment was recorded after that. So the two intentions do not merely disagree —\n  \
         one of them names knowledge the other's world has not yet recognized.\n\n  \
         Collapsing them would take a decision neither party made: operations advancing its own\n  \
         tip to finance's instant, and then answering what becomes of the commitment operations\n  \
         omitted and finance kept. A lineage is a tree, both intentions stand as branches, and\n  \
         inventing the world that reconciles them is not this process's to do."
    );

    Ok(())
}

/// A conflict in the terms the record uses, rather than in the terms a debug format uses.
fn broken(conflict: &ApplicabilityConflict) -> String {
    match conflict {
        ApplicabilityConflict::HistoricalFreezing { commitment } => format!(
            "{} is frozen in the target: history already settled it",
            brief(&commitment.to_string())
        ),
        ApplicabilityConflict::HistoricalUnavailability {
            commitment,
            recorded_at,
            known_at,
        } => format!(
            "{} was recorded {}, and the target recognizes history only to {}",
            brief(&commitment.to_string()),
            recorded_at.to_iso(),
            known_at.to_iso()
        ),
        ApplicabilityConflict::DependencyBreakage {
            dependent,
            missing_dependency,
        } => format!(
            "{} would be left depending on {}, which the result drops",
            brief(&dependent.to_string()),
            brief(&missing_dependency.to_string())
        ),
        ApplicabilityConflict::MissingDependency {
            commitment,
            dependency,
        } => format!(
            "{} depends on {}, which the result does not hold",
            brief(&commitment.to_string()),
            brief(&dependency.to_string())
        ),
    }
}

/// Every world a record claimed that the merge does not produce again.
///
/// This is what decides whether anything is written. Separated from the reporting around it so that
/// it can be refused without a repository: the arrangement here happens to reproduce all six worlds,
/// and a guard whose refusing branch has never run is a hypothesis about the code rather than a
/// check on it.
fn unreproduced(produced: &[String], claimed: &[WorldRecord]) -> Vec<String> {
    claimed
        .iter()
        .map(|world| world.thesis.clone())
        .filter(|thesis| !produced.contains(thesis))
        .collect()
}

/// Which of the three shapes a decision is, for output that has to distinguish two of one kind.
fn kind(decision: &Decision) -> &'static str {
    match decision {
        Decision::Genesis { .. } => "genesis",
        Decision::Advance { .. } => "advance",
        Decision::Fork { .. } => "fork   ",
    }
}

/// Pair every agent identity the journal produced with the label the journal gave it.
///
/// The two sequences are the same admissions counted differently — replay hands back identities in
/// admission order and carries no names — so walking them together is the only place a label and an
/// identity meet.
fn parties(journal: &[Admission], admitted: &Replayed) -> Vec<(AgentId, String)> {
    journal
        .iter()
        .filter_map(|entry| match entry {
            Admission::Agent { label, .. } => Some(label.clone()),
            _ => None,
        })
        .enumerate()
        .filter_map(|(position, label)| admitted.agents.get(position).map(|id| (*id, label)))
        .collect()
}

fn named(who: &[(AgentId, String)], agent: AgentId) -> Option<&str> {
    who.iter()
        .find(|(id, _)| *id == agent)
        .map(|(_, label)| label.as_str())
}

fn describe(owner: &str, record: &Corroborated) {
    println!(
        "  {owner}: {} entries, {} decisions, {} worlds",
        record.journal.len(),
        record.decisions.len(),
        record.lineage.decided().len()
    );
}

fn identity(thesis: &Thesis) -> String {
    thesis.id().to_string()
}

fn brief(id: &str) -> &str {
    &id[..id.len().min(12)]
}

fn rule(title: &str) {
    println!("\n\n=== {title} ===\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claimed(thesis: &str) -> WorldRecord {
        WorldRecord {
            thesis: thesis.to_owned(),
            thesis_parent: None,
            known_at: "2026-01-06".to_owned(),
            event_head: None,
            frozen: BTreeSet::new(),
            open: BTreeSet::new(),
        }
    }

    /// The guard that decides whether anything is written, at the value that makes it refuse.
    ///
    /// The arrangement in this directory reproduces every world, so this branch never runs there.
    /// It is asserted here because a check whose refusal has never been seen is a hypothesis, and
    /// what it must do is *name* the world that did not come back — a count would send a reader
    /// back to the two files to find out which.
    #[test]
    fn a_world_the_merge_does_not_produce_is_named_rather_than_counted() {
        let produced = vec!["aa".to_owned(), "bb".to_owned()];

        assert_eq!(
            unreproduced(&produced, &[claimed("aa"), claimed("cc")]),
            vec!["cc".to_owned()],
            "the world that did not come back is the one reported"
        );
    }

    /// And it stays silent where every claim is met, which is the case that must not block a write.
    #[test]
    fn a_record_whose_worlds_all_come_back_reports_nothing() {
        let produced = vec!["aa".to_owned(), "bb".to_owned()];

        assert!(unreproduced(&produced, &[claimed("aa"), claimed("bb")]).is_empty());
    }
}
