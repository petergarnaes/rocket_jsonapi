use serde::Serialize;
use rocket_jsonapi::data::{ResourceIdentifiable, PrimaryObjectType, ResourceObjectType};
use rocket_jsonapi::links::{Linkify, LinksObject, LinkObject};
use rocket_jsonapi::links::LinksObject::{Object};
use rocket_jsonapi::relationship::{HaveRelationship, RelationObjectify, AllRelationships, RelationObject};

#[derive(Serialize)]
struct ArticleLinkMeta {
    message: &'static str
}

const ARTICLE_LINK_META_MESSAGE: &'static str = "It works!";
const ARTICLE_LINK_META: ArticleLinkMeta = ArticleLinkMeta { message: ARTICLE_LINK_META_MESSAGE };

struct Article {
    id: i32,
    title: String,
    article: String
}

impl ResourceIdentifiable for Article {
    fn get_type(&self) -> &'static str {
        &"article"
    }

    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Linkify for Article {
    fn get_links() -> Vec<LinksObject> {
        vec![Object("self".to_string(), LinkObject::new("".to_owned(), Box::new(ARTICLE_LINK_META)))]
    }
}

impl HaveRelationship<Author> for Article {
    fn get_relation(&self) -> Author {
        Author(Person {id: 1, name: "Test Mac Testy".to_owned() })
    }
}

impl HaveRelationship<ProofReader> for Article {
    fn get_relation(&self) -> ProofReader {
        ProofReader(Person {id: 2, name: "Naw Ni Nu".to_owned() })
    }
}

/*
*/
impl AllRelationships for Article {
    fn get_all_relation_objects(&self) -> Vec<RelationObject> {
        vec![
            <dyn RelationObjectify<Author>>::get_relation_object(self),
            <dyn RelationObjectify<ProofReader>>::get_relation_object(self)
        ]
    }
}


struct Person {
    id: i32,
    name: String
}

impl ResourceIdentifiable for Person {
    fn get_type(&self) -> &'static str {
        &"person"
    }

    fn get_id(&self) -> String {
        self.id.to_string()
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
    fn get_type(&self) -> &'static str {
        self.0.get_type()
    }

    fn get_id(&self) -> String {
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
    fn get_type(&self) -> &'static str {
        self.0.get_type()
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }
}

impl Linkify for ProofReader {}