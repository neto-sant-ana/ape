//! `Interpretation` — a Thesis and the cut it recognizes, handed to Hermeneia as one thing.
//!
//! A Thesis is projectable at the head its cut recognizes and nowhere else. That rule is not
//! kept by anyone remembering it: a selection and a chain passed as separate arguments can
//! always be mismatched, and the answer would look ordinary — settlements the cut never
//! recognized, or an earlier world reported as the current one.
//!
//! So the pair is never offered apart. The chain is resolved here, from the recognized head,
//! and folded into an accumulation bound to that head. Hermeneia refuses an event beyond it and
//! refuses to interpret before reaching it, so even the fold underneath cannot answer for
//! another cut.
//!
//! ```text
//! Interpretation::of(thesis, knowledge)
//!     → chain = recognized_chain(thesis.cut().event_head())
//!     → Accumulation::recognizing(head).absorb(selection, chain)
//! ```
//!
//! One fold, many questions. Conditions answer to an effective time and feasibility does not,
//! which is why they stay separate methods over one accumulation rather than one bundled
//! result: interpreting is `view(fold, T)`, and the fold is what an interpretation holds.

use super::frozen::recognized_chain;
use super::{Thesis, ThesisError};

use crate::canon::CanonicalKnowledge;

use crate::engine::hermeneia::{
    Accumulation, FeasibilityReport, HermeneiaError, Hypothesis, ProjectedConditions,
};

use crate::kernel::axiom::Knowledge;

use crate::kernel::value_objects::Date;

/// What interpreting a Thesis reads: the canonical records that resolve its cut, and the bare
/// entities a projection derives consequences from.
pub trait InterpretableKnowledge: Knowledge + CanonicalKnowledge {}
impl<K: Knowledge + CanonicalKnowledge> InterpretableKnowledge for K {}

pub struct Interpretation {
    accumulation: Accumulation,
}
impl Interpretation {
    pub fn of<K: InterpretableKnowledge>(
        thesis: &Thesis,
        knowledge: &K,
    ) -> Result<Self, ThesisError> {
        let head = thesis.cut().event_head();
        let chain = recognized_chain(knowledge, head)?;

        let selection: Vec<_> = thesis.selection().resolved().collect();

        let mut accumulation = Accumulation::recognizing(head);
        accumulation.absorb(knowledge, &selection, &chain)?;

        Ok(Self { accumulation })
    }

    pub fn conditions_at(&self, at: &Date) -> Result<ProjectedConditions, HermeneiaError> {
        self.accumulation.conditions_at(at)
    }

    pub fn feasibility_under(
        &self,
        hypothesis: Hypothesis,
    ) -> Result<FeasibilityReport, HermeneiaError> {
        self.accumulation.feasibility_under(hypothesis)
    }
}
