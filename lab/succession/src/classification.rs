//! What each claim in the corpus was judged to be, as data rather than as prose.
//!
//! The judgements live here so that a reader can disagree with **one** of them without re-reading
//! 13,382 words, and so that the guard can derive what they must satisfy. Every unhoused claim
//! carries its verbatim text for the same reason: a classification a reader has to take on trust is
//! a reading wearing an experiment's clothes.
//!
//! # The two questions, from `00-testimony/00-protocol.md`
//!
//! 1. **Housed?** Could this claim be read off the record by somebody with the four files and no
//!    prose? If yes, it names which carrier holds it.
//! 2. **If not**, which of the five kinds is it — and the five are fixed by the protocol. A claim
//!    fitting none of them is [`Verdict::Unhoused`] with `None`, never forced into the nearest box.
//!
//! # The third verdict, which is a deviation from the protocol and is recorded as one
//!
//! The protocol's binary presumes every claim is **about the record**. Contact with the corpus
//! produced a large class that is not: sentences explaining what the *engine* is and how it works —
//! *"a Commitment is knowledge, a Thesis is which knowledge you mean"*. Asking whether a record
//! houses a tutorial is the wrong question, because a record is not supposed to hold one.
//!
//! So [`Verdict::Exposition`] exists, and the reason it is not a widening of the five kinds is that
//! it is on the other axis: the kinds classify *unhoused material a record might want*, and this is
//! material a record should not want. Widening the kinds is forbidden by the protocol; adding a
//! verdict is what the protocol already did once, in the skip list.
//!
//! It is a deviation all the same, taken during Phase 2 rather than pre-registered, and
//! `02-the-third-verdict.md` states what it costs.

use crate::corpus::Run;

/// One claim, and what it was judged to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub run: Run,
    /// Verbatim, trimmed only of surrounding markdown. Abridged with `…` where a sentence is long
    /// and the elision changes nothing a reader would classify differently.
    pub text: &'static str,
    pub verdict: Verdict,
    /// What the laboratory has since done about a claim that **asks** for something.
    ///
    /// Present exactly for [`Kind::Want`] and [`Kind::Loss`], and absent for everything else, which
    /// the constructors enforce rather than a guard: the data the invariant refers to is all local to
    /// one claim, so it belongs in construction.
    pub standing: Option<Standing>,
}

/// Where a want or a loss stands today, so that this experiment cannot re-report the queue.
///
/// **Added in Phase 2, after the operator asked whether the opaque constraint had already been
/// handled** — it had, in one half and not the other, and the first commit of this experiment
/// reported the recurrence as fresh evidence. Without this field the experiment rediscovers what the
/// laboratory already holds, which is the drift the charter was written to stop. See
/// `04-a-want-has-a-standing.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// In `QUEUE.md` or a `candidates/` file, at this path from the repository root. Checked to
    /// exist.
    ///
    /// **Tracked means selectable.** The queue is what gets read when work is chosen, so an item in
    /// it can be picked.
    Tracked(&'static str),
    /// In a result document and nowhere the queue reads, at this path from the repository root.
    ///
    /// **The distinction from [`Standing::Tracked`] is this experiment's sharpest method finding.**
    /// A finding in a result document is findable by somebody who already knows to look and is
    /// invisible to selection: the queue orders items *in the queue*, so ripeness cannot reach it,
    /// ever. `agents/01`'s two frictions have been in its result since the first experiment and are
    /// in no queue.
    Recorded(&'static str),
    /// Served since the testimony was written, and by what.
    ///
    /// A met want is the most informative thing in the corpus: it is the only kind of evidence that
    /// says what happens **after** a boundary grows.
    Met(&'static str),
    /// The engine or the laboratory has ruled that the record deliberately does not carry it, and
    /// where that ruling is written.
    ///
    /// **These are H4's target, and realising that is what the category is for.** A want nobody
    /// noticed is a gap; a want the ontology *correctly* refuses is something else — `02-hindsight`'s
    /// own result says of one of them that it is *absent by design rather than by omission, and
    /// filling it is an application's business if any application wants it*. H4 asks exactly that
    /// question: not whether these belong in a primitive, which is settled and settled *no*, but
    /// whether they belong **beside** the entity.
    ByDesign(&'static str),
    /// Nowhere — not the queue, not a candidate, not a result document, not a ruling.
    Untracked,
}

impl Standing {
    /// The document this standing points at, relative to the repository root, if it points at one.
    pub fn cited(&self) -> Option<&'static str> {
        match self {
            Standing::Tracked(where_) | Standing::Recorded(where_) | Standing::ByDesign(where_) => {
                Some(where_)
            }
            Standing::Met(_) | Standing::Untracked => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Readable off the record, and by what.
    Housed(Carrier),
    /// Not readable. `Some(kind)` is one of the protocol's five; `None` is *fits none of them*.
    Unhoused(Option<Kind>),
    /// A claim about the engine rather than about this record. See the module docstring.
    Exposition,
}

/// What holds a housed claim.
///
/// Split into the kernel's own names and the record-level carriers, because the guard can check the
/// first against `core/`'s sources and cannot check the second — so keeping them apart is what makes
/// the checkable half checkable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carrier {
    /// A kernel entity, by the name the kernel gives it. Derived-checked.
    Entity(&'static str),
    /// One of the four files a repository holds. For the in-memory runs, its equivalent: the
    /// admitted knowledge, the decided worlds, and what a decision stood on.
    File(File),
    /// A value the engine computes rather than stores.
    Derived(Derived),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    Journal,
    Lineage,
    Worlds,
    Custody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Derived {
    Feasibility,
    Conditions,
    Applicability,
    Interpretation,
    Cut,
    Selection,
    WorldIdentity,
}

/// The five kinds, derived from `05-reconciliation` alone and fixed by the protocol before the other
/// seven were opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// An alternative weighed and rejected, with the reason.
    RoadNotTaken,
    /// Something reached for at the boundary and not found.
    Want,
    /// A confidence or weakness attached to a claim the record does hold.
    Qualification,
    /// A statement that the record cannot say something.
    Loss,
    /// A limit of how the work was verified, rather than of the record.
    MethodLimit,
}

/// Claims the protocol excludes from classification, counted rather than dropped so that coverage is
/// measurable.
///
/// Section headings, restatements of the task, and pure narration. Counted per run because a run
/// whose testimony is mostly narration would otherwise look like a run with few claims.
pub fn skipped(run: Run) -> usize {
    match run {
        Run::SingleAgent => 9,
        Run::Hindsight => 0,
        Run::NarrativeMismatch => 0,
        Run::MultiagentA => 0,
        Run::MultiagentB => 0,
        Run::MultiagentBPrime => 0,
        Run::MultiagentReader => 0,
        Run::Reconciliation => 0,
    }
}
