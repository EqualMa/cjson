#[macro_export]
#[doc(hidden)]
macro_rules! __expand_or {
    ([         ][$($or:tt)*]) => ($($or)*);
    ([$($e:tt)+][$($or:tt)*]) => ($($e )+);
}

#[macro_export]
macro_rules! json {
    (
        $lit:literal
    ) => {
        $crate::__private_json_const!(
            {}
            $lit
        )
    };
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        const $const_block:block
    ) => {
        $crate::__private_json_const!(
            { $(const $CONST: $ConstTy $(= $const_value)?;)* }
            $const_block
        )
    };
    ($well_known_ident:ident) => {
        $crate::__private_json_const!(
            {}
            $crate::__private::well_known_ident::$well_known_ident
        )
    };
    (($runtime_expr:expr)) => {
        $runtime_expr
    };
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        [$($array_content:tt)*]
    ) => {
        $crate::__private_json_after_array_start!(
            [
                prev[]
                current_compile_time[
                    left_bracket()
                ]
                after_value {
                    do(EOF)
                    outer_const_generics [$({ $CONST $ConstTy $(= $const_value)? })*]
                }
            ]
            $($array_content)*
        )
    };
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
macro_rules! __private_json_const {
    (
        $used_const_generics:tt
        $const_value:expr
    ) => {
        $crate::__private_impl_to_json_const! {
            then_bang($crate::__private_json_const_then!)
            then_rest()
            vis()
            $used_const_generics
            $const_value
        }
    };
}

#[macro_export]
macro_rules! __private_json_const_then {
    (
        mod($($items:tt)*)
        type $type:tt
        value($value:expr)
    ) => {{
        $($items)*
        $value
    }};
}

#[macro_export]
macro_rules! __private_json_after_array_start {
    // EOF
    (
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    right_bracket()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime items
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        ..($runtime_items:expr)
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_array_comma! {
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_items($runtime_items)
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // item
    (
        $state:tt
        $($rest:tt)+
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang(
                    $crate::__private_json_after_array_comma!
                )
                before_value()
            }
            $state
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_value {
    // literal
    (
        // options
        {
            after_comma_bang($($after_comma_bang:tt)+)
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        $lit:literal
        $(, $($rest:tt)*)?
    ) => {
        $($after_comma_bang)+ {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                    json_value($lit)
                ]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // const { ... }
    (
        // options
        {
            after_comma_bang($($after_comma_bang:tt)+)
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        const $const_block:block
        $(, $($rest:tt)*)?
    ) => {
        $($after_comma_bang)+ {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                    json_value($const_block)
                ]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // well_known_ident
    (
        // options
        {
            after_comma_bang($($after_comma_bang:tt)+)
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        $well_known_ident:ident
        $(, $($rest:tt)*)?
    ) => {
        $($after_comma_bang)+ {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                    json_value($crate::__private::well_known_ident::$well_known_ident)
                ]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // array
    (
        // options
        {
            after_comma_bang $after_comma_bang:tt
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        [$($array_content:tt)*]
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_array_start! {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                    left_bracket()
                ]
                after_value {
                    do(
                        after_comma {
                            after_comma_bang $after_comma_bang
                            rest($($($rest)*)?)
                            after_value $after_value
                        }
                    )
                }
            ]
            $($array_content)*
        }
    };
    // object
    (
        // options
        {
            after_comma_bang $after_comma_bang:tt
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        {$($object_content:tt)*}
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_object_start! {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                    left_brace()
                ]
                after_value {
                    do(
                        after_comma {
                            after_comma_bang $after_comma_bang
                            rest($($($rest)*)?)
                            after_value $after_value
                        }
                    )
                }
            ]
            $($object_content)*
        }
    };
    // runtime expr
    (
        // options
        {
            after_comma_bang($($after_comma_bang:tt)+)
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        ($runtime_expr:expr)
        $(, $($rest:tt)*)?
    ) => {
        $($after_comma_bang)+ {
            [
                prev[
                    prev $prev
                    current {
                        compile_time[
                            $($current_compile_time)*
                            $($before_value)*
                        ]
                        runtime[
                            json_value($runtime_expr)
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // macro
    (
        // options
        {
            after_comma_bang $after_comma_bang:tt
            before_value($($before_value:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        $well_known_macro:ident $bang:tt $well_known_macro_body:tt
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_macro! {
            $well_known_macro $bang $well_known_macro_body
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_value)*
                ]
                after_value {
                    do(
                        after_comma {
                            after_comma_bang $after_comma_bang
                            rest($($($rest)*)?)
                            after_value $after_value
                        }
                    )
                }
            ]
        }
    };
}

#[macro_export]
macro_rules! __private_json_after_array_comma {
    (
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    right_bracket()
                ]
            ]
            after_value $after_value
        }
    };
    // TODO: ..($runtime_items:expr)
    // item, ...
    (
        $state:tt
        $($rest:tt)+
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang(
                    $crate::__private_json_after_array_comma!
                )
                before_value(
                    comma()
                )
            }
            $state
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_type_with_const_generics {
    (
        $Type:ident
        // outer const generics
        [$({$CONST:ident $ConstTy:ty $(= $const_value:expr)?})*]
    ) => {
        $Type::<
            $({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),*
        >
    };
}

#[macro_export]
macro_rules! __private_json_after_value {
    // (
    //     [
    //         prev $prev:tt
    //         current_compile_time[
    //             $($current_compile_time:tt)*
    //         ]
    //         after_value $after_value:tt
    //     ]
    // ) => {};
    (
        chunks[
            prev_compile_runtime $prev:tt
            last_compile_time $last_compile_time:tt
        ]
        after_value {
            do(
                after_comma {
                    after_comma_bang($($after_comma_bang:tt)+)
                    rest($($rest:tt)*)
                    after_value $after_value:tt
                }
            )
        }
    ) => {
        $($after_comma_bang)+ {
            [
                prev $prev
                current_compile_time $last_compile_time
                after_value $after_value
            ]
            $($rest)*
        }
    };
    (
        chunks[
            prev_compile_runtime $prev:tt
            last_compile_time $last_compile_time:tt
        ]
        after_value {
            do(
                after_object_colon {
                    rest($($rest:tt)*)
                    after_value $after_value:tt
                }
            )
        }
    ) => {
        $crate::__private_json_after_object_colon! {
            [
                prev $prev
                current_compile_time $last_compile_time
                after_value $after_value
            ]
            $($rest)*
        }
    };
    (
        chunks[
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
        after_value {
            do(EOF)
            outer_const_generics $outer_const_generics:tt
        }
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state($crate::r#const::State::INIT)
            then(
                $crate::r#const::CompileTimeChunk::<
                    $crate::__private_json_type_with_const_generics![
                        HasConstCompileTimeChunk
                        $outer_const_generics
                    ]
                >::JSON_VALUE
            )
            tokens $only_compile_time
            outer_const_generics $outer_const_generics
        }
    };
    (
        chunks[
            prev_compile_runtime $prev_compile_runtime:tt
            last_compile_time $last_compile_time:tt
        ]
        after_value {
            do(EOF)
            outer_const_generics $outer_const_generics:tt
        }
    ) => {
        $crate::r#const::AssertJsonValueChunks(
            $crate::__private_json_concat_chunks! {
                prev_state($crate::r#const::State::INIT)
                outer_const_generics $outer_const_generics
                compile_runtime $prev_compile_runtime
                then_macro_bang(
                    $crate::__private_json_after_value_concat_chunks_then!
                )
                then_macro_rest(
                    last_compile_time $last_compile_time
                )
            }
        )
    };
}

#[macro_export]
macro_rules! __private_json_after_value_concat_chunks_then {
    (
        prev_compile_runtime($prev_compile_runtime:expr)
        PrevState($PrevState:ident)
        outer_const_generics $outer_const_generics:tt
        last_compile_time $last_compile_time:tt
    ) => {
        $crate::r#const::ChunkConcat(
            $prev_compile_runtime,
            $crate::__private_json_concat_only_compile_time_tokens! {
                prev_state(
                    $PrevState::STATE
                )
                then(
                    $crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT
                )
                tokens $last_compile_time
                outer_const_generics $outer_const_generics
            },
        )
    };
}

#[macro_export]
macro_rules! __private_json_concat_chunks {
    (
        prev_state $prev_state:tt
        outer_const_generics $outer_const_generics:tt
        compile_runtime[]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            prev_state $prev_state
            outer_const_generics $outer_const_generics
            $($then_macro_rest)*
        }
    };
    (
        prev_state $prev_state:tt
        outer_const_generics $outer_const_generics:tt
        compile_runtime[
            prev $prev:tt
            current $current:tt
        ]
        then_macro_bang $then_macro_bang:tt
        then_macro_rest $then_macro_rest:tt
    ) => {
        $crate::__private_json_concat_chunks! {
            prev_state $prev_state
            outer_const_generics $outer_const_generics
            compile_runtime $prev
            then_macro_bang(
                $crate::__private_json_concat_chunks_then!
            )
            then_macro_rest(
                current $current
                then_macro_bang $then_macro_bang
                then_macro_rest $then_macro_rest
            )
        }
    };
}

#[macro_export]
macro_rules! __private_json_concat_chunks_then {
    (
        // first chunk
        prev_state $prev_state:tt
        outer_const_generics $outer_const_generics:tt
        current {
            compile_time $compile_time:tt
            runtime[
                $runtime_kind:ident ($runtime_expr:expr)
            ]
        }
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state $prev_state
            then(
                let cjson_prev_compile_runtime = $crate::__private::runtime_kinds::$runtime_kind(
                    $crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT,
                    $runtime_expr,
                );

                enum PrevState {}

                impl PrevState {
                    const STATE: $crate::r#const::State =
                        <HasConstCompileTimeChunk as $crate::r#const::HasConstCompileTimeChunk>::CHUNK
                            .next_state()
                            .$runtime_kind();
                }

                $($then_macro_bang)+ {
                    prev_compile_runtime(cjson_prev_compile_runtime)
                    PrevState(PrevState)
                    outer_const_generics $outer_const_generics
                    $($then_macro_rest)*
                }

            )
            tokens $compile_time
            outer_const_generics $outer_const_generics
        }
    };
    (
        prev_compile_runtime($prev_compile_runtime:expr)
        PrevState($PrevState:ident)
        outer_const_generics $outer_const_generics:tt
        current {
            compile_time $compile_time:tt
            runtime[
                $runtime_kind:ident ($runtime_expr:expr)
            ]
        }
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state(
                $PrevState::STATE
            )
            then(
                let cjson_prev_compile_runtime = $crate::r#const::ChunkConcat(
                    $prev_compile_runtime,
                    $crate::__private::runtime_kinds::$runtime_kind(
                        $crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::DEFAULT,
                        $runtime_expr,
                    ),
                );

                {
                    enum PrevState {}

                    impl PrevState {
                        const STATE: $crate::r#const::State =
                            <HasConstCompileTimeChunk as $crate::r#const::HasConstCompileTimeChunk>::CHUNK
                                .next_state()
                                .$runtime_kind();
                    }

                    $($then_macro_bang)+ {
                        prev_compile_runtime(cjson_prev_compile_runtime)
                        PrevState(PrevState)
                        outer_const_generics $outer_const_generics
                        $($then_macro_rest)*
                    }
                }
            )
            tokens $compile_time
            outer_const_generics $outer_const_generics
        }
    };
}

macro_rules! __private_json_compile_runtime {
    () => {};
}

#[macro_export]
macro_rules! __private_json_concat_only_compile_time_tokens {
    (
        prev_state $prev_state:tt
        then($($then:tt)*)
        tokens $tokens:tt
        outer_const_generics[
            $({ $CONST:ident $ConstTy:ty $(= $const_value:expr)? })*
        ]
    ) => {{
        enum HasConstCompileTimeChunk
            <$( const $CONST: $ConstTy, )*>
        {}

        $crate::__private_impl_for_only_compile_time_tokens! {
            prev_state $prev_state
            tokens $tokens
            impl_generics($( const $CONST: $ConstTy, )*)
            for(
                HasConstCompileTimeChunk
                <$( $CONST, )*>
            )
        }

        $($then)*
    }};
}

#[macro_export]
macro_rules! __private_impl_for_only_compile_time_tokens {
    (
        prev_state $prev_state:tt
        tokens $tokens:tt
        impl_generics($($impl_generics:tt)*)
        for($For:ty)
    ) => {
        impl
            <$($impl_generics)*>
            $For
        {
            const STATED_CHUNK_STRING: $crate::r#const::StatedChunkString<
                {
                    $crate::__private_json_concat_compile_time_tokens_len! {
                        $tokens
                    }
                },
            > = {
                let mut buf = $crate::r#const::StatedChunkBuf::new($prev_state);

                $crate::__private_json_concat_compile_time_tokens_buf! {
                    buf
                    $tokens
                }

                buf.assert()
            };
        }

        impl
            <$($impl_generics)*>
            $crate::r#const::HasConstCompileTimeChunk
            for $For
        {
            const CHUNK: $crate::r#const::StatedChunkStr<'static> =
                Self::STATED_CHUNK_STRING.as_str();
        }
    };
}

#[macro_export]
macro_rules! __private_json_concat_compile_time_tokens {
    (
        $tokens:tt
    ) => {{
        const __CJSON_NEXT_STATE: $crate::r#const::State;
        enum HasConstCompileTimeChunk {}
        impl $crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
            const PREV_STATE: $crate::r#const::State = __CJSON_PREV_STATE;
            const CHUNK: &'static $crate::__private::str = compile_error!(stringify!($tokens));
            const NEXT_STATE: $crate::r#const::State = __CJSON_NEXT_STATE;
        }

        {
            const __CJSON_PREV_STATE: $crate::r#const::State = __CJSON_NEXT_STATE;
        }
    }};
}

#[macro_export]
macro_rules! __private_json_concat_compile_time_tokens_len {
    (
        [
            $($name:ident ( $($($args:tt)+)? ))*
        ]
    ) => {
        $crate::r#const::ChunkLen::DEFAULT
        $(
            .$name (
                $($crate::__private_json_expand_token_args_for_len! {
                    $name
                    $($args)+
                })?
            )
        )*
        .len()
    };
}

#[macro_export]
macro_rules! __private_json_concat_compile_time_tokens_buf {
    (
        $buf:ident
        [
            $($name:ident $args:tt)*
        ]
    ) => {
        $(
            $buf = $crate::__private_json_concat_compile_time_token_buf! {
                $buf
                $name
                $args
            };
        )*
    };
}

#[macro_export]
macro_rules! __private_json_concat_compile_time_token_buf {
    (
        $buf:ident
        json_value
        ($json_value:expr)
    ) => {
        $crate::r#const::ConstIntoJsonValueString(
            $crate::r#const::ConstIntoJson($json_value).const_into_json(),
        )
        .const_concat_after_stated_chunk_buf($buf)
    };
    (
        $buf:ident
        json_string_fragment
        ($json_string_fragment:expr)
    ) => {
        $crate::r#const::ConstIntoJsonStringFragment($json_string_fragment)
            .const_concat_after_stated_chunk_buf($buf)
    };
    (
        $buf:ident
        $name:ident
        ()
    ) => {
        $buf.$name()
    };
}

#[macro_export]
macro_rules! __private_json_expand_token_args_for_len {
    (json_value $v:expr) => {
        $crate::r#const::ConstIntoJsonValueString(
            $crate::r#const::ConstIntoJson($v).const_into_json(),
        )
        .const_into_json_value_string_len()
    };
    (json_string_fragment $v:expr) => {
        $crate::r#const::ConstIntoJsonStringFragment($v).const_into_json_string_fragment_len()
    };
}

#[macro_export]
macro_rules! __private_json_after_object_start {
    // EOF
    (
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    right_brace()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime fields
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        ..($runtime_fields:expr)
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_object_comma! {
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_fields($runtime_fields)
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // field name
    (
        $state:tt
        $($rest:tt)+
    ) => {
        $crate::__private_json_object_field_name! {
            {
                before_field_name()
            }
            $state
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_object_field_name {
    (
        {
            before_field_name($($before_field_name:tt)*)
        }
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        $(-)? const $field_name:block
        = $($rest:tt)+
    ) => {
        $crate::__private_json_after_object_colon! {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_field_name)*
                    double_quote()
                    json_string_fragment($field_name)
                    double_quote()
                ]
                after_value $after_value
            ]
            $($rest)+
        }
    };
    (
        // options
        {
            before_field_name($($before_field_name:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // tokens
        $field_name:literal
        = $($rest:tt)+
    ) => {
        $crate::__private_json_after_object_colon! {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_field_name)*
                    double_quote()
                    json_string_fragment($field_name)
                    double_quote()
                ]
                after_value $after_value
            ]
            $($rest)+
        }
    };
    (
        {
            before_field_name($($before_field_name:tt)*)
        }
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        $well_known_macro:ident $bang:tt $well_known_macro_body:tt
        = $($rest:tt)+
    ) => {
        $crate::__private_json_macro! {
            $well_known_macro $bang $well_known_macro_body
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    $($before_field_name)*
                ]
                after_value {
                    do(
                        after_object_colon {
                            rest($($rest)+)
                            after_value $after_value
                        }
                    )
                }
            ]
        }
    };
    (
        // options
        {
            before_field_name($($before_field_name:tt)*)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // tokens
        ($runtime_field_name:expr)
        = $($rest:tt)+
    ) => {
        $crate::__private_json_after_object_colon! {
            [
                prev[
                    prev $prev
                    current {
                        compile_time[
                            $($current_compile_time)*
                            $($before_field_name)*
                            double_quote()
                        ]
                        runtime[
                            json_string_fragment($runtime_field_name)
                        ]
                    }
                ]
                current_compile_time[
                    double_quote()
                ]
                after_value $after_value
            ]
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_after_object_colon {
    // $lit:literal
    (
        $state:tt
        $lit:literal
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang($crate::__private_json_after_object_field_value!)
                before_value(colon())
            }
            $state
            $lit
            , $($($rest)*)?
        }
    };
    // ($runtime:expr)
    // $well_known_ident:ident
    // [$($array_content:tt)*]
    (
        $state:tt
        $v:tt
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang($crate::__private_json_after_object_field_value!)
                before_value(colon())
            }
            $state
            $v
            , $($($rest)*)?
        }
    };
    // const { ... }
    (
        $state:tt
        const $const_block:block
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang($crate::__private_json_after_object_field_value!)
                before_value(colon())
            }
            $state
            const $const_block
            , $($($rest)*)?
        }
    };
    // macro
    (
        $state:tt
        $well_known_macro:ident $bang:tt $well_known_macro_body:tt
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang($crate::__private_json_after_object_field_value!)
                before_value(colon())
            }
            $state
            $well_known_macro $bang $well_known_macro_body
            , $($($rest)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_json_after_object_field_value {
    (
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    right_brace()
                ]
            ]
            after_value $after_value
        }
    };
    // TODO: ..($runtime_fields:expr)
    // field_value, ...
    (
        $state:tt
        $($rest:tt)+
    ) => {
        $crate::__private_json_object_field_name! {
            {
                before_field_name(
                    comma()
                )
            }
            $state
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_macro {
    ($macro:ident $bang:tt {$($body:tt)*} $state:tt) => [ $crate::__private_json_macro! { $macro $bang ($($body)*) $state} ];
    ($macro:ident $bang:tt [$($body:tt)*] $state:tt) => [ $crate::__private_json_macro! { $macro $bang ($($body)*) $state} ];
    ($macro:ident $bang:tt $body:tt       $state:tt) => {
        $crate::__private::well_known_macro::$macro! { $body $state }
    };
}

#[macro_export]
macro_rules! __private_json_string {
    (
        ($($body:tt)*)
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
    ) => {
        $crate::__private_json_in_string!(
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    double_quote()
                ]
                after_value $after_value
            ]
            $($body)*
        )
    };
}

#[macro_export]
macro_rules! __private_json_in_string {
    // EOF
    (
        [
            prev $prev:tt
            current_compile_time[
                $($current_compile_time:tt)*
            ]
            after_value $after_value:tt
        ]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime $prev
                last_compile_time[
                    $($current_compile_time)*
                    double_quote()
                ]
            ]
            after_value $after_value
        }
    };
    // fragment
    (
        $state:tt
        $($rest:tt)+
    ) => {
        $crate::__private_json_string_fragment! {
            {
                after_bang(
                    $crate::__private_json_in_string!
                )
            }
            $state
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_string_fragment {
    // literal
    (
        // options
        {
            after_bang($($after_bang:tt)+)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        $lit:literal
        $(, $($rest:tt)*)?
    ) => {
        $($after_bang)+ {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    json_string_fragment($lit)
                ]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // const { ... }
    (
        // options
        {
            after_bang($($after_bang:tt)+)
        }
        // state
        [
            prev $prev:tt
            current_compile_time[$($current_compile_time:tt)*]
            after_value $after_value:tt
        ]
        // tokens
        const $const_block:block
        $(, $($rest:tt)*)?
    ) => {
        $($after_bang)+ {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    json_string_fragment($const_block)
                ]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // runtime expr after runtime expr
    (
        // options
        {
            after_bang($($after_bang:tt)+)
        }
        // state
        [
            prev[
                prev $prev:tt
                current {
                    compile_time $current_compile_time:tt
                    runtime[
                        json_string_fragment($prev_fragment:expr)
                    ]
                }
            ]
            current_compile_time[]
            after_value $after_value:tt
        ]
        // tokens
        ($runtime_expr:expr)
        $(, $($rest:tt)*)?
    ) => {
        $($after_bang)+ {
            [
                prev[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_string_fragment(
                                $crate::ser::texts::Chain(
                                    $prev_fragment,
                                    $runtime_expr,
                                )
                            )
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
    // runtime expr
    (
        // options
        {
            after_bang($($after_bang:tt)+)
        }
        // state
        [
            prev $prev:tt
            current_compile_time $current_compile_time:tt
            after_value $after_value:tt
        ]
        // tokens
        ($runtime_expr:expr)
        $(, $($rest:tt)*)?
    ) => {
        $($after_bang)+ {
            [
                prev[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_string_fragment($runtime_expr)
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($($rest)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_json_well_known_macro_json_string {
    ($($t:tt)*) => {
        $crate::__private_json_string! {
            $($t)*
        }
    };
}

#[cfg(test)]
mod tests;
