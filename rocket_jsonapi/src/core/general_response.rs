use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::JsonApiError;
use crate::lib::*;
use crate::relationship::Relationships;
use crate::response::JsonApiCollection;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde_json::to_string as serialize;
use std::io::Cursor;

struct ResponseError<'a>(&'a Vec<JsonApiError>);

impl Serialize for ResponseError<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiResponseError", 1)?;
        state.serialize_field("errors", &self.0)?;
        state.end()
    }
}

pub struct JsonApiResponse<Data>(pub Status, pub Result<Data, Vec<JsonApiError>>);

impl<Data> Serialize for JsonApiResponse<Data>
where
    // TODO implement Includify + Relationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.1 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&ResponseError(err)),
        }
        // TODO handle json_api field
    }
}

impl<Data> Serialize for JsonApiResponse<JsonApiCollection<Data>>
where
    // TODO implement Includify + Relationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.1 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&ResponseError(err)),
        }
        // TODO handle json_api field
    }
}

fn construct_response<'r, Data>(
    response_body: String,
    response: Result<Data, Vec<JsonApiError>>,
    status: Status,
) -> Result<Response<'r>, Status>
//where
//    Data: Serialize + ResourceIdentifiable + Linkify,
{
    match response {
        Ok(_data) => Ok(Response::build()
            .header(ContentType::JsonApi)
            .status(status)
            .sized_body(Cursor::new(response_body))
            .finalize()),
        Err(_error) => {
            let response = Response::build()
                .header(ContentType::JsonApi)
                .status(status)
                .sized_body(Cursor::new(response_body))
                .finalize();
            Ok(response)
        }
    }
}

impl<'r, Data> Responder<'r> for JsonApiResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn respond_to(self, _request: &Request<'_>) -> Result<Response<'r>, Status> {
        // TODO improve or think about what to do in this case...
        let response = serialize(&self).map_err(|_e| Status::InternalServerError)?;

        construct_response(response, self.1, self.0)
    }
}

impl<'r, Data> Responder<'r> for JsonApiResponse<JsonApiCollection<Data>>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, _request: &Request<'_>) -> Result<Response<'r>, Status> {
        // TODO improve or think about what to do in this case...
        let response = serialize(&self).map_err(|_e| Status::InternalServerError)?;

        construct_response(response, self.1, self.0)
    }
}

#[cfg(test)]
mod test_serialize {
    #![allow(dead_code)]
    // Test that the various parts serialize properly
    use crate::core::general_response::JsonApiResponse;
    use crate::error::{JsonApiError, JsonApiResponseError};
    use crate::response::JsonApiCollection;
    use crate::{json_api_error, Linkify, ResourceIdentifiable, ResourceType};
    use rocket::http::Status;
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct Test {
        id: i32,
        message: String,
    }

    impl ResourceType for Test {
        fn get_type() -> &'static str {
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
            serde_json::to_value(JsonApiResponse::<Test>(Status::Ok, Ok(test_instance))).unwrap();
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
        let test_instance_value = serde_json::to_value(JsonApiResponse::<JsonApiCollection<Test>>(
            Status::Ok,
            Ok(JsonApiCollection::data(vec![
                test_instance1,
                test_instance2,
            ])),
        ))
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
        let test_instance_value = serde_json::to_value(JsonApiResponse::<JsonApiCollection<Test>>(
            Status::Forbidden,
            Err(vec![test_error1, test_error2]),
        ))
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
}
