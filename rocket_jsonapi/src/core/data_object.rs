use crate::core::links_serialize::LinksSerialize;
use crate::core::serialize_no_conversion::CanSerializeNoConversion;
use crate::lib::*;
use crate::response::JsonApiCollection;

// Struct for data, will be parsed correctly
pub struct JsonApiPrimaryDataObject<'a, Data>(pub &'a Data);

impl<'a, Data> Serialize for JsonApiPrimaryDataObject<'a, Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 3)?;
        state.serialize_field("data", &ResourceIdentifiableWrapper(self.0))?;
        let links = self.0.get_links();
        if !links.is_empty() {
            state.serialize_field("links", &LinksSerialize(&links))?;
        }
        // TODO Includify and Relationships
        state.end()
    }
}

impl<'a, Data> Serialize for JsonApiPrimaryDataObject<'a, JsonApiCollection<Data>>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 3)?;
        state.serialize_field("data", &JsonApiPrimaryDataObjectArray(&(self.0).0))?;
        let links = &(self.0).1;
        if !links.is_empty() {
            state.serialize_field("links", &LinksSerialize(links))?;
        }
        // TODO Includify and Relationships
        state.end()
    }
}

struct JsonApiPrimaryDataObjectArray<'a, Data>(&'a Vec<Data>);

impl<'a, Data> Serialize for JsonApiPrimaryDataObjectArray<'a, Data>
where
    Data: Serialize + ResourceIdentifiable,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for res in self.0 {
            seq.serialize_element(&ResourceIdentifiableWrapper(res))?;
        }
        seq.end()
    }
}

// Newtype to customize parsing of ResourceIdentifiable, so we don't need to allocate a new data
// structure
struct ResourceIdentifiableWrapper<'a, R>(&'a R);

impl<'a, R> Serialize for ResourceIdentifiableWrapper<'a, R>
where
    R: Serialize + ResourceIdentifiable,
{
    default fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceIdentifier", 3)?;
        state.serialize_field("id", &self.0.get_id().to_string())?;
        state.serialize_field("type", R::get_type())?;
        state.serialize_field("attributes", &self.0)?;
        state.end()
    }
}

// Specialized case where we can simply read the Id, without having to convert it to a string first
impl<'a, Data> Serialize for ResourceIdentifiableWrapper<'a, Data>
where
    Data: Serialize + ResourceIdentifiable<IdType: CanSerializeNoConversion>,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceIdentifier", 3)?;
        // Specialized part, here we simply read the Id value, no conversion needed
        state.serialize_field("id", &self.0.get_id().as_str())?;
        state.serialize_field("type", Data::get_type())?;
        state.serialize_field("attributes", &self.0)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::data_object::{
        JsonApiPrimaryDataObject, JsonApiPrimaryDataObjectArray, ResourceIdentifiableWrapper,
    };
    use crate::resource::ResourceType;
    use crate::response::JsonApiCollection;
    use crate::{Linkify, ResourceIdentifiable};
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct Test {
        id: i32,
        message: String,
    }

    impl ResourceType for Test {
        fn get_type() -> &'static str {
            &"Test"
        }
    }

    impl ResourceIdentifiable for Test {
        type IdType = i32;

        fn get_id(&self) -> &Self::IdType {
            &self.id
        }
    }

    impl Linkify for Test {}

    #[test]
    fn serialize_resource_identifiable_wrapper() {
        let test_instance = Test {
            id: 5,
            message: "Hello".to_string(),
        };
        let test_instance_value =
            serde_json::to_value(ResourceIdentifiableWrapper(&test_instance)).unwrap();
        let test_equals_value = json!({
            "id": "5",
            "type": "Test",
            "attributes": {
                "id": 5,
                "message": "Hello"
            }
        });
        assert_eq!(test_instance_value, test_equals_value);
    }

    #[test]
    fn serialize_resource_identifiable_wrapper_string_id() {
        #[derive(Serialize)]
        struct T {
            id: Box<String>,
            message: String,
        }
        impl ResourceType for T {
            fn get_type() -> &'static str {
                &"T"
            }
        }
        impl ResourceIdentifiable for T {
            type IdType = Box<String>;

            fn get_id(&self) -> &Self::IdType {
                &self.id
            }
        }
        let test_instance = T {
            id: Box::new("12".to_string()),
            message: "Hello".to_string(),
        };
        let test_instance_value =
            serde_json::to_value(ResourceIdentifiableWrapper(&test_instance)).unwrap();
        let test_equals_value = json!({
            "id": "12",
            "type": "T",
            "attributes": {
                "id": "12",
                "message": "Hello"
            }
        });
        assert_eq!(test_instance_value, test_equals_value);
    }

    #[test]
    fn serialize_json_primary_data_object() {
        let test_instance = Test {
            id: 5,
            message: "Hello".to_string(),
        };
        let test_instance_value =
            serde_json::to_value(JsonApiPrimaryDataObject(&test_instance)).unwrap();
        let test_equals_value = json!({
            "data": {
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Hello"
                }
            }
        });
        assert_eq!(test_instance_value, test_equals_value);
    }

    #[test]
    fn serialize_json_primary_data_object_with_vec() {
        let test_instance1 = Test {
            id: 5,
            message: "Hello".to_string(),
        };
        let test_instance2 = Test {
            id: 6,
            message: "Hallo".to_string(),
        };
        let test_instance_value =
            serde_json::to_value(JsonApiPrimaryDataObject(&JsonApiCollection::data(vec![
                test_instance1,
                test_instance2,
            ])))
            .unwrap();
        let test_equals_value = json!({
            "data": [{
                "id": "5",
                "type": "Test",
                "attributes": {
                    "id": 5,
                    "message": "Hello"
                }
            }, {
                "id": "6",
                "type": "Test",
                "attributes": {
                    "id": 6,
                    "message": "Hallo"
                }
            }]
        });
        assert_eq!(test_instance_value, test_equals_value);
    }

    #[test]
    fn serialize_json_primary_data_object_array() {
        let test_instance1 = Test {
            id: 5,
            message: "Hello".to_string(),
        };
        let test_instance2 = Test {
            id: 6,
            message: "Hallo".to_string(),
        };
        let test_instance_value = serde_json::to_value(JsonApiPrimaryDataObjectArray(&vec![
            test_instance1,
            test_instance2,
        ]))
        .unwrap();
        let test_equals_value = json!([{
            "id": "5",
            "type": "Test",
            "attributes": {
                "id": 5,
                "message": "Hello"
            }
        }, {
            "id": "6",
            "type": "Test",
            "attributes": {
                "id": 6,
                "message": "Hallo"
            }
        }]);
        assert_eq!(test_instance_value, test_equals_value);
    }
}
