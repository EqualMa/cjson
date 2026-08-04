use cjson::{impl_json, ser::IntoJson, ser::ToJson2 as ToJson};

macro_rules! assert_json_eq {
    ($v:expr, $eq:expr) => {
        assert_eq!(::cjson::ser::ToJsonExt::to_json_as_string(&$v), $eq);
        assert_eq!(
            ::cjson::ser::ToJsonExt::to_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>(&$v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
        // TODO: test async try
        assert_eq!(::cjson::ser::IntoJsonExt::into_json_as_string($v), $eq);
        assert_eq!(
            ::cjson::ser::IntoJsonExt::into_json_as_try::<::cjson::ser::IoWrite<Vec<u8>>>($v)
                .unwrap()
                .0,
            $eq.as_bytes()
        );
    };
}

struct Null;
impl_json!(|self: Null| null);

#[test]
fn null() {
    assert_json_eq!(Null, "null");
}

struct Runtime<T>(T);
impl_json!(
    impl_generics![T],
    where_clause_to![T: ToJson],
    where_clause_into![T: IntoJson],
    |self: Runtime<T>| (auto_ref!(self.0)) as auto_ref![T],
);

struct RuntimeDeriveFrom<T>(T);
impl_json!(
    impl_generics![T],
    derive_from![T],
    |self: RuntimeDeriveFrom<T>| (auto_ref!(self.0)) as auto_ref![T],
);

#[test]
fn runtime() {
    assert_json_eq!(Runtime(cjson::values::Finite::new_f32(1.2).unwrap()), "1.2");
    assert_json_eq!(
        RuntimeDeriveFrom(cjson::values::Finite::new_f64(3.14159).unwrap()),
        "3.14159"
    );
}

struct Literal;
impl_json!(|self: Literal| 1u8);
#[test]
fn literal() {
    assert_json_eq!(Literal, "1");
}

struct Const;
impl_json!(|self: Const| const { false });

#[test]
fn r#const() {
    assert_json_eq!(Const, "false");
}

const V: &str = "hello\tworld!";
struct Const2;
impl_json!(|self: Const2| const { V });
#[test]
fn const2() {
    assert_json_eq!(Const2, "\"hello\\tworld!\"");
}

struct Const3<const V: bool>;
impl_json!(
    impl_generics![const V: bool],
    |self: Const3<V>| json_value_generic_const! { V }
);
#[test]
fn r#const3() {
    assert_json_eq!(Const3::<false>, "false");
    assert_json_eq!(Const3::<true>, "true");
}

struct ArrayCompileTime;
impl_json!(|self: ArrayCompileTime| [true, false]);
#[test]
fn array_compile_time() {
    assert_json_eq!(ArrayCompileTime, "[true,false]");
}

const V2: bool = true;
struct ArrayCompileTime2<const NOT_USED: u8>;
impl_json!(
    impl_generics![const N: u8],
    //
    |self: ArrayCompileTime2<N>| [[const { V2 }], null]
);
#[test]
fn array_compile_time2() {
    assert_json_eq!(ArrayCompileTime2::<0>, "[[true],null]");
}

struct ArrayRuntime(u8);
impl_json!(|self: ArrayRuntime| [1u8, (self.0) as u8, 3u8]);
#[test]
fn array_runtime() {
    assert_json_eq!(ArrayRuntime(2), "[1,2,3]");
}

struct ArrayRuntime2(u8);
impl_json!(|self: ArrayRuntime2| [1u8, (auto_ref!(self.0)) as auto_ref![u8], 3u8]);
#[test]
fn array_runtime2() {
    assert_json_eq!(ArrayRuntime2(20), "[1,20,3]");
}

struct MyU8<const V: u8>;
impl_json!(
    impl_generics![const V: u8],
    //
    |self: MyU8<V>| json_value_generic_const! { V }
);
struct ArrayRuntime3<const V: u8>;
impl_json!(impl_generics![const V: u8], |self: ArrayRuntime3<V>| [
    1u8,
    (&MyU8) as &'static MyU8<V>,
    json_value_generic_const![V, 1],
    3u8
]);
#[test]
fn array_runtime3() {
    assert_json_eq!(ArrayRuntime3::<0>, "[1,0,0,3]");
    assert_json_eq!(ArrayRuntime3::<2>, "[1,2,2,3]");
}

struct ObjectCompileTime;
impl_json!(|self: ObjectCompileTime| { "name" = ["value"] });
#[test]
fn object_compile_time() {
    assert_json_eq!(ObjectCompileTime, r#"{"name":["value"]}"#);
}

struct ObjectRuntime<A, B>(A, B);

impl_json!(
    impl_generics![A, B],
    where_clause_to![
        A: ToJson,
        B: ToJson,
    ],
    where_clause_into![
        A: IntoJson,
        B: IntoJson,
    ],
    |self: ObjectRuntime<A, B>| {
        "values" = [
            {
                "kind" = "A";
                "value" = (auto_ref!(self.0)) as auto_ref!(A);
            },
            {
                "kind" = "B";
                "value" = (auto_ref!(self.1)) as auto_ref!(B);
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

impl_json!(
    impl_generics![A, B],
    where_clause_to![
        A: cjson::ser::ToJsonString2,
        B: ToJson,
    ],
    where_clause_into![
        A: cjson::ser::IntoJsonString,
        B: IntoJson,
    ],
    |self: ObjectRuntime2<A, B>| {
        //
        json_string!("namespace:", (auto_ref!(self.0)) as auto_ref![A]) =
            (auto_ref!(self.1)) as auto_ref!(B)
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
impl_json!(|self: JsonStringCompileTime| json_string!["hello", " ", "world", "\n"]);

#[test]
fn json_string_compile_time() {
    assert_json_eq!(JsonStringCompileTime, r#""hello world\n""#);
}

struct JsonStringRuntime<'a> {
    to: &'a str,
    msg: &'a str,
    from: &'a str,
}
impl_json!(
    impl_generics!['a],
    |self: JsonStringRuntime<'a>| json_string![
        "Dear",
        " ",
        (self.to) as &'a str,
        "\n",
        (&self.msg) as &str,
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

pub enum NeverWithJsonX {}

impl_json!(|self: NeverWithJsonX| #[json_x(macro())]
// Note that we don't need to parenthesize the matched expr
// because the whole match expression is parsed as a single expr after #[json_x]
match auto_deref!(self) {});

pub enum NeverWithMatch {}

// Note that we have to parenthesize the matched expr
// so that it stays as one TokenTree
impl_json!(|self: NeverWithMatch| match (auto_deref!(self)) {});
