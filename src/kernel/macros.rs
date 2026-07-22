//! Declarative macros for defining domain entities and their identities.
//!
//! - `define_id` — defines a newtype over `[u8; 32]` for a strongly-typed id,
//!   giving each entity a distinct id type so foreign references can't be mixed up.
//!
//! - `define_entity` — defines an entity struct wrapping a shared `Identity`
//!   and derives its id from the SHA-256 hash of `name` plus the core fields.
//!   Fields tagged `#[serde(alias = "no_hash")]` are kept for serialization but
//!   excluded from the id, so mutating them leaves the identity stable. It also
//!   generates a paired input struct (named after `via`) carrying `name` and
//!   every field, so construction takes a single named-field value instead of a
//!   long positional argument list.
//!
//! - `define_value_object` — defines a plain, id-less value object (struct or enum),
//!   compared by value.
//!
//! - `hash_field_conditional` — internal helper driving that per-field decision:
//!   it feeds a field into the hasher unless it carries the `no_hash` marker.
//!
//! - `define_error` — defines a `thiserror` error enum from `Variant => "message"` pairs.

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        pub struct $name([u8; 32]);

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::hex::serde::serialize(&self.0, serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let bytes = ::hex::serde::deserialize(deserializer)?;
                Ok(Self(bytes))
            }
        }

        impl From<[u8; 32]> for $name {
            fn from(value: [u8; 32]) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", ::hex::encode(self.0))
            }
        }

        impl AsRef<[u8; 32]> for $name {
            fn as_ref(&self) -> &[u8; 32] {
                &self.0
            }
        }
    };
}

macro_rules! define_entity {
    (pub struct $name:ident($id_type:ty) via $input:ident {
        $($( #[$attr:meta] )* $field_name:ident : $field_type:ty),* $(,)?
    }) => {
        #[derive(Debug, Clone, ::serde::Serialize)]
        pub struct $name {
            base: $crate::kernel::entities::identification::Identity<$id_type>,
            $($( #[$attr] )* pub $field_name : $field_type),*
        }

        #[derive(Debug, Clone)]
        pub struct $input {
            pub name: String,
            $(pub $field_name : $field_type),*
        }

        impl $name {
            pub fn create(input: $input) -> Result<Self, $crate::kernel::entities::identification::IdentityError> {
                let $input { name, $($field_name),* } = input;

                let mut hasher = <::sha2::Sha256 as ::sha2::Digest>::new();
                let _ = &mut hasher;

                if let Ok(bytes) = ::postcard::to_allocvec(&name) {
                    ::sha2::Digest::update(&mut hasher, &bytes);
                }

                $(
                    hash_field_conditional!(hasher, $field_name, [ $( #[$attr] )* ]);
                )*

                let hash_result = ::sha2::Digest::finalize(hasher);
                let mut id_bytes = [0u8; 32];
                id_bytes.copy_from_slice(&hash_result);

                Ok(Self {
                    base: $crate::kernel::entities::identification::Identity::new(<$id_type>::from(id_bytes), name)?,
                    $($field_name),*
                })
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::kernel::entities::identification::Identity<$id_type>;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl AsRef<$crate::kernel::entities::identification::Identity<$id_type>> for $name {
            fn as_ref(&self) -> &$crate::kernel::entities::identification::Identity<$id_type> {
                &self.base
            }
        }
    };
}

macro_rules! define_value_object {
    (pub struct $name:ident {
        $($field_name:ident : $field_type:ty),* $(,)?
    }) => {
        #[derive(Debug, Clone, ::serde::Serialize, PartialEq)]
        pub struct $name {
            $($field_name : $field_type),*
        }
    };

    (pub enum $name:ident {
        $( $body:tt )*
    }) => {
        #[derive(Debug, Clone, ::serde::Serialize, PartialEq)]
        pub enum $name {
            $( $body )*
        }
    };
}

macro_rules! hash_field_conditional {
    // Case A: The tag #[serde(alias = "no_hash")] was found inside the package.
    // Interrupts the hash generation.
    ($hasher:ident, $field_name:ident, [ #[serde(alias = "no_hash")] $( #[$rest:meta] )* ]) => {};

    // Case B: Any other attribute was found.
    // Discards it and analyzes the rest of the list.
    ($hasher:ident, $field_name:ident, [ #[$first:meta] $( #[$rest:meta] )* ]) => {
        hash_field_conditional!($hasher, $field_name, [ $( #[$rest] )* ]);
    };

    // Case C: The package was completely empty (every attribute was cleared or there was no attribute at all).
    // Serialization is run and hashing happens normally!
    ($hasher:ident, $field_name:ident, [ ]) => {
        if let Ok(bytes) = ::postcard::to_allocvec(&$field_name) {
            ::sha2::Digest::update(&mut $hasher, &bytes);
        }
    };
}

macro_rules! define_error {
    (pub enum $name:ident {
        $( $variant:ident => $message:literal),* $(,)?
    }) => {
        #[derive(Debug, ::thiserror::Error)]
        pub enum $name {
            $(
                #[error($message)]
                $variant,
            )*
        }
    };
}
