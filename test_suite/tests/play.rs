#![allow(dead_code)]
use rocket_jsonapi::links::LinksObject::Object;
use rocket_jsonapi::links::{LinkObject, Linkify, LinksObject};
use rocket_jsonapi::relationship::{
    AllRelationships, HaveRelationship, RelationObject, RelationObjectify,
};
use rocket_jsonapi::ResourceIdentifiable;
use serde::Serialize;

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
}

impl ResourceIdentifiable for Article {
    type IdType = i32;

    fn get_type(&self) -> &'static str {
        &"article"
    }

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

impl HaveRelationship<Author> for Article {
    fn get_relation(&self) -> Author {
        Author(Person {
            id: 1,
            name: "Test Mac Testy".to_owned(),
        })
    }
}

impl HaveRelationship<ProofReader> for Article {
    fn get_relation(&self) -> ProofReader {
        ProofReader(Person {
            id: 2,
            name: "Naw Ni Nu".to_owned(),
        })
    }
}

/*
*/
impl AllRelationships for Article {
    fn get_all_relation_objects(&self) -> Vec<RelationObject> {
        vec![
            <dyn RelationObjectify<Author>>::get_relation_object(self),
            <dyn RelationObjectify<ProofReader>>::get_relation_object(self),
        ]
    }
}

struct Person {
    id: i32,
    name: String,
}

impl ResourceIdentifiable for Person {
    type IdType = i32;

    fn get_type(&self) -> &'static str {
        &"person"
    }

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
struct Author(Person);

impl ResourceIdentifiable for Author {
    type IdType = i32;

    fn get_type(&self) -> &'static str {
        self.0.get_type()
    }

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

impl ResourceIdentifiable for ProofReader {
    type IdType = i32;

    fn get_type(&self) -> &'static str {
        self.0.get_type()
    }

    fn get_id(&self) -> &Self::IdType {
        self.0.get_id()
    }
}

impl Linkify for ProofReader {}
