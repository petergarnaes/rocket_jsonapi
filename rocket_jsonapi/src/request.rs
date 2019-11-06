//! # Validating JSON:API requests
use crate::lib::*;
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::Data;
use rocket::Request;

pub struct JsonApiGetRequest;
pub struct JsonApiDataRequest<Data>(Data);

fn acceptable_json_api_request(request: &Request) -> Result<(), Status> {
    let content_type_media_type_test = request
        .content_type()
        .filter(|content_type| {
            content_type.0.top() == "application"
                && content_type.0.sub() == "vnd.api+json"
                && content_type.0.params().count() == 0
        })
        .is_some();
    let accept_test = request
        .accept()
        .iter()
        .flat_map(|accept| accept.iter())
        .any(|query_media_type| {
            query_media_type.top() == "application"
                && query_media_type.sub() == "vnd.api+json"
                && query_media_type.0.params().count() == 0
        });
    if !content_type_media_type_test {
        // Specification states only what the status code must be, user must descide themselves
        // via rocket error catcher what to respond with
        return Err(Status::UnsupportedMediaType);
    }
    if !accept_test {
        // Specification states only what the status code must be, user must descide themselves
        // via rocket error catcher what to respond with
        return Err(Status::NotAcceptable);
    }
    Ok(())
}

impl<'a, 'r> FromRequest<'a, 'r> for JsonApiGetRequest {
    // TODO right error?
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<JsonApiGetRequest, Self::Error> {
        match acceptable_json_api_request(request) {
            Ok(()) => request::Outcome::Success(JsonApiGetRequest),
            Err(status) => request::Outcome::Failure((status, ())),
        }
    }
}

impl<InputData> FromDataSimple for JsonApiDataRequest<InputData>
where
    for<'de> InputData: Deserialize<'de>,
{
    // TODO right error?
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match acceptable_json_api_request(request) {
            Ok(()) => {
                // TODO does reader interface open up for DoS attacks
                match serde_json::from_reader(data.open()) {
                    Ok(result) => data::Outcome::Success(JsonApiDataRequest(result)),
                    Err(_err) => data::Outcome::Failure((Status::BadRequest, ())),
                }
            }
            Err(status) => data::Outcome::Failure((status, ())),
        }
    }
}
