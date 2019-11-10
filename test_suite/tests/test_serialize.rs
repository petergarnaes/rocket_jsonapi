#![allow(dead_code)]
// Test that the various parts serialize properly
use rocket::http::Status;
use rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
use rocket_jsonapi::response::JsonApiDataResponse;
use rocket_jsonapi::{json_api_error, Linkify, ResourceIdentifiable, ResourceType};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Test {
    id: i32,
    message: String,
}

impl ResourceType for Test {
    fn get_type(&self) -> &'static str {
        &"Test"
    }
}

impl ResourceIdentifiable for Test {
    type IdType = i32;

    fn get_id(&self) -> &Self::IdType {
        &self.id
    }
}

impl Linkify for Test {}

#[test]
fn serialize_json_api_response() {
    let test_instance = Test {
        id: 5,
        message: "Hello".to_string(),
    };
    let test_instance_value =
        serde_json::to_value(JsonApiDataResponse::<Test>(Ok(test_instance))).unwrap();
    let test_equals_value = json!({
        "data": {
            "id": "5",
            "type": "Test",
            "attributes": {
                "id": 5,
                "message": "Hello"
            }
        }
    });
    assert_eq!(test_instance_value, test_equals_value);
}

#[test]
fn serialize_json_api_response_array() {
    let test_instance1 = Test {
        id: 5,
        message: "Hello".to_string(),
    };
    let test_instance2 = Test {
        id: 6,
        message: "World".to_string(),
    };
    let test_instance_value = serde_json::to_value(JsonApiDataResponse::<Vec<Test>>(Ok(vec![
        test_instance1,
        test_instance2,
    ])))
    .unwrap();
    let test_equals_value = json!({
        "data": [{
            "id": "5",
            "type": "Test",
            "attributes": {
                "id": 5,
                "message": "Hello"
            }
        },{
            "id": "6",
            "type": "Test",
            "attributes": {
                "id": 6,
                "message": "World"
            }
        }]
    });
    assert_eq!(test_instance_value, test_equals_value);
}

#[test]
fn serialize_json_api_response_error() {
    let test_error1 = json_api_error!(
        title = String::from("Super error"),
        code = String::from("15")
    );
    let test_error2 = json_api_error!(
        title = String::from("Medium error"),
        code = String::from("17")
    );
    let test_instance_value = serde_json::to_value(JsonApiDataResponse::<Vec<Test>>(Err(
        JsonApiResponseError::new(Status::BadRequest, vec![test_error1, test_error2]),
    )))
    .unwrap();
    let test_equals_value = json!({
        "errors": [{
            "title": "Super error",
            "code": "15"
        },{
            "title": "Medium error",
            "code": "17"
        }]
    });
    assert_eq!(test_instance_value, test_equals_value);
}

#[test]
fn test_serialize_as_vec_of_errors() {
    let test_errors = vec![
        JsonApiError {
            id: Some(String::from("1")),
            title: Some(String::from("I like turtles")),
            ..Default::default()
        },
        JsonApiError {
            id: Some(String::from("2")),
            status: Some(String::from("400")),
            ..Default::default()
        },
    ];
    let test_instance_value = serde_json::to_value(test_errors).unwrap();
    let test_equals_value = json!([{
        "id": "1",
        "title": "I like turtles",
    },{
        "id": "2",
        "status": "400",
    }]);
    assert_eq!(test_instance_value, test_equals_value);
}
