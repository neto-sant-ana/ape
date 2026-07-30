//! `Advancement` — a Thesis recognizing later history, and what that history imposed.
//!
//! An advancement carries more than the Thesis it produced, because it is the only moment
//! at which a commitment enters a continuation without anyone having chosen it. When an
//! Event settles a commitment the parent did not select, recognizing that Event means
//! selecting the commitment: the fact is shared, and a Thesis omitting it could not
//! interpret the chain that contains it.
//!
//! ```text
//! Imposed(T, H') = Frozen(H') − Commitments(T)
//! ```

use std::collections::BTreeSet;

use super::Thesis;

use crate::kernel::entities::CommitmentId;

#[derive(Debug, Clone)]
pub struct Advancement {
    thesis: Thesis,
    imposed: BTreeSet<CommitmentId>,
}
impl Advancement {
    pub(super) fn new(thesis: Thesis, imposed: BTreeSet<CommitmentId>) -> Self {
        Self { thesis, imposed }
    }

    pub fn thesis(&self) -> &Thesis {
        &self.thesis
    }

    pub fn into_thesis(self) -> Thesis {
        self.thesis
    }

    pub fn imposed(&self) -> &BTreeSet<CommitmentId> {
        &self.imposed
    }
}
