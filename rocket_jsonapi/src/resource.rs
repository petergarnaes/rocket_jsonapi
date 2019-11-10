use crate::lib::*;

pub trait ResourceType {
    /// Returns the resource type
    fn get_type(&self) -> &'static str;
}

pub trait ResourceIdentifiable: ResourceType {
    /// Trait implemented on data objects so they can be parsed as resource objects.
    ///
    /// [See specification](https://jsonapi.org/format/#document-resource-objects). For this very reason
    /// it is required that this trait is implemented on data returned from a `JsonApiDataResponse`.
    ///
    /// The trait requires the [ResourceType] to be implemented, because a resource object requires
    /// a type.
    ///
    /// ### Using `#[derive(ResourceIdentifiable)]`
    ///
    /// Import the derive macro:
    /// ```rust
    /// use rocket_jsonapi::ResourceIdentifiable;
    /// ```
    /// When derived, it defaults to using the field named `id` on the implementing `struct`.
    /// The `type` defaults to the name of the `struct`. Example:
    /// ```rust
    /// # use rocket_jsonapi::{ResourceType, ResourceIdentifiable};
    /// #
    /// #[derive(ResourceType, ResourceIdentifiable)]
    /// struct Article { // "Article" is returned by get_type()
    ///     id: i32, // id field is returned by derived get_id()
    ///     author_name: String,
    ///     text: String
    /// }
    /// ```
    ///
    /// #### Customizing `#[derive(ResourceIdentifiable)]` behaviour
    ///
    /// Both `id` and `type` can be changed when deriving.
    ///
    /// `#[resource_ident_id = "id_field"]` changes the field that functions as the `id`.
    ///
    /// `#[resource_ident_type = "CustomType"]` changes the `type`.
    ///
    /// Example:
    /// ```rust
    /// # use rocket_jsonapi::{ResourceType, ResourceIdentifiable};
    /// #
    /// #[derive(ResourceType, ResourceIdentifiable)]
    /// #[resource_ident_id = "author_name"]
    /// #[resource_ident_type = "Chapter"]
    /// struct Article { // "Chapter" is returned by get_type()
    ///     id: i32,
    ///     author_name: String, // author_name field is returned by derived get_id()
    ///     text: String
    /// }
    /// ```

    /// The type of the id returned by `get_id(&self)`, must implement ToString, because the
    /// specification states resource ids must be strings
    type IdType: ToString;

    /// Returns the resource id
    fn get_id(&self) -> &Self::IdType;
}
