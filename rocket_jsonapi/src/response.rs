//! # Returning valid JSON:API responses
//!
//! This module exports types for responding with JSON:API compliant responses.
//!
//! This excludes the types for metadata like `links` and `relationship`.
use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::core::general_response::JsonApiResponse;
use crate::error::{JsonApiError, JsonApiResponseError};
use crate::lib::*;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};

pub struct JsonApiCollection<Data>(pub Vec<Data>, pub Vec<Link>);

impl<Data> JsonApiCollection<Data> {
    pub fn data(vec: Vec<Data>) -> Self {
        JsonApiCollection(vec, vec![])
    }
    pub fn data_w_links(links: Vec<Link>, vec: Vec<Data>) -> Self {
        JsonApiCollection(vec, links)
    }
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
/// To return data of type `Data`, your return type should be: `JsonApiDataResponse<Data>`. `Data` must
/// implement `serde::Serialize`, `rocket_jsonapi::ResourceIdentifiable` and
/// `rocket_jsonapi::Linkify`.
///
/// `JsonApiDataResponse` is a wrapper of `Result<Data,JsonApiResponseError>`, so constructing
/// `JsonApiDataResponse` is as simple as `JsonApiDataResponse(Ok(data))`.
///
/// # Example
///
/// Here is a simple example:
///
/// ```rust
/// # #![feature(decl_macro)]
/// # #[macro_use]
/// # use rocket::*;
/// # use crate::rocket_jsonapi::response::JsonApiDataResponse;
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
/// fn simple() -> JsonApiDataResponse<Test> {
///     JsonApiDataResponse(Ok(Test {
///         id: 1,
///         message: String::from("Hello!"),
///     }))
/// }
/// ```
///
/// Which outputs:
///
/// ```text
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
/// it like this: `JsonApiDataResponse(Err(error))`. It is your responsibility to choose the appropriate
/// http status code for the error. See `JsonApiResponseError` for more.
///
/// Example:
/// ```rust
/// # #![feature(decl_macro)]
/// # #[macro_use]
/// # use rocket::get;
/// # use rocket::http::Status;
/// # use crate::rocket_jsonapi::response::JsonApiDataResponse;
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
/// fn simple_error() -> JsonApiDataResponse<Test> {
///     JsonApiDataResponse(Err(JsonApiResponseError::new(
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
/// ```text
/// Status code: 400, Content-Type: application/vnd.api+json
/// ---
/// {
///     "errors": [{
///         "id": "5",
///         "status": "406"
///     }]
/// }
/// ```
pub struct JsonApiDataResponse<Data>(pub Result<Data, JsonApiResponseError>);

impl<'r, Data> Responder<'r> for JsonApiDataResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        let general_response: JsonApiResponse<Data> = self.into();
        general_response.respond_to(request)
    }
}

impl<'r, Data> Responder<'r> for JsonApiDataResponse<JsonApiCollection<Data>>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        let general_response: JsonApiResponse<JsonApiCollection<Data>> = self.into();
        general_response.respond_to(request)
    }
}

impl<Data> From<JsonApiDataResponse<Data>> for JsonApiResponse<Data> {
    fn from(data_response: JsonApiDataResponse<Data>) -> Self {
        match data_response.0 {
            Ok(data) => JsonApiResponse(Status::Ok, Ok(data)),
            Err(err) => JsonApiResponse::<Data>(err.0, Err(err.1)),
        }
    }
}

pub enum JsonApiCreateResponse<Data> {
    /// Data is accepted and created, [spec](https://jsonapi.org/format/#crud-creating-responses-201)
    Created(Data),
    /// Used when data is accepted, but maybe needs asynchronous processing and is not created yet,
    /// [spec](https://jsonapi.org/format/#crud-creating-responses-202)
    Accepted(Data),
    /// Used when responding as a 201 create, but with no returned data.
    /// [spec](https://jsonapi.org/format/#crud-creating-responses-204)
    NoContent,
    UnsupportedClientId(Option<Vec<JsonApiError>>),
    Forbidden(Option<Vec<JsonApiError>>),
    NotFound(Option<Vec<JsonApiError>>),
    AlreadyExists(Option<Vec<JsonApiError>>),
    /// Specification says you can respond with any status you want,
    /// [spec](https://jsonapi.org/format/#crud-creating-responses-other)
    Other(Status, Result<Data, Vec<JsonApiError>>),
}

impl<'r, Data> Responder<'r> for JsonApiCreateResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        match self {
            JsonApiCreateResponse::Created(data) => {
                JsonApiResponse(Status::Created, Ok(data)).respond_to(request)
            }
            JsonApiCreateResponse::Accepted(data) => {
                JsonApiResponse(Status::Accepted, Ok(data)).respond_to(request)
            }
            JsonApiCreateResponse::NoContent => Ok(Response::build()
                .header(ContentType::JsonApi)
                .status(Status::NoContent)
                .finalize()),
            JsonApiCreateResponse::UnsupportedClientId(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::Forbidden, Err(err)).respond_to(request)
            }
            JsonApiCreateResponse::Forbidden(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::Forbidden, Err(err)).respond_to(request)
            }
            JsonApiCreateResponse::NotFound(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::NotFound, Err(err)).respond_to(request)
            }
            JsonApiCreateResponse::AlreadyExists(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::Conflict, Err(err)).respond_to(request)
            }
            JsonApiCreateResponse::Other(status, res) => {
                JsonApiResponse(status, res).respond_to(request)
            }
        }
    }
}

pub enum JsonApiUpdateResponse<Data> {
    /// Data is accepted and updated, [spec](https://jsonapi.org/format/#crud-updating-responses-200)
    Updated(Data),
    /// Used when data is accepted, but maybe needs asynchronous processing and is not created yet,
    /// [spec](https://jsonapi.org/format/#crud-updating-responses-202)
    Accepted(Data),
    /// Used when responding as a 200 ok, but with no returned data.
    /// [spec](https://jsonapi.org/format/#crud-updating-responses-204)
    NoContent,
    Forbidden(Option<Vec<JsonApiError>>),
    NotFound(Option<Vec<JsonApiError>>),
    // TODO what to name this case?
    InvalidUpdate(Option<Vec<JsonApiError>>),
    //Conflict(Option<Vec<JsonApiError>>),
    /// Specification says you can respond with any status you want,
    /// [spec](https://jsonapi.org/format/#crud-updating-responses-other)
    Other(Status, Result<Data, Vec<JsonApiError>>),
}

impl<'r, Data> Responder<'r> for JsonApiUpdateResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        match self {
            JsonApiUpdateResponse::Updated(data) => {
                JsonApiResponse(Status::Ok, Ok(data)).respond_to(request)
            }
            JsonApiUpdateResponse::Accepted(data) => {
                JsonApiResponse(Status::Accepted, Ok(data)).respond_to(request)
            }
            JsonApiUpdateResponse::NoContent => Ok(Response::build()
                .header(ContentType::JsonApi)
                .status(Status::NoContent)
                .finalize()),
            JsonApiUpdateResponse::Forbidden(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::Forbidden, Err(err)).respond_to(request)
            }
            JsonApiUpdateResponse::NotFound(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::NotFound, Err(err)).respond_to(request)
            }
            JsonApiUpdateResponse::InvalidUpdate(error) => {
                let err = match error {
                    Some(errors) => errors,
                    None => vec![],
                };
                JsonApiResponse::<Data>(Status::Conflict, Err(err)).respond_to(request)
            }
            JsonApiUpdateResponse::Other(status, res) => {
                JsonApiResponse(status, res).respond_to(request)
            }
        }
    }
}
