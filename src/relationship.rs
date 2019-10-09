use crate::lib::*;
use crate::data::{ResourceIdentifiable, ResourceIdentifier, ResourceObjectType};
use crate::core::data_object::to_resource_identifier;
use std::marker::PhantomData;
use crate::data::ResourceObjectType::{NoResource, Single, Multiple};
use crate::meta::Metafiable;

//pub type Relationship = Box<dyn ResourceIdentifiable>;
//pub type Relationships = Vec<Relationship>;

pub struct RelationObject {
    data: ResourceObjectType<ResourceIdentifier>,
    links: String
}

pub trait HaveRelationship<To> {
    fn get_relation(&self) -> To;
}

pub trait RelationObjectify<T>: HaveRelationship<T> {
    fn get_relation_object(&self) -> RelationObject;
}

pub trait AllRelationships {
    fn get_all_relation_objects(&self) -> Vec<RelationObject>;
}

trait RelationObjectifyMeta<To, Meta>: RelationObjectify<To> {
    fn get_meta() -> Meta;
}

default impl<From, To> RelationObjectify<To> for From where To: ResourceIdentifiable, From: HaveRelationship<To> {
    fn get_relation_object(&self) -> RelationObject {
        let rel = self.get_relation();
        //RelationObject { data: to_resource_identifier(&rel), links: NoLink }
        RelationObject { data: Single(ResourceIdentifier {id: rel.get_id(), object_type: rel.get_type() }), links: "".to_owned() }
    }
}

default impl<From, To> RelationObjectify<Vec<To>> for From where To: ResourceIdentifiable, From: HaveRelationship<Vec<To>> {
    fn get_relation_object(&self) -> RelationObject {
        let rel = self.get_relation();
        //RelationObject { data: to_resource_identifier(&rel), links: NoLink }
        let res_idents = rel.iter().map(|r| ResourceIdentifier {id: r.get_id(), object_type: r.get_type()}).collect();
        RelationObject { data: Multiple(res_idents), links: "".to_owned() }
        //RelationObject { data: Single(ResourceIdentifier {id: rel.get_id(), object_type: rel.get_type() }), links: NoLink }
    }
}

// TODO implement for a set of objects that could be different? Can be done with a sort of wrapper type with the current API

/*
impl<From, To> RelationObjectify<To> for From where To: Linkifiable, From: HaveRelationship<To> {
    fn get_relation_object(&self) -> RelationObject {
        let rel = self.get_relation();
        RelationObject { data: to_resource_identifier(&rel), links: self.get_href() }
    }
}
*/

/*
impl<From, To, Meta> RelationObjectifyMeta<To, Meta> for From where To: ResourceIdentifiable + HaveRelationship<To> {
    fn get_relation_object(&self) -> RelationObject {
        let rel = self.get_relation();
        RelationObject { data: to_resource_identifier(&rel), links: self.get_href() }
    }
}
*/

pub trait RelationObjectifyResIden<T: ResourceIdentifiable>: HaveRelationship<T> {
    fn get_relation_object(&self) -> RelationObject;
}

pub trait RelationObjectifyLink<T>: HaveRelationship<T> {
    fn get_relation_object(&self) -> RelationObject;
}

impl<From, To> RelationObjectifyResIden<To> for From where To: ResourceIdentifiable, From: HaveRelationship<To> {
    fn get_relation_object(&self) -> RelationObject {
        let rel = self.get_relation();
        //RelationObject { data: to_resource_identifier(&rel), links: NoLink }
        RelationObject { data: Single(ResourceIdentifier {id: rel.get_id(), object_type: rel.get_type() }), links: "".to_owned() }
    }
}

// TODO collect all implementations of HaveRelationship for some type...
// Maybe make user implement some trait and function that return all, something like:
// vec!(HaveRelationship<Author>::get_relation_object(&article), HaveRelationship<ProofReader>::get_relation_object(&article), ...)
// Maybe a macro could do this? Still need some dyn Trait magic to abstract over all the different
// type implementations...
// OR! Remove the PhantomData on ResourceIdentifier, and make the to_resource_identifier use the feature
// where function arguments or return types can be traits (ie. fn foo(impl Trait)). This means that
// RelationObject does not have to be type dependent either, so we can construct it without type
// constraint issues.
