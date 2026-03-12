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
            assert!(matches!(s, b"\"1\\n234\\t5\\\\67\""));
            v
        },
    }
}

const _: () = {
    test_simple();
};
