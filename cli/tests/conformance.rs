//! The engine's conformance suite, run against this application's adapters.
//!
//! Thread safety is not claimed, so `verify_thread_safe` is not run: the experiment
//! excludes concurrent processes, and claiming a contract the adapter does not hold would
//! be worse than not claiming it.

use ape_cli::archive::ResidentArchive;
use ape_cli::history::ResidentHistory;

#[test]
fn resident_history_conforms() {
    ape::canon::conformance::verify(ResidentHistory::new);
}

/// The archive holds worlds within one process, which is the only place it can hold them.
///
/// A `Thesis` derives `Serialize` and not `Deserialize`, so an archive written out cannot be
/// read back in — the same boundary the reconstruction experiment met at the Canon, met again
/// one layer up. Conforming to the port is therefore a claim about ancestry inside a running
/// application, and about nothing that crosses a process.
#[test]
fn resident_archive_conforms() {
    ape::engine::thesis::conformance::verify(ResidentArchive::new);
}
