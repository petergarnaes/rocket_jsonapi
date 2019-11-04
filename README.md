# `rocket_jsonapi` - JSON:API implementation for Rocket.rs

[JSON:API](https://jsonapi.org/) is a specification for how a RESTful json api should act. This library is an 
implementation for [Rocket.rs](https://rocket.rs/), such that request and response parsing follow all conventions laid 
out in the specification.

This crate uses a trait based approach for constructing valid JSON:API responses. This means that responses are enriched
by implementing traits for fields like `links`, `meta` etc. on the data-objects that are being responded.

This framework requires a lot of implementations on the data objects returned, for things like `links`, 
`meta`, etc. Through the `rocket_jsonapi_derive` crate, these traits can be easily derived.
The crate can also generate basic implementations, like static link responses, static relationships etc.
[More about `rocket_jsonapi_derive` macros](TODO).

This crate aims for easy onboarding, gradual enrichment of responses, specification adherence, reducing boilerplate
and hiding specification details.

## Documentation

[See docs](TODO)
[See guide](TODO)

## Requirements

This library uses Rust-2018 syntax. Because this crate uses rocket and the `specialization` feature, rust nightly is 
needed.

## Links
 
 - [Docs](TODO)
 - [Guide](TODO)
 - [JSON:API](https://jsonapi.org/)
 - [Rocket.rs](https://rocket.rs/)