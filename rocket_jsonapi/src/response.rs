//! # Returning valid JSON:API responses
//!
//! This module exports types for responding with JSON:API compliant responses.
//!
//! This excludes the types for metadata like `links` and `relationship`.
use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::JsonApiResponseError;
use crate::lib::*;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde_json::to_string as serialize;
use std::io::Cursor;

pub trait ResourceType {
    /// Returns the resource type
    fn get_type(&self) -> &'static str;
}

pub trait ResourceIdentifiable: ResourceType {
    /// Trait implemented on data objects so they can be parsed as resource objects.
    ///
    /// [See specification](https://jsonapi.org/format/#document-resource-objects). For this very reason
    /// it is required that this trait is implemented on data returned from a `JsonApiResponse`.
    ///
    /// The trait requires the [ResourceType] to be implemented, because a resource object requires
    /// a type.
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
    /// # use rocket_jsonapi::{ResourceType, ResourceIdentifiable};
    /// #
    /// #[derive(ResourceType, ResourceIdentifiable)]
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
    /// # use rocket_jsonapi::{ResourceType, ResourceIdentifiable};
    /// #
    /// #[derive(ResourceType, ResourceIdentifiable)]
    /// #[resource_ident_id = "author_name"]
    /// #[resource_ident_type = "Chapter"]
    /// struct Article { // "Chapter" is returned by get_type()
    ///     id: i32,
    ///     author_name: String, // author_name field is returned by derived get_id()
    ///     text: String
    /// }
    /// ```

    /// The type of the id returned by `get_id(&self)`, must implement ToString, because the
    /// specification states resource ids must be strings
    type IdType: ToString;

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
/// To return data of type `Data`, your return type should be: `JsonApiResponse<Data>`. `Data` must
/// implement `serde::Serialize`, `rocket_jsonapi::ResourceIdentifiable` and
/// `rocket_jsonapi::Linkify`.
///
/// `JsonApiResponse` is a wrapper of `Result<Data,JsonApiResponseError>`, so constructing
/// `JsonApiResponse` is as simple as `JsonApiResponse(Ok(data))`.
///
/// # Example
///
/// Here is a simple example:
///
/// ```rust
/// # #![feature(decl_macro)]
/// # #[macro_use]
/// # use rocket::*;
/// # use crate::rocket_jsonapi::response::JsonApiResponse;
/// # use crate::rocket_jsonapi::{Linkify, ResourceType, ResourceIdentifiable};
/// # use serde::Serialize;
/// #
/// #[derive(Serialize, ResourceType, ResourceIdentifiable, Linkify)]
/// struct Test {
///    id: i32,
///    message: String,
/// }
///
/// #[get("/simple")]
/// fn simple() -> JsonApiResponse<Test> {
///     JsonApiResponse(Ok(Test {
///         id: 1,
///         message: String::from("Hello!"),
///     }))
/// }
/// ```
///
/// Which outputs:
///
/// ```ignore
/// Status code: 200, Content-Type: application/vnd.api+json
/// ---
/// {
///     "data": {
///         "id": "1",
///         "type": "Test",
///         "attributes": {
///             "id": 5,
///             "message": "Hello!"
///         }
///     }
/// }
/// ```
///
/// ## Errors
///
/// When returning errors, you must construct an instance of `JsonApiResponseError`, and construct
/// it like this: `JsonApiResponse(Err(error))`. It is your responsibility to choose the appropriate
/// http status code for the error. See `JsonApiResponseError` for more.
///
/// Example:
/// ```rust
/// # #![feature(decl_macro)]
/// # #[macro_use]
/// # use rocket::{get};
/// # use rocket::http::Status;
/// # use crate::rocket_jsonapi::response::JsonApiResponse;
/// # use crate::rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
/// # use crate::rocket_jsonapi::{json_api_error, Linkify, ResourceIdentifiable, ResourceType};
/// # use serde::Serialize;
/// #[derive(Serialize, ResourceType, ResourceIdentifiable, Linkify)]
/// struct Test {
///    id: i32,
///    message: String,
/// }
///
/// #[get("/simple_error")]
/// fn simple_error() -> JsonApiResponse<Test> {
///     JsonApiResponse(Err(JsonApiResponseError::new(
///         Status::BadRequest,
///         vec![json_api_error!(
///             id = String::from("5"),
///             status = String::from("406")
///         )],
///     )))
/// }
/// ```
///
/// Which outputs:
///
/// ```ignore
/// Status code: 400, Content-Type: application/vnd.api+json
/// ---
/// {
///     "errors": [{
///         "id": "5",
///         "status": "406"
///     }]
/// }
/// ```
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
    fn respond_to(self, _request: &Request<'_>) -> Result<Response<'r>, Status> {
        // TODO improve or think about what to do in this case...
        let response = serialize(&self).map_err(|_e| Status::InternalServerError)?;

        match self.0 {
            Ok(_data) => Ok(Response::build()
                .header(ContentType::JsonApi)
                .sized_body(Cursor::new(response))
                .finalize()),
            Err(error) => {
                let response = Response::build()
                    .header(ContentType::JsonApi)
                    .status(error.get_error_code())
                    .sized_body(Cursor::new(response))
                    .finalize();
                Ok(response)
            }
        }
    }
}
