#![allow(dead_code)]
use rocket_jsonapi::links::LinksObject::Object;
use rocket_jsonapi::links::{LinkObject, Linkify, LinksObject};
use rocket_jsonapi::relationship::{
    AllRelationships, HaveRelationship, RelationObject, RelationObjectify,
};
use rocket_jsonapi::resource::ResourceType;
use rocket_jsonapi::ResourceIdentifiable;
use serde::Serialize;
use std::rc::Rc;

#[derive(Serialize)]
struct ArticleLinkMeta {
    message: &'static str,
}

const ARTICLE_LINK_META_MESSAGE: &'static str = "It works!";
const ARTICLE_LINK_META: ArticleLinkMeta = ArticleLinkMeta {
    message: ARTICLE_LINK_META_MESSAGE,
};

struct Article {
    id: i32,
    title: String,
    article: String,
    author: Author,
    proof_reader: ProofReader,
}

impl ResourceType for Article {
    fn get_type() -> &'static str {
        &"article"
    }
}

impl ResourceIdentifiable for Article {
    type IdType = i32;

    fn get_id(&self) -> &i32 {
        &self.id
    }
}

impl Linkify for Article {
    fn get_links() -> Vec<LinksObject> {
        vec![Object(
            "self".to_string(),
            LinkObject::new("".to_owned(), Box::new(ARTICLE_LINK_META)),
        )]
    }
}

impl HaveRelationship<'_, Author> for Article {
    fn get_relation(&self) -> Author {
        Author(self.author.0.clone())
    }
}

impl<'a> HaveRelationship<'a, &'a ProofReader> for Article {
    fn get_relation(&'a self) -> &'a ProofReader {
        &self.proof_reader
    }
}

/*
*/
impl AllRelationships for Article {
    fn get_all_relation_objects(&self) -> Vec<RelationObject> {
        vec![
            <dyn RelationObjectify<Author>>::get_relation_object(self),
            <dyn RelationObjectify<&ProofReader>>::get_relation_object(self),
        ]
    }
}

struct Person {
    id: i32,
    name: String,
}

impl ResourceType for Person {
    fn get_type() -> &'static str {
        &"person"
    }
}

impl ResourceIdentifiable for Person {
    type IdType = i32;

    fn get_id(&self) -> &Self::IdType {
        &self.id
    }
}

impl Linkify for Person {
    fn get_links() -> Vec<LinksObject> {
        unimplemented!()
    }
}

//type Author = Person;
struct Author(Rc<Person>);

impl ResourceType for Author {
    fn get_type() -> &'static str {
        Person::get_type()
    }
}

impl ResourceIdentifiable for Author {
    type IdType = i32;

    fn get_id(&self) -> &Self::IdType {
        self.0.get_id()
    }
}

impl Linkify for Author {
    fn get_links() -> Vec<LinksObject> {
        unimplemented!()
    }
}

struct ProofReader(Person);

impl ResourceType for &ProofReader {
    fn get_type() -> &'static str {
        Person::get_type()
    }
}

impl ResourceIdentifiable for &ProofReader {
    type IdType = i32;

    fn get_id(&self) -> &Self::IdType {
        self.0.get_id()
    }
}

impl Linkify for &ProofReader {}
