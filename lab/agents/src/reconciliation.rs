//! Two records that took the same world away and came back disagreeing.
//!
//! ```text
//! base            the world of experiments 01 to 04: opening +100 settled, a standing
//!                 arrangement of −20 intended, and one world at the 6th selecting it
//!
//! operations      stood the slot down and put a purchase in its place    −60, due the 20th
//! finance         learned of a receipt and took it into the base         +40, due the 12th
//! ```
//!
//! **Both parties read the base before either wrote**, which is experiment 04's own correction
//! applied: two parties in sequence are one line, and divergence is what parties produce when neither
//! has seen the other's writing. So the two journals agree for the whole base and disagree at the
//! entry after it, and `converge` refuses them.
//!
//! # Built twice rather than cloned
//!
//! [`crate::coordination::under`] is called once per party. A `Canon` is not cloneable and the base is
//! deterministic, so two runs of the same construction produce two journals that agree address for
//! address — which is the property the divergence is measured against, and building it by copying a
//! value would make it an artefact of the copy.
//!
//! It also means the two parties hold the **same base world by identity**, which experiment 09
//! measured for records that never met. That is what makes the arrangement fair rather than cruel: a
//! caller reconciling these two has a shared ancestor to work from, and does not have to invent one.
//!
//! # The advance the arrangement forces, and does not mention
//!
//! A fork inherits its parent's cut. The base recognizes the 6th, and everything either party learned
//! afterwards was recorded later than that — so a decision taking the other party's material into the
//! base is refused by `CommitmentNotKnownAtCut` until the cut is carried forward.
//!
//! Experiment 04's operations met exactly this and discovered it rather than being told. It is left
//! unmentioned here for the same reason: a friction the briefing names is a friction the briefing
//! caused.
//!
//! # Written whole, which is the boundary that grew
//!
//! Each repository goes down through `write_whole`, so each carries `custody.json` — the fourth file,
//! which did not exist when experiment 04's agents ran. Nothing in the task points at it, and
//! [F5](../../05-reconciliation/00-protocol.md) is the prediction that it stays unopened.

use ape::kernel::entities::{AgentId, CommitmentId};

use ape_cli::error::RepositoryError;
use ape_cli::journal::Admission;
use ape_cli::lineage::Taken;
use ape_cli::reading::WorldRecord;
use ape_cli::repository::{Repository, RepositoryInput};

use crate::coordination::{self, Line, Shared};
use crate::world::{self, Intention};

/// What operations puts in the standing arrangement's place, and when it is due.
pub const PURCHASE: u128 = 60;
pub const PURCHASE_DUE: u8 = 20;

/// What finance learned had been promised to the house, and when.
pub const RECEIPT: u128 = 40;
pub const RECEIPT_DUE: u8 = 12;

/// When each party recorded what it learned while the other could not see it.
///
/// Two instants rather than one, and they are the thing the reconciliation turns on: whoever recorded
/// earlier has to land first, because a Canon refuses an admission dated before its watermark. That is
/// the sentence experiment 04 handed over and `converge` now owns — met here by a caller.
pub const OPERATIONS_RECORDED: u8 = 7;
pub const FINANCE_RECORDED: u8 = 8;

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(PURCHASE != RECEIPT && PURCHASE != coordination::STANDING);
const _: () = assert!(RECEIPT != coordination::STANDING);
const _: () = assert!(coordination::OPENED < OPERATIONS_RECORDED);
const _: () = assert!(OPERATIONS_RECORDED < FINANCE_RECORDED);
const _: () = assert!(FINANCE_RECORDED < RECEIPT_DUE && RECEIPT_DUE < PURCHASE_DUE);

/// One party's record: everything it holds, and what it decided.
pub struct Parted {
    pub shared: Shared,
    /// What this party admitted that the other did not.
    pub own: CommitmentId,
    /// The party its decisions claim.
    pub party: AgentId,
}

impl Parted {
    /// Put it down the way an application does — one call, all four files.
    pub fn write(&self, repository: &Repository) -> Result<(), RepositoryError> {
        let worlds: Vec<WorldRecord> = self
            .shared
            .lineage
            .decided()
            .iter()
            .map(WorldRecord::of)
            .collect();

        repository.write_whole(RepositoryInput {
            journal: &self.shared.journal,
            lineage: &self.shared.decisions,
            worlds: &worlds,
        })
    }

    pub fn journal(&self) -> &[Admission] {
        &self.shared.journal
    }

    pub fn decisions(&self) -> &[Taken] {
        &self.shared.decisions
    }
}

/// The record operations kept: the slot stood down, a purchase in its place.
pub fn operations() -> Parted {
    let mut shared = coordination::under(world::cash());
    let party = shared.operations;

    let purchase = coordination::intend(
        &mut shared,
        Intention {
            magnitude: PURCHASE,
            incoming: false,
            due: PURCHASE_DUE,
            recorded_at: OPERATIONS_RECORDED,
            dependencies: [].into(),
        },
    );

    let standing = shared.standing;
    let base = shared.base;

    coordination::carry(&mut shared, base, OPERATIONS_RECORDED, party);

    let carried = shared
        .lineage
        .decided()
        .last()
        .expect("an advance produces a world")
        .id();

    coordination::decide(
        &mut shared,
        &Line {
            omitted: [standing].into(),
            introduced: [purchase].into(),
        },
        carried,
        party,
    );

    Parted {
        shared,
        own: purchase,
        party,
    }
}

/// The record finance kept: a receipt it learned of, taken into the base.
pub fn finance() -> Parted {
    let mut shared = coordination::under(world::cash());
    let party = shared.finance;

    let receipt = coordination::intend(
        &mut shared,
        Intention {
            magnitude: RECEIPT,
            incoming: true,
            due: RECEIPT_DUE,
            recorded_at: FINANCE_RECORDED,
            dependencies: [].into(),
        },
    );

    let base = shared.base;

    coordination::carry(&mut shared, base, FINANCE_RECORDED, party);

    let carried = shared
        .lineage
        .decided()
        .last()
        .expect("an advance produces a world")
        .id();

    coordination::decide(
        &mut shared,
        &Line {
            omitted: [].into(),
            introduced: [receipt].into(),
        },
        carried,
        party,
    );

    Parted {
        shared,
        own: receipt,
        party,
    }
}

/// How much of the base both parties hold, derived rather than counted by hand.
///
/// The number the refusal carries, reached the other way: the length of the journal before either
/// party added anything. A phase that took it from the refusal alone would be checking the refusal
/// against itself.
pub fn base_entries() -> usize {
    coordination::under(world::cash()).journal.len()
}
