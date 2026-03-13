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

const _: () = {
    test_simple();
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
    assert_eq!(to_json_string(nested), r#"["",null,"123\t456"]"#)
}
