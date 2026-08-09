use cjson::{
    json_to_async_try, json_to_string, json_to_try,
    ser::{ConsumeJson, json_kinds},
    values::ObjectOfIter,
};

#[cfg(todo)]
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

#[expect(unused_imports)] // TODO: test async try
use json_to_async_try as __todo;

macro_rules! assert_json {
    ($eq:expr, $($json:tt)*) => {{
        assert_eq!(json_to_string!($($json)*), $eq);
        assert_eq!(
            (|| -> Result<
                ::cjson::ser::IoWrite<Vec<u8>>,
                <::cjson::ser::IoWrite<Vec<u8>> as ::cjson::ser::traits::TryConsumeTextChunk>::Err,
            > {
                Ok(json_to_try!($($json)*))
            })().unwrap().0,
            $eq.as_bytes()
        );
    }};
}

fn test3<const V: u8>() {
    const { assert!(V == 5) }

    assert_json!("1", 1i8);
    assert_json!("\"hello world\"", const { "hello\x20world" },);
    assert_json!("5", json_value_generic_const!(V));
    assert_json!("null", null);
    assert_json!("1", (1));

    assert_json!("[]", []);
    assert_json!("[true]", [true]);

    assert_json!("[true,1,2,false]", [true, (1), (2), false]);
    assert_json!("[true,1,2,3,false]", [true, (1), ..([2, 3]), false],);
    assert_json!("[[1,2],false]", [([1, 2]), false],);
    assert_json!("[true,false]", [true, (false)]);

    assert_json!("[1,2,false]", [..([1, 2]), false]);
    assert_json!("[1,2,3,4,false]", [..([1, 2]), ..([3, 4]), false],);
    assert_json!("[true,1,2]", [true, ..([1, 2])]);
    assert_json!("[true,1,2,false]", [true, ..([1, 2]), false],);
    assert_json!("[1,2,false,true]", [..([1, 2]), ..([false, true])],);

    assert_json!("{}", {});
    assert_json!(r#"{"hello":"world"}"#, { "hello" = "world" },);

    assert_json!(r#"{"false":false,"":null}"#, {
        "false" = (false);
        "" = null;
    },);
    assert_json!(r#"{"one":1,"true":true}"#, {
        "one" = 1u8;
        ("true") = true;
    },);
    assert_json!(r#"{"one":1,"false":false}"#, {
        "one" = (1);
        ("false") = false;
    },);
    assert_json!(r#"{"one":1,"false":false}"#, {
        "one" = (1);
        ..(ObjectOfIter([] as [(&str, &str); 0]));
        ("false") = false;
    },);
    assert_json!(r#"{"one":1,"two":2,"false":false}"#, {
        "one" = (1);
        ..(ObjectOfIter([("two", 2)]));
        ("false") = false;
    },);
    assert_json!(r#"{"one":1}"#, { ("one") = 1u8 });
    assert_json!(r#"{"one":1}"#, {
        ..(ObjectOfIter([] as [(&str, u8); 0]));
        "one" = 1u8;
    },);
    assert_json!(r#"{"false":false,"one":1}"#, {
        ..(ObjectOfIter([("false", false)]));
        "one" = 1u8;
    },);
    assert_json!(r#"{"one":1}"#, {
        ..(ObjectOfIter([] as [(&str, u8); 0]));
        ..(ObjectOfIter([] as [(&str, u8); 0]));
        "one" = 1i8;
    },);

    assert_json!(r#"{"one":1}"#, {
        "one" = 1u8;
        ..(ObjectOfIter([] as [(&str, u8); 0]));
    });
    assert_json!(r#"{"one":1,"two":2}"#, {
        "one" = 1i8;
        ..(ObjectOfIter([("two", 2)]));
    });
    assert_json!(r#"{"one":1,"two":2}"#, {
        "one" = 1i8;
        "two" = (2);
    });
    assert_json!(r#"{"one":1,"three":3}"#, {
        "one" = 1i8;
        ..(ObjectOfIter([] as [(&str, i8); 0]));
        "three" = 3u8;
    });
    assert_json!(r#"{"one":1,"two":2,"three":3}"#, {
        "one" = 1i8;
        ..(ObjectOfIter([("two", 2)]));
        "three" = 3u8;
    });

    assert_json!(r#"{}"#, { ..(ObjectOfIter([] as [(&str, &str); 0])) },);
    assert_json!(r#"{"one":1,"two":2}"#, {
        ..(ObjectOfIter([("one", 1), ("two", 2)]))
    },);

    assert_json!("\"\"", "");
    assert_json!("\"\"", json_string!());
    assert_json!("\"a:b\"", json_string!("a", ":", "b"));
    assert_json!("\"abc\"", json_string!("a", "b", "", "c"));

    assert_json!("\"abc\"", json_string!("a", ("b"), "c"));
    assert_json!("\"ab\"", json_string!("a", (""), "b"));
    assert_json!("\"ab_c\"", json_string!("a", ("b"), ("_"), "c"),);
    assert_json!("\"abcd\"", json_string!("a", (""), ("bc"), "d"),);

    assert_json!("\"ab\"", json_string!(("a"), "b"));
    assert_json!("\"b\"", json_string!((""), "b"));
    assert_json!("\"abc\"", json_string!(("a"), ("b"), "c"));
    assert_json!("\"bc\"", json_string!((""), ("b"), "c"));
    assert_json!("\"ac\"", json_string!(("a"), (""), "c"));
    assert_json!("\"c\"", json_string!((""), (""), "c"));

    assert_json!("\"012\"", json_string!("01", ("2"),));
    assert_json!("\"01\"", json_string!("01", (""),));
    assert_json!("\"012345\"", json_string!("012", ("3"), ("45")),);
    assert_json!("\"0123\"", json_string!("012", ("3"), ("")),);
    assert_json!("\"01234\"", json_string!("012", (""), ("34")),);
    assert_json!("\"012\"", json_string!("012", (""), ("")));

    assert_json!("\"012\"", json_string!(("0"), "1", ("2"),));
    assert_json!("\"12\"", json_string!((""), "1", ("2"),));
    assert_json!("\"01\"", json_string!(("0"), "1", (""),));
    assert_json!("\"1\"", json_string!((""), "1", (""),));
    assert_json!(
        "\"01234567\"",
        json_string!(("0"), ("1"), "23", ("45"), ("67")),
    );
}
