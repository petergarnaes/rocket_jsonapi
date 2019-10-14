# `rocket_jsonapi` - JSON:API implementation for Rocket.rs

[JSON:API](https://jsonapi.org/) is a specification for how a RESTful json api should act. This library is an 
implementation for [Rocket.rs](https://rocket.rs/), such that request and response parsing follow all conventions laid out in the 
specification.

`rocket_jsonapi` puts emphasis on correctness and following all conventions.

This crate uses the `specialization` feature, to give multiple options for what, and how much of the optional features 
you want to include.

This crate to be easy to get started with, so it is quick and easy to build _correct_ and conventional webservices.
At the same time, it is possible to fully enrich the responses with all the optional fields.

## Requirements

This library uses Rust2018 syntax. Because this crate uses rocket and the `specialization` feature, rust nightly is needed.

## Usage example


## What is provided

TODO
 - Something about request guards
 - Something about response parsing, http codes etc.

## Getting started

TODO
 - Explain `JsonApiDataResponse`
 - Explain `JsonApiResult` and `JsonApiData` enum
 - Explain `JsonApiDataObject` implementation
 - Hand wave at `links`, `included` etc.

## Links
 
 - More examples
 - API documentation
 - [JSON:API](https://jsonapi.org/)
 - [Rocket.rs](https://rocket.rs/)