use crate::lib::*;
use crate::links::JsonApiLinks;
use crate::relationship::RelationObject;

pub trait ResourceIdentifiable {
    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> String;
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
    pub object_type: &'static str
}

impl ResourceIdentifier {
    pub fn create_identifier<T: ResourceIdentifiable>(resource: &T) -> Self {
        ResourceIdentifier { id: resource.get_id(), object_type: resource.get_type() }
    }
}

impl ResourceIdentifiable for ResourceIdentifier {
    fn get_type(&self) -> &'static str {
        self.object_type
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

pub enum ResourceObjectType<Data> {
    Single(Data),
    Multiple(Vec<Data>),
    NoResource
}

// Should _probably_ not be used?
pub trait ResourceObjectable<Data: ResourceIdentifiable> {
    fn get_data(&self) -> PrimaryObjectType<Data>;
    fn get_link(&self) -> Option<JsonApiLinks>;
    //fn get_relationships(&self) -> Option<Relationships>;
}

pub enum PrimaryObjectType<Data: ResourceIdentifiable> {
    Single(Data),
    Multiple(Vec<Data>)
}


pub struct JsonApiResourceObject<Data: ResourceIdentifiable> {
    pub data: PrimaryObjectType<Data>,
    pub links: Option<JsonApiLinks>,
    // TODO how do we figure out the type of the relationship?
    //pub relationships: Option<RelationObject>,
    //pub relationshipz: Option<Vec<Box<dyn Relationshipify>>>
    // TODO meta
}

// Struct for data, will be parsed correctly
//#[derive(Serialize)]
pub struct JsonApiPrimaryDataObject<Data: ResourceIdentifiable, Included> {
    pub data: PrimaryObjectType<Data>,
    pub links: Option<JsonApiLinks>,
    // Primary Data specific, should also be dynamic
    pub included: Option<Vec<Included>>,
    // TODO relationships
    //pub relationships: Option<Vec<Box<dyn ResourceIdentifiable>>>
}

impl<Data: ResourceIdentifiable> JsonApiPrimaryDataObject<Data, ()> {
    pub fn from_data(data: PrimaryObjectType<Data>) -> JsonApiPrimaryDataObject<Data, ()> {
        //JsonApiPrimaryDataObject {data, links: None, included: None, relationships: None::<_> }
        JsonApiPrimaryDataObject {data, links: None, included: None }
    }
    pub fn from_data_links(data: PrimaryObjectType<Data>, links: JsonApiLinks) -> JsonApiPrimaryDataObject<Data, ()> {
        //JsonApiPrimaryDataObject { data, links: Some(links), included: None, relationships: None::<_> }
        JsonApiPrimaryDataObject { data, links: Some(links), included: None }
    }
}

impl<Data: ResourceIdentifiable, Included> JsonApiPrimaryDataObject<Data, Included> {
    pub fn from_data_links_included(data: PrimaryObjectType<Data>, links: JsonApiLinks, included: Vec<Included>) -> JsonApiPrimaryDataObject<Data, Included> {
        //JsonApiPrimaryDataObject { data, links: Some(links), included: Some(included), relationships: None::<_> }
        JsonApiPrimaryDataObject { data, links: Some(links), included: Some(included) }
    }
}

