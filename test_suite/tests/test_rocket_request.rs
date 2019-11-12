#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_jsonapi::{Linkify, ResourceIdentifiable, ResourceType};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, ResourceType, ResourceIdentifiable, Linkify)]
struct Test {
    id: i32,
    message: String,
}

mod test_request_headers {
    use crate::Test;
    use rocket::http::{Header, Status};
    use rocket::local::Client;
    use rocket_jsonapi::request::{JsonApiCreateRequest, JsonApiRequest, JsonApiUpdateRequest};
    use rocket_jsonapi::response::JsonApiDataResponse;

    #[get("/simple")]
    fn simple(_req: JsonApiRequest) -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(Test {
            id: 1,
            message: String::from("Hello!"),
        }))
    }

    #[post("/simple_data", data = "<input>")]
    fn simple_data(input: JsonApiCreateRequest<Test>) -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(input.0))
    }

    #[patch("/simple_update", data = "<_input>")]
    fn simple_update(_input: JsonApiUpdateRequest<Test>) -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(Test {
            id: 5,
            message: String::from("Bob"),
        }))
    }

    #[test]
    fn test_request_simple_ok() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.get("/simple");
        //request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
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
        let request = client.get("/simple");
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NotAcceptable);
    }

    #[test]
    fn test_request_accept_header_with_params_406() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.get("/simple");
        //request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json; arg=val"));
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::NotAcceptable);
    }

    #[test]
    fn test_request_accept_header_least_one_valid() {
        let rocket = rocket::ignite().mount("/", routes![simple]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.get("/simple");
        //request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new(
            "Accept",
            "application/vnd.api+json; arg=val, application/vnd.api+json",
        ));
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create_request_simple_ok() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
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
    fn test_create_request_accept_header_least_one_valid() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        //request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new(
            "Accept",
            "application/vnd.api+json; arg=val, application/vnd.api+json",
        ));
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create_request_missing_content_type_header() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::UnsupportedMediaType);
    }

    #[test]
    fn test_create_request_content_type_header_with_params_415() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        request.add_header(Header::new(
            "Content-Type",
            "application/vnd.api+json; chartset=UTF-8",
        ));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::UnsupportedMediaType);
    }

    #[test]
    fn test_update_request_simple_ok() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "id": "5",
                "attributes": {
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
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
    fn test_update_request_accept_header_least_one_valid() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        //request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new(
            "Accept",
            "application/vnd.api+json; arg=val, application/vnd.api+json",
        ));
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "id": "5",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_update_request_missing_content_type_header() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "id": "5",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::UnsupportedMediaType);
    }

    #[test]
    fn test_update_request_content_type_header_with_params_415() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        request.add_header(Header::new(
            "Content-Type",
            "application/vnd.api+json; chartset=UTF-8",
        ));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "id": "5",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::UnsupportedMediaType);
    }
}

mod test_request_input {
    use crate::Test;
    use rocket::http::{Header, Status};
    use rocket::local::Client;
    use rocket_jsonapi::request::{JsonApiCreateRequest, JsonApiUpdateRequest};
    use rocket_jsonapi::response::JsonApiDataResponse;

    #[post("/simple_data", data = "<input>")]
    fn simple_data(input: JsonApiCreateRequest<Test>) -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(input.0))
    }

    #[patch("/simple_update", data = "<_input>")]
    fn simple_update(_input: JsonApiUpdateRequest<Test>) -> JsonApiDataResponse<Test> {
        JsonApiDataResponse(Ok(Test {
            id: 5,
            message: String::from("Bob"),
        }))
    }

    #[test]
    fn test_update_request_invalid_type() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "NotTest",
                "id": "5",
                "attributes": {
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
    }

    #[test]
    fn test_update_request_missing_id() {
        let rocket = rocket::ignite().mount("/", routes![simple_update]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.patch("/simple_update");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
    }

    #[test]
    fn test_create_request_invalid_type() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "NotTest",
                "attributes": {
                    "id": 1,
                    "message": "Hay!"
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
    }

    #[test]
    fn test_create_request_missing_field() {
        let rocket = rocket::ignite().mount("/", routes![simple_data]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut request = client.post("/simple_data");
        request.add_header(Header::new("Content-Type", "application/vnd.api+json"));
        request.add_header(Header::new("Accept", "application/vnd.api+json"));
        request = request.body(
            r#"
        {
            "data": {
                "type": "Test",
                "attributes": {
                    "id": 1
                }
            }
        }
        "#,
        );
        let response = request.dispatch();
        // Test HTTP status code
        assert_eq!(response.status(), Status::Conflict);
    }

    // TODO return conflict if client ID already exists
}
