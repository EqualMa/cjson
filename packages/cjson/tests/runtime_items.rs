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

impl<T: ToJsonArray> ToJsonArray for BetweenBrackets<T> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl<T1: ToJsonArray, T2: ToJsonArray> ToJsonArray for BetweenBracketsChained<T1, T2> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

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
        ..(&self.0) as &'cjson_lt_to_json T,
        null,
    ],
);

impl_to_json!(
    impl_generics![A: ToJsonArray, B: ToJsonArray],
    |self: AfterArrayStartBeforeItemChained<A, B>| [
        //
        ..(&self.0) as &'cjson_lt_to_json A,
        ..(&self.1) as &'cjson_lt_to_json B,
        null,
    ],
);

impl<T: ToJsonArray> ToJsonArray for AfterArrayStartBeforeItem<T> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl<T1: ToJsonArray, T2: ToJsonArray> ToJsonArray for AfterArrayStartBeforeItemChained<T1, T2> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

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

impl<T: ToJsonArray> ToJsonArray for AfterItemBeforeBracket<T> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl<V: ToJson, T1: ToJsonArray, T2: ToJsonArray> ToJsonArray
    for AfterItemBeforeBracketChained<V, T1, T2>
{
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl_to_json!(
    impl_generics![T: ToJsonArray],
    |self: AfterItemBeforeBracket<T>| [
        //
        false,
        ..(&self.0) as &'cjson_lt_to_json T,
    ],
);

impl_to_json!(
    impl_generics![V: ToJson, A: ToJsonArray, B: ToJsonArray],
    |self: AfterItemBeforeBracketChained<V, A, B>| [
        //
        (&self.0) as &'cjson_lt_to_json V,
        ..(&self.1) as &'cjson_lt_to_json A,
        ..(&self.2) as &'cjson_lt_to_json B,
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

impl<T: ToJsonArray> ToJsonArray for AfterItemBeforeItem<T> {
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl<T1: ToJsonArray, T2: ToJsonArray, V: ToJson> ToJsonArray
    for AfterItemBeforeItemChained<T1, T2, V>
{
    type ToJsonArray<'a>
        = <Self as ToJson>::ToJson<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        <Self as ToJson>::to_json(self)
    }
}

impl_to_json!(
    impl_generics![T: ToJsonArray],
    |self: AfterItemBeforeItem<T>| [
        //
        false,
        ..(&self.0) as &'cjson_lt_to_json T,
        1u8,
    ],
);

impl_to_json!(
    impl_generics![A: ToJsonArray, B: ToJsonArray,V: ToJson, ],
    |self: AfterItemBeforeItemChained<A, B, V>| [
        //
        true,
        ..(&self.0) as &'cjson_lt_to_json A,
        ..(&self.1) as &'cjson_lt_to_json B,
        (&self.2) as &'cjson_lt_to_json V,
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
