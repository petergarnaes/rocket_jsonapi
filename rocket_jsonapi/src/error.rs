use crate::lib::*;
use rocket::http::Status;
use std::error::Error;

type ErrorCode = Status;

#[derive(Debug)]
pub struct JsonApiResponseError(pub ErrorCode, pub Vec<JsonApiError>);

impl JsonApiResponseError {
    pub fn new(error_code: ErrorCode, errors: Vec<JsonApiError>) -> Self {
        JsonApiResponseError(error_code, errors)
    }

    pub fn from_item<I: Into<JsonApiError>>(error_code: ErrorCode, error: I) -> Self {
        JsonApiResponseError(error_code, vec![error.into()])
    }

    pub fn from_items<Items: Into<JsonApiError>>(
        error_code: ErrorCode,
        errors: Vec<Items>,
    ) -> Self {
        let mut json_api_errors = Vec::with_capacity(errors.len());
        for e in errors {
            json_api_errors.push(e.into())
        }
        JsonApiResponseError(error_code, json_api_errors)
    }

    /// Can be used for quickly mocking up your JSON:API implementation, although it is strongly
    /// recommended that `from_items` is used, to implement a more informative error response.
    pub fn from_error<E: Error>(error_code: ErrorCode, error: E) -> Self {
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
    pub fn from_errors<E: Error>(error_code: ErrorCode, errors: Vec<E>) -> Self {
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
        let mut state = serializer.serialize_struct("JsonApiResponseError", 1)?;
        state.serialize_field("errors", &self.1)?;
        state.end()
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
