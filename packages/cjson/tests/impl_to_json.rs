use cjson::{
    impl_to_json,
    ser::{ToJson, exts::TextExt as _},
};

#[test]
fn impl_to_json() {
    {
        struct Null;
        impl_to_json!(|self: Null| null);

        assert_eq!(Null.to_json().into_string().into_inner(), "null");
    }
    {
        struct Runtime<T: ToJson>(T);
        impl_to_json!(
            impl_generics![T],
            where_clause![where T: ToJson],
            |self: Runtime<T>| (&self.0) as &'cjson_lt_to_json T,
        );
        assert_eq!(
            Runtime(cjson::values::Finite::new_f32(1.2).unwrap())
                .to_json()
                .into_string()
                .into_inner(),
            "1.2"
        );
    }
    {
        struct Literal;
        impl_to_json!(|self: Literal| 1u8);

        assert_eq!(Literal.to_json().into_string().into_inner(), "1");
    }
    {
        struct Const;
        impl_to_json!(|self: Const| const { false });
        assert_eq!(Const.to_json().into_string().into_inner(), "false");
    }
    {
        const V: &str = "hello\tworld!";
        struct Const;
        impl_to_json!(|self: Const| const { V });
        assert_eq!(
            Const.to_json().into_string().into_inner(),
            "\"hello\\tworld!\""
        );
    }
    {
        struct Const<const V: bool>;
        impl_to_json!(
            impl_generics![const V: bool],
            {
                const V: bool;
            },
            |self: Const<V>| const { V }
        );
        assert_eq!(Const::<false>.to_json().into_string().into_inner(), "false");
        assert_eq!(Const::<true>.to_json().into_string().into_inner(), "true");
    }

    {
        struct ArrayCompileTime;
        impl_to_json!(|self: ArrayCompileTime| [true, false]);

        assert_eq!(
            ArrayCompileTime.to_json().into_string().into_inner(),
            "[true,false]"
        );
    }

    {
        const V: bool = true;
        struct ArrayCompileTime<const NOT_USED: u8>;
        impl_to_json!(
            impl_generics![const N: u8],
            {
                const N: u8;
            },
            |self: ArrayCompileTime<N>| [[const { V }], null]
        );

        assert_eq!(
            ArrayCompileTime::<0>.to_json().into_string().into_inner(),
            "[[true],null]"
        );
    }

    {
        struct ArrayRuntime(u8);
        impl_to_json!(|self: ArrayRuntime| [1u8, (self.0) as u8, 3u8]);

        assert_eq!(
            ArrayRuntime(2).to_json().into_string().into_inner(),
            "[1,2,3]"
        );
    }

    {
        struct ArrayRuntime(u8);
        impl_to_json!(|self: ArrayRuntime| [1u8, (&self.0) as &'cjson_lt_to_json u8, 3u8]);

        assert_eq!(
            ArrayRuntime(20).to_json().into_string().into_inner(),
            "[1,20,3]"
        );
    }

    {
        struct MyU8<const V: u8>;
        impl_to_json!(
            impl_generics![const V: u8],
            {
                const V: u8;
            },
            |self: MyU8<V>| const { V }
        );

        struct ArrayRuntime<const V: u8>;
        impl_to_json!(
            impl_generics![const V: u8],
            {
                const V: u8;
            },
            |self: ArrayRuntime<V>| [1u8, (MyU8) as MyU8<V>, 3u8]
        );

        assert_eq!(
            ArrayRuntime::<0>.to_json().into_string().into_inner(),
            "[1,0,3]"
        );
    }

    {
        struct ObjectCompileTime;
        impl_to_json!(|self: ObjectCompileTime| { "name" = ["value"] });

        assert_eq!(
            ObjectCompileTime.to_json().into_string().into_inner(),
            r#"{"name":["value"]}"#
        );
    }

    {
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

        assert_eq!(
            ObjectRuntime("hello\tworld!", 2)
                .to_json()
                .into_string()
                .into_inner(),
            r#"{"values":[{"kind":"A","value":"hello\tworld!"},{"kind":"B","value":2}]}"#
        );
    }

    {
        struct ObjectRuntime<A, B>(A, B);

        impl_to_json!(
            impl_generics![A, B],
            where_clause![
                where
                    A: cjson::ser::ToJsonStringFragment,
                    B: ToJson,
            ],
            |self: ObjectRuntime<A, B>| {
                json_string!("namespace:", (&self.0) as &'cjson_lt_to_json A) =
                    (&self.1) as &'cjson_lt_to_json B
            }
        );

        assert_eq!(
            ObjectRuntime("crlf", "\r\n")
                .to_json()
                .into_string()
                .into_inner(),
            r#"{"namespace:crlf":"\r\n"}"#
        );
    }

    {
        struct JsonStringCompileTime;
        impl_to_json!(|self: JsonStringCompileTime| json_string!["hello", " ", "world", "\n"]);

        assert_eq!(
            JsonStringCompileTime.to_json().into_string().into_inner(),
            r#""hello world\n""#
        );
    }

    {
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

        assert_eq!(
            JsonStringRuntime {
                to: "Alice",
                msg: "hello",
                from: "Bob"
            }
            .to_json()
            .into_string()
            .into_inner(),
            r#""Dear Alice\nhello\nfrom Bob""#
        );
    }
}
