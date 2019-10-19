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

pub fn to_resource_identifier<R: ResourceIdentifiable>(
    res: &R,
) -> ResourceObjectType<ResourceIdentifier> {
    Single(ResourceIdentifier {
        id: res.get_id().to_string(),
        object_type: res.get_type(),
    })
}
/*
pub fn to_resource_identifier<R: ResourceIdentifiable>(res: &ResourceObjectType<R>) -> ResourceObjectType<ResourceIdentifier> {
    match res {
        Single(resource) => Single(ResourceIdentifier::create_identifier(resource)),
        Multiple(resource_vec) => Multiple(
            resource_vec.iter().map(|r| ResourceIdentifier::create_identifier(r)).collect()
        ),
        NoResource => NoResource
    }
}
*/
