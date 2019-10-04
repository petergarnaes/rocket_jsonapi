use rocket_jsonapi::data::{ResourceIdentifiable, PrimaryObjectType, ResourceObjectType};
use rocket_jsonapi::links::{Linkify, Links};
use rocket_jsonapi::links::Links::{LinksSelf};
use rocket_jsonapi::links::LinksObject::{Url};
use rocket_jsonapi::relationship::{HaveRelationship, RelationObjectify, AllRelationships, RelationObject};
use rocket_jsonapi::data::ResourceObjectType::{Single, Multiple};

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
    type MS = ();
    type MR = ();

    fn produce_link(&self) -> Option<Links<Self::MS, Self::MR>> {
        Some(
            LinksSelf(
                Url(format!("/article/{}", self.id.to_string()))
            )
        )
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
    type MS = ();
    type MR = ();

    fn produce_link(&self) -> Option<Links<Self::MS, Self::MR>> {
        Some(
            LinksSelf(
                Url(format!("/person/{}", self.id.to_string()))
            )
        )
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
    type MS = ();
    type MR = ();

    fn produce_link(&self) -> Option<Links<Self::MS, Self::MR>> {
        Some(
            LinksSelf(
                Url(format!("/person/{}", self.0.id.to_string()))
            )
        )
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

fn test(a: Article) {
    <dyn RelationObjectify<Author>>::get_relation_object(&a);
}

