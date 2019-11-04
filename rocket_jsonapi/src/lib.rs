#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]

//! # JSON:API + Rocket.rs
//!
//! This library helps provide JSON:API compliant API's through Rocket.rs in a simple way. The
//! library provides:
//!
//!  - A [RequestGuard](https://rocket.rs/v0.4/guide/requests/#request-guards) and
//!     [DataGuard](https://api.rocket.rs/v0.4/rocket/data/trait.FromData.html#data-guards) for
//!     verifying and deserializing incoming requests
//!  - A [Responder](https://rocket.rs/v0.4/guide/responses/#responder) for serializing correctly,
//!     with all metadata added
//!  - A set of traits implemented on data, to provide metadata like `links` or `relationships`
//!     like the specification allows.
//!  - A set of macros and derive macros to reduce boilerplate
mod core;

mod lib {
    pub use crate::links::*;
    pub use crate::response::ResourceIdentifiable;
    pub use serde::ser::{SerializeSeq, SerializeStruct};
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    // Possible to include core modules if we want them globally
}

pub mod info;
pub mod response;
pub use response::ResourceIdentifiable;
pub mod request;

// Exposing Linkify on root level path, because macros can only be on root level
pub mod links;
pub use links::Linkify;

pub mod relationship;

pub mod error;

#[cfg(feature = "rocket_jsonapi_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate rocket_jsonapi_derive;
#[cfg(feature = "rocket_jsonapi_derive")]
#[doc(hidden)]
pub use rocket_jsonapi_derive::*;
