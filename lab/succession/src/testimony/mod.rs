//! The classification, one module per testimony, in the order `01-the-corpus.md` fixed.
//!
//! Split by run rather than held in one list so that a reader can review one agent's judgements
//! against one agent's words, which is the only way a disagreement is cheap to state.

use crate::classification::Claim;
use crate::corpus::Run;

pub mod hindsight;
pub mod multiagent_a;
pub mod multiagent_b;
pub mod narrative_mismatch;
pub mod single_agent;

/// Every claim classified so far, across every testimony that has been read.
///
/// The guard compares the runs represented here against [`Run::ALL`] and names the ones missing, so
/// a partial classification cannot report agreement — Phase 2 lands run by run, and each landing is
/// a smaller claim than the experiment's.
pub fn classified() -> Vec<Claim> {
    let mut claims = Vec::new();

    claims.extend_from_slice(single_agent::CLAIMS);
    claims.extend_from_slice(hindsight::CLAIMS);
    claims.extend_from_slice(narrative_mismatch::CLAIMS);
    claims.extend_from_slice(multiagent_a::CLAIMS);
    claims.extend_from_slice(multiagent_b::CLAIMS);

    claims
}

/// The testimonies not yet classified, which is how an unfinished Phase 2 says so out loud.
pub fn unread() -> Vec<Run> {
    let read: Vec<Run> = classified().iter().map(|claim| claim.run).collect();

    Run::ALL
        .into_iter()
        .filter(|run| !read.contains(run))
        .collect()
}
