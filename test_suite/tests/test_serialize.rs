#![allow(dead_code)]
// Test that the various parts serialize properly
use rocket_jsonapi::ResourceIdentifiable;
use serde_json::json;
use serde_json::to_string;

#[test]
fn test_serialize_full() {
    let result = json!({
        "": ""
    });
}
