#[macro_export]
macro_rules! json_write {
    (
        $consumer:expr,
        $($json:tt)*
    ) => {
        $crate::__private_json_write! {
            { base }  // maybe_try
            ($consumer) // consumer
            $($json)*
        }
    };
}

#[macro_export]
macro_rules! json_write_try {
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
macro_rules! json_write_async_try {
    (
        $consumer:expr,
        $($json:tt)*
    ) => {
        $crate::__private_json_write! {
            { async_try .await? }  // maybe_try
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
        $(,)?
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
        $(,)?
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
        $(,)?
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try,
            <_
                as $crate::ser::IntoJson>::json_provide_into::json_provide_into_try::json_provide_into_async_try(
                $crate::macro_helpers::well_known_ident::$well_known_ident,
                $consumer
            )
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        ($runtime_expr:expr) $(as $RuntimeType:ty)?
        $(,)?
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try,
            <$crate::__expand_or![[$($RuntimeType)?][_]]
                as $crate::ser::IntoJson>::json_provide_into::json_provide_into_try::json_provide_into_async_try(
                $runtime_expr,
                $consumer
            )
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        [$($array_content:tt)*]
        $(,)?
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
        $(,)?
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
        $(,)?
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
        $maybe_try:tt,
        <$Ty:ty as $Trait:path>
        ::$base:ident
        ::$try:ident
        ::$async_try:ident
        $args:tt
    ) => {
        $crate::__private_json_maybe_try! {
            $maybe_try
            (<$Ty as $Trait>::)
            [$base]
            [$try]
            [$async_try]
            $args
        }
    };
    (
        { base }
        $(($($pre:tt)*))?
        [ $($base:tt     )* ]
        [ $($try:tt      )* ]
        [ $($async_try:tt)* ]
        $t:tt
    ) => {
        $($($pre)*)?
        $($base)*
        $t
    };
    (
        { try_ ? }
        $(($($pre:tt)*))?
        [ $($base:tt     )* ]
        [ $($try:tt      )* ]
        [ $($async_try:tt)* ]
        $t:tt
    ) => {
        $($($pre)*)?
        $($try)*
        $t
        ?
    };
    (
        { async_try .await? }
        $(($($pre:tt)*))?
        [ $($base:tt     )* ]
        [ $($try:tt      )* ]
        [ $($async_try:tt)* ]
        $t:tt
    ) => {
        $($($pre)*)?
        $($async_try)*
        $t
        .await?
    };
}

#[macro_export]
macro_rules! __private_json_write_const {
    (
        $maybe_try:tt
        $consumer:tt
        $const_value:expr
    ) => {
        $crate::__private_json_write_const_value_str! {
            $maybe_try
            $consumer
            ($const_value)
            .const_into_json_value_string::<{
                $crate::r#const::ConstIntoJsonValueString(
                    $crate::r#const::ConstIntoJson($const_value).const_into_json(),
                )
                .const_into_json_value_string_len()
            }>()
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_generic_const {
    (
        $maybe_try:tt
        $consumer:tt
        ($const_value:expr, $capacity:expr $(,)?)
    ) => {
        $crate::__private_json_write_const_value_str! {
            $maybe_try
            $consumer
            ($const_value)
            .const_into_json_value_string_with_cap::<{$capacity}>()
        }
    };
    (
        $maybe_try:tt
        $consumer:tt
        ($const_value:expr                 $(,)?)
    ) => {
        $crate::__private_json_write_const_value_str! {
            $maybe_try
            $consumer
            ($const_value)
            .const_into_json_value_string_without_const_len()
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_const_value_str {
    (
        {$async_try_mod:ident $($async_try_postfix:tt)*}
        ($consumer:expr)
        ($const_value:expr)
        $($after_value:tt)+
    ) => {
        <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::consume_any_value(
            $consumer,
            const {
                const {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::r#const::ConstIntoJsonValueString(
                            $crate::r#const::ConstIntoJson($const_value).const_into_json(),
                        )
                        $($after_value)+
                    )
                }
                .const_as_json_value_str()
            },
            (),
        ) $($async_try_postfix)*
    };
}

#[macro_export]
macro_rules! __private_json_write_eof {
    (
        json_value_generic_const
        $json_value_generic_const_body:tt
        $maybe_try:tt
        $consumer:tt
    ) => {
        $crate::__private_json_write_generic_const! {
            $maybe_try
            $consumer
            $json_value_generic_const_body
        }
    };
    (
        EmptyArray {}
        {$maybe_try:ident $($async_try_postfix:tt)*}
        ($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON>::consume_empty_array(
            $consumer,
            (),
        ) $($async_try_postfix)*
    };
    (
        ArrayOfItems
        $v:tt // TODO: refactor
        { $async_try_mod:ident $($async_try_postfix:tt)* }
        ($consumer:expr)
    ) => {
        $crate::__private_json_write_chained_content! {
            ($crate::ser::json_kinds::Array)
            [$v]
            { $async_try_mod $($async_try_postfix)* }
            <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::start_to_consume_chained_arrays(
                $consumer, ()
            )
        }
    };
    (
        EmptyObject {}
        {$maybe_try:ident $($async_try_postfix:tt)*}
        ($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON>::consume_empty_object(
            $consumer,
            (),
        ) $($async_try_postfix)*
    };
    (
        ObjectOfKvs
        $v:tt // TODO: refactor
        { $async_try_mod:ident $($async_try_postfix:tt)* }
        ($consumer:expr)
    ) => {
        $crate::__private_json_write_chained_content! {
            ($crate::ser::json_kinds::Object)
            [$v]
            { $async_try_mod $($async_try_postfix)* }
            <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::start_to_consume_chained_objects(
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
                $try_consume_full:ident // TODO: remove
            }
        }
        {$maybe_try:ident $($async_try_postfix:tt)*}
        ($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON>::$consume_full(
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
        ) $($async_try_postfix)*
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
        ($ChainableJsonKind:ty)
        [{($last_items:expr) $(as $LastItemsType:ty)?}]
        { $maybe_try:ident $($async_try_postfix:tt)* }
        $w:expr
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_CHAINED<$ChainableJsonKind>>::end_with::<
            $crate::__expand_or![[$($LastItemsType)?][_]]
        >($w, $last_items) $($async_try_postfix)*
    };
    (
        ($ChainableJsonKind:ty)
        [
            {($items:expr) $(as $ItemsType:ty)?}
            $($rest:tt)+
        ]
        { $maybe_try:ident $($async_try_postfix:tt)* }
        $w:expr
    ) => {{
        let mut w = $w;
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_CHAINED<$ChainableJsonKind>>::extend::<
            $crate::__expand_or![[$($ItemsType)?][_]]
        >(&mut w, $items) $($async_try_postfix)* ;
        $crate::__private_json_write_chained_content! {
            @($ChainableJsonKind)
            [$($rest)+]
            { $maybe_try $($async_try_postfix)* }
            w
        }
    }};
    (
        @
        ($ChainableJsonKind:ty)
        [
            {($items:expr) $(as $ItemsType:ty)?}
            $($rest:tt)+
        ]
        { $maybe_try:ident $($async_try_postfix:tt)* }
        $w:ident
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_CHAINED<$ChainableJsonKind>>::extend::<
            $crate::__expand_or![[$($ItemsType)?][_]]
        >(&mut $w, $items) $($async_try_postfix)* ;
        $crate::__private_json_write_chained_content! {
            @($ChainableJsonKind)
            [$($rest)+]
            { $maybe_try $($async_try_postfix)* }
            $w
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks {
    (
        kind json_string
        $($rest:tt)+
    ) => {
        $crate::__private_json_write_json_string_chunks! {
            $($rest)+
        }
    };
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
macro_rules! __private_json_write_json_string_chunks {
    (
        prev_compile_runtime[
            prev $prev_compile_runtime:tt
            current {
                compile_time $compile_time:tt
                runtime[ json_string_fragment($v:expr) $(as $RuntimeType:ty)? ]
            }
        ]
        last_compile_time[ double_quote() ]
        maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
        consumer $consumer:tt
    ) => {
        $crate::__private::macro_used_names::$maybe_try::CONSUME_IN_JSON_STRING::end_with::<
            $crate::__expand_or![[$($RuntimeType)?][_]]
        >(
            $crate::__private_json_write_json_string_prev_of_last_runtime! {
                $prev_compile_runtime
                $compile_time
                maybe_try { $maybe_try $($async_try_postfix)* }
                consumer $consumer
            },
            $v
        ) $($async_try_postfix)*
    };
    (
        prev_compile_runtime $prev_compile_runtime:tt
        last_compile_time $last_compile_time:tt
        maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
        consumer $consumer:tt
    ) => {
        $crate::__private::macro_used_names::$maybe_try::CONSUME_IN_JSON_STRING::end_with_last_chunk(
            {
                $crate::__private_json_write_json_string_prev! {
                    $prev_compile_runtime
                    mut()
                    w
                    maybe_try { $maybe_try $($async_try_postfix)* }
                    consumer $consumer
                }

                w
            },
            $crate::__private_json_chunk_as_str! {
                $last_compile_time
                [$crate::r#const::LastChunkOfJsonStringAsArray]
                []
                ($crate::r#const::states::TOP_LEVEL_IN_STRING)
            }
        ) $($async_try_postfix)*
    };
}

#[macro_export]
macro_rules! __private_json_write_json_string_prev {
    (
        [
            prev[]
            current {
                compile_time[ double_quote() ]
                runtime [
                    json_string_fragment($v:expr)
                ]
            }
        ]
        mut($($mut_:tt)?)
        $w:ident
        maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
        consumer($consumer:expr)
    ) => {
        let $($mut_)? $w = <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON>::start_to_consume_chunks_of_json_string(
            $consumer,
            $v,
            ()
        ) $($async_try_postfix)*;
    };
    (
        [
            prev[]
            current {
                compile_time $compile_time:tt
                runtime $runtime:tt
            }
        ]
        mut $mut_:tt
        $w:ident
        maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
        consumer $consumer:tt
    ) => {
        let mut $w = $crate::__private_json_write_json_string_prev_of_last_runtime! {
            []
            $compile_time
            maybe_try { $maybe_try $($async_try_postfix)* }
            consumer $consumer
        };

        $crate::__private_json_write_json_string_runtime_fragment! {
            $runtime
            (&mut $w)
            { $maybe_try $($async_try_postfix)* }
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
        mut $mut_:tt
        $w:ident
        maybe_try $maybe_try:tt
        consumer $consumer:tt
    ) => {
        $crate::__private_json_write_json_string_prev! {
            $prev
            mut(mut)
            $w
            maybe_try $maybe_try
            consumer $consumer
        }

        $crate::__private_json_write_json_string_compile_time_fragment! {
            $compile_time
            (&mut $w)
            $maybe_try
        }

        $crate::__private_json_write_json_string_runtime_fragment! {
            $runtime
            (&mut $w)
            $maybe_try
        }
    };
}

#[macro_export]
macro_rules! __private_json_write_json_string_compile_time_fragment {
    (
        []
        $w:tt
        $maybe_try:tt
    ) => {};
    (
        $compile_time:tt
        ($w:expr)
        { $maybe_try:ident $($async_try_postfix:tt)* }
    ) => {
        $crate::__private::macro_used_names::$maybe_try::CONSUME_IN_JSON_STRING::consume_fragment_as_str(
            $w,
            $crate::__private_json_chunk_as_str! {
                $compile_time
                [$crate::r#const::JsonStringFragmentAsArray]
                []
                ($crate::r#const::states::TOP_LEVEL_IN_STRING)
            },
        ) $($async_try_postfix)*;
    };
}

#[macro_export]
macro_rules! __private_json_write_json_string_runtime_fragment {
    (
        [ json_string_fragment($v:expr) $(as $RuntimeType:ty)? ]
        ($w:expr)
        { $maybe_try:ident $($async_try_postfix:tt)* }
    ) => {
        $crate::__private::macro_used_names::$maybe_try::CONSUME_IN_JSON_STRING::consume_fragment::<
            $crate::__expand_or![[$($RuntimeType)?][_]]
        >(
            $w,
            $v,
        ) $($async_try_postfix)*;
    };
}

#[macro_export]
macro_rules! __private_json_write_json_string_prev_of_last_runtime {
    (
        [] // prev_compile_runtime
        $compile_time:tt
        maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
        consumer($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON>::start_to_consume_chunks_of_json_string_with_first_chunk(
            $consumer,
            $crate::__private_json_chunk_as_str! {
                $compile_time
                [$crate::r#const::FirstChunkOfJsonStringAsArray]
                []
                ($crate::r#const::State::INIT)
            },
            ()
        ) $($async_try_postfix)*
    };
    (
        $prev_compile_runtime:tt
        []
        maybe_try $maybe_try:tt
        consumer $consumer:tt
    ) => {{
        $crate::__private_json_write_json_string_prev! {
            $prev_compile_runtime
            mut()
            w
            maybe_try $maybe_try
            consumer $consumer
        }

        w
    }};
    (
        $prev_compile_runtime:tt
        $compile_time:tt
        maybe_try $maybe_try:tt
        consumer $consumer:tt
    ) => {{
        $crate::__private_json_write_json_string_prev! {
            $prev_compile_runtime
            mut(mut)
            w
            maybe_try $maybe_try
            consumer $consumer
        }

        $crate::__private_json_write_json_string_compile_time_fragment! {
            $compile_time
            (&mut w)
            $maybe_try
        }

        w
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
                runtime[$runtime_kind:ident $($runtime:tt)+]
            }
        ]
        $args:tt
        $then:tt
    ) => {
        $crate::__private_json_write_chunks_group_open_runtime! {
            $group_open
            $runtime_kind [{$($runtime)+}] // TODO: refactor
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
        { $maybe_try:ident $($async_try_postfix:tt)* }
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

        let $new_w = <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS<_>>::consume_intermediate_chunk(
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
        ) $($async_try_postfix)*;
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_first {
    (
        {
            kind $kind:tt
            maybe_try { $maybe_try:ident $($async_try_postfix:tt)* }
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

        let $new_w = <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS_FROM_INIT<_>>::consume_contentful_first_chunk(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!(
                $maybe_try
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
        ) $($async_try_postfix)*;
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_start_to_consume_non_empty {
    (
        $async_try_mod:ident
        NonEmptyArray
        ($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::start_to_consume_chunks_of_non_empty_array($consumer, ())
    };
    (
        $async_try_mod:ident
        NonEmptyObject
        ($consumer:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON>::start_to_consume_chunks_of_non_empty_object($consumer, ())
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_last {
    (
        NonEmptyArray
        { $maybe_try:ident $($async_try_postfix:tt)* }
        [right_bracket()]
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS<_>>::end_with_right_bracket(
            $w, ()
        ) $($async_try_postfix)*
    };
    (
        NonEmptyObject
        { $maybe_try:ident $($async_try_postfix:tt)* }
        [right_brace()]
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS<_>>::end_with_right_brace(
            $w, ()
        ) $($async_try_postfix)*
    };
    (
        $kind:ident
        { $maybe_try:ident $($async_try_postfix:tt)* }
        $chunk:tt
        $PrevState:ident
        $w:ident
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS<_>>::consume_contentful_last_chunk(
            $w,
            const {
                const {
                    <$crate::__private_json_concat_compile_time_tokens_type![
                        ($crate::__private::write::$kind::contentful_last_chunk)
                        $chunk
                        (::ChunkType<
                            {
                                $crate::__private_json_concat_compile_time_tokens_len! {
                                    $chunk
                                }
                            },
                            $PrevState,
                        >)
                    ]>::from_array_vec({
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
        ) $($async_try_postfix)*
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
        [$runtime_kind:ident $runtime_args:tt $(as $RuntimeType:ty)?]
        [$PrevState:ty]
    ) => {
        $crate::__private::state_then_runtime::$runtime_kind::<$PrevState>
    };
}

#[macro_export]
macro_rules! __private_json_write_runtime {
    (
        { $maybe_try:ident $($async_try_postfix:tt)* }
        [$runtime_kind:ident ($($runtime_args:tt)*) $(as $RuntimeType:ty)?]
        ($w:expr)
    ) => {
        <_ as $crate::__private::macro_used_names::$maybe_try::CONSUME_JSON_CHUNKS<_>>::$runtime_kind::<
            $crate::__expand_or![[$($RuntimeType)?][_]]
        >(
            $w,
            $($runtime_args)*
        ) $($async_try_postfix)*
    };
}

#[macro_export]
macro_rules! __private_json_write_chunks_group_open_runtime {
    (
        left_bracket json_value[{($value:expr) $(as $Type:ty)?}]
        {
            kind NonEmptyArray
            maybe_try {$async_try_mod:ident $($async_try_postfix:tt)*}
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::__private::macro_used_names::$async_try_mod::READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY>::left_bracket_value::<
            $crate::__expand_or![[$($Type)?][_]]
        >(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!($async_try_mod NonEmptyArray $consumer),
            $value
        ) $($async_try_postfix)*;
        use $crate::r#const::states::LeftBracketValue as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
    (
        left_bracket json_items_after_array_start_before_item[
            {($items:expr) $(as $ItemsType:ty)?}
            $({($rest_items:expr) $(as $RestItemsType:ty)?})*
        ]
        {
            kind NonEmptyArray
            maybe_try {$async_try_mod:ident $($async_try_postfix:tt)*}
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::__private::macro_used_names::$async_try_mod::READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY>::left_bracket_items_before_item::<
            $crate::__expand_or![[$($ItemsType)?][_]]
        >(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!($async_try_mod NonEmptyArray $consumer),
            $items
        ) $($async_try_postfix)*;

        $crate::__wrap_each! {
            [$({
                let w = <_ as $crate::__private::macro_used_names::CONSUME_JSON_CHUNKS<_>>::json_items_after_array_start_before_item::<
                    $crate::__expand_or![[$($RestItemsType)?][_]]
                >(
                    w,
                    $rest_items
                )
            })*]
            ( $($async_try_postfix)*; )
        }

        use $crate::r#const::states::LeftBracketItemsBeforeItem as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
    (
        left_brace json_kvs_after_object_start_before_kv[
            {($items:expr) $(as $ItemsType:ty)?}
            $({($rest_items:expr) $(as $RestItemsType:ty)?})*
        ]
        {
            kind NonEmptyObject
            maybe_try {$async_try_mod:ident $($async_try_postfix:tt)*}
            consumer $consumer:tt
        }
        $then:tt
    ) => {
        let w = <_ as $crate::__private::macro_used_names::$async_try_mod::READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT>::left_brace_kvs_before_kv::<
            $crate::__expand_or![[$($ItemsType)?][_]]
        >(
            $crate::__private_json_write_chunks_start_to_consume_non_empty!($async_try_mod NonEmptyObject $consumer),
            $items
        ) $($async_try_postfix)*;
        $crate::__wrap_each! {
            [$({
                let w = <_ as $crate::__private::macro_used_names::$async_try_mod::CONSUME_JSON_CHUNKS<_>>::json_kvs_after_object_start_before_kv::<
                    $crate::__expand_or![[$($RestItemsType)?][_]]
                >(
                    w,
                    $rest_items
                )
            })*]
            ( $($async_try_postfix)*; )
        }

        use $crate::r#const::states::LeftBraceKvsBeforeKv as __CJsonPrevState;
        $crate::__private_json_write_chunks_then! {
            __CJsonPrevState w
            $then
        }
    };
}

// TODO: refactor other code with this macro
#[macro_export]
macro_rules! __private_json_chunk_as_str {
    (
        $chunk:tt
        [$($Type:tt)*]
        [$($TypeParams:tt)*]
        ($initial_state:expr)
    ) => {
        const {
            const {
                $($Type)*::<
                    {
                        $crate::__private_json_concat_compile_time_tokens_len! {
                            $chunk
                        }
                    },
                    $($TypeParams)*
                >::from_array_vec({
                    let mut buf = $crate::r#const::StatedChunkBuf::new(
                        $initial_state
                    );
                    $crate::__private_json_concat_compile_time_tokens_buf! {
                        buf $chunk
                    }
                    buf
                })
            }
            .as_str()
        }
    };
}

#[macro_export]
macro_rules! __wrap_each {
    ([$($braced_t:tt)*]$append:tt) => {
        $(
            $crate::__wrap_one! {
                $braced_t
                $append
            }
        )*
    };
}

#[macro_export]
macro_rules! __wrap_one {
    ({$($t:tt)*}($($append:tt)*)) => {
        $($t)* $($append)*
    };
}
