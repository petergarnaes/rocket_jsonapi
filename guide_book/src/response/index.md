# JSON:API responses

A basic example of how to respond can be seen in [getting started - hello world](/getting-started.md#hello-world).

`rocket_jsonapi` provides a [responder](https://rocket.rs/v0.4/guide/responses/#responder) called 
`JsonApiResponse<Data>(Result<Data, JsonApiResponseError>)`.
The user provided `struct` given to `JsonApiResponse` must implement the following traits:
 - `ResourceIdentifiable`, ensures the data can be parsed as a [resource object](https://jsonapi.org/format/#document-resource-objects).
 [Details here](/response/resources.md).
 - `Linkify`, ensures the [links](https://jsonapi.org/format/#document-links) metadata is returned in the response, if 
 any.
 - `Relationships`, ensures the [relationship](https://jsonapi.org/format/#document-resource-object-relationships) 
 metadata is returned in the response, if any.
 - `Includify`, for setting the [top-level](https://jsonapi.org/format/#document-top-level) `include` field with 
 included resources related to the primary data, if any.

But don't worry, all traits, except `ResourceIdentifiable`, can be derived as _empty_ implementations.
An empty implementation means the result of the implementation will not add anything to the response.

`ResourceIdentifiable` can also be derived, but it has to be implemented so that it returns an `id` and `type` for a 
`struct`.
When derived, it defaults to the `struct`'s `id` field, and the name of the `struct` as the type. 
To customize this behaviour, see [this](/response/resources.md).