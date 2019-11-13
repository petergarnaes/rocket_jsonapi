//! # Responding with relationship metadata
use crate::core::resource_identifier::ResourceIdentifierObject;
use crate::lib::*;

//pub type Relationship = Box<dyn ResourceIdentifiable>;
//pub type Relationships = Vec<Relationship>;

#[derive(Serialize)]
struct ResIdenObjNonGeneric {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: &'static str,
}

impl<To> From<ResourceIdentifierObject<To>> for ResIdenObjNonGeneric
where
    To: ResourceIdentifiable,
{
    fn from(res: ResourceIdentifierObject<To>) -> Self {
        ResIdenObjNonGeneric {
            // TODO clone needed?
            id: res.get_id().to_string(),
            object_type: To::get_type(),
        }
    }
}

impl<Data> From<&Data> for ResIdenObjNonGeneric
where
    Data: ResourceIdentifiable,
{
    default fn from(data: &Data) -> Self {
        ResIdenObjNonGeneric {
            // TODO clone needed?
            id: data.get_id().to_string(),
            object_type: Data::get_type(),
        }
    }
}

pub struct RelationObject {
    data: Vec<ResIdenObjNonGeneric>,
    links: String,
}

pub trait RelationObjectify<'a, To>: HaveRelationship<'a, To> {
    fn get_relation_object(&'a self) -> RelationObject;
}

pub trait HaveRelationship<'a, To> {
    fn get_relation(&'a self) -> To;
}

pub trait AllRelationships {
    fn get_all_relation_objects(&self) -> Vec<RelationObject>;
}

trait RelationObjectifyMeta<'a, Meta, To>: RelationObjectify<'a, To> {
    fn get_meta() -> Meta;
}

impl<'a, From, To> RelationObjectify<'a, To> for From
where
    To: ResourceIdentifiable + Linkify,
    From: HaveRelationship<'a, To>,
{
    default fn get_relation_object(&'a self) -> RelationObject {
        let rel = self.get_relation();
        RelationObject {
            data: vec![ResIdenObjNonGeneric {
                id: rel.get_id().to_string(),
                object_type: To::get_type(),
            }],
            links: "".to_owned(),
        }
    }
}

impl<'a, From, To> RelationObjectify<'a, Vec<To>> for From
where
    To: ResourceIdentifiable + Linkify,
    From: HaveRelationship<'a, Vec<To>>,
{
    fn get_relation_object(&'a self) -> RelationObject {
        let rel = self.get_relation();
        RelationObject {
            data: rel.iter().map(|to| to.into()).collect(),
            links: "".to_owned(),
        }
    }
}
