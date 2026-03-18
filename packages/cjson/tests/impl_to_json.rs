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
}
