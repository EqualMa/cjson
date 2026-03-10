#[macro_export]
#[doc(hidden)]
macro_rules! __expand_or {
    ([         ][$($or:tt)*]) => ($($or)*);
    ([$($e:tt)+][$($or:tt)*]) => ($($e )+);
}

#[macro_export]
macro_rules! json {
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        $lit:literal
    ) => {
        $crate::__private_json!(
            [$({$CONST $ConstTy $(= $const_value)?})*]
            const { $lit }
        )
    };
    (
        $(const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ; $(,)?)*
        const $const_block:block
    ) => {
        $crate::__private_json!(
            [$({$CONST $ConstTy $(= $const_value)?})*]
            const $const_block
        )
    };
    ($well_known_ident:ident) => {
        $crate::__private_json!(
            []
            const { $crate::__private::well_known_ident::$well_known_ident }
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
                    outer_const_generics [$({ const $CONST: $ConstTy $(= $const_value)? })*]
                }
            ]
            $($array_content)*
        )
    };
    ({$($object_content:tt)*}) => {
        compiler_error! {}
    };
}

#[cfg(test)]
const TEST: () = {
    const V: u8 = 2;
    json!(1u8);
    json!(const V: u8 = V;, const { V });
    _ = json!([]);
    json!([1u8]);
    json!([1u8, true]);
};

#[macro_export]
macro_rules! __private_json {
    (
        [] // outer const generics
        const $const_block:block
    ) => {
        const {
            enum HasConstJsonValue {}

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

            $crate::r#const::ConstJsonValue::<HasConstJsonValue>::new()
        }
    };
    (
        [$({$CONST:ident $ConstTy:ty $(= $const_value:expr)?})+] // outer const generics
        const $const_block:block
    ) => {
        const {
            enum HasConstJsonValue
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

            $crate::r#const::ConstJsonValue::<HasConstJsonValue::
                <$({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),+>
            >::new()
        }
    };
    (
        $outer_const_generics:tt // outer const generics
        ($runtime_expr:expr)
    ) => {
        $runtime_expr
    };
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
//
// ($runtime_value:expr)
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
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
        after_value {
            do(EOF)
            outer_const_generics $outer_const_generics:tt
        }
    ) => {
        // $crate::ser::texts::Value::new_no_json_whitespace()

        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state($crate::r#const::State::INIT)
            then(
                $crate::r#const::CompileTimeChunk::<HasConstCompileTimeChunk>::JSON_VALUE
            )
            tokens $only_compile_time
        }
    };
}

#[macro_export]
macro_rules! __private_json_concat_chunks {
    (
        prev_state $prev_state:tt
        then $then:tt
        chunks[
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state $prev_state
            then $then
            tokens $only_compile_time
        }
    };
}

#[macro_export]
macro_rules! __private_json_concat_only_compile_time_tokens {
    (
        prev_state($prev_state:expr)
        then($($then:stmt)*)
        tokens $tokens:tt
    ) => {{
        enum HasConstCompileTimeChunk {}

        impl HasConstCompileTimeChunk {
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

        impl $crate::r#const::HasConstCompileTimeChunk for HasConstCompileTimeChunk {
            const CHUNK: $crate::r#const::StatedChunkStr<'static> =
                Self::STATED_CHUNK_STRING.as_str();
        }

        $($then)*
    }};
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
        crate::r#const::ConstIntoJsonValueString(
            crate::r#const::ConstIntoJson($json_value).const_into_json(),
        )
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
        crate::r#const::ConstIntoJsonValueString(
            crate::r#const::ConstIntoJson($v).const_into_json(),
        )
        .const_into_json_value_string_len()
    };
}

#[cfg(todo)]
macro_rules! __private_parse_array_content {
    ({$const_or_runtime:ident $values:tt $before:expr} ($runtime_expr:expr) $(, $($rest:tt)*)?) => {
        $crate::__private_parse_array_content! {
            {runtime ($before, $runtime_expr)}
            $($($rest)*)?
        }
    };
    ({const   ($($values:tt)*) $before:expr} $lit:literal $(, $($rest:tt)*)?) => {
        $crate::__private_parse_array_content! {
            {const   ($($values)* ($lit)) $before}
            $($($rest)*)?
        }
    };
    ({runtime $values:tt $before:expr} $lit:literal $(, $($rest:tt)*)?) => {

    };
}

#[cfg(test)]
mod tests;
