use cjson::{
    impl_to_json, json,
    ser::{ToJson, ToJsonArray, exts::TextExt as _},
};

macro_rules! assert_json_eq {
    ($v:expr,$eq:expr) => {
        assert_eq!($v.to_json().into_string().into_inner(), $eq)
    };
}

struct BetweenBrackets<T: ToJsonArray>(T);

impl_to_json!(
    impl_generics![T],
    where_clause![where T: ToJsonArray],
    |self: BetweenBrackets<T>| [
        //
        ..(&self.0) as &'cjson_lt_to_json T
    ],
);

struct BetweenBracketsChained<T1: ToJsonArray, T2: ToJsonArray>(T1, T2);

impl_to_json!(
    impl_generics![T1: ToJsonArray, T2: ToJsonArray],
    |self: BetweenBracketsChained<T1, T2>| [
        ..(&self.0) as &'cjson_lt_to_json T1,
        ..(&self.1) as &'cjson_lt_to_json T2,
    ],
);

#[test]
fn between_brackets() {
    assert_json_eq!(BetweenBrackets(&[] as &[bool]), "[]");
    assert_json_eq!(BetweenBrackets(&[1] as &[_]), "[1]");
    assert_json_eq!(BetweenBrackets(&[1, 2, 3] as &[_]), "[1,2,3]");

    assert_json_eq!(BetweenBracketsChained(&[] as &[u8], &[] as &[bool]), "[]");
    assert_json_eq!(
        BetweenBracketsChained(&[] as &[u8], &[true, false] as &[bool]),
        "[true,false]"
    );
    assert_json_eq!(
        BetweenBracketsChained(&[1i8] as &[_], &[] as &[&str]),
        "[1]"
    );
    assert_json_eq!(
        BetweenBracketsChained(&[0, 1] as &[_], &["hello", "\t", "world"] as &[_]),
        r#"[0,1,"hello","\t","world"]"#
    );
}
