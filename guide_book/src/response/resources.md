# Resource objects

In the JSON:API specification a resource is the term for a piece of data.

In the JSON:API specification a resource object can be globally identified by its __id__ _and_ __type__.
To make a `struct` a resource object, `ResourceIdentifiable` must be implemented.

## Implementing `ResourceIdentifiable`

This trait has two methods: 

 - `fn get_id(&self) -> &Self::IdType` returns a reference to the field that is the ID in the implementing `struct`
 - `fn get_type(&self) -> &'static str` returns the type of the resource
 
`IdType` is an associated type of the `id` field returned by `get_id()`. The `IdType` must implement `ToString`, because
the `id` in a resource object must be a string.

### Using `#[derive(ResourceIdentifiable)]`

Import the derive macro:
```rust
use rocket_jsonapi::ResourceIdentifiable;
```
When derived, it defaults to using the field named `id` on the implementing `struct`.
The `type` defaults to the name of the `struct`. Example:
```rust
##[derive(ResourceIdentifiable)]
struct Article { // "Article" is returned by get_type()
    id: i32, // id field is returned by derived get_id()
    author_name: String,
    text: String
}
``` 

#### Customizing `#[derive(ResourceIdentifiable)]` behaviour

Both `id` and `type` can be changed when deriving.

`#[resource_ident_id = "id_field"]` changes the field that functions as the `id`.

`#[resource_ident_type = "CustomType"]` changes the `type`.

Example:
```rust
##[derive(ResourceIdentifiable)]
##[resource_ident_id = "author_name"]
##[resource_ident_type = "Chapter"]
struct Article { // "Chapter" is returned by get_type()
    id: i32, 
    author_name: String, // author_name field is returned by derived get_id()
    text: String
}
``` 
