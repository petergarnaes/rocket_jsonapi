use crate::lib::*;

pub trait ResourceIdentifiable {
    type IdType: ToString;

    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> &Self::IdType;
}

// TODO maybe move to core, and hide for user
#[derive(Serialize)]
pub struct ResourceIdentifier {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: &'static str,
}

impl ResourceIdentifier {
    pub fn create_identifier<T: ResourceIdentifiable>(resource: &T) -> Self {
        ResourceIdentifier {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl<T: ResourceIdentifiable> From<&T> for ResourceIdentifier {
    fn from(resource: &T) -> Self {
        ResourceIdentifier {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl ResourceIdentifiable for ResourceIdentifier {
    type IdType = String;

    fn get_type(&self) -> &'static str {
        self.object_type
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

// TODO remove below
pub enum ResourceObjectType<Data> {
    Single(Data),
    Multiple(Vec<Data>),
    NoResource,
}
