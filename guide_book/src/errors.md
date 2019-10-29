# How to respond with errors

Remember that `JsonApiResponse` takes as input a `Result<Data, JsonApiResponseError>`. A `JsonApiResponseError` takes an
error code and a `JsonApiError` as arguments. An example:
```rust
use rocket_jsonapi::json_api_error;
use rocket::http::Status;

##[get("/error")]
fn always_errors(request: JsonApiRequest) -> JsonApiResponse<()> {
    let error = json_api_error!(
        status = String::from("400"),
        detail = String::from("Always happens"),
    );
    JsonApiResponse(Err(JsonApiResponseError(Status::BadRequest, vec![error])))
}
```

The error code is a `Status` from `rocket`, so it is up to you to choose appropriately. The `json_api_error` constructs 
an error object, which will be further explained in the next section.

## Error objects

The [specification](https://jsonapi.org/format/#errors) states that error objects must only contain certain fields, but
which ones you choose to implement is entirely optional. The fields can be seen 
[here](https://jsonapi.org/format/#error-objects).

An error object is modelled as a `JsonApiError` ([doc](TODO)). It implements `Default`, and is therefore easy to 
construct. Even easier is using the `rocket_json_api::json_api_error` macro, and define the fields you want:
```rust
fn as_error(my_error: MyError) -> JsonApiError {
    json_api_error!(
        id = String::from("1"),
        detail = error.to_string(), 
        code = my_error.code
    )
}
``` 

### Recommended patterns for constructing error objects

Something about into constructors, and the macro for aggregating stuff that can be converted into error objects.
