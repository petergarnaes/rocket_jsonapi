//! # Validating JSON:API requests
use crate::core::input_data::{JsonApiCreateResource, JsonApiUpdateResource};
use crate::lib::*;
use crate::resource::ResourceType;
use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, MediaType, Status};
use rocket::request::{self, FromRequest};
use rocket::Data;
use rocket::Request;
use serde::export::PhantomData;
use serde_json::error::Category;
use serde_json::{Map, Value};

pub struct JsonApiRequest;
// TODO add Option<ClientId>? How do we help users return a 403 if unsupported? Make enum?
pub struct JsonApiCreateRequest<Data>(pub Data);
pub struct JsonApiUpdateRequest<Data> {
    pub id: String,
    pub attributes: Map<String, Value>,
    phantom: PhantomData<Data>,
}

fn acceptable_json_api_content_type(request: &Request) -> Result<(), Status> {
    // JSON API v. 1.0
    // Servers MUST respond with a 415 Unsupported Media Type status code if a request specifies
    // the header Content-Type: application/vnd.api+json with any media type parameters.
    let content_type_media_type_test = request
        .content_type()
        .filter(|content_type| {
            (**content_type) == ContentType::JsonApi && content_type.0.params().count() == 0
        })
        .is_some();
    if !content_type_media_type_test {
        // Specification states only what the status code must be, user must decide themselves
        // via rocket error catcher what to respond with
        return Err(Status::UnsupportedMediaType);
    }
    Ok(())
}

fn acceptable_json_api_accept(request: &Request) -> Result<(), Status> {
    // JSON API v. 1.0
    // Servers MUST respond with a 406 Not Acceptable status code if a request’s Accept header
    // contains the JSON:API media type and all instances of that media type are modified with
    // media type parameters.
    let accept_test = request
        .accept()
        .iter()
        .flat_map(|accept| accept.iter())
        .any(|query_media_type| {
            query_media_type.0 == MediaType::JsonApi && query_media_type.0.params().count() == 0
        });
    if !accept_test {
        // Specification states only what the status code must be, user must decide themselves
        // via rocket error catcher what to respond with
        return Err(Status::NotAcceptable);
    }
    Ok(())
}

fn acceptable_json_api_data_request(request: &Request) -> Result<(), Status> {
    acceptable_json_api_accept(request)?;
    acceptable_json_api_content_type(request)
}

impl<'a, 'r> FromRequest<'a, 'r> for JsonApiRequest {
    // TODO right error?
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<JsonApiRequest, Self::Error> {
        match acceptable_json_api_accept(request) {
            Ok(()) => request::Outcome::Success(JsonApiRequest),
            Err(status) => request::Outcome::Failure((status, ())),
        }
    }
}

impl<InputData> FromDataSimple for JsonApiCreateRequest<InputData>
where
    for<'de> InputData: ResourceType + Deserialize<'de>,
{
    // TODO right error?
    // A server SHOULD include error details and provide enough information to recognize the source
    // of the conflict.
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match acceptable_json_api_data_request(request) {
            Ok(()) => {
                // TODO does reader interface open up for DoS attacks
                let b: Result<JsonApiCreateResource<InputData>, serde_json::error::Error> =
                    serde_json::from_reader(data.open());
                match b {
                    Ok(result) => data::Outcome::Success(JsonApiCreateRequest(result.data.0)),
                    Err(err) => match err.classify() {
                        // A server MUST return 409 Conflict when processing a POST request in which
                        // the resource object’s type is not among the type(s) that constitute the
                        // collection represented by the endpoint.
                        Category::Data => data::Outcome::Failure((Status::Conflict, ())),
                        _ => data::Outcome::Failure((Status::BadRequest, ())),
                    },
                }
            }
            Err(status) => data::Outcome::Failure((status, ())),
        }
    }
}

impl<InputData> FromDataSimple for JsonApiUpdateRequest<InputData>
where
    for<'de> InputData: ResourceType + Deserialize<'de>,
{
    // TODO right error?
    // A server SHOULD include error details and provide enough information to recognize the source
    // of the conflict.
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match acceptable_json_api_data_request(request) {
            Ok(()) => {
                // TODO does reader interface open up for DoS attacks
                let b: Result<JsonApiUpdateResource, serde_json::error::Error> =
                    serde_json::from_reader(data.open());
                match b {
                    Ok(result) => {
                        if result.data.resource_type != InputData::get_type() {
                            return data::Outcome::Failure((Status::Conflict, ()));
                        }
                        data::Outcome::Success(JsonApiUpdateRequest::<InputData> {
                            id: result.data.id,
                            attributes: result.data.attributes,
                            phantom: PhantomData,
                        })
                    }
                    Err(err) => match err.classify() {
                        // Specification: A server MUST return 409 Conflict when processing a PATCH
                        // request in which the resource object’s type and id do not match the
                        // server’s endpoint.
                        Category::Data => data::Outcome::Failure((Status::Conflict, ())),
                        _ => data::Outcome::Failure((Status::BadRequest, ())),
                    },
                }
            }
            Err(status) => data::Outcome::Failure((status, ())),
        }
    }
}
