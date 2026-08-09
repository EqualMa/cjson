#![cfg(feature = "proc-macro")]

use cjson::{self as my_json, IntoAndToJson, IntoJson, ToJson};

macro_rules! assert_json_eq {
    ($v:expr, $eq:expr) => {
        assert_eq!(::cjson::ser::ToJsonExt::to_json_as_string(&$v), $eq);
        assert_eq!(
            ::cjson::ser::ToJsonExt::to_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>(&$v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
        // TODO: test async try
        assert_eq!(::cjson::ser::IntoJsonExt::into_json_as_string($v), $eq);
        assert_eq!(
            ::cjson::ser::IntoJsonExt::into_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>($v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
    };
}

#[derive(IntoAndToJson)]
struct UnitStruct;

#[test]
fn unit_struct() {
    assert_json_eq!(UnitStruct, "null");
}

#[derive(IntoAndToJson)]
struct UnitTuple();

#[test]
fn unit_tuple() {
    assert_json_eq!(UnitTuple(), "[]");
}

#[derive(IntoAndToJson)]
struct TransparentImplicit(u8);

#[derive(IntoAndToJson)]
#[cjson(transparent)]
struct TransparentExplicit<'a>(&'a str);

#[derive(IntoAndToJson)]
#[cjson(
    where_to = (T: ToJson),
    where_into = (T: IntoJson),
)]
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

#[derive(IntoAndToJson)]
struct Tuple(u8, cjson::values::Finite<f32>);

#[test]
fn tuple() {
    assert_json_eq!(
        Tuple(1, cjson::values::Finite::new_f32(2.3).unwrap()),
        "[1,2.3]"
    );
}

#[derive(IntoAndToJson)]
struct ObjEmpty {}

#[test]
fn obj_empty() {
    assert_json_eq!(ObjEmpty {}, "{}");
}

#[derive(IntoAndToJson)]
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

#[derive(IntoAndToJson)]
#[cjson(derive_from(V))]
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

#[derive(IntoAndToJson)]
enum Never {}

#[test]
fn never() {
    assert_json_eq!(None::<Never>, "null");
}

#[derive(IntoAndToJson)]
enum EnumOnlyUnit {
    OnlyUnit,
}

#[derive(IntoAndToJson)]
enum EnumOne {
    Only(),
}

#[derive(IntoAndToJson)]
#[cjson(any_value)]
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
