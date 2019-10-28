# Rocket.rs + JSON:API guide

[JSON:API](https://jsonapi.org/) is a specification for how a RESTful json api should act. This library is an 
implementation for [Rocket.rs](https://rocket.rs/), such that request and response parsing follow all conventions laid 
out in the specification.

This crate uses a trait based approach for constructing valid JSON:API responses. This means that responses are enriched
by implementing traits for fields like `links`, `meta` etc. on the data-objects that are being responded.

Through the `derive` feature, basic implementations for these traits can be derived, like static link responses, static 
relationships etc.

This crate aims for easy onboarding, gradual enrichment of responses, specification adherence, reducing boilerplate
and hiding specification details.
