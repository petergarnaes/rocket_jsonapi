use crate::lib::*;
use crate::data::{ResourceIdentifiable, ResourceIdentifier, ResourceObjectType};
use crate::data::ResourceObjectType::{Single};

#[derive(Serialize)]
pub struct DataObject<'a, Data: Serialize> {
    id: String,
    #[serde(rename = "type")]
    object_type: &'a str,
    attributes: &'a Data
}

pub fn create_data_object<Data: ResourceIdentifiable + Serialize>(d: &Data) -> DataObject<Data> {
    DataObject {
        id: d.get_id(),
        object_type: d.get_type(),
        attributes: d
    }
}

pub fn to_resource_identifier<R: ResourceIdentifiable>(res: &R) -> ResourceObjectType<ResourceIdentifier> {
    Single(ResourceIdentifier {id: res.get_id(), object_type: res.get_type() })
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
