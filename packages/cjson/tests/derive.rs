#![cfg(feature = "proc-macro")]

use cjson::{self as my_json, ToJson, ser::exts::TextExt as _};

macro_rules! assert_json_eq {
    ($v:expr,$eq:expr) => {
        assert_eq!($v.to_json().into_string().into_inner(), $eq)
    };
}

#[derive(ToJson)]
struct UnitStruct;

#[test]
fn unit_struct() {
    assert_json_eq!(UnitStruct, "null");
}

#[derive(ToJson)]
struct UnitTuple();

#[test]
fn unit_tuple() {
    assert_json_eq!(UnitTuple(), "[]");
}

#[derive(ToJson)]
struct Transparent(u8);

#[test]
fn transparent() {
    assert_json_eq!(Transparent(56), "56");
}

#[derive(ToJson)]
struct Tuple(u8, cjson::values::Finite<f32>);

#[test]
fn tuple() {
    assert_json_eq!(
        Tuple(1, cjson::values::Finite::new_f32(2.3).unwrap()),
        "[1,2.3]"
    );
}

#[derive(ToJson)]
struct ObjEmpty {}

#[test]
fn obj_empty() {
    assert_json_eq!(ObjEmpty {}, "{}");
}

#[derive(ToJson)]
#[cjson(crate(my_json))]
struct ObjOneField {
    name: String,
}

#[test]
fn obj_one_field() {
    assert_json_eq!(
        ObjOneField {
            name: "hello\tworld".into(),
        },
        r#"{"name":"hello\tworld"}"#
    );
}

#[derive(ToJson)]
#[cjson(where = (V: ToJson))]
#[cjson(crate(::cjson))]
struct ObjFields<'a, V, const UNUSED: u32> {
    name: &'a str,
    value: V,
}

#[test]
fn obj_fields() {
    assert_json_eq!(
        ObjFields::<_, 0> {
            name: "hello",
            value: 1
        },
        r#"{"name":"hello","value":1}"#
    );
}

#[derive(ToJson)]
enum Never {}

#[test]
fn never() {
    assert_json_eq!(None::<Never>, "null");
}

/* TODO:
#[derive(ToJson)]
enum EnumOnlyUnit {
    OnlyUnit,
}

#[derive(ToJson)]
enum EnumOne {
    Only(),
}

#[test]
fn enum_one() {
    assert_json_eq!(None::<Never>, "null");
}
*/
