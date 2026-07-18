use cjson::{
    impl_to_json,
    ser::{ToJson2 as ToJson, ToJsonArray2 as ToJsonArray},
};

macro_rules! assert_json_eq {
    ($v:expr, $eq:expr) => {
        assert_eq!(::cjson::ser::ToJsonExt::to_json_as_string(&$v), $eq)
    };
}

macro_rules! json {
    ([]) => {
        [] as [u8; 0]
    };
    ($e:expr) => {
        $e
    };
}

struct BetweenBrackets<T: ToJsonArray>(T);

impl_to_json!(
    impl_generics![T],
    where_clause![where T: ToJsonArray],
    |self: BetweenBrackets<T>| [
        //
        ..(&self.0) as &'_ T
    ],
);

struct BetweenBracketsChained<T1: ToJsonArray, T2: ToJsonArray>(T1, T2);

impl_to_json!(
    impl_generics![T1: ToJsonArray, T2: ToJsonArray],
    |self: BetweenBracketsChained<T1, T2>| [..(&self.0) as &'_ T1, ..(&self.1) as &'_ T2,],
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

pub struct AfterArrayStartBeforeItem<T: ToJsonArray>(T);
pub struct AfterArrayStartBeforeItemChained<A: ToJsonArray, B: ToJsonArray>(A, B);

impl_to_json!(
    impl_generics![T: ToJsonArray],
    |self: AfterArrayStartBeforeItem<T>| [
        //
        ..(&self.0) as &'_ T,
        null,
    ],
);

impl_to_json!(
    impl_generics![A: ToJsonArray, B: ToJsonArray],
    |self: AfterArrayStartBeforeItemChained<A, B>| [
        //
        ..(&self.0) as &'_ A,
        ..(&self.1) as &'_ B,
        null,
    ],
);

#[test]
fn after_array_start_before_item() {
    assert_json_eq!(AfterArrayStartBeforeItem(&[] as &[u8]), "[null]");
    assert_json_eq!(AfterArrayStartBeforeItem(&[1u8] as &[_]), "[1,null]");
    assert_json_eq!(AfterArrayStartBeforeItem(&[1u8, 2u8] as &[_]), "[1,2,null]");

    assert_json_eq!(
        AfterArrayStartBeforeItemChained(json!([]), json!([])),
        "[null]"
    );
    assert_json_eq!(
        AfterArrayStartBeforeItemChained(json!([]), json!([true])),
        "[true,null]"
    );
    assert_json_eq!(
        AfterArrayStartBeforeItemChained(json!([false]), json!([])),
        "[false,null]"
    );
    assert_json_eq!(
        AfterArrayStartBeforeItemChained(json!([false]), json!([true])),
        "[false,true,null]"
    );
}

pub struct AfterItemBeforeBracket<T: ToJsonArray>(T);
pub struct AfterItemBeforeBracketChained<V: ToJson, A: ToJsonArray, B: ToJsonArray>(V, A, B);

impl_to_json!(
    impl_generics![T: ToJsonArray],
    |self: AfterItemBeforeBracket<T>| [
        //
        false,
        ..(&self.0) as &'_ T,
    ],
);

impl_to_json!(
    impl_generics![V: ToJson, A: ToJsonArray, B: ToJsonArray],
    |self: AfterItemBeforeBracketChained<V, A, B>| [
        //
        (&self.0) as &'_ V,
        ..(&self.1) as &'_ A,
        ..(&self.2) as &'_ B,
    ],
);

#[test]
fn after_item_before_bracket() {
    assert_json_eq!(AfterItemBeforeBracket(&[] as &[u8]), "[false]");
    assert_json_eq!(AfterItemBeforeBracket(&[1u8] as &[_]), "[false,1]");
    assert_json_eq!(AfterItemBeforeBracket(&[1u8, 2u8] as &[_]), "[false,1,2]");

    assert_json_eq!(
        AfterItemBeforeBracketChained(json!(false), json!([]), json!([])),
        "[false]"
    );
    assert_json_eq!(
        AfterItemBeforeBracketChained(1, json!([]), json!([true])),
        "[1,true]"
    );
    assert_json_eq!(
        AfterItemBeforeBracketChained(json!(2u8), json!([false]), json!([])),
        "[2,false]"
    );
    assert_json_eq!(
        AfterItemBeforeBracketChained("hello", json!([false]), json!([true])),
        "[\"hello\",false,true]"
    );
}

pub struct AfterItemBeforeItem<T: ToJsonArray>(T);
pub struct AfterItemBeforeItemChained<A: ToJsonArray, B: ToJsonArray, V: ToJson>(A, B, V);

impl_to_json!(
    impl_generics![T: ToJsonArray],
    |self: AfterItemBeforeItem<T>| [
        //
        false,
        ..(&self.0) as &'_ T,
        1u8,
    ],
);

impl_to_json!(
    impl_generics![A: ToJsonArray, B: ToJsonArray,V: ToJson, ],
    |self: AfterItemBeforeItemChained<A, B, V>| [
        //
        true,
        ..(&self.0) as &'_ A,
        ..(&self.1) as &'_ B,
        (&self.2) as &'_ V,
    ],
);

#[test]
fn after_item_before_item() {
    assert_json_eq!(AfterItemBeforeItem(&[] as &[u8]), "[false,1]");
    assert_json_eq!(AfterItemBeforeItem(&[1u8] as &[_]), "[false,1,1]");
    assert_json_eq!(AfterItemBeforeItem(&[1u8, 2u8] as &[_]), "[false,1,2,1]");

    assert_json_eq!(
        AfterItemBeforeItemChained(json!([]), json!([]), json!(false)),
        "[true,false]"
    );
    assert_json_eq!(
        AfterItemBeforeItemChained(json!([]), json!([true]), 1),
        "[true,true,1]"
    );
    assert_json_eq!(
        AfterItemBeforeItemChained(json!([false]), json!([]), json!(2u8)),
        "[true,false,2]"
    );
    assert_json_eq!(
        AfterItemBeforeItemChained(json!([false]), json!([true]), "hello"),
        "[true,false,true,\"hello\"]"
    );
}
