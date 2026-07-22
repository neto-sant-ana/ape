//! Failures from computing an entity's content-addressed id.

define_error! {
    pub enum IdentityError {
        Serialization => "failed to serialize a field while computing the entity id",
    }
}
