#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::local::Client;
use rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
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
fn simple() -> JsonApiResponse<Test> {
    JsonApiResponse(Ok(Test {
        id: 1,
        message: String::from("Hello!"),
    }))
}

#[get("/simple_error")]
fn simple_error() -> JsonApiResponse<Test> {
    JsonApiResponse(Err(JsonApiResponseError(
        Status::NotAcceptable,
        vec![json_api_error!(
            id = String::from("5"),
            status = String::from("406")
        )],
    )))
}

#[test]
fn rocket_simple_ok_response() {
    let rocket = rocket::ignite().mount("/", routes![simple]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/simple").dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::Ok);
    // Test header response
    let headers = response.headers();
    assert_eq!(
        headers.get_one("Content-Type").unwrap(),
        "application/vnd.api+json"
    );
    // Test body response
    let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
    let expected_json = json!({
        "data": {
            "id": "1",
            "type": "Test",
            "attributes": {
                "id": 1,
                "message": "Hello!"
            }
        }
    });
    assert_eq!(requested_json, expected_json);
}

#[test]
fn rocket_simple_error_response() {
    let rocket = rocket::ignite().mount("/", routes![simple_error]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/simple_error").dispatch();
    // Test HTTP status code
    assert_eq!(response.status(), Status::NotAcceptable);
    // Test header response
    let headers = response.headers();
    assert_eq!(
        headers.get_one("Content-Type").unwrap(),
        "application/vnd.api+json"
    );
    // Test body response
    let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
    let expected_json = json!({
        "errors": [{
            "id": "5",
            "status": "406",
        }]
    });
    assert_eq!(requested_json, expected_json);
}
