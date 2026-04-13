use crate::ser::{ToJson, exts::TextExt};

struct TestSimple<
    Empty: ToJson + Copy,
    Mixed: ToJson + Copy,
    Nested: ToJson + Copy,
    NegLiteral: ToJson + Copy,
> {
    empty: Empty,
    mixed: Mixed,
    nested: Nested,
    neg_literal: NegLiteral,
}

const fn test_simple()
-> TestSimple<impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy, impl ToJson + Copy> {
    TestSimple {
        empty: {
            let v: crate::r#const::array::EmptyArray = json!([]);

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
            let v = json!([[["\t", [[[]]]], false]]);
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, br#"[[["\t",[[[]]]],false]]"#));
            v
        },
        neg_literal: {
            let v = json!([-1i8]);
            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"[-1]"));
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
        one: (json!([(1)]), "[1]"),
        two: (json!([(1), null, (3)]), "[1,null,3]"),
        nested: (
            json!([1u8, [2u8, [(3), 4u8], [(5)],], 6u8]),
            "[1,[2,[3,4],[5]],6]",
        ),
    }
}

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
        neg_literal,
    } = test_simple();

    assert_eq!(to_json_string(empty), "[]");
    assert_eq!(
        to_json_string(mixed),
        r#"[false,true,1,2,null,"","hello","\nworld"]"#
    );

    assert_eq!(to_json_string(nested), r#"[[["\t",[[[]]]],false]]"#);
    assert_eq!(to_json_string(neg_literal), "[-1]");

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
        let mut v = json!([true, [[null,], "hello\tworld"]])
            .to_json()
            .into_text_chunks();

        assert_eq!(
            next!(v),
            Some(br#"[true,[[null],"hello\tworld"]]"#.as_slice()),
        );

        assert_eq!(next!(v), None);
    }

    {
        let v = json!([true, [(1), [(json!([null]))], ("hello\tworld")]]);
        let mut v = v.to_json().into_text_chunks();

        assert_eq!(next!(v), Some(b"[true,[".as_slice()));
        assert_eq!(next!(v), Some(b"1".as_slice()));
        assert_eq!(next!(v), Some(b",[".as_slice()));
        assert_eq!(next!(v), Some(b"[null]".as_slice()));
        assert_eq!(next!(v), Some(b"],".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"hello".as_slice()));
        assert_eq!(next!(v), Some(b"\\t".as_slice()));
        assert_eq!(next!(v), Some(b"world".as_slice()));
        assert_eq!(next!(v), Some(b"\"".as_slice()));
        assert_eq!(next!(v), Some(b"]]".as_slice()));

        assert_eq!(next!(v), None);
    }
}
