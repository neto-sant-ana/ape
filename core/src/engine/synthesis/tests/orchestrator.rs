//! The three conclusions a transfer reaches, and the order that produces them.

use std::collections::BTreeSet;

use super::{Fixture, ids, introducing, omitting};

use crate::engine::synthesis::{
    ApplicabilityConflict, ApplicabilityStatus, SynthesisError, synthesize,
};
use crate::engine::thesis::{InMemoryArchive, Thesis, ThesisArchive, ThesisError, ThesisId};

fn archived(theses: &[&Thesis]) -> InMemoryArchive {
    let mut archive = InMemoryArchive::default();

    for thesis in theses {
        archive.put_thesis((*thesis).clone()).unwrap();
    }

    archive
}

#[test]
fn an_effective_transfer_with_nothing_against_it_is_applicable() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[elsewhere])).unwrap();

    let archive = archived(&[&base, &source, &target]);
    let report = synthesize(&archive, &knowledge, base.id(), source.id(), target.id()).unwrap();

    assert!(report.is_applicable());
    assert!(report.conflicts().is_empty());

    let ApplicabilityStatus::Applicable {
        transfer,
        candidate,
    } = report.status()
    else {
        panic!("an effective transfer with no conflict is applicable")
    };

    assert_eq!(transfer.introduce().collect::<BTreeSet<_>>(), ids(&[added]));
    assert_eq!(
        candidate.resolved().collect::<BTreeSet<_>>(),
        ids(&[shared, added, elsewhere]),
        "the candidate is the Target with the transfer applied",
    );
}

/// Neither applicable nor conflicted: the world already contains the difference.
#[test]
fn a_transfer_the_target_already_contains_is_already_applied() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[added])).unwrap();

    let archive = archived(&[&base, &source, &target]);
    let report = synthesize(&archive, &knowledge, base.id(), source.id(), target.id()).unwrap();

    assert_eq!(report.status(), &ApplicabilityStatus::AlreadyApplied);
    assert!(
        !report.is_applicable(),
        "there is no effective change to apply",
    );
    assert!(
        report.conflicts().is_empty(),
        "and no invariant was violated either",
    );
    assert!(
        !report.difference().is_empty(),
        "the Source did decide something; the Target had reached it too",
    );
}

#[test]
fn a_transfer_against_an_invariant_is_conflicted() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[settled, kept]);
    let source = base.fork(&knowledge, omitting(&[settled])).unwrap();

    knowledge.settle(settled);
    let advanced = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = advanced.thesis();

    let archive = archived(&[&base, &source, target]);
    let report = synthesize(&archive, &knowledge, base.id(), source.id(), target.id()).unwrap();

    assert!(!report.is_applicable());
    assert_eq!(
        report.conflicts(),
        [ApplicabilityConflict::HistoricalFreezing {
            commitment: settled
        }],
    );

    let ApplicabilityStatus::Conflicted { attempted, .. } = report.status() else {
        panic!("a violated invariant conflicts")
    };

    assert_eq!(
        attempted.remove().collect::<BTreeSet<_>>(),
        ids(&[settled]),
        "what was asked for stays visible in the report",
    );
}

/// An incoherent Base ends the operation. It is a precondition, not a finding: no invariant
/// was broken, there is simply no intentional difference to speak of — and the conflicts a
/// candidate would show are noise attributed to a transfer nobody could have intended.
#[test]
fn an_incoherent_base_is_refused_rather_than_reported() {
    let mut knowledge = Fixture::default();
    let here = knowledge.commit((3, 31), BTreeSet::new());
    let dependency = knowledge.commit((4, 30), BTreeSet::new());
    let dependent = knowledge.commit((5, 31), ids(&[dependency]));

    let base = knowledge.genesis(&[here, dependency]);
    let source = base.fork(&knowledge, omitting(&[dependency])).unwrap();
    let unrelated = knowledge.genesis(&[dependency, dependent]);

    let archive = archived(&[&base, &source, &unrelated]);

    assert!(
        matches!(
            synthesize(&archive, &knowledge, base.id(), source.id(), unrelated.id()),
            Err(SynthesisError::IncoherentBase { base: b, source_thesis: s, target_thesis: t })
                if b == base.id() && s == source.id() && t == unrelated.id()
        ),
        "no report is produced, so the dependency it would break is never attributed",
    );
}

#[test]
fn a_thesis_the_archive_does_not_hold_is_refused() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let base = knowledge.genesis(&[shared]);

    let archive = archived(&[&base]);
    let absent = ThesisId::from([9; 32]);

    assert!(matches!(
        synthesize(&archive, &knowledge, base.id(), absent, base.id()),
        Err(SynthesisError::Thesis(ThesisError::UnknownThesis(id))) if id == absent
    ));
}

/// The same three Theses answer the same way, which is what makes a report comparable.
#[test]
fn the_same_trio_produces_the_same_report() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[elsewhere])).unwrap();

    let archive = archived(&[&base, &source, &target]);

    assert_eq!(
        synthesize(&archive, &knowledge, base.id(), source.id(), target.id()).unwrap(),
        synthesize(&archive, &knowledge, base.id(), source.id(), target.id()).unwrap(),
    );
}
