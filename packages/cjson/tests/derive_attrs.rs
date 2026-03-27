#![cfg(feature = "proc-macro")]

use cjson::{self as my_json, ToJson, ser::exts::TextExt as _};

macro_rules! assert_json_eq {
    ($v:expr,$eq:expr) => {
        assert_eq!($v.to_json().into_string().into_inner(), $eq)
    };
}

#[derive(ToJson)]
#[cjson(to("Struct To"))]
struct StructTo;

#[test]
fn struct_to() {
    assert_json_eq!(StructTo, "\"Struct To\"");
}

#[derive(ToJson)]
struct StructFieldTo(#[cjson(to("Struct Field To"))] ());

#[test]
fn struct_field_to() {
    assert_json_eq!(StructFieldTo(()), "\"Struct Field To\"");
}
