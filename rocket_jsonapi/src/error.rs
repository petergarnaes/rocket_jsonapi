//! # Responding with errors
//!
//! JSON:API allows responding with an error state. This module defines the Err return type of
//! `JsonApiResponse` and related definitions.
//!
//! ## Example
//!
//! ## Using `Into<JsonApiError>` pattern
//!
//! It might be convenient to define your own error type, define how it converts into `JsonApiError`
//! and return your error, like so:
//!
//! ```rust
//! # use crate::rocket_jsonapi::json_api_error;
//! # use rocket_jsonapi::error::{JsonApiError, JsonApiResponseError};
//! # use rocket::http::Status;
//! enum MyError {
//!     DBError,
//!     InputError,
//! }
//!
//! impl From<MyError> for JsonApiError {
//!     fn from(error: MyError) -> Self {
//!         match error {
//!             DBError => json_api_error!(id = String::from("1")),
//!             InputError => json_api_error!(id = String::from("2"))
//!         }
//!     }
//! }
//!
//! let error_response = JsonApiResponseError::from_items(Status::BadRequest,
//!     vec![MyError::DBError, MyError::InputError]
//! );
//! ```

use crate::lib::*;
use rocket::http::Status;
use std::error::Error;

type ErrorCode = Status;

/// Error format that can be serialized as a valid JsonApi error response
///
/// Is constructed by a http status code and a list of `JsonApiError` that are JSON:API compatible
#[derive(Debug)]
pub struct JsonApiResponseError(ErrorCode, Vec<JsonApiError>);

impl JsonApiResponseError {
    /// Constructs instance of `JsonApiResponseError`
    pub fn new(error_code: ErrorCode, errors: Vec<JsonApiError>) -> Self {
        JsonApiResponseError(error_code, errors)
    }

    /// Constructs instance of `JsonApiResponseError` from any data that implements
    /// `Into<JsonApiError>`. This is helpful if your endpoint results in error types that can
    /// be converted to `JsonApiError`. To see an example, look at `error` module docs.
    pub fn from_item<I: Into<JsonApiError>>(error_code: ErrorCode, error: I) -> Self {
        JsonApiResponseError(error_code, vec![error.into()])
    }

    /// Constructs instance of `JsonApiResponseError` from any data that implements
    /// `Into<JsonApiError>`. This is helpful if your endpoint results in error types that can
    /// be converted to `JsonApiError`. To see an example, look at `error` module docs.
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

    /// Returns http status code of the error
    pub fn get_error_code(self) -> Status {
        self.0
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

/// Struct representing a JSON:API compatible error
///
/// The specific format is documentet in the
/// [JSON:API specification](https://jsonapi.org/format/#error-objects)
///
/// Is best initialized by the macro `json_api_error!()`
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

/// Takes a variable set of field assignments, and initializes an instance of `JsonApiError`
///
/// ## Example
///
/// ```rust
/// # use crate::rocket_jsonapi::json_api_error;
/// # use crate::rocket_jsonapi::error::JsonApiError;
/// let error = json_api_error!(
///     id = String::from("1"),
///     status = String::from("400"),
///     detail = String::from("Something went wrong..."),
/// );
/// ```
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
