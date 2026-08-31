//! Phase 4 — a second classifier's answers, and the agreement between the two readings.
//!
//! A session that had not read the protocol, the charter, the observations or the classification was
//! given the 46 claims of `05-reconciliation`, the twelve record files, and one question: *could
//! somebody holding only these files establish this?* Its briefing is described in
//! `00-testimony/08-the-second-classifier.md`, with digests published before it existed.
//!
//! # Why its answers live here as data
//!
//! So the agreement is **computed** rather than counted by the reader whose work is being checked.
//! This row's stated hazard is a classifier who wants a particular answer; a hand-tallied agreement
//! score would be that hazard applied to the instrument built to detect it.
//!
//! # How the two readings are compared
//!
//! Fixed in `08` before the run, because a comparison decided afterwards is not a check:
//!
//! ```text
//! Housed                → YES
//! Unhoused, Exposition  → NO
//! ```

use crate::classification::Verdict;
use crate::corpus::Run;
use crate::testimony::classified;

/// The claims the second classifier answered YES, by their number in `claims.md` — which is the
/// order they appear in the testimony, and therefore the order of this crate's own list.
///
/// Seventeen of forty-six. Transcribed from its `ANSWER.md`, which is the one hand-copied list in
/// this experiment and is unavoidable: the classifier wrote prose, and prose is what it was asked
/// for.
pub const ESTABLISHED: [usize; 17] = [
    1, 2, 3, 4, 13, 14, 15, 16, 20, 21, 22, 26, 28, 29, 30, 32, 33,
];

/// Everything it flagged as genuinely difficult, in its own numbering.
///
/// Kept because a disagreement it *predicted* is worth less against this reading than one it did
/// not — it said of 5, 9 and 12 that grading on the sentence's principal assertion would flip all
/// three, which is a disagreement about the briefing's rule rather than about the record.
pub const DIFFICULT: [usize; 7] = [5, 8, 9, 12, 15, 28, 32];

/// Whether this reading called a claim housed, in `claims.md` order.
pub fn housed_here() -> Vec<bool> {
    classified()
        .iter()
        .filter(|claim| claim.run == Run::Reconciliation)
        .map(|claim| matches!(claim.verdict, Verdict::Housed(_)))
        .collect()
}

/// Every claim the two readings answered differently, by number.
pub fn disagreements() -> Vec<usize> {
    housed_here()
        .into_iter()
        .enumerate()
        .filter(|(index, mine)| ESTABLISHED.contains(&(index + 1)) != *mine)
        .map(|(index, _)| index + 1)
        .collect()
}
