use crate::core::data_object::JsonApiPrimaryDataObject;
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
pub struct JsonApiResponse<Data, Error>(pub Result<Data, Error>);

impl<Data, Error> Serialize for JsonApiResponse<Data, Error>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
    Error: Serialize,
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

impl<Data, Error> Serialize for JsonApiResponse<Vec<Data>, Error>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
    Error: Serialize,
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

    #[derive(Debug, Serialize)]
    struct ErrorMessage(String);

    impl std::fmt::Display for ErrorMessage {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for ErrorMessage {}

    #[test]
    fn serialize_json_api_response() {
        let test_instance = Test {
            id: 5,
            message: "Hello".to_string(),
        };
        let test_instance_value =
            serde_json::to_value(JsonApiResponse::<Test, ErrorMessage>(Ok(test_instance))).unwrap();
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
        let test_instance_value =
            serde_json::to_value(JsonApiResponse::<Vec<Test>, ErrorMessage>(Ok(vec![
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
}
