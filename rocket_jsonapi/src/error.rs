use crate::lib::*;
use std::error::Error;

type ErrorCode = u32;

#[derive(Debug)]
pub struct JsonApiResponseError(ErrorCode, Vec<JsonApiError>);

impl JsonApiResponseError {
    fn new(error_code: ErrorCode, errors: Vec<JsonApiError>) -> Self {
        JsonApiResponseError(error_code, errors)
    }

    fn from_item<I: Into<JsonApiError>>(error_code: ErrorCode, error: I) -> Self {
        JsonApiResponseError(error_code, vec![error.into()])
    }

    fn from_items<Items: Into<JsonApiError>>(error_code: ErrorCode, errors: Vec<Items>) -> Self {
        let mut json_api_errors = Vec::with_capacity(errors.len());
        for e in errors {
            json_api_errors.push(e.into())
        }
        JsonApiResponseError(error_code, json_api_errors)
    }

    /// Can be used for quickly mocking up your JSON:API implementation, although it is strongly
    /// recommended that `from_items` is used, to implement a more informative error response.
    fn from_error<E: Error>(error_code: ErrorCode, error: E) -> Self {
        JsonApiResponseError(
            error_code,
            vec![JsonApiError {
                detail: Some(error.to_string()),
                ..Default::default()
            }],
        )
    }

    /// Can be used for quickly mocking up your JSON:API implementation, although it is strongly
    /// recommended that `from_items` is used, to implement a more informative error response.
    fn from_errors<E: Error>(error_code: ErrorCode, errors: Vec<E>) -> Self {
        let mut json_api_errors = Vec::with_capacity(errors.len());
        for e in errors {
            json_api_errors.push(JsonApiError {
                detail: Some(e.to_string()),
                ..Default::default()
            })
        }
        JsonApiResponseError(error_code, json_api_errors)
    }
}

impl Serialize for JsonApiResponseError {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.1.serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Default, Serialize)]
pub struct JsonApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    // TODO source
    // TODO links
    // TODO meta
}

#[macro_export]
macro_rules! json_api_error {
    // Appends `key: Some(val),` to body
    (@as_field ($ident:ident = $e:expr, $($tail:tt)*) -> ($($body:tt)*)) => {
        json_api_error!(@as_field ($($tail)*) -> ($($body)* $ident: Some($e),))
    };
    //(@as_field (status = $e:expr, $($tail:tt)*) -> ($($body:tt)*)) => {
    //    json_api_error!(@as_field ($($tail)*) -> ($($body)* status: Some($e),))
    //};
    // Exit rule
    (@as_field ($(,)*) -> ($($body:tt)*)) => {JsonApiError{$($body)* ..Default::default()}};
    // Entry rule
    ($($body:tt)*) => {
        {
            json_api_error!(@as_field ($($body)*,) -> ())
        }
    };
}

// TODO macro rule like vec! for constructing Vec<JsonApiError> by calling .into() on each element
// in input

#[cfg(test)]
mod json_api_error_macro_tests {
    use crate::error::JsonApiError;

    #[test]
    fn test_generate_single_field() {
        let generated_error = json_api_error!(id = String::from("1"));
        let result_error = JsonApiError {
            id: Some(String::from("1")),
            status: None,
            code: None,
            detail: None,
            title: None,
        };
        assert_eq!(generated_error, result_error);
    }

    #[test]
    fn test_generate_multiple_fields() {
        let generated_error = json_api_error!(id = String::from("1"), status = String::from("409"));
        let result_error = JsonApiError {
            id: Some(String::from("1")),
            status: Some(String::from("409")),
            code: None,
            detail: None,
            title: None,
        };
        assert_eq!(generated_error, result_error);
    }

    #[test]
    fn test_generate_all_fields() {
        let generated_error = json_api_error!(
            id = String::from("1"),
            status = String::from("409"),
            code = String::from("9"),
            detail = String::from("Failed completely and utterly, please god help me!"),
            title = String::from("Super failure")
        );
        let result_error = JsonApiError {
            id: Some(String::from("1")),
            status: Some(String::from("409")),
            code: Some(String::from("9")),
            detail: Some(String::from(
                "Failed completely and utterly, please god help me!",
            )),
            title: Some(String::from("Super failure")),
        };
        assert_eq!(generated_error, result_error);
    }
}

#[cfg(test)]
mod json_api_serialize_tests {
    use crate::error::{JsonApiError, JsonApiResponseError};
    use serde_json::json;

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
        let test_instance = JsonApiResponseError(400, test_errors);
        let test_instance_value = serde_json::to_value(test_instance).unwrap();
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
