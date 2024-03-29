use crate::ResourceType;
use serde::de::{MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(PartialEq, Debug)]
pub struct InputDataWrapper<InputData>(pub InputData);

impl<InputData> Deref for InputDataWrapper<InputData> {
    type Target = InputData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Make a Deserialize for a single resource object input type with no user ID
// How do we make the attributes field deserialize as input type? Try looking at derive output with
// cargo expand --lib/--bin of some test data, where a field is an owned struct that also has
// Deserialize derived
impl<'de, InputData> Deserialize<'de> for InputDataWrapper<InputData>
where
    InputData: ResourceType + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO expand with relationship
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum CreateResourceField {
            Type,
            Attributes,
        }
        struct CreateResourceVisistor<'de, D> {
            marker: PhantomData<D>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de, D: ResourceType + Deserialize<'de>> Visitor<'de> for CreateResourceVisistor<'de, D> {
            type Value = InputDataWrapper<D>;

            fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
                formatter.write_str("struct InputDataWrapper")
            }

            // TODO should we implement this? Which method does serde_json use?
            /*
            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                unimplemented!()
            }
            */

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut resource_type: Option<String> = None;
                let mut attributes: Option<D> = None;
                while let Some(key) =
                    match serde::de::MapAccess::next_key::<CreateResourceField>(&mut map) {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    }
                {
                    match key {
                        CreateResourceField::Type => {
                            if resource_type.is_some() {
                                return Err(<A::Error as serde::de::Error>::duplicate_field(
                                    "type",
                                ));
                            }
                            resource_type =
                                Some(match serde::de::MapAccess::next_value::<String>(&mut map) {
                                    Ok(val) => val,
                                    Err(err) => return Err(err),
                                });
                        }
                        CreateResourceField::Attributes => {
                            if attributes.is_some() {
                                return Err(<A::Error as serde::de::Error>::duplicate_field(
                                    "attributes",
                                ));
                            }
                            attributes =
                                Some(match serde::de::MapAccess::next_value::<D>(&mut map) {
                                    Ok(val) => val,
                                    Err(err) => return Err(err),
                                });
                        }
                    }
                }
                let resource_type = match resource_type {
                    Some(t) => t,
                    None => match serde::private::de::missing_field("type") {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    },
                };
                let attributes = match attributes {
                    Some(t) => t,
                    None => match serde::private::de::missing_field("attributes") {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    },
                };
                // Check type field of resource object to see that the given type matches the
                // desired type
                let expected_type = D::get_type();
                if expected_type != resource_type {
                    return Err(<A::Error as serde::de::Error>::invalid_value(
                        serde::de::Unexpected::Str(resource_type.as_str()),
                        &expected_type,
                    ));
                }
                Ok(InputDataWrapper(attributes))
            }
        }
        const FIELDS: &'static [&str] = &["type", "attributes"];
        deserializer.deserialize_struct(
            "InputDataWrapper",
            FIELDS,
            CreateResourceVisistor {
                marker: PhantomData::<InputData>,
                lifetime: PhantomData,
            },
        )
    }
}

/// Data type representing the deserialized document of a json:api POST request
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonApiCreateResource<InputData: ResourceType> {
    pub data: InputDataWrapper<InputData>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateWrapper {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub attributes: Map<String, Value>,
}

/// Data type representing the deserialized document of a json:api PATCH request
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonApiUpdateResource {
    pub data: UpdateWrapper,
}

#[cfg(test)]
mod test_create_resource {
    use crate::core::input_data::{InputDataWrapper, JsonApiCreateResource};
    use crate::ResourceType;
    use serde::Deserialize;

    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        message: String,
        stock: i64,
    }

    impl ResourceType for Test {
        fn get_type() -> &'static str {
            &"Test"
        }
    }

    #[test]
    fn deserialize_resource_object() {
        let resource_object_json_raw = r#"
            {
                "type": "Test",
                "attributes": {
                    "message": "Hello",
                    "stock": 12
                }
            }
        "#;
        let resource_object_test: InputDataWrapper<Test> =
            serde_json::from_str(resource_object_json_raw).unwrap();
        assert_eq!(
            *resource_object_test,
            Test {
                message: String::from("Hello"),
                stock: 12
            }
        )
    }

    #[test]
    fn deserialize_data_resource_object() {
        let resource_object_json_raw = r#"
            {
                "data": {
                    "type": "Test",
                    "attributes": {
                        "message": "Hello",
                        "stock": 12
                    }
                }
            }
        "#;
        let resource_object_test: JsonApiCreateResource<Test> =
            serde_json::from_str(resource_object_json_raw).unwrap();
        assert_eq!(
            *resource_object_test.data,
            Test {
                message: String::from("Hello"),
                stock: 12
            }
        )
    }

    #[test]
    fn deserialize_resource_object_invalid_type() {
        let resource_object_json_raw = r#"
            {
                "type": "NotTheRightType",
                "attributes": {
                    "message": "Hello",
                    "stock": 12
                }
            }
        "#;
        let resource_object_test: serde_json::error::Result<InputDataWrapper<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }

    #[test]
    fn deserialize_resource_object_invalid_object() {
        let resource_object_json_raw = r#"
            {
                "type": "NotTheRightType",
                "attributes": [
                    "message": "Hello",
                    "stock": 12
                ]
            }
        "#;
        let resource_object_test: serde_json::error::Result<InputDataWrapper<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_syntax()),
        }
    }

    #[test]
    fn deserialize_resource_object_invalid_attributes() {
        let resource_object_json_raw = r#"
            {
                "type": "NotTheRightType",
                "attributes": {
                    "message": "Hello",
                    "stock": 12,
                    "should_not_be_here": true
                }
            }
        "#;
        let resource_object_test: serde_json::error::Result<InputDataWrapper<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }

    #[test]
    fn deserialize_resource_object_duplicate_field() {
        let resource_object_json_raw = r#"
            {
                "type": "Test",
                "type": "Test",
                "attributes": {
                    "message": "Hello",
                    "stock": 12
                }
            }
        "#;
        let resource_object_test: serde_json::error::Result<InputDataWrapper<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }

    #[test]
    fn deserialize_resource_object_invalid_unknown_fields() {
        let resource_object_json_raw = r#"
            {
                "should_not_be_here": true,
                "type": "Test",
                "attributes": {
                    "message": "Hello",
                    "stock": 12
                }
            }
        "#;
        let resource_object_test: serde_json::error::Result<InputDataWrapper<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }

    #[test]
    fn deserialize_data_resource_object_invalid_field() {
        let resource_object_json_raw = r#"
            {
                "sometinh_other_than_data": {
                    "type": "Test",
                    "attributes": {
                        "message": "Hello",
                        "stock": 12
                    }
                }
            }
        "#;
        let resource_object_test: serde_json::Result<JsonApiCreateResource<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }

    #[test]
    fn deserialize_data_resource_object_invalid_unknown_fields() {
        let resource_object_json_raw = r#"
            {
                "data": {
                    "type": "Test",
                    "attributes": {
                        "message": "Hello",
                        "stock": 12
                    }
                },
                "should_not_be_here": true
            }
        "#;
        let resource_object_test: serde_json::Result<JsonApiCreateResource<Test>> =
            serde_json::from_str(resource_object_json_raw);
        match resource_object_test {
            Ok(_res) => assert!(false),
            Err(err) => assert!(err.is_data()),
        }
    }
}

#[cfg(test)]
mod test_update_resource {
    use crate::core::input_data::{JsonApiUpdateResource, UpdateWrapper};
    use crate::ResourceType;
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        a: String,
        b: i64,
        c: bool,
    }

    impl ResourceType for Test {
        fn get_type() -> &'static str {
            &"Test"
        }
    }

    #[test]
    fn deserialize_patch_object() {
        let resource_object_json_raw = r#"
            {
                "id": "5",
                "type": "Test",
                "attributes": {
                    "message": "Hello"
                }
            }
        "#;
        let resource_object_test: UpdateWrapper =
            serde_json::from_str(resource_object_json_raw).unwrap();
        assert_eq!(resource_object_test.id.as_str(), "5");
        assert_eq!(resource_object_test.resource_type.as_str(), "Test");
        assert!(resource_object_test.attributes.contains_key("message"));
        assert!(match resource_object_test.attributes.get("message") {
            Some(val) => match val {
                Value::String(string) => string.as_str() == "Hello",
                _ => false,
            },
            None => false,
        });
    }

    #[test]
    fn deserialize_data_patch_object() {
        let resource_object_json_raw = r#"
            {
                "data": {
                    "id": "5",
                    "type": "Test",
                    "attributes": {
                        "message": "Hello"
                    }
                }
            }
        "#;
        let resource_object_test: JsonApiUpdateResource =
            serde_json::from_str(resource_object_json_raw).unwrap();
        assert_eq!(resource_object_test.data.id.as_str(), "5");
        assert_eq!(resource_object_test.data.resource_type.as_str(), "Test");
        assert!(resource_object_test.data.attributes.contains_key("message"));
        assert!(match resource_object_test.data.attributes.get("message") {
            Some(val) => match val {
                Value::String(string) => string.as_str() == "Hello",
                _ => false,
            },
            None => false,
        });
    }
}
