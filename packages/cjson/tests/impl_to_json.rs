use cjson::{
    impl_to_json,
    ser::{ToJson, exts::TextExt as _},
};

macro_rules! assert_json_eq {
    ($v:expr,$eq:expr) => {
        assert_eq!($v.to_json().into_string().into_inner(), $eq)
    };
}

struct Null;
impl_to_json!(|self: Null| null);

#[test]
fn null() {
    assert_json_eq!(Null, "null");
}

struct Runtime<T: ToJson>(T);
impl_to_json!(
    impl_generics![T],
    where_clause![where T: ToJson],
    |self: Runtime<T>| (&self.0) as &'cjson_lt_to_json T,
);

#[test]
fn runtime() {
    assert_json_eq!(Runtime(cjson::values::Finite::new_f32(1.2).unwrap()), "1.2");
}

struct Literal;
impl_to_json!(|self: Literal| 1u8);
#[test]
fn literal() {
    assert_json_eq!(Literal, "1");
}

struct Const;
impl_to_json!(|self: Const| const { false });

#[test]
fn r#const() {
    assert_json_eq!(Const, "false");
}

const V: &str = "hello\tworld!";
struct Const2;
impl_to_json!(|self: Const2| const { V });
#[test]
fn const2() {
    assert_json_eq!(Const2, "\"hello\\tworld!\"");
}

struct Const3<const V: bool>;
impl_to_json!(
    impl_generics![const V: bool],
    {
        const V: bool;
    },
    |self: Const3<V>| const { V }
);
#[test]
fn r#const3() {
    assert_json_eq!(Const3::<false>, "false");
    assert_json_eq!(Const3::<true>, "true");
}

struct ArrayCompileTime;
impl_to_json!(|self: ArrayCompileTime| [true, false]);
#[test]
fn array_compile_time() {
    assert_json_eq!(ArrayCompileTime, "[true,false]");
}

const V2: bool = true;
struct ArrayCompileTime2<const NOT_USED: u8>;
impl_to_json!(
    impl_generics![const N: u8],
    {
        const N: u8;
    },
    |self: ArrayCompileTime2<N>| [[const { V2 }], null]
);
#[test]
fn array_compile_time2() {
    assert_json_eq!(ArrayCompileTime2::<0>, "[[true],null]");
}

struct ArrayRuntime(u8);
impl_to_json!(|self: ArrayRuntime| [1u8, (self.0) as u8, 3u8]);
#[test]
fn array_runtime() {
    assert_json_eq!(ArrayRuntime(2), "[1,2,3]");
}

struct ArrayRuntime2(u8);
impl_to_json!(|self: ArrayRuntime2| [1u8, (&self.0) as &'cjson_lt_to_json u8, 3u8]);
#[test]
fn array_runtime2() {
    assert_json_eq!(ArrayRuntime2(20), "[1,20,3]");
}

struct MyU8<const V: u8>;
impl_to_json!(
    impl_generics![const V: u8],
    {
        const V: u8;
    },
    |self: MyU8<V>| const { V }
);
struct ArrayRuntime3<const V: u8>;
impl_to_json!(
    impl_generics![const V: u8],
    {
        const V: u8;
    },
    |self: ArrayRuntime3<V>| [1u8, (MyU8) as MyU8<V>, 3u8]
);
#[test]
fn array_runtime3() {
    assert_json_eq!(ArrayRuntime3::<0>, "[1,0,3]");
}

struct ObjectCompileTime;
impl_to_json!(|self: ObjectCompileTime| { "name" = ["value"] });
#[test]
fn object_compile_time() {
    assert_json_eq!(ObjectCompileTime, r#"{"name":["value"]}"#);
}

struct ObjectRuntime<A, B>(A, B);

impl_to_json!(
    impl_generics![A, B],
    where_clause![
        where
            A: ToJson,
            B: ToJson,
    ],
    |self: ObjectRuntime<A, B>| {
        "values" = [
            {
                "kind" = "A";
                "value" = (&self.0) as &'cjson_lt_to_json A;
            },
            {
                "kind" = "B";
                "value" = (&self.1) as &'cjson_lt_to_json B;
            },
        ]
    }
);
#[test]
fn object_runtime() {
    assert_json_eq!(
        ObjectRuntime("hello\tworld!", 2),
        r#"{"values":[{"kind":"A","value":"hello\tworld!"},{"kind":"B","value":2}]}"#
    );
}

struct ObjectRuntime2<A, B>(A, B);

impl_to_json!(
    impl_generics![A, B],
    where_clause![
        where
            A: cjson::ser::ToJsonString,
            B: ToJson,
    ],
    |self: ObjectRuntime2<A, B>| {
        json_string!("namespace:", (&self.0) as &'cjson_lt_to_json A) =
            (&self.1) as &'cjson_lt_to_json B
    }
);
#[test]
fn object_runtime2() {
    assert_json_eq!(
        ObjectRuntime2("crlf", "\r\n"),
        r#"{"namespace:crlf":"\r\n"}"#
    );
}

struct JsonStringCompileTime;
impl_to_json!(|self: JsonStringCompileTime| json_string!["hello", " ", "world", "\n"]);

#[test]
fn json_string_compile_time() {
    assert_json_eq!(JsonStringCompileTime, r#""hello world\n""#);
}

struct JsonStringRuntime<'a> {
    to: &'a str,
    msg: &'a str,
    from: &'a str,
}
impl_to_json!(
    impl_generics!['a],
    |self: JsonStringRuntime<'a>| json_string![
        "Dear",
        " ",
        (self.to) as &'a str,
        "\n",
        (&self.msg) as &'cjson_lt_to_json str,
        "\nfrom ",
        (&self.from) as &'a str,
    ]
);

#[test]
fn json_string_runtime() {
    assert_json_eq!(
        JsonStringRuntime {
            to: "Alice",
            msg: "hello",
            from: "Bob"
        },
        r#""Dear Alice\nhello\nfrom Bob""#
    );
}

// struct JsonItemsRuntime<T>(T);
// impl_to_json!(impl_generics![T], |self: JsonItemsRuntime<T>| [
//     ..(&self.0) as &'cjson_lt_to_json T,
// ]);
