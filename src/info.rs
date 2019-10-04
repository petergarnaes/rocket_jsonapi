use crate::lib::*;

// Print jsonapi field
pub trait JsonApi {
    type Meta: Serialize;
    fn get_json_api_field(&self) -> &Self::Meta;
}
