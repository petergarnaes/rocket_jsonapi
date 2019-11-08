#![allow(dead_code)]
// These test simply verifies that rocket_jsonapi_derive is able to produce code that compiles.

use rocket_jsonapi::Linkify;
use rocket_jsonapi::{ResourceIdentifiable, ResourceType};

#[test]
fn test_gen_linkify_simple() {
    #[derive(Linkify)]
    struct Simple {
        name: String,
        count: u32,
    }
    assert_linkify::<Simple>();
}

#[test]
fn test_gen_resource_identifiable_simple() {
    #[derive(ResourceType, ResourceIdentifiable)]
    struct SimpleResource {
        id: String,
        bob: Box<String>,
    }
    assert_resource_identifiable::<SimpleResource>();
    let simple_resource = SimpleResource {
        id: "1".to_string(),
        bob: Box::new("bob".to_string()),
    };
    assert_eq!("SimpleResource", simple_resource.get_type());
    assert_eq!("1", simple_resource.get_id());
}

#[test]
fn test_gen_resource_identifiable_custom_type() {
    #[derive(ResourceType, ResourceIdentifiable)]
    #[resource_ident_type = "test"]
    struct SimpleResourceCustomType {
        id: String,
        bob: Box<String>,
    }
    assert_resource_identifiable::<SimpleResourceCustomType>();
    let simple_resource = SimpleResourceCustomType {
        id: "1".to_string(),
        bob: Box::new("bob".to_string()),
    };
    assert_eq!("test", simple_resource.get_type());
    assert_eq!("1", simple_resource.get_id());
}

#[test]
fn test_gen_resource_identifiable_id_i32() {
    #[derive(ResourceType, ResourceIdentifiable)]
    struct Resource {
        id: i32,
        message: String,
    }
    assert_resource_identifiable::<Resource>();
    let resource = Resource {
        id: 1,
        message: "test".to_string(),
    };
    let test_id: i32 = 1;
    assert_eq!(test_id, *resource.get_id());
}

#[derive(Debug)]
struct CustomId {
    part1: i32,
    part2: bool,
}

impl ToString for CustomId {
    fn to_string(&self) -> String {
        format!("{}/{}", self.part1, self.part2)
    }
}

impl PartialEq for CustomId {
    fn eq(&self, other: &Self) -> bool {
        self.part1.eq(&other.part1) && self.part2.eq(&other.part2)
    }
}

#[test]
fn test_gen_resource_identifiable_id_custom() {
    #[derive(ResourceType, ResourceIdentifiable)]
    struct Resource {
        id: CustomId,
        message: String,
    }
    assert_resource_identifiable::<Resource>();
    let resource = Resource {
        id: CustomId {
            part1: 5,
            part2: false,
        },
        message: "test".to_string(),
    };
    let test_id = CustomId {
        part1: 5,
        part2: false,
    };
    assert_eq!(test_id, *resource.get_id());
}

fn assert_linkify<T: Linkify>() {}
fn assert_resource_type<T: ResourceType>() {}
fn assert_resource_identifiable<T: ResourceIdentifiable>() {}
