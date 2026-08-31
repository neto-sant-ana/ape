//! The eight testimonies, and where each one is.
//!
//! One `ANSWER.md` per agent run, written as the deliverable of a task and not for this row. The
//! digests and word counts are published in `00-testimony/01-the-corpus.md`, pinned before anything
//! was read.
//!
//! # Why an enum rather than a path
//!
//! A claim names its run by a variant, so a misspelled path cannot become a ninth testimony that
//! nothing checks — and [`Run::ALL`] is what lets the guard compare *what was classified* against
//! *what the laboratory holds*, which is the assertion that the sweep did not silently read seven.

/// Which agent's testimony a claim was read out of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Run {
    /// 01, in-memory boundary, engine `db3f965`. The first, and the only one whose boundary is gone.
    SingleAgent,
    /// 02, in-memory. What could have been known against what was consulted.
    Hindsight,
    /// 03, in-memory. The run whose prose was sorted against the derivation and found wrong.
    NarrativeMismatch,
    /// 04, the repository boundary. One of two parties.
    MultiagentA,
    /// 04, the other party.
    MultiagentB,
    /// 04, a re-run of [`Run::MultiagentB`] after its arrangement was corrected.
    MultiagentBPrime,
    /// 04, the only agent that read rather than wrote. The control for P5.
    MultiagentReader,
    /// 05, the repository with four files, and the row's first enforced isolation.
    Reconciliation,
}

impl Run {
    /// Every testimony in the corpus, in the order `01-the-corpus.md` fixed before any was read.
    pub const ALL: [Run; 8] = [
        Run::SingleAgent,
        Run::Hindsight,
        Run::NarrativeMismatch,
        Run::MultiagentA,
        Run::MultiagentB,
        Run::MultiagentBPrime,
        Run::MultiagentReader,
        Run::Reconciliation,
    ];

    /// Where the testimony is, relative to `lab/agents`.
    pub fn path(&self) -> &'static str {
        match self {
            Run::SingleAgent => "01-single-agent/run-01/ANSWER.md",
            Run::Hindsight => "02-hindsight/run-01/ANSWER.md",
            Run::NarrativeMismatch => "03-narrative-mismatch/run-b/ANSWER.md",
            Run::MultiagentA => "04-multiagent/run-a/ANSWER.md",
            Run::MultiagentB => "04-multiagent/run-b/ANSWER.md",
            Run::MultiagentBPrime => "04-multiagent/run-b-prime/ANSWER.md",
            Run::MultiagentReader => "04-multiagent/run-reader/ANSWER.md",
            Run::Reconciliation => "05-reconciliation/run-a/ANSWER.md",
        }
    }

    /// Whether the run left anything in a record, or only read one.
    ///
    /// P5's control: the kinds a reader produces should differ from the kinds a writer produces, and
    /// if they do not, the kinds belong to the record rather than to what the agent was doing.
    ///
    /// **`Hindsight` is a reader, and the corpus document published it as a writer.** Its own
    /// testimony says *the audit reads only — it admits nothing to the canon and stores nothing in
    /// the archive*, which was found in Phase 2 and is corrected here rather than where it was
    /// convenient. It builds alternative worlds in memory to interpret them, and persists none: the
    /// line is what a run **left behind**, because that is what P5 is about. See
    /// `03-the-corpus-had-two-readers.md`.
    pub fn wrote(&self) -> bool {
        !matches!(self, Run::MultiagentReader | Run::Hindsight)
    }

    /// Whether the run's boundary was the repository or the in-memory engine.
    ///
    /// The other axis the corpus permits: a kind appearing only under one boundary is a property of
    /// that boundary rather than of the record.
    pub fn had_repository(&self) -> bool {
        matches!(
            self,
            Run::MultiagentA
                | Run::MultiagentB
                | Run::MultiagentBPrime
                | Run::MultiagentReader
                | Run::Reconciliation
        )
    }
}
