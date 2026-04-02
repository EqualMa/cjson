#![cfg(feature = "proc-macro")]

use cjson::{self as my_json, ToJson, ser::exts::TextExt as _};

macro_rules! assert_json_eq {
    ($v:expr,$eq:expr) => {
        assert_eq!($v.to_json().into_string().into_inner(), $eq)
    };
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
