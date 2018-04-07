extern crate chrono;
#[macro_use]
extern crate json_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

mod events;
mod json_uuid;
mod models;

use serde_json::Value;

pub use events::{pc, xbox};
pub use models::{Match, Status};

/// Filters empty links from a "links" section
pub fn filter_empty_links(val: Value) -> Value {
    match val {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|&(_, ref v)| match *v {
                    Value::String(ref s) => !s.is_empty(),
                    _ => true,
                })
                .collect(),
        ),
        _ => val,
    }
}

/// Crawls a tree looking for a "links" section
pub fn find_links(val: Value) -> Value {
    match val {
        Value::Array(arr) => Value::Array(arr.into_iter().map(find_links).collect()),
        Value::Object(obj) => Value::Object(
            obj.into_iter()
                .map(|(k, v)| {
                    if k == "links" {
                        (k, filter_empty_links(v))
                    } else {
                        (k, find_links(v))
                    }
                })
                .collect(),
        ),
        _ => val,
    }
}
