#![feature(specialization)]
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
