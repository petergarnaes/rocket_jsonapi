use crate::core::data_object::create_data_object;
use crate::lib::*;
use crate::links::Linkify;

pub trait ResourceIdentifiable {
    type IdType: ToString;

    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> &Self::IdType;
}

/*
impl ResourceIdentifiable for Box<dyn ResourceIdentifiable> {
    fn get_type() -> &'static str {
        RI::get_type()
    }

    fn get_id(&self) -> String {
        (**self).get_id()
    }
}
*/

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

pub enum ResourceObjectType<Data> {
    Single(Data),
    Multiple(Vec<Data>),
    NoResource,
}

pub enum PrimaryObjectType<Data: ResourceIdentifiable> {
    Single(Data),
    Multiple(Vec<Data>),
}

pub struct JsonApiResourceObject<Data: ResourceIdentifiable> {
    pub data: PrimaryObjectType<Data>,
    //pub links: Option<JsonApiLinks>,
    // TODO how do we figure out the type of the relationship?
    //pub relationships: Option<RelationObject>,
    //pub relationshipz: Option<Vec<Box<dyn Relationshipify>>>
    // TODO meta
}

// Struct for data, will be parsed correctly
//#[derive(Serialize)]
pub struct JsonApiPrimaryDataObject<Data: ResourceIdentifiable, Included> {
    pub data: PrimaryObjectType<Data>,
    // Primary Data specific, should also be dynamic
    pub included: Option<Vec<Included>>,
    // TODO relationships
    //pub relationships: Option<Vec<Box<dyn ResourceIdentifiable>>>
}

impl<'a, Data: ResourceIdentifiable> JsonApiPrimaryDataObject<Data, ()> {
    pub fn from_data(data: PrimaryObjectType<Data>) -> JsonApiPrimaryDataObject<Data, ()> {
        //JsonApiPrimaryDataObject {data, links: None, included: None, relationships: None::<_> }
        JsonApiPrimaryDataObject {
            data,
            included: None,
        }
    }
    pub fn from_data_links(data: PrimaryObjectType<Data>) -> JsonApiPrimaryDataObject<Data, ()> {
        //JsonApiPrimaryDataObject { data, links: Some(links), included: None, relationships: None::<_> }
        JsonApiPrimaryDataObject {
            data,
            included: None,
        }
    }
}

impl<Data: ResourceIdentifiable, Included> JsonApiPrimaryDataObject<Data, Included> {
    pub fn from_data_links_included(
        data: PrimaryObjectType<Data>,
        included: Vec<Included>,
    ) -> JsonApiPrimaryDataObject<Data, Included> {
        //JsonApiPrimaryDataObject { data, links: Some(links), included: Some(included), relationships: None::<_> }
        JsonApiPrimaryDataObject {
            data,
            included: Some(included),
        }
    }
}

/*
impl<Data, Included> Serialize for JsonApiPrimaryDataObject<Data, Included>
    where Data: Serialize + ResourceIdentifiable, Included: Serialize {
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 1)?;
        match &self.data {
            PrimaryObjectType::Single(data) => {
                state.serialize_field("data", &create_data_object(data))?;
            },
            PrimaryObjectType::Multiple(data_vec) => {
                let data_object_vec = data_vec.iter().map(create_data_object).collect::<Vec<_>>();
                state.serialize_field("data", &data_object_vec)?;
            }
        };
        match &self.included {
            Some(inclusions) => {
                state.serialize_field("included", &inclusions)?;
            }
            None => {}
        };
        state.end()
    }
}
*/

// TODO could we make this the only implementation, and make default implementation for Linkify and
// whatever I deside on for Relationships? Since these fields are optional it would make sense to
// provide the user the option to return nothing, based on the data object (ie. NoLink enum and
// corresponding in relationship). It would also make gradual and modular implementation possible,
// ie. only implement Linkify, or only relationship, etc.
impl<Data, Included> Serialize for JsonApiPrimaryDataObject<Data, Included>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
    Included: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 1)?;
        match &self.data {
            PrimaryObjectType::Single(data) => {
                state.serialize_field("data", &create_data_object(data))?;
            }
            PrimaryObjectType::Multiple(data_vec) => {
                let data_object_vec = data_vec.iter().map(create_data_object).collect::<Vec<_>>();
                state.serialize_field("data", &data_object_vec)?;
            }
        };
        let links = Data::get_links();
        match links.len() {
            0 => {
                // TODO do not parse the links field
            }
            _ => {
                // TODO parse each element as a nested object in parent links object, use provided key
            }
        }
        match &self.included {
            Some(inclusions) => {
                state.serialize_field("included", &inclusions)?;
            }
            None => {}
        };
        state.end()
    }
}
