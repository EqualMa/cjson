use crate::ser::{ToJson, exts::TextExt};

const fn assert_to_json<T: ToJson>(v: T) -> T {
    assert!(core::mem::size_of_val(&v) == 0);
    v
}

struct TestSimple<Empty: ToJson + Copy, Mixed: ToJson + Copy, Nested: ToJson + Copy> {
    empty: Empty,
    mixed: Mixed,
    nested: Nested,
}

const fn test_simple() -> TestSimple<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestSimple {
        empty: {
            let v = json!([]);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"[]"));
            v
        },
        mixed: {
            let v = json!([false, true, 1u8, 2u128, null, "", "hello", "\nworld"]);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(
                s,
                br#"[false,true,1,2,null,"","hello","\nworld"]"#
            ));
            v
        },
        nested: {
            // let v = json!([[]]);
            let v = json!([]);
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"[]"));
            v
        },
    }
}

const fn test_runtime() {}
const fn test_nested() {}

const _: () = {
    test_simple();
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

    assert_eq!(to_json_string(empty), "[]");
    assert_eq!(
        to_json_string(mixed),
        r#"[false,true,1,2,null,"","hello","\nworld"]"#
    );
}
