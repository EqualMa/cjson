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
                    do(EOF json_array)
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
                    do(EOF $well_known_macro)
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
                    do(EOF json_object)
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
            vis()
            branch_name_or_empty()
            then_bang($crate::__private_json_const_then!)
            then_rest()
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
    (
        $state:tt
        ..[ $($array_inner:tt)* ]
        $(,)?
    ) => {
        $crate::__private_json_after_array_start! {
            $state
            $($array_inner)*
        }
    };
    (
        $state:tt
        ..[ $($array_inner:tt)* ]
        , $($rest:tt)+
    ) => {
        $crate::__private_json_array_detect_trailing_comma! {
            {$($array_inner)*}
            [
                $crate::__private_json_after_array_start! {
                    $state
                    $($array_inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_array_start! {
                    $state
                    $($array_inner)*
                    ,
                    $($rest)+
                }
            ]
        }
    };
    // runtime items
    (
        $state:tt
        ..($runtime_items:expr)
        $(as $runtime_type:ty)?
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_items! {
            $state
            [after_array_start ($runtime_items) $($runtime_type)?]
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
macro_rules! __private_json_after_runtime_items {
    // runtime_items_after_array_start + EOF
    (
        [
            prev $prev:tt
            current_compile_time $current_compile_time:tt
            after_value $after_value:tt
        ]
        [after_array_start ($runtime_items:expr) $($runtime_type:ty)?]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_items_between_brackets($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                last_compile_time[
                    right_bracket()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime_items_after_item + EOF
    (
        [
            prev $prev:tt
            current_compile_time $current_compile_time:tt
            after_value $after_value:tt
        ]
        [after_array_item ($runtime_items:expr) $($runtime_type:ty)?]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_items_after_item($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                last_compile_time[
                    right_bracket()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime_items + runtime_items
    (
        $state:tt
        [$items_kind:ident ($prev_runtime_items:expr) $($prev_runtime_type:ty)?]
        ..($runtime_items:expr)
        $(as $runtime_type:ty)?
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_items! {
            $state
            [
                $items_kind
                (
                    $crate::values::ChainArray($prev_runtime_items, $runtime_items)
                )
                $crate::values::ChainArray<
                    $crate::__expand_or![[$($prev_runtime_type)?][_]],
                    $crate::__expand_or![[$($runtime_type     )?][_]],
                >
            ]
            $($($rest)*)?
        }
    };
    // runtime_items + const_items
    (
        $state:tt
        $runtime:tt
        ..[ $($array_inner:tt)* ]
        $(,)?
    ) => {
        $crate::__private_json_after_runtime_items! {
            $state
            $runtime
            $($array_inner)*
        }
    };
    // runtime_items + const_items
    (
        $state:tt
        $runtime:tt
        ..[ $($array_inner:tt)* ]
        , $($rest:tt)+
    ) => {
        $crate::__private_json_array_detect_trailing_comma! {
            {$($array_inner)*}
            [
                $crate::__private_json_after_runtime_items! {
                    $state
                    $runtime
                    $($array_inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_runtime_items! {
                    $state
                    $runtime
                    $($array_inner)*
                    ,
                    $($rest)+
                }
            ]
        }
    };
    // runtime_items_after_array_start  + item
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        [after_array_start ($runtime_items:expr) $($runtime_type:ty)?]
        $($rest:tt)+
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang(
                    $crate::__private_json_after_array_comma!
                )
                before_value(
                )
            }
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_items_after_array_start_before_item($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($rest)+
        }
    };
    // runtime_items_after_item  + item
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        [after_array_item ($runtime_items:expr) $($runtime_type:ty)?]
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
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_items_after_item($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
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
        $(as $runtime_type:ty)?
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
                            $(as $runtime_type)?
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
    (
        $state:tt
        ..[ $($array_inner:tt)* ]
        $(,)?
    ) => {
        $crate::__private_json_after_array_comma! {
            $state
            $($array_inner)*
        }
    };
    (
        $state:tt
        ..[ $($array_inner:tt)* ]
        , $($rest:tt)+
    ) => {
        $crate::__private_json_array_detect_trailing_comma! {
            {$($array_inner)*}
            [
                $crate::__private_json_after_array_comma! {
                    $state
                    $($array_inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_array_comma! {
                    $state
                    $($array_inner)*
                    ,
                    $($rest)+
                }
            ]
        }
    };
    // runtime items
    (
        $state:tt
        ..($runtime_items:expr)
        $(as $runtime_type:ty)?
        $(, $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_items! {
            $state
            [after_array_item ($runtime_items) $($runtime_type:ty)?]
            $($($rest)*)?
        }
    };
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
macro_rules! __private_json_eof_normalize {
    // EmptyArray
    (
        kind(json_array)
        chunks[
            prev_compile_runtime[]
            last_compile_time[
                left_bracket()
                right_bracket()
            ]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            EmptyArray {}
            $($then_macro_rest)*
        }
    };
    // NonEmptyArray only_compile_time
    (
        kind(json_array)
        chunks[
            prev_compile_runtime[]
            last_compile_time[
                left_bracket()
                $($rest:tt)+
            ]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            only_compile_time {
                kind NonEmptyArray
                chunk [
                    left_bracket()
                    $($rest)+
                ]
                CONST_ASSOC(JSON_ARRAY_NON_EMPTY)
            }
            $($then_macro_rest)*
        }
    };
    // ArrayOfItems
    (
        kind(json_array)
        chunks[
            prev_compile_runtime[
                prev[]
                current {
                    compile_time[left_bracket()]
                    runtime[
                        json_items_between_brackets($runtime_expr:expr)
                        $(as $RuntimeType:ty)?
                    ]
                }
            ]
            last_compile_time [right_bracket()]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            ArrayOfItems {
                ($runtime_expr)
                $(as $RuntimeType)?
            }
            $($then_macro_rest)*
        }
    };
    // NonEmptyArray runtime_chunks
    (
        kind(json_array)
        chunks $chunks:tt
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            runtime_chunks {
                kind NonEmptyArray
                chunks $chunks
                path($crate::r#const::array::NonEmptyArray)
            }
            $($then_macro_rest)*
        }
    };
    // EmptyObject
    (
        kind(json_object)
        chunks[
            prev_compile_runtime[]
            last_compile_time[
                left_brace()
                right_brace()
            ]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            EmptyObject {}
            $($then_macro_rest)*
        }
    };
    // NonEmptyObject only_compile_time
    (
        kind(json_object)
        chunks[
            prev_compile_runtime[]
            last_compile_time[
                left_brace()
                $($rest:tt)+
            ]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            only_compile_time {
                kind NonEmptyObject
                chunk [
                    left_brace()
                    $($rest)+
                ]
                CONST_ASSOC(JSON_OBJECT_NON_EMPTY)
            }
            $($then_macro_rest)*
        }
    };
    // ObjectOfKvs
    (
        kind(json_object)
        chunks[
            prev_compile_runtime[
                prev[]
                current {
                    compile_time[left_brace()]
                    runtime[
                        json_kvs_between_braces($runtime_expr:expr)
                        $(as $RuntimeType:ty)?
                    ]
                }
            ]
            last_compile_time [right_brace()]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            ObjectOfKvs {
                ($runtime_expr)
                $(as $RuntimeType)?
            }
            $($then_macro_rest)*
        }
    };
    // NonEmptyObject runtime_chunks
    (
        kind(json_object)
        chunks $chunks:tt
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            runtime_chunks {
                kind NonEmptyObject
                chunks $chunks
                path($crate::r#const::object::NonEmptyObject)
            }
            $($then_macro_rest)*
        }
    };
    // json_string only_compile_time
    (
        kind(json_string)
        chunks[
            prev_compile_runtime[]
            last_compile_time[
                double_quote()
                $($rest:tt)+
            ]
        ]
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            only_compile_time {
                kind json_string
                chunk [
                    double_quote()
                    $($rest)+
                ]
                CONST_ASSOC(JSON_STRING)
            }
            $($then_macro_rest)*
        }
    };
    // json_string runtime_chunks
    (
        kind(json_string)
        chunks $chunks:tt
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $($then_macro_bang)+ {
            runtime_chunks {
                kind json_string
                chunks $chunks
                path($crate::r#const::string::JsonString)
            }
            $($then_macro_rest)*
        }
    };
}

#[macro_export]
macro_rules! __private_json_eof {
    (
        EmptyArray {}
        outer_const_generics $outer_const_generics:tt
    ) => {
        $crate::r#const::array::EmptyArray
    };
    (
        ArrayOfItems {
            ($runtime_expr:expr)
            $(as $RuntimeType:ty)?
        }
        outer_const_generics $outer_const_generics:tt
    ) => {
        $crate::r#const::array::ArrayOfItems $( ::<$RuntimeType> )? ($runtime_expr)
    };
    (
        EmptyObject {}
        outer_const_generics $outer_const_generics:tt
    ) => {
        $crate::r#const::object::EmptyObject
    };
    (
        ObjectOfKvs {
            ($runtime_expr:expr)
            $(as $RuntimeType:ty)?
        }
        outer_const_generics $outer_const_generics:tt
    ) => {
        $crate::r#const::object::ObjectOfKvs $( ::<$RuntimeType> )? ($runtime_expr)
    };
    (
        only_compile_time {
            kind $kind:tt
            chunk $only_compile_time:tt
            CONST_ASSOC($CONST_ASSOC:ident)
        }
        outer_const_generics $outer_const_generics:tt
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state($crate::r#const::State::INIT)
            then(
                $crate::r#const::CompileTimeChunk::<
                    $crate::__private_json_type_with_const_generics![
                        HasConstCompileTimeChunk
                        $outer_const_generics
                    ]
                >::$CONST_ASSOC
            )
            tokens $only_compile_time
            outer_const_generics $outer_const_generics
        }
    };
    (
        runtime_chunks {
            kind $kind:tt
            chunks[
                prev_compile_runtime $prev_compile_runtime:tt
                last_compile_time $last_compile_time:tt
            ]
            path($($path:tt)+)
        }
        outer_const_generics $outer_const_generics:tt
    ) => {
        $($path)+ ::new($crate::r#const::value::Value::new($crate::__private_json_concat_chunks! {
            prev_state($crate::r#const::State::INIT)
            outer_const_generics $outer_const_generics
            compile_runtime $prev_compile_runtime
            then_macro_bang(
                $crate::__private_json_after_value_concat_chunks_then!
            )
            then_macro_rest(
                last_compile_time $last_compile_time
            )
        }))
    };
}

#[macro_export]
macro_rules! __private_json_after_value {
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
        chunks $chunks:tt
        after_value {
            do(EOF $json_value_kind:tt)
            outer_const_generics $outer_const_generics:tt
        }
    ) => {
        $crate::__private_json_eof_normalize! {
            kind($json_value_kind)
            chunks $chunks
            then_macro_bang( $crate::__private_json_eof! )
            then_macro_rest(
                outer_const_generics $outer_const_generics
            )
        }
    };
    (
        chunks $chunks:tt
        after_value {
            EOF_impl_to_json {
                kind $json_value_kind:tt
            }
            $args:tt
        }
    ) => {
        $crate::__private_json_eof_normalize! {
            kind $json_value_kind
            chunks $chunks
            then_macro_bang( $crate::__private_impl_to_json_eof! )
            then_macro_rest(
                $args
            )
        }
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
macro_rules! __private_json_array_detect_trailing_comma {
    [{ $($e:expr,)* }[$($has_trailing_semi:tt)*][$($no_trailing_semi:tt)*]] => [$($has_trailing_semi)*];
    [{ $($e:expr),+ }[$($has_trailing_semi:tt)*][$($no_trailing_semi:tt)*]] => [$($no_trailing_semi)* ];
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
                $(as $runtime_type:ty)?
            ]
        }
        then_macro_bang($($then_macro_bang:tt)+)
        then_macro_rest($($then_macro_rest:tt)*)
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state $prev_state
            then(
                let cjson_prev_compile_runtime = $crate::__private::runtime_kinds::$runtime_kind
                $(::<_, $runtime_type>)?
                (
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
                $(as $runtime_type:ty)?
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
                    $crate::__private::runtime_kinds::$runtime_kind
                    $(::<_, $runtime_type>)?
                    (
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
    (
        $state:tt
        ..{ $($object_inner:tt)* }
        $(;)?
    ) => {
        $crate::__private_json_after_object_start! {
            $state
            $($object_inner)*
        }
    };
    (
        $state:tt
        ..{ $($object_inner:tt)* }
        ; $($rest:tt)+
    ) => {
        $crate::__private_json_object_detect_trailing_semi! {
            {$($object_inner)*}
            [
                $crate::__private_json_after_object_start! {
                    $state
                    $($object_inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_object_start! {
                    $state
                    $($object_inner)*
                    ;
                    $($rest)+
                }
            ]
        }
    };
    // runtime kvs
    (
        $state:tt
        ..($runtime_kvs:expr)
        $(as $runtime_type:ty)?
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_kvs! {
            $state
            [after_object_start ($runtime_kvs) $($runtime_type)?]
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
macro_rules! __private_json_after_runtime_kvs {
    // runtime_kvs_after_object_start + EOF
    (
        [
            prev $prev:tt
            current_compile_time $current_compile_time:tt
            after_value $after_value:tt
        ]
        [after_object_start ($runtime_items:expr) $($runtime_type:ty)?]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_kvs_between_braces($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                last_compile_time[
                    right_brace()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime_kvs_after_field_value + EOF
    (
        [
            prev $prev:tt
            current_compile_time $current_compile_time:tt
            after_value $after_value:tt
        ]
        [after_object_field_value ($runtime_items:expr) $($runtime_type:ty)?]
        // EOF
    ) => {
        $crate::__private_json_after_value! {
            chunks[
                prev_compile_runtime[
                    prev $prev
                    current {
                        compile_time $current_compile_time
                        runtime[
                            json_kvs_after_field_value($runtime_items)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                last_compile_time[
                    right_brace()
                ]
            ]
            after_value $after_value
        }
    };
    // runtime_kvs + runtime_kvs
    (
        $state:tt
        [$kind:ident ($prev_runtime:expr) $($prev_runtime_type:ty)?]
        ..($runtime:expr)
        $(as $runtime_type:ty)?
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_kvs! {
            $state
            [
                $kind
                (
                    $crate::values::ChainObject($prev_runtime, $runtime)
                )
                $crate::values::ChainObject<
                    $crate::__expand_or![[$($prev_runtime_type)?][_]],
                    $crate::__expand_or![[$($runtime_type     )?][_]],
                >
            ]
            $($($rest)*)?
        }
    };
    // runtime_kvs + const_kvs
    (
        $state:tt
        $runtime:tt
        ..{ $($inner:tt)* }
        $(;)?
    ) => {
        $crate::__private_json_after_runtime_kvs! {
            $state
            $runtime
            $($inner)*
        }
    };
    // runtime_kvs + const_kvs
    (
        $state:tt
        $runtime:tt
        ..[ $($inner:tt)* ]
        ; $($rest:tt)+
    ) => {
        $crate::__private_json_object_detect_trailing_semi! {
            {$($inner)*}
            [
                $crate::__private_json_after_runtime_items! {
                    $state
                    $runtime
                    $($inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_runtime_items! {
                    $state
                    $runtime
                    $($inner)*
                    ;
                    $($rest)+
                }
            ]
        }
    };
    // runtime_kvs_after_object_start  + field_name
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        [after_object_start ($runtime:expr) $($runtime_type:ty)?]
        $($rest:tt)+
    ) => {
        $crate::__private_json_object_field_name! {
            {
                before_field_name(
                )
            }
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_kvs_after_object_start_before_field_name($runtime)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($rest)+
        }
    };
    // runtime_kvs_after_field_value  + field_name
    (
        [
            prev $prev:tt
            current_compile_time $compile_time:tt
            after_value $after_value:tt
        ]
        [after_object_field_value ($runtime:expr) $($runtime_type:ty)?]
        $($rest:tt)+
    ) => {
        $crate::__private_json_object_field_name! {
            {
                before_field_name(
                    comma()
                )
            }
            [
                prev[
                    prev $prev
                    current {
                        compile_time $compile_time
                        runtime[
                            json_kvs_after_field_value($runtime)
                            $(as $runtime_type)?
                        ]
                    }
                ]
                current_compile_time[]
                after_value $after_value
            ]
            $($rest)+
        }
    };
}

#[macro_export]
macro_rules! __private_json_object_detect_trailing_semi {
    [{ $($e:expr;)* }[$($has_trailing_semi:tt)*][$($no_trailing_semi:tt)*]] => [$($has_trailing_semi)*];
    [{ $($e:expr);+ }[$($has_trailing_semi:tt)*][$($no_trailing_semi:tt)*]] => [$($no_trailing_semi)* ];
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
        $(as $runtime_type:ty)?
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
                            $(as $runtime_type)?
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
    // {$($object_content:tt)*}
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
    (
        $state:tt
        ($runtime_expr:expr) as $runtime_type:ty
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_value! {
            {
                after_comma_bang($crate::__private_json_after_object_field_value!)
                before_value(colon())
            }
            $state
            ($runtime_expr) as $runtime_type
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
    (
        $state:tt
        ..{ $($object_inner:tt)* }
        $(;)?
    ) => {
        $crate::__private_json_after_object_field_value! {
            $state
            $($object_inner)*
        }
    };
    (
        $state:tt
        ..{ $($object_inner:tt)* }
        ; $($rest:tt)+
    ) => {
        $crate::__private_json_object_detect_trailing_semi! {
            {$($object_inner)*}
            [
                $crate::__private_json_after_object_field_value! {
                    $state
                    $($object_inner)*
                    $($rest)+
                }
            ]
            [
                $crate::__private_json_after_object_field_value! {
                    $state
                    $($object_inner)*
                    ;
                    $($rest)+
                }
            ]
        }

    };
    // runtime kvs
    (
        $state:tt
        ..($runtime:expr)
        $(as $runtime_type:ty)?
        $(; $($rest:tt)*)?
    ) => {
        $crate::__private_json_after_runtime_items! {
            $state
            [after_object_field_value ($runtime) $($runtime_type:ty)?]
            $($($rest)*)?
        }
    };
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
        $crate::__private_json_in_string! {
            [
                prev $prev
                current_compile_time[
                    $($current_compile_time)*
                    double_quote()
                ]
                after_value $after_value
            ]
            $($body)*
        }
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
                        $(as $prev_runtime_type:ty)?
                    ]
                }
            ]
            current_compile_time[]
            after_value $after_value:tt
        ]
        // tokens
        ($runtime_expr:expr)
        $(as $runtime_type:ty)?
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
                                $crate::values::ChainString(
                                    $prev_fragment,
                                    $runtime_expr,
                                )
                                as $crate::values::ChainString<
                                    $crate::__expand_or![[$($prev_runtime_type)?][_]],
                                    $crate::__expand_or![[$($runtime_type)?     ][_]],
                                >
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
        $(as $runtime_type:ty)?
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
                            $(as $runtime_type)?
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
