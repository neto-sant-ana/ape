//! What the application refuses, as distinct from what the engine refuses.
//!
//! An engine error carried through unchanged says where it came from; what this adds is
//! the cases the engine has no opinion about, because they arise from the application
//! reading a world it assembled itself.

use ape::engine::hermeneia::HermeneiaError;
use ape::kernel::entities::CommitmentId;

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error(transparent)]
    Hermeneia(#[from] HermeneiaError),

    /// A projection named a commitment the knowledge behind it could not resolve, which
    /// means the two were not read from the same world.
    #[error("commitment {0} is projected but absent from knowledge")]
    UnknownCommitment(CommitmentId),
}
