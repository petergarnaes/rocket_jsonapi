use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::JsonApiResponseError;
use crate::lib::*;

/// Trait implemented on data objects so they can be parsed as resource objects. [See
/// specification](https://jsonapi.org/format/#document-resource-objects). For this very reason it
/// is required that this trait is implemented on data returned as a `JsonApiResponse`.
///
/// To add `relationships` and `links` to the resource object, `Linkify` and `AllRelationships`
/// needs to be implemented on the same data object as `ResourceIdentifiable`. Please see the
/// documentation for `Linkify` and `AllRelationships` for further documentation.
pub trait ResourceIdentifiable {
    type IdType: ToString;

    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> &Self::IdType;
}

/// Return type for a Rocket.rs route that responds with a JSON:API response. This object
/// serializes into a top-level document, with correct HTTP conventions.
///
/// [See top-level document specification](https://jsonapi.org/format/#document-top-level).
///
/// [See HTTP convention specification](https://jsonapi.org/format/#content-negotiation-servers).
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

#[cfg(test)]
mod tests {
    use crate::error::{JsonApiError, JsonApiResponseError};
    use crate::json_api_error;
    use crate::response::JsonApiResponse;
    use crate::{Linkify, ResourceIdentifiable};
    use serde::Serialize;
    use serde_json::json;
    use std::error::Error;
    use std::fmt::Formatter;

    #[derive(Serialize)]
    struct Test {
        id: i32,
        message: String,
    }

    impl ResourceIdentifiable for Test {
        type IdType = i32;

        fn get_type(&self) -> &'static str {
            &"Test"
        }

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
            serde_json::to_value(JsonApiResponse::<Test>(Ok(test_instance))).unwrap();
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
        let test_instance_value = serde_json::to_value(JsonApiResponse::<Vec<Test>>(Ok(vec![
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
        let test_instance_value = serde_json::to_value(JsonApiResponse::<Vec<Test>>(Err(
            JsonApiResponseError(400, vec![test_error1, test_error2]),
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
}
