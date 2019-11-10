use crate::lib::*;
use crate::resource::ResourceType;

/// Object to represent a "resource identifier object", which is an object that identifies an
/// individual resource. [See specification](https://jsonapi
/// .org/format/#document-resource-identifier-objects).
///
/// They are returned when resources are linked, for example in relationships or included. [See
/// resource linkage](https://jsonapi.org/format/#document-resource-object-linkage).
#[derive(Serialize)]
pub struct ResourceIdentifierObject {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: &'static str,
}

impl ResourceIdentifierObject {
    /// Creates a resource identifier object from an object that can be serialized as a resource
    /// object.
    pub fn create_identifier<T: ResourceIdentifiable>(resource: &T) -> Self {
        ResourceIdentifierObject {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl<T: ResourceIdentifiable> From<&T> for ResourceIdentifierObject {
    fn from(resource: &T) -> Self {
        ResourceIdentifierObject {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl ResourceType for ResourceIdentifierObject {
    fn get_type(&self) -> &'static str {
        self.object_type
    }
}

impl ResourceIdentifiable for ResourceIdentifierObject {
    type IdType = String;

    fn get_id(&self) -> &String {
        &self.id
    }
}

/// Wrapper newtype for serializing a `ResourceIdentifiable` object as a `ResourceIdentifierObject`.
///
/// A “resource identifier object” is an object that identifies an individual resource. [See
/// specification](https://jsonapi.org/format/#document-resource-identifier-objects).
///
/// They are returned when resources are linked, for example in relationships or included. [See
/// resource linkage](https://jsonapi.org/format/#document-resource-object-linkage).
struct ToResourceIdentifierObject<'a, R>(&'a R);

impl<'a, R> Serialize for ToResourceIdentifierObject<'a, R>
where
    R: ResourceIdentifiable,
{
    default fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceIdentifierObject", 3)?;
        state.serialize_field("id", &self.0.get_id().to_string())?;
        state.serialize_field("type", self.0.get_type())?;
        state.end()
    }
}

impl<'a, R> Serialize for ToResourceIdentifierObject<'a, Vec<R>>
where
    R: ResourceIdentifiable,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for res in self.0 {
            seq.serialize_element(&ToResourceIdentifierObject(res))?;
        }
        seq.end()
    }
}
