//! Declarative macros for the Canon.
//!
//! - `recordable_anytime` — implements `RecordableAfter` returning `None` for
//!   assertions with no lower bound on `recorded_at` (definitional entities, and
//!   eligibility as a forward declaration).
//!
//! - `canonical_admission` — generates the uniform `admit_*` methods of the Canon.

macro_rules! recordable_anytime {
    ($($type:ty),+ $(,)?) => {
        $(
            impl $crate::canon::record::RecordableAfter for $type {
                fn recordable_after(&self) -> Option<&$crate::kernel::value_objects::Date> {
                    None
                }
            }
        )+
    };
}

macro_rules! canonical_admission {
    ($($method:ident($input:ty) -> $id:ty { $axiom:ident, $put:ident }),+ $(,)?) => {
        $(
            pub fn $method(
                &mut self,
                input: $input,
                recorded_at: $crate::kernel::value_objects::Date,
            ) -> Result<$id, $crate::canon::CanonError> {
                let assertion = $crate::kernel::axiom::Axiom::new(&self.history).$axiom(input)?;
                let id = assertion.id();

                self.history.$put($crate::canon::Canonical::new(assertion, recorded_at)?);

                Ok(id)
            }
        )+
    };
}
