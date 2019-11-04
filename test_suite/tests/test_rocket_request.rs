#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{Header, Status};
use rocket::local::Client;
use rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
use rocket_jsonapi::request::JsonApiGetRequest;
use rocket_jsonapi::response::JsonApiResponse;
use rocket_jsonapi::{json_api_error, Linkify, ResourceIdentifiable};
use serde::Serialize;
use serde_json::{from_str, json, Value};

#[derive(Serialize, ResourceIdentifiable, Linkify)]
struct Test {
    id: i32,
    message: String,
}

#[get("/simple")]
fn simple(req: JsonApiGetRequest) -> JsonApiResponse<Test> {
    JsonApiResponse(Ok(Test {
        id: 1,
        message: String::from("Hello!"),
    }))
}

#[test]
fn test_request_simple_ok() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get("/simple");
    request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
    request.add_header(Header::new("Accept", "application/vnd.api+json"));
    let response = request.dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::Ok);
    // Test header response
    let headers = response.headers();
    assert_eq!(
        headers.get_one("Content-Type").unwrap(),
        "application/vnd.api+json"
    );
}

#[test]
fn test_request_missing_accept_header() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get("/simple");
    request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
    let response = request.dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::NotAcceptable);
}

#[test]
fn test_request_accept_header_with_params_406() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get("/simple");
    request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
    request.add_header(Header::new("Accept", "application/vnd.api+json; arg=val"));
    let response = request.dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::NotAcceptable);
}

#[test]
fn test_request_missing_content_type_header() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get("/simple");
    request.add_header(Header::new("Accept", "application/vnd.api+json"));
    let response = request.dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::UnsupportedMediaType);
}

#[test]
fn test_request_content_type_header_with_params_415() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut request = client.get("/simple");
    request.add_header(Header::new(
        "Content-Type",
        "application/vnd.api+json; chartset=UTF-8",
    ));
    request.add_header(Header::new("Accept", "application/vnd.api+json"));
    let response = request.dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::UnsupportedMediaType);
}
