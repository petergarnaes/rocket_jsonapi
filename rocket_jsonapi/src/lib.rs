#![feature(specialization)]
#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]
mod core;

mod lib {
    pub use crate::data::*;
    pub use crate::links::*;
    pub use serde::ser::{SerializeSeq, SerializeStruct};
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    // Possible to include core modules if we want them globally
}

pub mod info;
pub mod response;

// Exposing Linkify on root level path, because macros can only be on root level
pub mod links;
pub use links::Linkify;

pub mod data;
pub use data::ResourceIdentifiable;
pub mod relationship;

// Der skal filosoferes over hvordan vi med statisk opbygning kan lave en dynamisk data struktur.
// Måske virkelig abuse dyn Trait
// F.eks. hvordan printer vi et eller flere MAY felter på en struktur? Ideelt set skal man bare
// implementere nogle simple traits, men specialization er ikke god nok til at man kan implementere
// alle permutationer af traits, kun kombinationer som set uden kontekst aldrig kan clashe kan
// fungere sammen.
// Hvis brugeren selv skal opbygge, kunne det måske gøres med builder pattern? https://doc.rust-lang.org/1.8.0/std/fs/struct.DirBuilder.html
// https://www.reddit.com/r/rust/comments/4jgvho/idiomatic_way_to_implement_optional_arguments/

// Måske kan lidt trait implementationer sammen med noget macro magi lave klisteret?

#[cfg(feature = "rocket_jsonapi_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate rocket_jsonapi_derive;
#[cfg(feature = "rocket_jsonapi_derive")]
#[doc(hidden)]
pub use rocket_jsonapi_derive::*;
