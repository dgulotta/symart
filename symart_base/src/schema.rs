use serde_json::{json, Value};
use strum::IntoEnumIterator;

use crate::symmetry::SymmetryGroup;

pub trait JsonSchema {
    fn schema() -> Value;
}

pub fn size_even() -> Value {
    json!({
        "type": "integer",
        "title": "Size",
        "minumum": 2,
        "maximum": 65536,
        "multipleOf": 2,
        "default": 256
    })
}

pub fn size() -> Value {
    json!({
        "type": "integer",
        "title": "Size",
        "minimum": 1,
        "maximum": 65536,
        "default": 256
    })
}

pub fn width() -> Value {
    json!({
        "type": "integer",
        "title": "Width",
        "minimum": 1,
        "maximum": 65536,
        "default": 1600
    })
}

pub fn height() -> Value {
    json!({
        "type": "integer",
        "title": "Height",
        "minimum": 1,
        "maximum": 65536,
        "default": 900
    })
}

pub fn num_colors() -> Value {
    json!({
        "type": "integer",
        "title": "Colors",
        "minimum": 1,
        "maximum": 65536,
        "default": 25
    })
}

pub fn enum_strings<T>() -> Vec<String>
where
    T: IntoEnumIterator + ?Sized,
    T::Iterator: Iterator,
    <T::Iterator as Iterator>::Item: std::fmt::Display,
{
    T::iter().map(|x| format!("{}", x)).collect()
}

pub fn symmetries() -> Value {
    let mut v = enum_strings::<SymmetryGroup>();
    v.push("Random".to_string());
    json!({
        "type": "string",
        "title": "Symmetry",
        "enum": v,
        "default": "Random"
    })
}

pub fn require_all(v: &mut Value) {
    let keys = v["properties"]
        .as_object()
        .unwrap()
        .keys()
        .map(|v| Value::String(v.clone()))
        .collect();
    v["required"] = Value::Array(keys);
}
