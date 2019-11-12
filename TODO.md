# TODO

## Crate stuff

 - Dependencies on syn/quote in derive crate, should it use the same?
 - macro hygiene

## API stuff

 - [x] Overhaul module structure, it is getting messy in `response.rs` and `data.rs`, new modules probably needed
 - [x] Handle top level `errors` parsing properly, should be able to handle multiple errors and parse the top level key
  `errros`
    - [x] Make error representation
    - [x] Construct macro to easily create error object with variable amount of fields
    - [x] Macro positive testing
    - Macro negative testing
    - [x] Create static method constructors for top-level error representation (JsonApiResponseError)
    - Create macro similar to `vec!` that takes different types of elements that implements `Into<JsonApiError>` and
     returns `JsonApiResponseError`.
    - [x] `Serialize` testing
    - [x] Integrate with JsonApiDataResponse
    - [x] Maybe use Rocket.js error codes, if they have it, instead of error code being a u32?
    - [x] Better error macro, that uses [Push-down accumulation](https://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html)
    and maybe some [incremental TT munching](https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html)
    to generate a proper JsonApiError constructor, instead of mutating it one field at a time.
    - [x] Test top level serializing of errors
    - Change `JsonApiDataResponse` to take Vec of errors that implement `AsJsonApiError`? `JsonApiError` can have default 
    implementation. But what about ErrorCode?
    - Implement `source` field for `JsonApiError`
    - Implement `links` field for `JsonApiError`
    - Implement `meta` field for `JsonApiError`
 - [x] Disable unused warnings in `test_suite` crate.
 - [x] Remove `DataObject` and any other types that are constructed when serializing, we don't want the overhead!
 - [x] Fix deriving of `ResourceIdentifiable`! Should be able to handle `IdType` now.
 - [x] Deriving `ResourceIdentifiable` should handle when `IdType=&str`, can copying be avoided?
 - [x] Write serialization tests of all the newtypes: `ResourceIdentifiableWrapper`, `JsonApiPrimaryDataObject` and
  `JsonApiPrimaryDataObjectArray`
 - [x] Hide `ResourceIdentifier` from user, move to core
 - [x] Make `ResourceIdentifierWrapper` with serialization implementation, so when constructing resource identifiers, we
  can simply read the objects we convert, instead of constructing new.
 - [x] Write full-stack'ish serialization tests for `JsonApiDataResponse` with all sorts of implementations for the wrapped
  type.
 - [x] Move many of the tests of public APIs to `test_suite` crate
 - Test derive of `ResourceType`
 - Expand `Linkify` derivable API, so static links, relationships etc. can be included
 - Make `Relationships` derivable, consider its current API
 - Make the `Included` API, probably use same approach as relationships API
 - Make all our traits derivable with newtypes, so inheritance boilerplate can be reduced. This pattern could make
  for some nifty implementations with heavy re-use of code through inheritance, only overriding the parts the user
   wishes. For example, a `Person` could also be used as a relationship, like `Author`, but where `links` are changed
   , or maybe entirely excluded. See serde transparent attribute
 - [x] Implement `JsonApi` as a [request guard](https://rocket.rs/v0.4/guide/requests/#custom-guards), see rockets `Json
 ` as [reference](https://github.com/SergioBenitez/Rocket/blob/master/contrib/lib/src/json.rs).
    - Look into if reader interface of Data input is an attack vector for DoS
    - Are we using the right errors in our `FromRequest` and `FromData` implementation in our `FromRequest` and `FromData` implementationss?
 - [x] Implement `JsonApi` as a [data guard](https://api.rocket.rs/v0.4/rocket/data/trait.FromData.html)
    - [x] Testing POST data requests w. deserialization
    - [x] Negative testing POST data requests w. deserialization
 - Proper deserialization of input data types
    - [x] deserialising input types for insert requests
    - [x] Implement type check
    - [x] Deny unkown fields for resource type deserializing
    - [x] Deny unkown fields for data wrapper deserializing 
    - [x] Negative testing of deserializing input types for insert
    - Support deserializing relationships for input types for insert
    - [x] deserialising input types for update (with id) requests
    - Negative testing of deserializing input types for update (with id)
    - Support deserializing relationships for input types for update (with id)
    - [x] Deserialize input type with variable fields for PATCH updates
    - Support client generated ID for insert statement
 - [x] Implement `JsonApi` as a [responser](https://rocket.rs/v0.4/guide/responses/#custom-responders), see rockets `Json
 ` as [reference](https://github.com/SergioBenitez/Rocket/blob/master/contrib/lib/src/json.rs).
 - [x] Implement `JsonApiCreateRequest`
 - [x] Test `JsonApiCreateRequest`
    - [x] Test `JsonApiCreateRequest` validation of type of incoming request, give 409 Conflict
    - [x] Test `JsonApiCreateRequest` validation of input data missing fields give 409 Conflict
 - [x] Implement `JsonApiUpdateRequest`
    - [x] Make `ResourceType`'s only method `get_type` static: `fn get_type() -> &'static str`
    - [x] Make `JsonApiUpdateRequest` generic on `InputData` so it can validate type of incoming request
 - [x] Test `JsonApiUpdateRequest`
    - [x] Test `JsonApiUpdateRequest` validation of type of incoming request give 409 Conflict
    - [x] Test `JsonApiUpdateRequest` validation of missing id give 409 conflict
 - [x] Test `JsonApiDataResponse` as a `Responder`
 - [x] Make `JsonApiCreateResponse` with all the different types of responses allowed for creating data
    - [x] Test `JsonApiCreateResponse`
 - Make `JsonApiUpdateResponse` with all the different types of responses allowed for updating data
    - Test `JsonApiUpdateResponse`
 - Rename `resource_identifier` module. Right now the name does not quite reflect its content.
 - Move `ResourceObjectIdentifier` to the public API
    - Make a newtype `ToResourceObjectIdentifier` that serializes the inner type as a resource object identifier
 - Change `derive` of `ResourceType` to always use `#[inline]`
 - Probably a ton more, that I forgot...
 
## Documentation stuff

 - Make proper RustDoc that is presentable and easy to navigate
    - Change doc and use [] to link internally in docs
    - Update docs to reflect `ResourceType`
    - Add favicon
    - Add logo
 - Expand README
    - Link to docs
    - Expand on what the framework does, and does not do
