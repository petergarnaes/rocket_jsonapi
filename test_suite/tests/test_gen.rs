// These test simply verifies that rocket_jsonapi_derive is able to produce code that compiles.

use rocket_jsonapi::Linkify;
use rocket_jsonapi::ResourceIdentifiable;

#[test]
fn test_gen_linkify_simple() {
    #[derive(Linkify)]
    struct Simple {
        name: String,
        count: u32
    }
    assert_linkify::<Simple>();
}

#[test]
fn test_gen_resource_identifiable_simple() {
    #[derive(ResourceIdentifiable)]
    struct SimpleResource {
        id: String,
        bob: Box<String>
    }
    assert_resource_identifiable::<SimpleResource>();
    let simple_resource = SimpleResource {
        id: "1".to_string(),
        bob: Box::new("bob".to_string())
    };
    assert_eq!("SimpleResource", simple_resource.get_type());
    assert_eq!("1", simple_resource.get_id());
}

#[test]
fn test_gen_resource_identifiable_custom_type() {
    #[derive(ResourceIdentifiable)]
    #[resource_ident_type = "test"]
    struct SimpleResourceCustomType {
        id: String,
        bob: Box<String>
    }
    assert_resource_identifiable::<SimpleResourceCustomType>();
    let simple_resource = SimpleResourceCustomType {
        id: "1".to_string(),
        bob: Box::new("bob".to_string())
    };
    assert_eq!("test", simple_resource.get_type());
    assert_eq!("1", simple_resource.get_id());
}

fn assert_linkify<T: Linkify>() {}
fn assert_resource_identifiable<T: ResourceIdentifiable>() {}
