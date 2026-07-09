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
            (<_ as $crate::ser::ConsumeJson>::)
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
            consume_any_value
            try_consume_any_value
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
            ($consumer, ())
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
            write {
                $consume_full:ident
                $try_consume_full:ident
            }
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            $consume_full
            $try_consume_full
            (
                $consumer,
                const {
                    const {
                        $crate::__private::only_compile_time::$kind::AsArray::from_array_vec({
                                let mut buf = $crate::r#const::StatedChunkBuf::<{
                                    $crate::__private_json_concat_compile_time_tokens_len! {
                                        $only_compile_time
                                    }
                                }>::new($crate::r#const::State::INIT);

                                $crate::__private_json_concat_compile_time_tokens_buf! {
                                    buf
                                    $only_compile_time
                                }

                                buf
                        })
                    }.as_str()
                },
                ()
            )
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
        $consumer:tt
    ) => {
        $crate::__private_json_write_chunks! {
            kind $kind
            prev_compile_runtime $prev_compile_runtime
            last_compile_time $last_compile_time
            maybe_try $maybe_try
            consumer $consumer
        }
        // compile_error! {
        //     stringify! {
        //         {
        //     kind $kind:tt
        //     chunks[
        //         prev_compile_runtime $prev_compile_runtime:tt
        //         last_compile_time $last_compile_time:tt
        //     ]
        //     path($($path)+)
        // }
        //     }
        // }
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks {
    (
        kind $kind:tt
        prev_compile_runtime $prev_compile_runtime:tt
        last_compile_time $last_compile_time:tt
        maybe_try $maybe_try:tt
        consumer($consumer:expr)
    ) => {{
        let w = <_ as $crate::ser::ConsumeJson>::start_to_consume_chunks_of_non_empty_array(
            $consumer,
            (),
        );

        $crate::__private_json_write_chunks_prev! {
            $prev_compile_runtime
            initial_w w
            {
                [$crate::__private_json_write_chunks_last!](
                    [$last_compile_time]
                    // PrevState
                    []
                    // w
                    []
                )
            }
        }
    }};
}

#[macro_export]
macro_rules! __private_json_write_chunks_prev {
    (
        [
            prev[]
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
        initial_w $w:ident
        $then:tt
    ) => {
        pub enum __CJsonNextState {}
        $crate::__private_json_write_chunks_first! {
            $compile_time
            __CJsonNextState
            w
            $w
        }

        type __CJsonNextStateThen =
            $crate::__private_json_state_then_runtime![$runtime[__CJsonNextState]];

        let w = $crate::__private_json_write_runtime! {
            $runtime
            (w)
        };

        use __CJsonNextStateThen as __CJsonPrevState;
        {
            $crate::__private_json_write_chunks_then! {
                __CJsonPrevState w
                $then
            }
        }
    };
    (
        [
            prev $prev:tt
            current {
                compile_time[]
                runtime $runtime:tt
            }
        ]
        initial_w $w:ident
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_prev! {
            $prev
            initial_w $w
            {
                [$crate::__private_json_write_chunks_runtime!](
                    []
                    // PrevState
                    []
                    // w
                    [$runtime $then]
                )
            }
        }
    };
    (
        [
            prev $prev:tt
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
        initial_w $w:ident
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_prev! {
            $prev
            initial_w $w
            {
                [$crate::__private_json_write_chunks_intermediate!](
                    [$compile_time]
                    // PrevState
                    [
                        __CJsonNextState
                        w // new_w
                    ]
                    // w
                    []
                )

                pub enum __CJsonNextState {}

                type __CJsonNextStateThen =
                    $crate::__private_json_state_then_runtime![$runtime[__CJsonNextState]];

                let w = $crate::__private_json_write_runtime! {
                    $runtime
                    (w)
                };

                {
                    use __CJsonNextStateThen as __CJsonPrevState;
                    {
                        $crate::__private_json_write_chunks_then! {
                            __CJsonPrevState w
                            $then
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_then {
    (
        $PrevState:ident
        $w:ident
        {
            [$($macro_bang:tt)+](
                [$($before:tt)*]
                // PrevState
                [$($between:tt)*]
                // w
                [$($after:tt)*]
            )
            $($then:stmt)*
        }
    ) => {
        $($macro_bang)+ {
            $($before)*
            $PrevState
            $($between)*
            $w
            $($after)*
        }

        $($then)*
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_intermediate {
    (
        $chunk:tt
        $PrevState:ident
        $NextState:ident
        $new_w:ident
        $w:ident
    ) => {
        impl $crate::r#const::HasConstState for $NextState {
            const STATE: $crate::r#const::State =
                $crate::__private_json_concat_compile_time_tokens_state!(
                    (<$PrevState as $crate::r#const::HasConstState>::STATE)
                    $chunk
                );
        }

        let $new_w = <_ as $crate::ser::ConsumeJsonChunks<_>>::consume_intermediate_chunk(
            $w,
            const {
                const {
                    $crate::r#const::IntermediateChunkAsArray::<
                        {
                            $crate::__private_json_concat_compile_time_tokens_len! {
                                $chunk
                            }
                        },
                        $PrevState,
                        $NextState,
                    >::from_array_vec({
                        let mut buf = $crate::r#const::StatedChunkBuf::new(
                            <$PrevState as $crate::r#const::HasConstState>::STATE,
                        );
                        $crate::__private_json_concat_compile_time_tokens_buf! {
                            buf $chunk
                        }
                        buf
                    })
                }
                .as_str()
            },
        );
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_first {
    (
        $chunk:tt
        $NextState:ident
        $new_w:ident
        $w:ident
    ) => {
        impl $crate::r#const::HasConstState for $NextState {
            const STATE: $crate::r#const::State =
                $crate::__private_json_concat_compile_time_tokens_state! {
                    ($crate::r#const::State::INIT)
                    $chunk
                };
        }

        let $new_w = <_ as $crate::ser::ConsumeJsonChunks<_>>::consume_contentful_first_chunk(
            $w,
            const {
                const {
                    $crate::r#const::ContentfulFirstChunkOfArrayAsArray::<
                        {
                            $crate::__private_json_concat_compile_time_tokens_len! {
                                $chunk
                            }
                        },
                        $NextState,
                    >::from_array_vec({
                        let mut buf = $crate::r#const::StatedChunkBuf::new(
                            $crate::r#const::State::INIT
                        );
                        $crate::__private_json_concat_compile_time_tokens_buf! {
                            buf $chunk
                        }
                        buf
                    })
                }
                .as_str()
            },
        );
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_last {
    (
        $chunk:tt
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::consume_contentful_last_chunk(
            $w,
            const {
                const {
                    $crate::r#const::ContentfulLastChunkOfArrayAsArray::<
                        {
                            $crate::__private_json_concat_compile_time_tokens_len! {
                                $chunk
                            }
                        },
                        $PrevState,
                    >::from_array_vec({
                        let mut buf = $crate::r#const::StatedChunkBuf::new(
                            <$PrevState as $crate::r#const::HasConstState>::STATE,
                        );
                        $crate::__private_json_concat_compile_time_tokens_buf! {
                            buf $chunk
                        }
                        buf
                    })
                }
                .as_str()
            },
        )
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_runtime {
    (
        $PrevState:ident
        $w:ident
        $runtime:tt
        $then:tt
    ) => {
        type __CJsonNextState = $crate::__private_json_state_then_runtime![$runtime[$PrevState]];

        let w = $crate::__private_json_write_runtime! {
            $runtime
            ($w)
        };

        {
            use __CJsonNextState as __CJsonPrevState;
            {
                $crate::__private_json_write_chunks_then! {
                    __CJsonPrevState w
                    $then
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __private_json_state_then_runtime {
    (
        [$runtime_kind:ident $runtime_args:tt]
        [$PrevState:ty]
    ) => {
        $crate::__private::state_then_runtime::$runtime_kind::<$PrevState>
    };
}

#[macro_export]
macro_rules! __private_json_write_runtime {
    (
        [$runtime_kind:ident ($($runtime_args:tt)*)]
        ($w:expr)
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::$runtime_kind(
            $w,
            $($runtime_args)*
        )
    };
}
