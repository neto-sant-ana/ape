//! The corroboration experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the earlier experiments give: a
//! comparison has to fail loudly. What is different here is what is under test — not a world,
//! and not a lineage, but the repository's ability to notice that it is wrong.
//!
//! So this harness does something none of the others do: it edits a persisted repository. The
//! phases that follow Phase 2 are claims about which of those edits stop being accepted.

use ape::kernel::value_objects::Date;

use ape_cli::reading::{self, ConflictRecord, OutcomeRecord, Reading};
use ape_cli::repository::Repository;
use ape_cli::subject::divergence::{self, Reasoned};

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

    reasoned
}

/// Rebuild in an operating-system process of its own, given the repository and nothing else.
fn rebuild(
    repository: &Repository,
    instance: ape::kernel::entities::ResourceInstanceId,
) -> Rebuilt {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ape-cli"))
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
        &reasoned.lineage,
        reasoned.subject.instance,
        &Date::parse(EFFECTIVE).expect("the effective instant is a date"),
    )
    .expect("the living lineage reads");

    assert_eq!(
        living, worlds,
        "the baseline holds across the process boundary"
    );
}
