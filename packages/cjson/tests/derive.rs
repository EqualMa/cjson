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
struct TransparentImplicit(u8);

#[derive(ToJson)]
#[cjson(transparent)]
struct TransparentExplicit<'a>(&'a str);

#[derive(ToJson)]
#[cjson(where = (T: ToJson))]
#[cjson(transparent)]
struct TransparentExplicitNamed<T> {
    only: T,
}

#[test]
fn transparent() {
    assert_json_eq!(TransparentImplicit(56), "56");
    assert_json_eq!(TransparentExplicit("56"), "\"56\"");
    assert_json_eq!(TransparentExplicitNamed { only: false }, "false");
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

#[derive(ToJson)]
enum EnumOnlyUnit {
    OnlyUnit,
}

#[derive(ToJson)]
enum EnumOne {
    Only(),
}

#[derive(ToJson)]
enum EnumMany {
    First(),
    Second,
    Third {},
    Runtime { v: u8 },
}

#[test]
fn enums() {
    assert_json_eq!(EnumOnlyUnit::OnlyUnit, "\"OnlyUnit\"");
    assert_json_eq!(EnumOne::Only(), r#"{"Only":[]}"#);
    assert_json_eq!(EnumMany::First(), r#"{"First":[]}"#);
    assert_json_eq!(EnumMany::Second, r#""Second""#);
    assert_json_eq!(EnumMany::Third {}, r#"{"Third":{}}"#);
    assert_json_eq!(EnumMany::Runtime { v: 1 }, r#"{"Runtime":{"v":1}}"#);
}
