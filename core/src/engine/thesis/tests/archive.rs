//! The reference archive answers the contract.

use crate::engine::thesis::InMemoryArchive;

#[test]
fn the_in_memory_archive_conforms() {
    crate::engine::thesis::conformance::verify(InMemoryArchive::default);
}
