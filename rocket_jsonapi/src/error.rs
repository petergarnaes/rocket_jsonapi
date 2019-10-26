use crate::lib::*;
use std::error::Error;

type ErrorCode = u32;

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
    /// recommended that `from_items` is used.
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
    /// recommended that `from_items` is used.
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

#[derive(Debug, PartialEq, Default)]
pub struct JsonApiError {
    id: Option<String>,
    status: Option<String>,
    code: Option<String>,
    title: Option<String>,
    detail: Option<String>,
    // TODO source
    // TODO links
    // TODO meta
}

#[macro_export]
macro_rules! json_api_error {
    { $( $field:ident = $val:expr ),* } => {
        {
            let mut error = JsonApiError {
                id: None,
                status: None,
                code: None,
                title: None,
                detail: None,
            };
            $(
                error.$field = Some($val);
            )*
            error
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
