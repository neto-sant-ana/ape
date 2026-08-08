//! The engine's conformance suite, run against this application's adapters.
//!
//! Thread safety is not claimed, so `verify_thread_safe` is not run: the experiment
//! excludes concurrent processes, and claiming a contract the adapter does not hold would
//! be worse than not claiming it.

use ape_cli::history::ResidentHistory;

#[test]
fn resident_history_conforms() {
    ape::canon::conformance::verify(ResidentHistory::new);
}
