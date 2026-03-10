use crate::ser::{ToJson, exts::TextExt};

mod array;

const fn assert_to_json<T: ToJson>(v: T) -> T {
    assert!(core::mem::size_of_val(&v) == 0);
    v
}

const fn test_null() -> impl ToJson + Copy {
    let v = json!(null);

    let s = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(s, b"null"));

    assert_to_json(v)
}

const fn test_false() -> impl ToJson + Copy {
    let v = json!(false);

    let s = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(s, b"false"));

    assert_to_json(v)
}

const fn test_true() -> impl ToJson + Copy {
    let v = json!(true);

    let s = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(s, b"true"));

    assert_to_json(v)
}

const fn test_const_bool<const BOOL: bool>() -> impl ToJson + Copy {
    let v = json!(
        //
        const BOOL: bool = BOOL;,
        const { BOOL }
    );

    let s = v.as_json_value_str().inner().as_bytes();

    if BOOL {
        assert!(matches!(s, b"true"));
    } else {
        assert!(matches!(s, b"false"));
    }

    assert_to_json(v)
}

const fn test_literal_0i8() -> impl ToJson + Copy {
    let v = json!(0i8);

    let s = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(s, b"0"));

    assert_to_json(v)
}

const fn test_literal_127i8() -> impl ToJson + Copy {
    let v = json!(127i8);

    let s = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(s, b"127"));

    assert_to_json(v)
}

const fn test_const_i8<const I: i8>() -> impl ToJson + Copy {
    let v = json!(
        //
        const V: i8 = I;,
        const { V }
    );

    let s = v.as_json_value_str().inner().as_bytes();

    match I {
        0 => assert!(matches!(s, b"0")),
        -128 => assert!(matches!(s, b"-128")),
        127 => assert!(matches!(s, b"127")),
        _ => panic!("not tested"),
    }

    assert_to_json(v)
}

const fn test_empty_string() -> impl ToJson + Copy {
    let v = json!("");
    let json = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(json, b"\"\""));
    v
}

const fn test_simple_string() -> impl ToJson + Copy {
    let v = json!("hello world");
    let json = v.as_json_value_str().inner().as_bytes();
    assert!(matches!(json, b"\"hello world\""));
    v
}

const fn test_escaped_string() -> impl ToJson + Copy {
    let v = json!("\x22\x5C\x2F\x08\x0C\x0A\x0D\x09\x00");
    let json = v.as_json_value_str().inner().as_bytes();

    assert!(matches!(json, br#""\"\\/\b\f\n\r\t\u0000""#));
    v
}

struct TestLiteralFloat<Zero, Int, Float, NegInt, NegFloat> {
    zero: Zero,
    int: Int,
    float: Float,
    neg_int: NegInt,
    neg_float: NegFloat,
}

const fn test_literal_f32() -> TestLiteralFloat<
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
> {
    TestLiteralFloat {
        zero: {
            let v = json!(0.0f32);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"0.0"));

            v
        },
        int: {
            let v = json!(1f32);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"1.0"));

            v
        },
        float: {
            let v = json!(const { f32::EPSILON });

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"1.1920929e-7"));

            v
        },
        neg_int: {
            let v = json!(-1.0f32);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"-1.0"));

            v
        },
        neg_float: {
            let v = json!(-3.14f32);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"-3.14"));

            v
        },
    }
}

const fn test_literal_f64() -> TestLiteralFloat<
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
    impl ToJson + Copy,
> {
    TestLiteralFloat {
        zero: {
            let v = json!(0.0f64);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"0.0"));

            v
        },
        int: {
            let v = json!(1f64);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"1.0"));

            v
        },
        float: {
            let v = json!(const { f64::EPSILON });

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"2.220446049250313e-16"));

            v
        },
        neg_int: {
            let v = json!(-1.0f64);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"-1.0"));

            v
        },
        neg_float: {
            let v = json!(-3.14f64);

            let s = v.as_json_value_str().inner().as_bytes();
            assert!(matches!(s, b"-3.14"));

            v
        },
    }
}

const _: () = {
    test_null();

    test_false();
    test_true();
    test_const_bool::<false>();
    test_const_bool::<true>();
    test_literal_0i8();
    test_literal_127i8();

    test_const_i8::<0>();
    test_const_i8::<127>();
    test_const_i8::<-128>();

    test_empty_string();
    test_simple_string();
    test_escaped_string();

    test_literal_f32();
    test_literal_f64();
};

#[cfg(feature = "alloc")]
#[test]
fn tests() {
    fn to_json_string(v: impl ToJson) -> alloc::string::String {
        v.to_json().into_string().into_inner()
    }

    assert_eq!(to_json_string(test_null()), "null");

    assert_eq!(to_json_string(test_false()), "false");
    assert_eq!(to_json_string(test_true()), "true");
    assert_eq!(to_json_string(test_const_bool::<false>()), "false");
    assert_eq!(to_json_string(test_const_bool::<true>()), "true");
    assert_eq!(to_json_string(test_literal_0i8()), "0");
    assert_eq!(to_json_string(test_literal_127i8()), "127");

    assert_eq!(to_json_string(test_const_i8::<0>()), "0");
    assert_eq!(to_json_string(test_const_i8::<127>()), "127");
    assert_eq!(to_json_string(test_const_i8::<-128>()), "-128");

    assert_eq!(to_json_string(test_empty_string()), "\"\"");
    assert_eq!(to_json_string(test_simple_string()), "\"hello world\"");
    assert_eq!(
        to_json_string(test_escaped_string()),
        r#""\"\\/\b\f\n\r\t\u0000""#
    );

    {
        let TestLiteralFloat {
            zero,
            int,
            float,
            neg_int,
            neg_float,
        } = test_literal_f32();

        assert_eq!(to_json_string(zero), "0.0");
        assert_eq!(to_json_string(int), "1.0");
        assert_eq!(to_json_string(float), "1.1920929e-7");
        assert_eq!(to_json_string(neg_int), "-1.0");
        assert_eq!(to_json_string(neg_float), "-3.14");
    }

    {
        let TestLiteralFloat {
            zero,
            int,
            float,
            neg_int,
            neg_float,
        } = test_literal_f64();

        assert_eq!(to_json_string(zero), "0.0");
        assert_eq!(to_json_string(int), "1.0");
        assert_eq!(to_json_string(float), "2.220446049250313e-16");
        assert_eq!(to_json_string(neg_int), "-1.0");
        assert_eq!(to_json_string(neg_float), "-3.14");
    }
}
