use cjson::{
    json, json_to_string, json_write,
    ser::{
        ConsumeJson, ConsumeJsonChunks as _, ConsumeJsonText, json_kinds, traits::ConsumeTextChunk,
    },
    values::ObjectOfIter,
};

fn test<const V: u8>() {
    let a = json!(json_string!("prefix:", ":postfix"));
    let b = json!(1u8);
    let b = json!(const V: u8;, const { V });
}

fn test2(w: impl ConsumeJson<ConsumeJsonKind = json_kinds::AnyValue>) {
    w.consume_any_value(
        const {
            const {
                cjson::r#const::ConstAsJsonValueStr(
                    cjson::r#const::ConstIntoJsonValueString(
                        cjson::r#const::ConstIntoJson(1u8).const_into_json(),
                    )
                    .const_into_json_value_string::<{
                        cjson::r#const::ConstIntoJsonValueString(
                            cjson::r#const::ConstIntoJson(1u8).const_into_json(),
                        )
                        .const_into_json_value_string_len()
                    }>(),
                )
            }
            .const_as_json_value_str()
        },
        (),
    );
}

fn test2_const<const V: u8>(w: impl ConsumeJson<ConsumeJsonKind = json_kinds::AnyValue>) {
    w.consume_any_value(
        const {
            const {
                cjson::r#const::ConstAsJsonValueStr(
                    cjson::r#const::ConstIntoJsonValueString(
                        cjson::r#const::ConstIntoJson(V).const_into_json(),
                    )
                    .const_into_json_value_string_without_const_len(),
                )
            }
            .const_as_json_value_str()
        },
        (),
    );
}

#[test]
fn test_all() {
    test3::<5>();
}

fn test3<const V: u8>() {
    const { assert!(V == 5) }
    assert_eq!(json_to_string!(1i8), "1");
    assert_eq!(
        json_to_string!(const { "hello\x20world" }),
        "\"hello world\""
    );
    #[cfg(todo)]
    {
        json_write!(w, json_value_generic_const!(V));
    }
    assert_eq!(json_to_string!(null), "null");
    assert_eq!(json_to_string!((1)), "1");

    assert_eq!(json_to_string!([]), "[]");
    assert_eq!(json_to_string!([true]), "[true]");

    assert_eq!(json_to_string!([true, (1), (2), false]), "[true,1,2,false]");
    assert_eq!(
        json_to_string!([true, (1), ..([2, 3]), false]),
        "[true,1,2,3,false]"
    );
    assert_eq!(json_to_string!([([1, 2]), false]), "[[1,2],false]");
    assert_eq!(json_to_string!([true, (false)]), "[true,false]");

    assert_eq!(json_to_string!([..([1, 2]), false]), "[1,2,false]");
    #[cfg(todo)]
    assert_eq!(
        json_to_string!([..([1, 2]), ..([3, 4]), false]),
        "[1,2,3,4,false]"
    );
    assert_eq!(json_to_string!([true, ..([1, 2])]), "[true,1,2]");
    assert_eq!(
        json_to_string!([true, ..([1, 2]), false]),
        "[true,1,2,false]"
    );
    assert_eq!(
        json_to_string!([..([1, 2]), ..([false, true])]),
        "[1,2,false,true]"
    );

    assert_eq!(json_to_string!({}), "{}");
    assert_eq!(
        json_to_string!({ "hello" = "world" }),
        r#"{"hello":"world"}"#
    );

    assert_eq!(
        json_to_string!({
            "false" = (false);
            "" = null;
        }),
        r#"{"false":false,"":null}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1u8;
            ("true") = true;
        }),
        r#"{"one":1,"true":true}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = (1);
            ("false") = false;
        }),
        r#"{"one":1,"false":false}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = (1);
            ..(ObjectOfIter([] as [(&str, &str); 0]));
            ("false") = false;
        }),
        r#"{"one":1,"false":false}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = (1);
            ..(ObjectOfIter([("two", 2)]));
            ("false") = false;
        }),
        r#"{"one":1,"two":2,"false":false}"#
    );
    assert_eq!(json_to_string!({ ("one") = 1u8 }), r#"{"one":1}"#);
    assert_eq!(
        json_to_string!({
            ..(ObjectOfIter([] as [(&str, u8); 0]));
            "one" = 1u8;
        }),
        r#"{"one":1}"#
    );
    assert_eq!(
        json_to_string!({
            ..(ObjectOfIter([("false", false)]));
            "one" = 1u8;
        }),
        r#"{"false":false,"one":1}"#
    );
    #[cfg(todo)]
    assert_eq!(
        json_to_string!({
            ..(ObjectOfKvs([]));
            ..(ObjectOfKvs([]));
            "one" = 1;
        }),
        r#"{"one":1}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1u8;
            ..(ObjectOfIter([] as [(&str, u8); 0]));
        }),
        r#"{"one":1}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1i8;
            ..(ObjectOfIter([("two", 2)]));
        }),
        r#"{"one":1,"two":2}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1i8;
            "two" = (2);
        }),
        r#"{"one":1,"two":2}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1i8;
            ..(ObjectOfIter([] as [(&str, i8); 0]));
            "three" = 3u8;
        }),
        r#"{"one":1,"three":3}"#
    );
    assert_eq!(
        json_to_string!({
            "one" = 1i8;
            ..(ObjectOfIter([("two", 2)]));
            "three" = 3u8;
        }),
        r#"{"one":1,"two":2,"three":3}"#
    );

    assert_eq!(
        json_to_string!({ ..(ObjectOfIter([] as [(&str, &str); 0])) }),
        r#"{}"#
    );
    assert_eq!(
        json_to_string!({ ..(ObjectOfIter([("one", 1), ("two", 2)])) }),
        r#"{"one":1,"two":2}"#
    );

    #[cfg(todo)]
    {
        json_write!(w, json_string!("a", ":", "b"));
    }
}
