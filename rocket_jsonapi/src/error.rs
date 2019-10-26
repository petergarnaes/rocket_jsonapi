use crate::lib::*;

type ErrorCode = u32;

struct JsonApiResponseError(ErrorCode, Vec<JsonApiError>);

#[derive(Debug, PartialEq)]
struct JsonApiError {
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
