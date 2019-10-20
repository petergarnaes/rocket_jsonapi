use crate::data::ResourceObjectType::Single;
use crate::data::{ResourceIdentifiable, ResourceIdentifier, ResourceObjectType};
use crate::lib::*;

#[derive(Serialize)]
pub struct DataObject<'a, Data: Serialize> {
    id: String,
    #[serde(rename = "type")]
    object_type: &'a str,
    attributes: &'a Data,
}

pub fn create_data_object<Data>(d: &Data) -> DataObject<Data>
where
    Data: ResourceIdentifiable + Serialize,
{
    DataObject {
        id: d.get_id().to_string(),
        object_type: d.get_type(),
        attributes: d,
    }
}
