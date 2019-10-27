use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::JsonApiResponseError;
use crate::lib::*;

/// Trait implemented on data objects so they can be parsed as resource objects. [See
/// specification](https://jsonapi.org/format/#document-resource-objects). For this very reason it
/// is required that this trait is implemented on data returned as a `JsonApiResponse`.
///
/// To add `relationships` and `links` to the resource object, `Linkify` and `AllRelationships`
/// needs to be implemented on the same data object as `ResourceIdentifiable`. Please see the
/// documentation for `Linkify` and `AllRelationships` for further documentation.
pub trait ResourceIdentifiable {
    type IdType: ToString;

    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> &Self::IdType;
}

/// Return type for a Rocket.rs route that responds with a JSON:API response. This object
/// serializes into a top-level document, with correct HTTP conventions.
///
/// [See top-level document specification](https://jsonapi.org/format/#document-top-level).
///
/// [See HTTP convention specification](https://jsonapi.org/format/#content-negotiation-servers).
pub struct JsonApiResponse<Data>(pub Result<Data, JsonApiResponseError>);

impl<Data> Serialize for JsonApiResponse<Data>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&err),
        }
        // TODO handle json_api field
    }
}

impl<Data> Serialize for JsonApiResponse<Vec<Data>>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&err),
        }
        // TODO handle json_api field
    }
}
