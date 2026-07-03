#[macro_export]
macro_rules! json_write {
    (
        $consumer:expr,
        $($json:tt)*
    ) => {
        $crate::__private_json_write! {
            { no_try }  // maybe_try
            ($consumer) // consumer
            $($json)*
        }
    };
}

#[macro_export]
macro_rules! json_try_write {
    (
        $consumer:expr,
        $($json:tt)*
    ) => {
        $crate::__private_json_write! {
            { try_ ? }  // maybe_try
            ($consumer) // consumer
            $($json)*
        }
    };
}

#[macro_export]
macro_rules! __private_json_write {
    (
        $maybe_try:tt
        $consumer:tt
        $lit:literal
        // literal should precede ident,
        // so that false and true work.
    ) => {
        $crate::__private_json_write_const! {
            $maybe_try
            $consumer
            $lit
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        const $const_block:block
    ) => {
        $crate::__private_json_write_const! {
            $maybe_try
            $consumer
            $const_block
        }
    };
    (
        $maybe_try:tt
        ($consumer:expr)
        $well_known_ident:ident
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            ( <_ as $crate::ser::IntoJson>:: )
            [json_provide_into]
            [json_try_provide_into]
            (
                $crate::__private::well_known_ident::$well_known_ident,
                $consumer
            )
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        ($runtime_expr:expr)
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            ( <_ as $crate::ser::IntoJson>:: )
            [json_provide_into]
            [json_try_provide_into]
            (
                $runtime_expr,
                $consumer
            )
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        [$($array_content:tt)*]
    ) => {
        $crate::__private_json_after_array_start! {
            [
                prev[]
                current_compile_time[
                    left_bracket()
                ]
                after_value {
                    write_end(
                        json_array
                        $maybe_try
                        $consumer
                    )
                }
            ]
            $($array_content)*
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        {$($object_content:tt)*}
    ) => {
        $crate::__private_json_after_object_start! {
            [
                prev[]
                current_compile_time[
                    left_brace()
                ]
                after_value {
                    write_end(
                        json_object
                        $maybe_try
                        $consumer
                    )
                }
            ]
            $($object_content)*
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        $well_known_macro:ident $bang:tt $well_known_macro_body:tt
    ) => {
        $crate::__private_json_macro!(
            $well_known_macro $bang $well_known_macro_body
            [
                prev[]
                current_compile_time[]
                after_value {
                    write_end(
                        $well_known_macro
                        $maybe_try
                        $consumer
                    )
                }
            ]
        )
    };
}

#[macro_export]
macro_rules! __private_json_maybe_try {
    (
        $maybe_try:tt
        $no_try:ident
        $try_:ident
        $paren_args:tt
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            (<_ as $crate::ser::ConsumeJson>)
            [$no_try]
            [$try_]
            $paren_args
        }
    };
    (
        { no_try }
        $(($($pre:tt)*))?
        [ $($no_try:tt)* ]
        [ $($try:tt   )* ]
        $t:tt
    ) => {
        $($($pre)*)?
        $($no_try)*
        $t
    };
    (
        { try_ ? }
        $(($($pre:tt)*))?
        [ $($no_try:tt)* ]
        [ $($try:tt   )* ]
        $t:tt
    ) => {
        $($($pre)*)?
        $($try)*
        $t
        ?
    };
}

#[macro_export]
macro_rules! __private_json_write_const {
    (
        $maybe_try:tt
        ($consumer:expr)
        $const_value:expr
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            ( <_ as $crate::ser::ConsumeJson>:: )
            [consume_any_value]
            [try_consume_any_value]
            (
                $consumer,
                const {
                    const {
                        $crate::r#const::ConstAsJsonValueStr(
                            $crate::r#const::ConstIntoJsonValueString(
                                $crate::r#const::ConstIntoJson($const_value).const_into_json(),
                            )
                            .const_into_json_value_string::<{
                                $crate::r#const::ConstIntoJsonValueString(
                                    $crate::r#const::ConstIntoJson($const_value).const_into_json(),
                                )
                                .const_into_json_value_string_len()
                            }>(),
                        )
                    }
                    .const_as_json_value_str()
                },
                (),
            )
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_eof {
    (
        EmptyArray {}
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            consume_empty_array
            try_consume_empty_array
            ($consumer)
        }
    };
    (
        ArrayOfItems {
            ($runtime_expr:expr)
            $(as $RuntimeType:ty)?
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::r#const::array::ArrayOfItems $( ::<$RuntimeType> )? ($runtime_expr)
    };
    (
        EmptyObject {}
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::r#const::object::EmptyObject
    };
    (
        ObjectOfKvs {
            ($runtime_expr:expr)
            $(as $RuntimeType:ty)?
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::r#const::object::ObjectOfKvs $( ::<$RuntimeType> )? ($runtime_expr)
    };
    (
        only_compile_time {
            kind $kind:tt
            chunk $only_compile_time:tt
            CONST_ASSOC($CONST_ASSOC:ident)
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_concat_only_compile_time_tokens! {
            prev_state($crate::r#const::State::INIT)
            then(
                $crate::r#const::CompileTimeChunk::<
                    $crate::__private_json_type_with_const_generics![
                        HasConstCompileTimeChunk
                        []
                    ]
                >::$CONST_ASSOC
            )
            tokens $only_compile_time
            outer_const_generics []
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
        $maybe_try:tt
        ($consumer:expr)
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
