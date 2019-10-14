// These test simply verifies that rocket_jsonapi_derive is able to produce code that compiles.

use rocket_jsonapi::Linkify;

#[test]
fn test_gen() {
    #[derive(Linkify)]
    struct Simple {
        name: String,
        count: u32
    }
    assert_ser::<Simple>()
}

fn assert_ser<T: Linkify>() {}
