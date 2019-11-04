use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::JsonApiResponseError;
use crate::lib::*;
use rocket::http::{ContentType, MediaType, Status};
use rocket::response::{Content, Responder};
use rocket::{Request, Response};
use serde_json::to_string as serialize;

/// Trait implemented on data objects so they can be parsed as resource objects. [See
/// specification](https://jsonapi.org/format/#document-resource-objects). For this very reason it
/// is required that this trait is implemented on data returned from a `JsonApiResponse`.
///
/// ### Using `#[derive(ResourceIdentifiable)]`
///
/// Import the derive macro:
/// ```rust
/// use rocket_jsonapi::ResourceIdentifiable;
/// ```
/// When derived, it defaults to using the field named `id` on the implementing `struct`.
/// The `type` defaults to the name of the `struct`. Example:
/// ```rust
/// # use rocket_jsonapi::ResourceIdentifiable;
/// #
/// #[derive(ResourceIdentifiable)]
/// struct Article { // "Article" is returned by get_type()
///     id: i32, // id field is returned by derived get_id()
///     author_name: String,
///     text: String
/// }
/// ```
///
/// #### Customizing `#[derive(ResourceIdentifiable)]` behaviour
///
/// Both `id` and `type` can be changed when deriving.
///
/// `#[resource_ident_id = "id_field"]` changes the field that functions as the `id`.
///
/// `#[resource_ident_type = "CustomType"]` changes the `type`.
///
/// Example:
/// ```rust
/// # use rocket_jsonapi::ResourceIdentifiable;
/// #
/// #[derive(ResourceIdentifiable)]
/// #[resource_ident_id = "author_name"]
/// #[resource_ident_type = "Chapter"]
/// struct Article { // "Chapter" is returned by get_type()
///     id: i32,
///     author_name: String, // author_name field is returned by derived get_id()
///     text: String
/// }
/// ```
pub trait ResourceIdentifiable {
    /// The type of the id returned by `get_id(&self)`, must implement ToString, because the
    /// specification states resource ids must be strings
    type IdType: ToString;

    /// Returns the resource type
    fn get_type(&self) -> &'static str;
    /// Returns the resource id
    fn get_id(&self) -> &Self::IdType;
}

/// # JSON:API Responder
///
/// Responder for Rocket.rs route that responds with a JSON:API response. This object
/// serializes into a top-level document, with correct HTTP conventions.
///
/// [See top-level document specification](https://jsonapi.org/format/#document-top-level).
///
/// [See HTTP convention specification](https://jsonapi.org/format/#content-negotiation-servers).
///
/// ## Usage
///
/// ## Example
///
/// ## Errors
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

impl<'r, Data> Responder<'r> for JsonApiResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        let json_api_mt = MediaType::new("application", "vnd.api+json");
        // TODO improve or think about what to do in this case...
        let response = serialize(&self).map_err(|_e| Status::InternalServerError)?;
        match self.0 {
            Ok(_data) => Content(ContentType(json_api_mt), response).respond_to(request),
            Err(error) => {
                let mut response =
                    Content(ContentType(json_api_mt), response).respond_to(request)?;
                response.set_status(error.0);
                Ok(response)
            }
        }
    }
}
