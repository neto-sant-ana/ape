//! The corroboration experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the earlier experiments give: a
//! comparison has to fail loudly. What is different here is what is under test — not a world,
//! and not a lineage, but the repository's ability to notice that it is wrong.
//!
//! So this harness does something none of the others do: it edits a persisted repository. The
//! phases that follow Phase 2 are claims about which of those edits stop being accepted.

use ape::canon::CanonicalHistory;
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::journal::{self, Admission, EntryId};
use ape_cli::lineage::{Decision, Taken};
use ape_cli::reading::{self, ConflictRecord, OutcomeRecord, Reading, WorldRecord};
use ape_cli::repository::Repository;
use ape_frontier::subject::divergence::{self, Reasoned};

/// The instant every world is interpreted at, as the divergence experiment fixed it.
const EFFECTIVE: &str = "2026-01-25";

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-corroboration-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// Run the divergence arrangement and leave it on disk.
///
/// The subject is that experiment's, unmodified and not re-described here. A corruption
/// measured against a repository this laboratory has not already declared correct would
/// measure nothing.
fn persisted(repository: &Repository) -> Reasoned {
    let reasoned = divergence::reasoned().expect("the arrangement holds");

    repository
        .write_journal(&reasoned.journal)
        .expect("the repository is writable");
    repository
        .write_lineage(&reasoned.decisions)
        .expect("the repository is writable");
    repository
        .write_worlds(&worlds(reasoned.lineage.decided()))
        .expect("the repository is writable");

    reasoned
}

/// Recompute every witness from the coordinate its decision names.
///
/// This is what a writer that never observed a prefix would produce, and therefore what
/// *removing* the witness amounts to: the field stays in the file and stops carrying anything
/// a reader could not work out for itself.
fn witness_recomputed(journal: &[Admission], decisions: &mut [Taken]) {
    let mut canon = ape::canon::Canon::new(ResidentHistory::new());
    let mut admitted = journal::Replayed::default();

    for taken in decisions.iter_mut() {
        journal::replay_through(&mut canon, journal, &mut admitted, &taken.after)
            .expect("the coordinate addresses the journal");

        taken.witness = admitted.entries.iter().cloned().collect();
    }
}

/// Rewrite each decision's reference so that it names the world its predecessor now produces.
///
/// A forger who moves a coordinate moves every world downstream of it, and `extends` names a
/// world by an identity derived from its content — so a forgery that leaves the original
/// references behind is not a forgery at all, it is a repository that refuses itself.
///
/// Doing this is what the convergence experiment added to the cost of a consistent forgery, and
/// doing it *here* is what keeps Phase 7 measuring the thing it set out to measure. It walks
/// the decisions one at a time, because the world a decision extends exists only once every
/// decision before it has been applied — and that this lineage is a chain is what makes "the
/// world before it" the right answer.
fn extends_recomputed(journal: &[Admission], decisions: &mut [Taken]) {
    for position in 1..decisions.len() {
        let mut canon = ape::canon::Canon::new(ResidentHistory::new());

        let (built, _) = ape_cli::lineage::rebuild(&mut canon, journal, &decisions[..position])
            .expect("the decisions before this one still apply");

        let parent = built.decided()[position - 1].id();

        match &mut decisions[position].decision {
            Decision::Advance { extends, .. } | Decision::Fork { extends, .. } => *extends = parent,
            Decision::Genesis { .. } => {}
        }
    }
}

/// What the decisions produced, as the repository records it.
fn worlds(lineage: &[ape::engine::thesis::Thesis]) -> Vec<WorldRecord> {
    lineage.iter().map(WorldRecord::of).collect()
}

/// Rebuild in an operating-system process of its own, given the repository and nothing else.
fn rebuild(
    repository: &Repository,
    instance: ape::kernel::entities::ResourceInstanceId,
) -> Rebuilt {
    let output = std::process::Command::new(ape_frontier::binary())
        .arg(repository.root())
        .arg(instance.to_string())
        .arg(EFFECTIVE)
        .output()
        .expect("the binary runs");

    Rebuilt {
        refused: !output.status.success(),
        complaint: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        lineage: serde_json::from_slice(&output.stdout).ok(),
    }
}

/// What a fresh process answered with.
///
/// A refusal and a lineage are kept apart rather than collapsed into a `Result`, because the
/// interesting outcome of this experiment is a third thing: a process that neither refused nor
/// returned what it should have.
struct Rebuilt {
    refused: bool,
    complaint: String,
    lineage: Option<Vec<Reading>>,
}

impl Rebuilt {
    fn worlds(&self) -> &[Reading] {
        self.lineage
            .as_deref()
            .unwrap_or_else(|| panic!("no lineage came back: {}", self.complaint))
    }
}

/// Phase 1 — Construct.
///
/// The baseline, and it is measured rather than inherited. Every later phase is a claim about
/// a difference from what is recorded here, so a phase that began from an assumed baseline
/// would be comparing against something nobody checked.
#[test]
fn phase_1_construct() {
    let repository = Repository::open(scratch("phase-1"));
    let reasoned = persisted(&repository);

    let rebuilt = rebuild(&repository, reasoned.subject.instance);

    assert!(
        !rebuilt.refused,
        "an intact repository is read: {}",
        rebuilt.complaint
    );

    let worlds = rebuilt.worlds();

    assert_eq!(worlds.len(), 3, "the genesis, the advancement and the fork");

    // The lineage comes back as a lineage, not as three unrelated worlds.
    assert_eq!(worlds[0].thesis_parent, None);
    assert_eq!(
        worlds[1].thesis_parent.as_deref(),
        Some(worlds[0].thesis.as_str())
    );
    assert_eq!(
        worlds[2].thesis_parent.as_deref(),
        Some(worlds[1].thesis.as_str())
    );

    // The genesis is the world whose reproduction the previous experiment had to repair: its
    // instant addresses a head in the journal it was rebuilt from, and its cut must not see
    // it. Recorded here as a value rather than as a property, because a corruption that
    // changes it is what the next phase is looking for.
    assert_eq!(worlds[0].known_at, "2026-01-10");
    assert_eq!(
        worlds[0].event_head, None,
        "the genesis recognizes no Event"
    );
    assert!(
        worlds[0].frozen.is_empty(),
        "nothing had settled when it was decided"
    );
    assert_eq!(
        worlds[0].conflicts,
        [ConflictRecord::OutOfBounds {
            instance: reasoned.subject.instance.to_string(),
            level: -70.0,
        }],
        "and its own bounds refuse it"
    );

    // The advancement recognizes the cancellation; the fork inherits that cut and spends what
    // the account can carry. Neither is refused.
    for world in &worlds[1..] {
        assert_eq!(world.event_head, worlds[1].canonical_head);
        assert!(world.conflicts.is_empty());
    }

    assert_eq!(
        worlds[2].conditions[&reasoned.subject.overspend.to_string()].outcome,
        OutcomeRecord::Cancelled
    );
    assert_eq!(worlds[2].level, 0.0, "nothing was ever fulfilled");
    assert_eq!(worlds[2].effective_at, EFFECTIVE);

    // And the derivation is live: reading the same worlds from the process that built them
    // agrees with the process that never met it. This is the divergence experiment's result,
    // re-established rather than assumed.
    let living = reading::all(
        reasoned.canon.history(),
        reasoned.lineage.decided(),
        reasoned.subject.instance,
        &Date::parse(EFFECTIVE).expect("the effective instant is a date"),
    )
    .expect("the living lineage reads");

    assert_eq!(
        living, worlds,
        "the baseline holds across the process boundary"
    );
}

/// What a fresh process did with a repository that had been edited.
#[derive(Debug, PartialEq)]
enum Verdict {
    /// It refused, and named something.
    Refused,
    /// A lineage came back, and it is the one an intact repository gives.
    Harmless,
    /// A lineage came back, and it is not the one that was reasoned about.
    Silent,
}

/// Persist the arrangement with one edit applied, and read it back in a fresh process.
///
/// The edit is expressed as a value rather than as surgery on the file, and written through
/// the repository's own writer. That is the realistic shape of this failure: not someone with
/// a text editor, but a writer that recorded something other than what happened.
fn corrupted(
    name: &str,
    baseline: &[Reading],
    edit: impl Fn(&mut Vec<Admission>, &mut Vec<Taken>, &Reasoned),
) -> (Verdict, Rebuilt) {
    let reasoned = divergence::reasoned().expect("the arrangement holds");
    let (mut journal, mut decisions) = (reasoned.journal.clone(), reasoned.decisions.clone());

    edit(&mut journal, &mut decisions, &reasoned);

    let repository = Repository::open(scratch(name));
    repository.write_journal(&journal).expect("writable");
    repository.write_lineage(&decisions).expect("writable");
    repository
        .write_worlds(&worlds(reasoned.lineage.decided()))
        .expect("writable");

    let rebuilt = rebuild(&repository, reasoned.subject.instance);

    let verdict = match &rebuilt.lineage {
        _ if rebuilt.refused => Verdict::Refused,
        Some(worlds) if worlds == baseline => Verdict::Harmless,
        Some(_) => Verdict::Silent,
        None => Verdict::Refused,
    };

    (verdict, rebuilt)
}

/// Phase 2 — Corrupt.
///
/// Six edits, each well-formed and each false, and what each one produces. Nothing here is
/// repaired: this is the instrument the later phases are measured against, and its value
/// depends on being recorded before anything is done about it.
///
/// The table is asserted whole rather than row by row, so that a phase which moves a row says
/// which row moved.
///
/// Everything refused here is refused by an invariant that exists for another reason — a
/// commitment must be selectable, an agent must be eligible, an address must resolve. Nothing
/// in the repository is looking for corruption; three of these tripped something on the way
/// past. The two that pass in silence are the two that alter what was *decided*.
#[test]
fn phase_2_corrupt() {
    let repository = Repository::open(scratch("phase-2-baseline"));
    let reasoned = persisted(&repository);
    let baseline = rebuild(&repository, reasoned.subject.instance)
        .worlds()
        .to_vec();

    // Repointed at an entry that exists, whose prefix still admits everything the genesis
    // selects. Every check the repository has passes.
    let (coordinate, repointed) = corrupted("repointed", &baseline, |_, decisions, reasoned| {
        let head = reasoned.canon.history().head().expect("a head");
        decisions[0].after = EntryId::of(head);
    });

    // The intention itself, altered. The journal is untouched and every address resolves.
    let (intention, narrowed) = corrupted("narrowed", &baseline, |_, decisions, reasoned| {
        decisions[0].decision = Decision::Genesis {
            known_at: "2026-01-10".into(),
            selection: [reasoned.subject.inflow].into(),
        };
    });

    // The control. Two roles are admitted on the same day and neither refers to the other, so
    // their order carries nothing — an entry's identity comes from its content. A check that
    // refuses this is refusing variation the record legitimately admits.
    let (harmless, _) = corrupted("roles-reordered", &baseline, |journal, _, _| {
        journal.swap(0, 1);
    });

    let (commitments, _) = corrupted("commitments-reordered", &baseline, |journal, _, _| {
        let (first, second) = (position(journal, 50.0), position(journal, 120.0));
        journal.swap(first, second);
    });

    let (eligibility, _) = corrupted("eligibility-removed", &baseline, |journal, _, _| {
        let at = journal
            .iter()
            .position(|entry| matches!(entry, Admission::Eligibility { .. }))
            .expect("the subject grants eligibility");
        journal.remove(at);
    });

    let (event, _) = corrupted("event-removed", &baseline, |journal, _, _| {
        let at = journal
            .iter()
            .position(|entry| matches!(entry, Admission::Event { .. }))
            .expect("the subject records an Event");
        journal.remove(at);
    });

    assert_eq!(
        [
            ("a coordinate repointed at an entry that exists", coordinate),
            ("the genesis's intention narrowed", intention),
            ("two roles reordered", harmless),
            ("two commitments reordered", commitments),
            ("an eligibility removed", eligibility),
            ("the cancelling Event removed", event),
        ],
        [
            (
                "a coordinate repointed at an entry that exists",
                Verdict::Refused
            ),
            ("the genesis's intention narrowed", Verdict::Refused),
            ("two roles reordered", Verdict::Harmless),
            ("two commitments reordered", Verdict::Refused),
            ("an eligibility removed", Verdict::Refused),
            ("the cancelling Event removed", Verdict::Refused),
        ],
        "the corruption table, as this experiment finds it"
    );

    // Nothing passes in silence any more, so what is left to assert is what the refusals
    // *say*. Half a refusal — one that reports the repository invalid without naming which
    // datum disagrees — sends a reader back to the bytes.
    assert!(
        repointed
            .complaint
            .contains("was admitted, and the decision was not taken against it"),
        "the sequence disagrees, and the entry is named: {}",
        repointed.complaint
    );
    // What turns this one changed, and the change belongs to the convergence experiment. A
    // narrowed genesis produces a different world, and the advancement after it names the world
    // it extends — so the reference stops resolving before anything compares a world at all.
    //
    // The refusal fires earlier and says less: it names a world that could not be found rather
    // than the coordinate that moved. Which is a statement about *this* lineage, where every
    // world but the last is extended by something. Narrowing the intention of a world nothing
    // extends still leaves the world witness as the only thing that turns it.
    assert!(
        narrowed
            .complaint
            .contains("which the lineage does not hold"),
        "the reference does not resolve, and the world is named: {}",
        narrowed.complaint
    );

    // The two refusals come from different witnesses, and the second is the one this phase
    // added. An altered intention leaves the journal untouched and every address resolving,
    // so nothing about the sequence can see it — only what it produces can.
    assert_ne!(
        repointed.complaint, narrowed.complaint,
        "a corruption of the coordinate and a corruption of the intention are not one finding"
    );

    let _ = baseline;
}

fn position(journal: &[Admission], magnitude: f64) -> usize {
    journal
        .iter()
        .position(|entry| {
            matches!(entry, Admission::Commitment { magnitude: Some(found), .. } if *found == magnitude)
        })
        .expect("the subject commits that magnitude")
}

/// Phase 6 — Terminate, reconstruct and compare.
///
/// Every rebuild in this harness already happens in a process of its own, and every refusal
/// already arrives on that process's stderr — the comparison lives inside reconstruction and
/// not inside a test, which is what the protocol asks and what a derived value checked only by
/// a harness would fail.
///
/// What is left to establish is the other half: that the recorded worlds are a witness and
/// never a source. A repository holding its journal and its worlds and no decisions describes
/// three worlds perfectly well, and cannot produce one.
#[test]
fn phase_6_reconstruct() {
    let repository = Repository::open(scratch("phase-6"));
    let reasoned = persisted(&repository);

    let intact = rebuild(&repository, reasoned.subject.instance);
    assert_eq!(intact.worlds().len(), 3, "the lineage reads");

    std::fs::remove_file(repository.lineage_path()).expect("the lineage file is there");

    let without_decisions = rebuild(&repository, reasoned.subject.instance);

    assert!(
        without_decisions.refused,
        "a repository that records its worlds and not its decisions produced worlds anyway"
    );

    // And the worlds it still holds are untouched, which is what makes the refusal a statement
    // about derivation rather than about the file being gone.
    assert_eq!(
        repository
            .read_worlds()
            .expect("the worlds are still there")
            .len(),
        3,
    );
}

/// Phase 7 — Forge.
///
/// The boundary of the result, measured rather than asserted. A repository edited *and* made
/// consistent — every derived value recomputed to match the edit — is not a disagreement, and
/// nothing here detects it.
///
/// This is not a defect to be repaired within this experiment. A record that checks itself
/// proves internal agreement and nothing about who wrote it; the rest needs a signature and a
/// key, which the boundary excludes.
#[test]
fn phase_7_forge() {
    let repository = Repository::open(scratch("phase-7-baseline"));
    let reasoned = persisted(&repository);
    let baseline = rebuild(&repository, reasoned.subject.instance)
        .worlds()
        .to_vec();

    // Half a forgery: the coordinate moved and the witness over the sequence made to agree
    // with it, while the recorded worlds still describe what was actually decided.
    let (half, _) = corrupted("forged-half", &baseline, |journal, decisions, r| {
        decisions[0].after = EntryId::of(r.canon.history().head().expect("a head"));
        witness_recomputed(journal, decisions);
    });

    assert_eq!(
        half,
        Verdict::Refused,
        "a forgery that leaves one derived value alone is still a disagreement"
    );

    // The whole of it. Every derived value is recomputed from what was written down, which is
    // what someone holding this code would do.
    let reasoned = divergence::reasoned().expect("the arrangement holds");
    let mut decisions = reasoned.decisions.clone();

    decisions[0].after = EntryId::of(reasoned.canon.history().head().expect("a head"));
    witness_recomputed(&reasoned.journal, &mut decisions);
    extends_recomputed(&reasoned.journal, &mut decisions);

    let mut canon = ape::canon::Canon::new(ResidentHistory::new());
    let (derived, _) = ape_cli::lineage::rebuild(&mut canon, &reasoned.journal, &decisions)
        .expect("the forged repository derives worlds of its own");

    let forged = Repository::open(scratch("forged-whole"));
    forged.write_journal(&reasoned.journal).expect("writable");
    forged.write_lineage(&decisions).expect("writable");
    forged
        .write_worlds(&worlds(derived.decided()))
        .expect("writable");

    let rebuilt = rebuild(&forged, reasoned.subject.instance);

    assert!(
        !rebuilt.refused,
        "a consistent forgery is not a disagreement: {}",
        rebuilt.complaint
    );

    let worlds = rebuilt.worlds();

    assert_ne!(
        worlds[0].thesis, baseline[0].thesis,
        "and it is a different lineage than the one that was reasoned about"
    );
    assert!(
        worlds[0].conflicts.is_empty(),
        "the refusal at -70 is gone, and nothing in the repository says so"
    );
}

/// Phase 5 — Subtract.
///
/// With the worlds recorded, what does the rest of the repository still have to hold? Answered
/// by removing things and seeing what stops working, rather than by arguing about it.
///
/// Two of the four the protocol lists are answered before the phase runs. The decisions cannot
/// go: a `Thesis` does not deserialize, so a recorded world is not a world and nothing else
/// produces one. The instant and the intention cannot go either — a cut is resolved from an
/// instant and a genesis selects what it selects, and a decision without them is not a
/// decision. Neither is expressible, which is its own kind of answer.
///
/// What is left is the pair this experiment added.
///
/// Every corruption here also recomputes `extends`, which the convergence experiment added. A
/// subtraction has to leave the repository consistent in every other respect or it stops
/// measuring the subtraction: a stale reference is a second inconsistency, and the refusal it
/// produces would be evidence about it rather than about what was removed.
#[test]
fn phase_5_subtract() {
    let repository = Repository::open(scratch("phase-5-baseline"));
    let reasoned = persisted(&repository);
    let baseline = rebuild(&repository, reasoned.subject.instance)
        .worlds()
        .to_vec();

    // Subtract the coordinate. Every decision points at the journal's last entry, so a reader
    // admits everything before deciding anything — which is the form the divergence experiment
    // refuted, expressed as data rather than as code.
    let (without_coordinate, refusal) =
        corrupted("no-coordinate", &baseline, |journal, decisions, _| {
            let last = {
                let mut canon = ape::canon::Canon::new(ResidentHistory::new());
                let mut admitted = journal::Replayed::default();
                journal::replay_remaining(&mut canon, journal, &mut admitted)
                    .expect("the journal admits");
                admitted
                    .entries
                    .last()
                    .cloned()
                    .expect("something admitted")
            };

            for taken in decisions.iter_mut() {
                taken.after = last.clone();
            }
            witness_recomputed(journal, decisions);
            extends_recomputed(journal, decisions);
        });

    assert_eq!(
        without_coordinate,
        Verdict::Refused,
        "a repository that cannot say where a decision was taken cannot be read at all"
    );
    assert!(
        refusal.complaint.contains("world 0 disagrees"),
        "and the world is what says so: {}",
        refusal.complaint
    );

    // Subtract the witness over the sequence, leaving the coordinate. The field stays and says
    // only what a reader would have computed, so nothing is compared that was not derived.
    let (without_witness, _) = corrupted("no-witness", &baseline, |journal, decisions, _| {
        witness_recomputed(journal, decisions);
    });

    assert_eq!(
        without_witness,
        Verdict::Harmless,
        "an intact repository survives losing it, which is the easy half"
    );

    // The hard half: with the witness gone, does anything still catch the corruption it was
    // introduced for?
    let (repointed_without_witness, complaint) = corrupted(
        "no-witness-repointed",
        &baseline,
        |journal, decisions, r| {
            let head = r.canon.history().head().expect("a head");
            decisions[0].after = EntryId::of(head);
            witness_recomputed(journal, decisions);
            extends_recomputed(journal, decisions);
        },
    );

    assert_eq!(
        repointed_without_witness,
        Verdict::Refused,
        "the world the coordinate produces is not the world that was recorded"
    );
    assert!(
        complaint.complaint.contains("world 0 disagrees"),
        "and it is the world witness that says so, not the sequence one: {}",
        complaint.complaint
    );
}
