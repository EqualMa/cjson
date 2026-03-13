use crate::ser::{ToJson, exts::TextExt};

macro_rules! match_minus_literal {
    (-$lit:literal) => {
        true
    };
    ($lit:literal) => {
        false
    };
}

macro_rules! one_literal {
    ($lit:literal) => {
        match_minus_literal!($lit)
    };
}

const _: () = {
    assert!(match_minus_literal!(-1));
    assert!(!one_literal!(-1));
};

struct TestSimple<Empty: ToJson + Copy, Mixed: ToJson + Copy, Nested: ToJson + Copy> {
    empty: Empty,
    mixed: Mixed,
    nested: Nested,
}

const fn test_simple() -> TestSimple<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestSimple {
        empty: {
            let v = json!({});

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"{}"));
            v
        },
        mixed: {
            const NUM: &str = "num";
            let v = json!({
                "false" = false;
                json_string!("tr", "ue") = true;
                -const { NUM } = -1i8;
                "null" = null;
                "array" = ["hello\nworld"];
            });

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(
                s,
                br#"{"false":false,"true":true,"num":-1,"null":null,"array":["hello\nworld"]}"#
            ));
            v
        },
        nested: {
            let v = json!({
                "0" = {
                    "1" = {
                        "2" = [{ "3" = [-4.5f64] }];
                    }
                };
            });
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, br#"{"0":{"1":{"2":[{"3":[-4.5]}]}}}"#));
            v
        },
    }
}

struct TestRuntime<Name: ToJson + Copy, Value: ToJson + Copy, Both: ToJson + Copy> {
    name: (Name, &'static str),
    value: (Value, &'static str),
    both: (Both, &'static str),
}

const fn test_runtime() -> TestRuntime<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestRuntime {
        name: (json!({ ("name") = "name" }), r#"{"name":"name"}"#),
        value: (json!({ "value" = ("value") }), r#"{"value":"value"}"#),
        both: (
            json!({
                ("both") = ("both");
                ("both2") = {
                    ("name") = json_string!("", "name", "");
                    json_string!("val", "ue") = ("value");
                };
            }),
            r#"{"both":"both","both2":{"name":"name","value":"value"}}"#,
        ),
    }
}

const _: () = {
    test_simple();
    test_runtime();
};

#[cfg(feature = "alloc")]
#[test]
fn tests() {
    fn to_json_string(v: impl ToJson) -> alloc::string::String {
        v.to_json().into_string().into_inner()
    }

    let TestSimple {
        empty,
        mixed,
        nested,
    } = test_simple();

    assert_eq!(to_json_string(empty), "{}");
    assert_eq!(
        to_json_string(mixed),
        r#"{"false":false,"true":true,"num":-1,"null":null,"array":["hello\nworld"]}"#
    );

    assert_eq!(
        to_json_string(nested),
        r#"{"0":{"1":{"2":[{"3":[-4.5]}]}}}"#
    );

    let TestRuntime {
        //
        name,
        value,
        both,
    } = test_runtime();

    assert_eq!(to_json_string(name.0), name.1);
    assert_eq!(to_json_string(value.0), value.1);
    assert_eq!(to_json_string(both.0), both.1);
}

#[test]
fn test_chunks() {
    use crate::ser::iter_text_chunk::IterTextChunk as _;
    use crate::ser::traits::IntoTextChunks as _;

    macro_rules! next {
        ($v:expr) => {
            $v.next_text_chunk().as_ref().map(|v| v.as_ref())
        };
    }

    {
        let mut v = json!({ json_string!("hell", "o") = json_string!("world", " ", "!") })
            .to_json()
            .into_text_chunks();

        assert_eq!(next!(v), Some(br#"{"hello":"world !"}"#.as_slice()),);

        assert_eq!(next!(v), None);
    }

    {
        let v = test_runtime().both.0;
        let mut v = v.to_json().into_text_chunks();

        assert_eq!(next!(v), Some(b"{\"".as_slice()));
        assert_eq!(next!(v), Some(b"both".as_slice()));
        assert_eq!(next!(v), Some(b"\":".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"both".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b",\"".as_slice()));
        assert_eq!(next!(v), Some(b"both2".as_slice()));
        assert_eq!(next!(v), Some(b"\":{\"".as_slice()));
        assert_eq!(next!(v), Some(b"name".as_slice()));
        assert_eq!(next!(v), Some(b"\":\"name\",\"value\":".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"value".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"}}".as_slice()));

        assert_eq!(next!(v), None);
    }
}
