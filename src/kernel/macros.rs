//! Declarative macros for defining domain entities and their identities.
//!
//! - `define_id` — defines a newtype over `[u8; 32]` for a strongly-typed id,
//!   giving each entity a distinct id type so foreign references can't be mixed up.
//!
//! - `define_entity` — defines an entity struct holding its content-addressed
//!   `id` plus its fields, and derives the id from the SHA-256 hash of every
//!   field. Entity fields are private with read-only getters (and an `id()`): an
//!   entity is immutable once constructed. It also generates a paired input struct
//!   (named after `via`) carrying every field, so construction takes a single
//!   named-field value instead of a long positional argument list.
//!
//! - `define_value_object` — defines a plain, id-less value object (struct or enum),
//!   compared by value.
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
        $($field_name:ident : $field_type:ty),* $(,)?
    }) => {
        #[derive(Debug, Clone, ::serde::Serialize)]
        pub struct $name {
            id: $id_type,
            $($field_name : $field_type),*
        }

        #[derive(Debug, Clone)]
        pub struct $input {
            $(pub $field_name : $field_type),*
        }

        impl $name {
            pub fn create(input: $input) -> Result<Self, $crate::kernel::entities::identification::IdentityError> {
                let $input { $($field_name),* } = input;

                let mut hasher = <::sha2::Sha256 as ::sha2::Digest>::new();

                $(
                    ::sha2::Digest::update(
                        &mut hasher,
                        &::postcard::to_allocvec(&$field_name).map_err(
                            |_| $crate::kernel::entities::identification::IdentityError::Serialization,
                        )?,
                    );
                )*

                let hash_result = ::sha2::Digest::finalize(hasher);
                let mut id_bytes = [0u8; 32];
                id_bytes.copy_from_slice(&hash_result);

                Ok(Self {
                    id: <$id_type>::from(id_bytes),
                    $($field_name),*
                })
            }

            pub fn id(&self) -> $id_type {
                self.id
            }

            $(
                pub fn $field_name(&self) -> &$field_type {
                    &self.$field_name
                }
            )*
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
