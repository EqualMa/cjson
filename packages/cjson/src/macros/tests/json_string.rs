use crate::ser::ToJson;

struct TestSimple<Empty: ToJson + Copy, Mixed: ToJson + Copy, Nested: ToJson + Copy> {
    empty: Empty,
    mixed: Mixed,
    nested: Nested,
}

const fn test_simple() -> TestSimple<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestSimple {
        empty: {
            let v = json!(json_string!());

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"\"\""));
            v
        },
        mixed: {
            let v = json!(json_string!("1\n23", "4\t", "5\\67"));

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"\"1\\n234\\t5\\\\67\""));
            v
        },
        nested: {
            let v = json!([json_string!(), null, json_string!("123", "\t", "456")]);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, br#"["",null,"123\t456"]"#));
            v
        },
    }
}

struct TestRuntime<One: ToJson + Copy, Two: ToJson + Copy, Nested: ToJson + Copy> {
    one: (One, &'static str),
    two: (Two, &'static str),
    nested: (Nested, &'static str),
}

const fn test_runtime() -> TestRuntime<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestRuntime {
        one: (json!([json_string!(("1"))]), r#"["1"]"#),
        two: (json!(json_string![("1"), "null", ("3")]), "\"1null3\""),
        nested: (
            json!([
                1u8,
                [2u8, json_string![("3"), "4"], [(5)],],
                json_string! {"6", ("7")}
            ]),
            r#"[1,[2,"34",[5]],"67"]"#,
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
        use crate::ser::exts::TextExt as _;
        v.to_json().into_string().into_inner()
    }

    let TestSimple {
        empty,
        mixed,
        nested,
    } = test_simple();

    assert_eq!(to_json_string(empty), "\"\"");
    assert_eq!(to_json_string(mixed), "\"1\\n234\\t5\\\\67\"");
    assert_eq!(to_json_string(nested), r#"["",null,"123\t456"]"#);

    let TestRuntime {
        //
        one,
        two,
        nested,
    } = test_runtime();
    assert_eq!(to_json_string(one.0), one.1);
    assert_eq!(to_json_string(two.0), two.1);
    assert_eq!(to_json_string(nested.0), nested.1);
}
