#![feature(specialization)]
#![feature(associated_type_defaults)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod core;

mod lib {
    pub use serde::{Serialize, Serializer, Deserialize, Deserializer};
    pub use serde::ser::{SerializeStruct};
    // Possible to include core modules if we want them globally
}

pub mod info;
pub mod response;
pub mod links;
pub mod data;
pub mod relationship;
pub mod meta;

// Der skal filosoferes over hvordan vi med statisk opbygning kan lave en dynamisk data struktur.
// Måske virkelig abuse dyn Trait
// F.eks. hvordan printer vi et eller flere MAY felter på en struktur? Ideelt set skal man bare
// implementere nogle simple traits, men specialization er ikke god nok til at man kan implementere
// alle permutationer af traits, kun kombinationer som set uden kontekst aldrig kan clashe kan
// fungere sammen.
// Hvis brugeren selv skal opbygge, kunne det måske gøres med builder pattern? https://doc.rust-lang.org/1.8.0/std/fs/struct.DirBuilder.html
// https://www.reddit.com/r/rust/comments/4jgvho/idiomatic_way_to_implement_optional_arguments/

// Måske kan lidt trait implementationer sammen med noget macro magi lave klisteret?
