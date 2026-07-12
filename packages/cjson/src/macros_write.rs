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
            $items:tt
            $(as $RuntimeType:ty)? // TODO: remove
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_write_chained_content! {
            ConsumeChainedArrays
            $items
            $maybe_try
            <_ as $crate::ser::ConsumeJson>::start_to_consume_chained_arrays(
                $consumer, ()
            )
        }
    };
    (
        EmptyObject {}
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            consume_empty_object
            try_consume_empty_object
            ($consumer, ())
        }
    };
    (
        ObjectOfKvs {
            $kvs:tt
            $(as $RuntimeType:ty)? // TODO: remove
        }
        $maybe_try:tt
        ($consumer:expr)
    ) => {
        $crate::__private_json_write_chained_content! {
            ConsumeChainedObjects
            $kvs
            $maybe_try
            <_ as $crate::ser::ConsumeJson>::start_to_consume_chained_objects(
                $consumer, ()
            )
        }
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
    };
}

#[macro_export]
macro_rules! __private_json_write_chained_content {
    (
        $(@)?
        $TraitConsumeChainedContent:ident
        ($last_items:expr)
        { $maybe_try:ident $($question:tt)? }
        $w:expr
    ) => {
        <_ as $crate::ser::$TraitConsumeChainedContent>::end_with($w, $last_items) $($question)?
    };
    (
        $TraitConsumeChainedContent:ident
        ($items:expr $(, $rest_items:expr)+)
        { $maybe_try:ident $($question:tt)? }
        $w:expr
    ) => {{
        let mut w = $w;
        <_ as $crate::ser::$TraitConsumeChainedContent>::extend(&mut w, $items) $($question)? ;
        $crate::__private_json_write_chained_content! {
            @($($rest_items),+)
            { $maybe_try $($question)? }
            w
        }
    }};
    (
        @
        $TraitConsumeChainedContent:ident
        ($items:expr $(, $rest_items:expr)+)
        { $maybe_try:ident $($question:tt)? }
        $w:ident
    ) => {
        <_ as $crate::ser::$TraitConsumeChainedContent>::extend(&mut $w, $items) $($question)? ;
        $crate::__private_json_write_chained_content! {
            @($($rest_items),+)
            { $maybe_try $($question)? }
            $w
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks {
    (
        kind $kind:tt
        prev_compile_runtime $prev_compile_runtime:tt
        last_compile_time $last_compile_time:tt
        maybe_try $maybe_try:tt
        consumer $consumer:tt
    ) => {{
        $crate::__private_json_write_chunks_prev! {
            $prev_compile_runtime
            {
                kind $kind
                maybe_try $maybe_try
                consumer $consumer
            }
            {
                [$crate::__private_json_write_chunks_last!](
                    [
                        $kind
                        $maybe_try
                        $last_compile_time
                    ]
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
                compile_time[
                    $group_open:ident()
                ]
                runtime[$runtime_kind:ident $runtime_args:tt]
            }
        ]
        $args:tt
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_group_open_runtime! {
            $group_open
            $runtime_kind $runtime_args
            $args
            $then
        }
    };
    (
        [
            prev[]
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
        {
            kind $kind:tt
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        pub enum __CJsonNextState {}
        $crate::__private_json_write_chunks_first! {
            {
                kind $kind
                maybe_try $maybe_try
                consumer $consumer
            }
            $compile_time
            __CJsonNextState
            w
        }

        type __CJsonNextStateThen =
            $crate::__private_json_state_then_runtime![$runtime[__CJsonNextState]];

        let w = $crate::__private_json_write_runtime! {
            $maybe_try
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
        {
            kind $kind:tt
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_prev! {
            $prev
            {
                kind $kind
                maybe_try $maybe_try
                consumer $consumer
            }
            {
                [$crate::__private_json_write_chunks_runtime!](
                    [$maybe_try]
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
        {
            kind $kind:tt
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_prev! {
            $prev
            {
                kind $kind
                maybe_try $maybe_try
                consumer $consumer
            }
            {
                [$crate::__private_json_write_chunks_intermediate!](
                    [$maybe_try $compile_time]
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
                    $maybe_try
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
        { $maybe_try:ident $($question:tt)? }
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
        ) $($question)?;
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_first {
    (
        {
            kind $kind:tt
            maybe_try { $maybe_try:ident $($question:tt)? }
            consumer $consume:tt
        }
        $chunk:tt
        $NextState:ident
        $new_w:ident
    ) => {
        impl $crate::r#const::HasConstState for $NextState {
            const STATE: $crate::r#const::State =
                $crate::__private_json_concat_compile_time_tokens_state! {
                    ($crate::r#const::State::INIT)
                    $chunk
                };
        }

        let $new_w = <_ as $crate::ser::ConsumeJsonChunks<_>>::consume_contentful_first_chunk(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(
                $kind
                $consume
            ),
            const {
                const {
                    $crate::__private::write::$kind::ContentfulFirstChunkAsArray::<
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
        ) $($question)?;
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_start_to_consume_non_empty {
    (
        NonEmptyArray
        ($consumer:expr)
    ) => {
        <_ as $crate::ser::ConsumeJson>::start_to_consume_chunks_of_non_empty_array($consumer, ())
    };
    (
        NonEmptyObject
        ($consumer:expr)
    ) => {
        <_ as $crate::ser::ConsumeJson>::start_to_consume_chunks_of_non_empty_object($consumer, ())
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_last {
    (
        NonEmptyArray
        { $maybe_try:ident $($question:tt)? }
        [right_bracket()]
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::end_with_right_bracket(
            $w, ()
        ) $($question)?
    };
    (
        NonEmptyObject
        { $maybe_try:ident $($question:tt)? }
        [right_brace()]
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::end_with_right_brace(
            $w, ()
        ) $($question)?
    };
    (
        $kind:ident
        { $maybe_try:ident $($question:tt)? }
        $chunk:tt
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::consume_contentful_last_chunk(
            $w,
            const {
                const {
                    $crate::__private::write::$kind::ContentfulLastChunkAsArray::<
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
        ) $($question)?
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_runtime {
    (
        $maybe_try:tt
        $PrevState:ident
        $w:ident
        $runtime:tt
        $then:tt
    ) => {
        type __CJsonNextState = $crate::__private_json_state_then_runtime![$runtime[$PrevState]];

        let w = $crate::__private_json_write_runtime! {
            $maybe_try
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
        { $maybe_try:ident $($question:tt)? }
        [$runtime_kind:ident ($($runtime_args:tt)*)]
        ($w:expr)
    ) => {
        <_ as $crate::ser::ConsumeJsonChunks<_>>::$runtime_kind(
            $w,
            $($runtime_args)*
        ) $($question)?
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_group_open_runtime {
    (
        left_bracket json_value($item:expr)
        {
            kind NonEmptyArray
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::ser::ReadyToConsumeJsonChunksOfNonEmptyArray>::left_bracket_value(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(NonEmptyArray $consumer),
            $item
        );
        use $crate::r#const::states::LeftBracketValue as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
    (
        left_bracket json_items_after_array_start_before_item($items:expr $(, $rest_items:expr)*)
        {
            kind NonEmptyArray
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::ser::ReadyToConsumeJsonChunksOfNonEmptyArray>::left_bracket_items_before_item(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(NonEmptyArray $consumer),
            $items
        );
        $(
            let w = <_ as $crate::ser::ConsumeJsonChunks<_>>::json_items_after_array_start_before_item(
                w,
                $rest_items
            );
        )*
        use $crate::r#const::states::LeftBracketItemsBeforeItem as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
    (
        left_brace json_value($item:expr)
        {
            kind NonEmptyArray
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::ser::ReadyToConsumeJsonChunksOfNonEmptyArray>::left_bracket_value(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(NonEmptyArray $consumer),
            $item
        );
        use $crate::r#const::states::LeftBracketValue as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
    (
        left_brace json_kvs_after_object_start_before_field_name($items:expr $(, $rest_items:expr)*)
        {
            kind NonEmptyObject
            maybe_try $maybe_try:tt
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::ser::ReadyToConsumeJsonChunksOfNonEmptyObject>::left_brace_kvs_before_kv(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(NonEmptyObject $consumer),
            $items
        );
        $(
            let w = <_ as $crate::ser::ConsumeJsonChunks<_>>::json_kvs_after_object_start_before_kv(
                w,
                $rest_items
            );
        )*
        use $crate::r#const::states::LeftBraceKvsBeforeKv as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
}
