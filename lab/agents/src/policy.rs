//! Whether an intention may proceed — a decision the engine has no opinion on.
//!
//! The rule is fixed before the agent runs and applied unchanged afterwards:
//!
//! > *An intention may proceed when, under a hypothesis the agent has named, the
//! > interpretation of what it selected reports no conflict.*
//!
//! Nothing here knows about cash, about limits, or about what a good decision is. It reads
//! what the engine derived and turns that into a decision about proceeding, which is the
//! part that belongs to whoever is operating rather than to whoever is interpreting.
//!
//! One property is intentional rather than incidental. `hypothesis` is a parameter because
//! nothing in the engine names one on a caller's behalf, so the rule cannot be applied
//! until somebody has said under what assumption the question is being asked. An agent that
//! never names one leaves this inapplicable, and that is a finding rather than a defect.

use ape::engine::hermeneia::{Conflict, HermeneiaError, Hypothesis};
use ape::engine::thesis::Interpretation;

#[derive(Debug, PartialEq)]
pub enum Verdict {
    MayProceed,
    Refused(Vec<Conflict>),
}

pub fn rule(
    interpretation: &Interpretation,
    hypothesis: Hypothesis,
) -> Result<Verdict, HermeneiaError> {
    let report = interpretation.feasibility_under(hypothesis)?;

    let conflicts = report.conflicts();

    Ok(if conflicts.is_empty() {
        Verdict::MayProceed
    } else {
        Verdict::Refused(conflicts.to_vec())
    })
}
