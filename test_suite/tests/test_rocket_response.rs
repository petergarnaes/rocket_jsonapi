#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_jsonapi::{Linkify, ResourceIdentifiable, ResourceType};
use serde::Serialize;

#[derive(Serialize, ResourceType, ResourceIdentifiable, Linkify)]
struct Test {
    id: i32,
    message: String,
}

mod test_output_data_response {
    use crate::Test;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
    use rocket_jsonapi::json_api_error;
    use rocket_jsonapi::response::JsonApiDataResponse;
    use serde_json::{from_str, json, Value};

    #[get("/simple")]
    fn simple() -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(Test {
            id: 1,
            message: String::from("Hello!"),
        }))
    }

    #[get("/simple_list")]
    fn simple_list() -> JsonApiDataResponse<Vec<Test>> {
        JsonApiDataResponse(Ok(vec![
            Test {
                id: 1,
                message: String::from("Hello!"),
            },
            Test {
                id: 2,
                message: String::from("Hay!"),
            },
        ]))
    }

    #[get("/simple_error")]
    fn simple_error() -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Err(JsonApiResponseError::new(
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
    fn rocket_simple_ok_list_response() {
        let rocket = rocket::ignite().mount("/", routes![simple_list]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple_list").dispatch();
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
            "data": [{
                "id": "1",
                "type": "Test",
                "attributes": {
                    "id": 1,
                    "message": "Hello!"
                }
            }, {
                "id": "2",
                "type": "Test",
                "attributes": {
                    "id": 2,
                    "message": "Hay!"
                }
            }]
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
}

mod test_output_data_response_links {
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket_jsonapi::links::{Link, LinkObject};
    use rocket_jsonapi::response::JsonApiDataResponse;
    use rocket_jsonapi::{Linkify, ResourceIdentifiable, ResourceType};
    use serde::Serialize;
    use serde_json::{from_str, json, Value};

    #[derive(Serialize, ResourceType, ResourceIdentifiable)]
    struct TestWithLinks {
        id: u64,
        message: String,
    }

    #[derive(Serialize)]
    struct Meta {
        stuff: String,
    }

    impl Linkify for TestWithLinks {
        fn get_links() -> Vec<Link> {
            vec![
                Link::Url(
                    "self",
                    String::from("http://fake.com/api/test_with_links/1"),
                ),
                Link::Object(
                    "something",
                    LinkObject {
                        href: String::from("http://fake.com/api/test_with_links/1/something"),
                        meta: Box::new(Meta {
                            stuff: String::from("stuff"),
                        }),
                    },
                ),
            ]
        }
    }

    #[get("/simple_links")]
    fn simple() -> JsonApiDataResponse<TestWithLinks> {
        JsonApiDataResponse(Ok(TestWithLinks {
            id: 1,
            message: String::from("Hello!"),
        }))
    }

    #[test]
    fn rocket_simple_ok_response() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple_links").dispatch();
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
                "type": "TestWithLinks",
                "attributes": {
                    "id": 1,
                    "message": "Hello!"
                }
            },
            "links": {
                "self": "http://fake.com/api/test_with_links/1",
                "something": {
                    "href": "http://fake.com/api/test_with_links/1/something",
                    "meta": {
                        "stuff": "stuff"
                    }
                }
            }
        });
        assert_eq!(requested_json, expected_json);
    }
}

mod test_create_response {
    use crate::Test;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::request::FromFormValue;
    use rocket_jsonapi::response::JsonApiCreateResponse;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, json, Value};

    #[derive(Serialize, Deserialize, FromFormValue)]
    enum CreateResponseTrigger {
        Created,
        Accepted,
        NoContent,
        UnsupportedClientId,
        Forbidden,
        NotFound,
        AlreadyExists,
        Other,
    }

    #[get("/simple?<trigger>")]
    fn simple(trigger: CreateResponseTrigger) -> JsonApiCreateResponse<Test> {
        let test = Test {
            id: 5,
            message: String::from("Bob"),
        };
        match trigger {
            CreateResponseTrigger::Created => JsonApiCreateResponse::Created(test),
            CreateResponseTrigger::Accepted => JsonApiCreateResponse::Accepted(test),
            CreateResponseTrigger::NoContent => JsonApiCreateResponse::NoContent,
            CreateResponseTrigger::UnsupportedClientId => {
                JsonApiCreateResponse::UnsupportedClientId(None)
            }
            CreateResponseTrigger::Forbidden => JsonApiCreateResponse::Forbidden(None),
            CreateResponseTrigger::NotFound => JsonApiCreateResponse::NotFound(None),
            CreateResponseTrigger::AlreadyExists => JsonApiCreateResponse::AlreadyExists(None),
            CreateResponseTrigger::Other => {
                JsonApiCreateResponse::Other(Status::BadGateway, Err(vec![]))
            }
        }
    }

    #[test]
    fn rocket_create_response_created() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Created").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Created);
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
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Bob"
                }
            }
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_accepted() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Accepted").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Accepted);
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
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Bob"
                }
            }
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_no_content() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=NoContent").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NoContent);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        assert!(response.body().is_none());
    }

    #[test]
    fn rocket_create_response_unsupported_client_id() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=UnsupportedClientId").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Forbidden);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_forbidden() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Forbidden").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Forbidden);
        let headers = response.headers();
        // Test header response
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_not_found() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=NotFound").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NotFound);
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_already_exists() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=AlreadyExists").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_create_response_other() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Other").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::BadGateway);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }
}

mod test_header_update_response {
    use crate::Test;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::request::FromFormValue;
    use rocket_jsonapi::response::JsonApiUpdateResponse;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, json, Value};

    #[derive(Serialize, Deserialize, FromFormValue)]
    enum UpdateResponseTrigger {
        Updated,
        Accepted,
        NoContent,
        Forbidden,
        NotFound,
        InvalidUpdate,
        Other,
    }

    #[get("/simple?<trigger>")]
    fn simple(trigger: UpdateResponseTrigger) -> JsonApiUpdateResponse<Test> {
        let test = Test {
            id: 5,
            message: String::from("Bob"),
        };
        match trigger {
            UpdateResponseTrigger::Updated => JsonApiUpdateResponse::Updated(test),
            UpdateResponseTrigger::Accepted => JsonApiUpdateResponse::Accepted(test),
            UpdateResponseTrigger::NoContent => JsonApiUpdateResponse::NoContent,
            UpdateResponseTrigger::Forbidden => JsonApiUpdateResponse::Forbidden(None),
            UpdateResponseTrigger::NotFound => JsonApiUpdateResponse::NotFound(None),
            UpdateResponseTrigger::InvalidUpdate => JsonApiUpdateResponse::InvalidUpdate(None),
            UpdateResponseTrigger::Other => {
                JsonApiUpdateResponse::Other(Status::BadGateway, Err(vec![]))
            }
        }
    }

    #[test]
    fn rocket_update_response_updated() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Updated").dispatch();
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
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Bob"
                }
            }
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_update_response_accepted() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Accepted").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Accepted);
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
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Bob"
                }
            }
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_update_response_no_content() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=NoContent").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NoContent);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        assert!(response.body().is_none());
    }

    #[test]
    fn rocket_update_response_forbidden() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Forbidden").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Forbidden);
        let headers = response.headers();
        // Test header response
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_update_response_not_found() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=NotFound").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NotFound);
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_update_response_invalid_update() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=InvalidUpdate").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }

    #[test]
    fn rocket_update_response_other() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/simple?trigger=Other").dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::BadGateway);
        // Test header response
        let headers = response.headers();
        assert_eq!(
            headers.get_one("Content-Type").unwrap(),
            "application/vnd.api+json"
        );
        // Test body response
        let requested_json: Value = from_str(response.body_string().unwrap().as_str()).unwrap();
        let expected_json = json!({
            "errors": []
        });
        assert_eq!(requested_json, expected_json);
    }
}
