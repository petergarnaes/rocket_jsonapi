# Getting started

`rocket_json_api` intends to make it as easy, and with as little boilerplate as possible, to make your Rocket.rs server 
compliant with the [specification](https://jsonapi.org/format/).

In order to do this, this library provides the following:
 - A [Rocket.rs request guard](/request/index.md) for ensuring requests conforms to the specification.
 - A [Rocket.rs responder](/response/index.md) to convert the output to json that adhereres to the specification.
 - A set of traits to implement on the output data, so the responder can construct rich metadata in the response.
 - A ton of macros to make help you write less code.

## Setup

This crate is published on [crates.io](https://crates.io/) and can be added to your project by adding it as a dependency:
```toml
[dependencies]
rocket_jsonapi = { version = "1.0", features = ["derive"] }
```
Where `version = "0.1"` is the latest release. The `derive` feature makes a lot of the code derivable.

This project uses Rust2018 syntax and unstable nightly features. This shouldn't be a big problem, since Rocket.rs does 
the same.

## Hello world

The following is a fully working example with Rocket.rs. It is a simple response to a JSON:API `GET` request for 
`/test_data`.
```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rocket_jsonapi::request::JsonApiRequest;
use rocket_jsonapi::response::JsonApiResponse;

#[derive(Serialize, ResourceIdentifiable, Linkify, Relationships)]
struct TestData {
    id: String
}

#[get("/test_data")]
fn test_data(json_api_repuest: JsonApiRequest) -> JsonApiResponse<TestData> {
    let test = TestData {id: String::from("1")};
    JsonApiResponse(Ok(test))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![test_data])
        .launch();
}
```
By deriving `ResourceIdentifiable`, `Linkify` and `Relationships` it is possible for `JsonApiResponse` to figure out 
the `id` of the data, create links and relationships. If no macro attributes are set, empty implementations will be 
derived, and the response will only include a `data` object in the response.

Derive of `ResourceIdentifiable` tries to find an `id` field in the struct. This can of course be customized or 
implemented by hand, see [resource section](/response/resources.md).

### Next steps

Enrich your responses through [links](/response/links.md) and [relationships](/response/relationships.md).

To see how you would insert or update through `POST` requests, see [request updates](/request/update.md).