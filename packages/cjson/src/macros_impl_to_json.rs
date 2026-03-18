#[macro_export]
macro_rules! impl_to_json {
    (
        $(impl_generics![$($impl_generics:tt)*],)?
        $(where_clause![$($where_clause:tt)*],)?
        $({$($used_const_generics:tt)*},)?
        |$_self:ident : $Type:ty| $($macro_body:tt)*
    ) => {
        $crate::__private_impl_to_json_parse! {
            ( $($macro_body)* )
            {$($($used_const_generics)*)?}
            {
                impl_generics($($($impl_generics)*)?)
                where_clause($($($where_clause)*)?)
                self($_self)
                type($Type)
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_parse {
    (
        ( $lit:literal $(,)? )
        {} // used_const_generics
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parse!(
            ( const { $lit } )
            {}
            $data
        )
    };
    (
        ( const $const_block:block )
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_const!(
            then_bang($crate::__private_impl_to_json_expand!)
            then_rest($data)
            vis(pub)
            $used_const_generics
            const $const_block
        )
    };
    (
        ( $well_known_ident:ident $(,)? )
        {}
        $data:tt
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod()
            type($crate::__private::well_known_ident::$well_known_ident)
            value($crate::__private::well_known_ident::$well_known_ident)
            $data
        }
    };
    (
        ( ($runtime_expr:expr) as $RuntimeType:ty $(,)? )
        {}
        $data:tt
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod()
            type($RuntimeType)
            value($runtime_expr)
            $data
        }
    };
    (
        ( [$($array_content:tt)*] $(,)? )
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_json_after_array_start!(
            [
                prev[]
                current_compile_time[
                    left_bracket()
                ]
                after_value {
                    EOF_impl_to_json(
                        $used_const_generics
                        $data
                    )
                }
            ]
            $($array_content)*
        )
    };
    // TODO:
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        $well_known_macro:ident $bang:tt $well_known_macro_body:tt
    ) => {
        $crate::__private_json_macro!(
            $well_known_macro $bang $well_known_macro_body
            [
                prev[]
                current_compile_time[]
                after_value {
                    do(EOF)
                    outer_const_generics [$({ $CONST $ConstTy $(= $const_value)? })*]
                }
            ]
        )
    };
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        {$($object_content:tt)*}
    ) => {
        $crate::__private_json_after_object_start!(
            [
                prev[]
                current_compile_time[
                    left_brace()
                ]
                after_value {
                    do(EOF)
                    outer_const_generics [$({ $CONST $ConstTy $(= $const_value)? })*]
                }
            ]
            $($object_content)*
        )
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_expand {
    (
        mod($($mod_tt:tt)*)
        type($ToJsonType:ty)
        value($to_json_value:expr)
        {
            impl_generics($($impl_generics:tt)*)
            where_clause($($where_clause:tt)*)
            self($_self:ident)
            type($Type:ty)
        }
    ) => { const _: () = {
        $($mod_tt)*

        impl< $($impl_generics)* > $crate::__private::ImplToJsonHelper
            for $Type
            $($where_clause)*
        {
            type ImplToJsonHelper<'cjson_lt_to_json> = $ToJsonType
            where Self: 'cjson_lt_to_json;
        }


        impl< $($impl_generics)* > $crate::ser::ToJson
            for $Type
            $($where_clause)*
        {
            type ToJson<'cjson_lt_to_json> = <
                <Self as $crate::__private::ImplToJsonHelper>::ImplToJsonHelper<'cjson_lt_to_json>
                as $crate::ser::ToJson
            >::ToJson<'cjson_lt_to_json>
            where Self: 'cjson_lt_to_json;

            fn to_json(&$_self) -> Self::ToJson<'_> {
                <
                    <Self as $crate::__private::ImplToJsonHelper>::ImplToJsonHelper<'_>
                    as $crate::ser::ToJson
                >::to_json(&$to_json_value)
            }
        }
    }; };
}

#[macro_export]
macro_rules! __private_impl_to_json_mod {
    (
        $compile_runtime:tt
        $last_compile_time:tt
        $used_const_generics:tt
    ) => {
        $crate::__private_impl_to_json_define_struct_with_generics! $used_const_generics

        mod cjson_macro_generated_types {
            $crate::__private_impl_to_json_mod_resolve! {
                $compile_runtime
                $last_compile_time
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_define_struct_with_generics {
    () => {};
    (
        // used const generics
        $(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)+
    ) => {
        pub struct CjsonMacroGeneratedChunkWithConstGenerics<
            T
            $(, const $CONST: $ConstTy)+
        >(T);
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_mod_resolve {
    (
        [$compile_runtime:tt]
        $last_compile_time:tt
    ) => {
        pub enum HasConstCompileTimeChunk {}
        pub mod next {
            pub enum HasConstCompileTimeChunk {}
        }
    };
    (
        [$compile_runtime:tt $($rest:tt)+]
        $last_compile_time:tt
    ) => {
        pub enum HasConstCompileTimeChunk {}
        pub mod next {
            $crate::__private_impl_to_json_mod_resolve! {
                [$($rest)+]
                $last_compile_time
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_impl {
    (
        $compile_runtime:tt
        $last_compile_time:tt
        {$(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)*}
    ) => {
        $crate::__private_impl_to_json_impl_resolve! {
            $compile_runtime
            $last_compile_time
            // prev_state
            ($crate::r#const::State::INIT)
            // prepend_impl_generics
            () // only the first compile_runtime impl doesn't need 'cjson_lt_to_json
            // impl_generics
            ($( const $CONST: $ConstTy, )*)
            // used_const_names
            ($($CONST,)*)
            // next_paths
            ()
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_impl_resolve {
    (
        []
        $compile_time:tt
        $prev_state:tt
        $prepend_impl_generics:tt
        $impl_generics:tt
        $used_const_names:tt
        ($($next_list:tt)*)
    ) => {
        $crate::__private_impl_for_only_compile_time_tokens! {
            prev_state $prev_state
            tokens $compile_time
            impl_generics $impl_generics
            for(
                $crate::__private_impl_to_json_for_type![
                    used_const_names $used_const_names
                    prefix_path(cjson_macro_generated_types:: $($next_list::)*)
                ]
            )
            prepend_impl_generics $prepend_impl_generics
        }
    };
    (
        [
            {
                compile_time $compile_time:tt
                runtime[
                    $runtime_kind:ident $runtime_expr:tt
                    $(as $runtime_type:ty)?
                ]
            }
            $($rest_compile_runtime:tt)*
        ]
        $last_compile_time:tt
        $prev_state:tt
        $prepend_impl_generics:tt
        $impl_generics:tt
        $used_const_names:tt
        ($($next_list:tt)*)
    ) => {
        $crate::__private_impl_to_json_impl_resolve! {
            []
            $compile_time
            $prev_state
            $prepend_impl_generics
            $impl_generics
            $used_const_names
            ($($next_list)*)
        }

        $crate::__private_impl_to_json_impl_resolve! {
            [$($rest_compile_runtime)*]
            $last_compile_time
            (<
                $crate::__private::runtime_kinds::$runtime_kind<
                    $crate::r#const::CompileTimeChunk<
                        $crate::__private_impl_to_json_for_type![
                            used_const_names $used_const_names
                            prefix_path(cjson_macro_generated_types:: $($next_list::)*)
                        ]
                    >,
                    // $runtime_type might use 'cjson_lt_to_json
                    $crate::__expand_or![[$($runtime_type)?][_]]
                > as $crate::r#const::RuntimeChunk
            >::NEXT_STATE)
            ('cjson_lt_to_json,)
            $impl_generics
            $used_const_names
            ($($next_list)* next)
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_type {
    (
        $compile_runtime:tt
        $last_compile_time:tt
        {$(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)*} // used const generics
    ) => {
        $crate::r#const::AssertJsonValueChunks<
            $crate::__private_impl_to_json_type_resolve! {
                $compile_runtime
                $last_compile_time
                ($( $CONST, )*)
                next_list()
            }
        >
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_type_resolve {
    (
        [{
            compile_time $compile_time:tt
            runtime[
                $runtime_kind:ident $runtime_expr:tt
                $(as $runtime_type:ty)?
            ]
        }]
        $last_compile_time:tt
        $used_const_names:tt
        next_list($($next_list:tt)*)
    ) => {
        $crate::r#const::ChunkConcat<
            $crate::__private::runtime_kinds::$runtime_kind<
                $crate::r#const::CompileTimeChunk<
                    $crate::__private_impl_to_json_for_type![
                        used_const_names $used_const_names
                        prefix_path(cjson_macro_generated_types:: $($next_list::)*)
                    ]
                >,
                $crate::__expand_or![[$($runtime_type)?][_]]
            >,
            $crate::r#const::CompileTimeChunk<
                $crate::__private_impl_to_json_for_type![
                    used_const_names $used_const_names
                    prefix_path(cjson_macro_generated_types:: $($next_list::)* next::)
                ]
            >,
        >
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_value {
    (
        $compile_runtime:tt
        $last_compile_time:tt
        $used_const_generics:tt
        // {$(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)*} // used const generics
    ) => {
        $crate::r#const::AssertJsonValueChunks(
            $crate::__private_impl_to_json_value_resolve!(
                $compile_runtime
                $last_compile_time
            )
        )
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_value_resolve {
    (
        [$compile_runtime:tt]
        $last_compile_time:tt
    ) => {
        $crate::r#const::ChunkConcat(
            $crate::__private_impl_to_json_value_compile_runtime! $compile_runtime,
            $crate::r#const::CompileTimeChunk::DEFAULT
        )
    };
    (
        [$compile_runtime:tt $($rest_compile_runtime:tt)+]
        $last_compile_time:tt
    ) => {
        $crate::r#const::ChunkConcat(
            $crate::__private_impl_to_json_value_compile_runtime! $compile_runtime,
            $crate::__private_impl_to_json_value_resolve!(
                [$($rest_compile_runtime)+]
                $last_compile_time
            )
        )
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_value_compile_runtime {
    (
        compile_time $compile_time:tt
        runtime[
            $runtime_kind:ident ($runtime_expr:expr)
            $(as $runtime_type:ty)?
        ]
    ) => {
        $crate::__private::runtime_kinds::$runtime_kind(
            $crate::r#const::CompileTimeChunk::DEFAULT,
            $runtime_expr,
        )
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_const {
    (
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
        vis($($vis:tt)*)
        {} // used const generics
        $const_block:expr
    ) => { $($then_bang)+ {
        mod(
            $($vis)* enum HasConstJsonValue {}

            impl $crate::r#const::HasConstJsonValue for HasConstJsonValue {
                const JSON_VALUE: $crate::ser::texts::Value<&'static $crate::__private::str> = {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::r#const::ConstIntoJsonValueString(
                            $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                        )
                        .const_into_json_value_string::<{
                            $crate::r#const::ConstIntoJsonValueString(
                                $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                            )
                            .const_into_json_value_string_len()
                        }>(),
                    )
                    .const_as_json_value_str()
                };
            }
        )
        type(
            $crate::r#const::ConstJsonValue::<HasConstJsonValue>
        )
        value(
            $crate::r#const::ConstJsonValue::<HasConstJsonValue>::DEFAULT
        )
        $($then_rest)*
    } };
    (
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
        vis($($vis:tt)*)
        {$(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)+} // used const generics
        $const_block:expr
    ) => { $($then_bang)+ {
        mod(
            $($vis)* enum HasConstJsonValue
                <$(const $CONST: $ConstTy),+>
                {}

            impl
                <$(const $CONST: $ConstTy),+>
                $crate::r#const::HasConstJsonValue for HasConstJsonValue
                <$(      $CONST          ),+>
            {
                const JSON_VALUE: $crate::ser::texts::Value<&'static $crate::__private::str> = {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::r#const::ConstIntoJsonValueString(
                            $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                        )
                        .const_into_json_value_string_without_const_len()
                        // TODO: rust limitation: generic parameters may not be used in const operations
                        // .const_into_json_value_string::<LEN>()
                    )
                    .const_as_json_value_str()
                };
            }
        )
        type(
            $crate::r#const::ConstJsonValue::<HasConstJsonValue::
                <$({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),+>
            >
        )
        value(
            $crate::r#const::ConstJsonValue::<HasConstJsonValue::
                <$({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),+>
            >::DEFAULT
        )
        $($then_rest)*
    } };
}

#[macro_export]
macro_rules! __private_impl_to_json_after_value {
    (
        [
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
        (
            // used_const_generics
            {$( const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ;)*}
            $data:tt
        )
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod(
                pub enum HasConstCompileTimeChunk<$( const $CONST: $ConstTy ),*> {}

                $crate::__private_impl_for_only_compile_time_tokens! {
                    prev_state($crate::r#const::State::INIT)
                    tokens $only_compile_time
                    impl_generics($( const $CONST: $ConstTy, )*)
                    for(HasConstCompileTimeChunk<$( $CONST ),*>)
                }
            )
            type(
                $crate::r#const::ConstJsonValue::<
                    $crate::r#const::CompileTimeChunkIsJsonValue<
                        HasConstCompileTimeChunk<$( $CONST ),*>
                    >
                >
            )
            value(
                $crate::r#const::CompileTimeChunk::<
                    HasConstCompileTimeChunk<$( $CONST ),*>
                >::JSON_VALUE
            )
            $data
        }
    };
    (
        [
            prev_compile_runtime $prev_compile_runtime:tt
            last_compile_time $last_compile_time:tt
        ]
        $args:tt
    ) => {
        $crate::__private_impl_to_json_after_value_mixed! {
            $prev_compile_runtime
            []
            ($last_compile_time $args)
        }
    };
}

// TODO: optimize for macro recursion limit
#[macro_export]
macro_rules! __private_impl_to_json_after_value_mixed {
    (
        [
            prev $prev:tt
            current $current:tt
        ]
        [ $($parsed:tt)* ]
        $args:tt
    ) => {
        $crate::__private_impl_to_json_after_value_mixed! {
            $prev
            [
                $current
                $($parsed)*
            ]
            $args
        }
    };
    (
        []
        $parsed:tt
        ($last_compile_time:tt ($used_const_generics:tt $data:tt))
    ) => {
        $crate::__private_impl_to_json_after_value_mixed_expand! {
            {
                $parsed
                $last_compile_time
                $used_const_generics
            }
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_after_value_mixed_expand {
    (
        $t:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod(
                $crate::__private_impl_to_json_mod! $t
                $crate::__private_impl_to_json_impl! $t
                // pub enum HasConstCompileTimeChunk<$( const $CONST: $ConstTy ),*> {}

                // $crate::__private_impl_for_only_compile_time_tokens! {
                //     prev_state($crate::r#const::State::INIT)
                //     tokens $only_compile_time
                //     impl_generics($( const $CONST: $ConstTy, )*)
                //     for(HasConstCompileTimeChunk<$( $CONST ),*>)
                // }
            )
            type(
                $crate::__private_impl_to_json_type! $t

                // $crate::r#const::ConstJsonValue::<
                //     $crate::r#const::CompileTimeChunkIsJsonValue<
                //         HasConstCompileTimeChunk<$( $CONST ),*>
                //     >
                // >
            )
            value(
                $crate::__private_impl_to_json_value! $t

                // $crate::r#const::CompileTimeChunk::<
                //     HasConstCompileTimeChunk<$( $CONST ),*>
                // >::JSON_VALUE
            )
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_after_value_compile_runtime {
    (
        // first compile_runtime chunk
        [
            prev[]
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
        // item_define()
        // item_impl()
        // type()
        // value()
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod(
                pub enum HasConstCompileTimeChunk<$( const $CONST: $ConstTy ),*> {}

                $crate::__private_impl_for_only_compile_time_tokens! {
                    prev_state($crate::r#const::State::INIT)
                    tokens $only_compile_time
                    impl_generics($( const $CONST: $ConstTy, )*)
                    for(HasConstCompileTimeChunk<$( $CONST ),*>)
                }
            )
            type(
                $crate::r#const::ConstJsonValue::<
                    $crate::r#const::CompileTimeChunkIsJsonValue<
                        HasConstCompileTimeChunk<$( $CONST ),*>
                    >
                >
            )
            value(
                $crate::r#const::CompileTimeChunk::<
                    HasConstCompileTimeChunk<$( $CONST ),*>
                >::JSON_VALUE
            )
            $data
        }
        compile_error! {
                    stringify! {
        {
                        compile_time $compile_time:tt
                        runtime $runtime:tt
                    }
                    }
                }
        // TODO:
    };
    (
        [
            prev $prev:tt
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
    ) => {
        compile_error! {
            stringify! {
                          compile_time $compile_time:tt
                runtime $runtime:tt
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_concat_only_compile_time_tokens {
    (
        used_const_generics[
            $( const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ;)*
        ]
        prev_state $prev_state:tt
        tokens $tokens:tt
        prefix_path $prefix_path:tt
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
    ) => { $($then_bang)+ {
        impl(
            $crate::__private_impl_for_only_compile_time_tokens! {
                prev_state $prev_state
                tokens $tokens:tt
                impl_generics($( const $CONST: $ConstTy, )*)
                for($crate::__private_impl_to_json_for_type![
                    used_const_names(
                        $( $CONST, )*
                    )
                    prefix_path $prefix_path
                ])
            }
        )
        $($then_rest)*
    } };
}

#[macro_export]
macro_rules! __private_impl_to_json_for_type {
    (used_const_names() prefix_path($($prefix_path:tt)*)) => {
        $($prefix_path)* HasConstCompileTimeChunk
    };
    (used_const_names($($consts:tt)+) prefix_path($($prefix_path:tt)*)) => {
        CjsonMacroGeneratedChunkWithConstGenerics<$($prefix_path)* HasConstCompileTimeChunk, $($consts)+>
    };
}
