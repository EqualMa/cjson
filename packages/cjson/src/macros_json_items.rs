/// Implement items in [`ToJson`](crate::ser::ToJson).
///
/// See [`json_fns!`] if you just want to implement `fn` items.
#[macro_export]
macro_rules! json_items {
    ($($closure:tt)*) => {
        $crate::__private_json_fns_parse_closure! {
            {$($closure)*}
            {$($closure)*}
            []
        }
    };
}

/// Implement fns in [`IntoJson`](crate::ser::IntoJson) or [`ToJson`](crate::ser::ToJson).
///
/// See [`json_items!`] if you want to implement all items.
#[macro_export]
macro_rules! json_fns {
    ($($closure:tt)*) => {
        $crate::__private_json_fns_parse_closure! {
            {$($closure)*}
            {$($closure)*}
            [just_fns()]
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __private_json_fns_parse_closure {
    (
        { | self         $(,)?| $($_json_comma:tt)* }
        { | $_self:ident $(,)?| $( $json_comma:tt)* }
        [$($prepend_args:tt)*]
    ) => {
        $crate::__private_impl_to_json_parse! {
            ($($json_comma)*)
            {
                expand_macro_bang($crate::__private_impl_to_json_parsed_as_into_body!)
                expand_macro_rest({
                    $($prepend_args)*
                    receiver($_self)
                    self($_self)
                })
            }
        }
    };
    (
        { | &        self         $(,)?| $($_json_comma:tt)* }
        { | $_ref:tt $_self:ident $(,)?| $( $json_comma:tt)* }
        [$($prepend_args:tt)*]
    ) => {
        $crate::__private_impl_to_json_parse! {
            ($($json_comma)*)
            {
                expand_macro_bang($crate::__private_impl_to_json_parsed_as_to_body!)
                expand_macro_rest({
                    $($prepend_args)*
                    receiver($_ref $_self)
                    self($_self)
                })
            }
        }
    };
    (
        { | mut         self         $(,)?| $($_json_comma:tt)* }
        { | $_mut:ident $_self:ident $(,)?| $( $json_comma:tt)* }
        [$($prepend_args:tt)*]
    ) => {
        $crate::__private_impl_to_json_parse! {
            ($($json_comma)*)
            {
                expand_macro_bang($crate::__private_impl_to_json_parsed_as_into_body!)
                expand_macro_rest({
                    $($prepend_args)*
                    receiver($_mut $_self)
                    self($_self)
                })
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_parse {
    (
        (
            match
            $matched:tt
            $match_body:tt
            $(,)?
        )
        $on_parsed:tt
    ) => {
        $crate::__private_impl_to_json_match! {
            ($matched)
            $match_body
            $on_parsed
        }
    };
    (
        $non_match_json_comma:tt
        $on_parsed:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            $non_match_json_comma
            $on_parsed
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_parsed_as_to_body {
    (
        $parsed:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parsed_as_body! {
            $parsed
            (
                {
                    ToJsonKind ToJsonKind
                    provide json_provide_to
                    provide_try json_provide_to_try
                    provide_async_try json_provide_to_async_try
                    ref(&)
                }
                $data
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_parsed_as_into_body {
    (
        $parsed:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parsed_as_body! {
            $parsed
            (
                {
                    ToJsonKind JsonKind
                    provide json_provide_into
                    provide_try json_provide_into_try
                    provide_async_try json_provide_into_async_try
                    ref()
                }
                $data
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_parsed_as_body {
    ({
        kind($Kind:ty)
        write_macro_bang($($write_macro_bang:tt)+)
        $(write_prev($($write_prev:tt)*))?
        $(write_rest($($write_rest:tt)*))?
        IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
    }
    ({
        ToJsonKind $ToJsonKind:ident
        provide $provide:ident
        provide_try $provide_try:ident
        provide_async_try $provide_async_try:ident
        ref($($ref:tt)?) // TODO_LATER: remove
    }{
        $(just_fns $just_fns:tt)?
        $(JsonKind($($CustomJsonKind:ty)?))?
        $(IS_CHAINABLE_AND_ALWAYS_EMPTY($($CUSTOM_IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)?))?
        receiver($($receiver:tt)+)
        self($_self:ident) // TODO_LATER: remove
        $(prepend_fn_and_const($($prepend_fn_and_const:tt)*))?
    })) => {
        $crate::__private_impl_to_json_expand_if_else! {($($just_fns)?){}{
            type $ToJsonKind = $crate::__expand_or![
                [$($($CustomJsonKind)?)?]
                [$Kind]
            ];
        }}
        fn $provide<__CJsonWriter: $crate::ser::ConsumeJson<
            ConsumeJsonKind: $crate::ser::json_kinds::JsonKind<
                Contains<Self::$ToJsonKind> = ()
            >
        >>(
            $($receiver)+,
            w: __CJsonWriter,
        ) -> $crate::ser::Consumed<Self::$ToJsonKind, __CJsonWriter> {
            $($($prepend_fn_and_const)*)?
            $($write_macro_bang)+ {
                $($($write_prev)*)?
                { base }
                (w)
                $($($write_rest)*)?
            }
        }

        fn $provide_try<__CJsonWriter: $crate::ser::TryConsumeJson<
            ConsumeJsonKind: $crate::ser::json_kinds::JsonKind<
                Contains<Self::$ToJsonKind> = ()
            >
        >>(
            $($receiver)+,
            w: __CJsonWriter,
        ) -> $crate::__private::Result<
            $crate::ser::Consumed<Self::$ToJsonKind, __CJsonWriter>,
            <<__CJsonWriter as $crate::ser::TryConsumeJson>::Writer as $crate::ser::traits::TryConsumeTextChunk>::Err
        > {
            $($($prepend_fn_and_const)*)?

            let out = $($write_macro_bang)+ {
                $($($write_prev)*)?
                { try_ ? }
                (w)
                $($($write_rest)*)?
            };

            #[allow(unreachable_code)] // this happens for empty types for example.
            $crate::__private::Result::Ok(out)
        }

        async fn $provide_async_try<__CJsonWriter: $crate::ser::AsyncTryConsumeJson<
            ConsumeJsonKind: $crate::ser::json_kinds::JsonKind<
                Contains<Self::$ToJsonKind> = ()
            >
        >>(
            $($receiver)+,
            w: __CJsonWriter,
        ) -> $crate::__private::Result<
            $crate::ser::Consumed<Self::$ToJsonKind, __CJsonWriter>,
            <<__CJsonWriter as $crate::ser::AsyncTryConsumeJson>::Writer as $crate::ser::traits::AsyncTryConsumeTextChunk>::Err
        > {
            $($($prepend_fn_and_const)*)?

            let out = $($write_macro_bang)+ {
                $($($write_prev)*)?
                { async_try .await? }
                (w)
                $($($write_rest)*)?
            };

            #[allow(unreachable_code)] // this happens for empty types for example.
            $crate::__private::Result::Ok(out)
        }

        $crate::__private_impl_to_json_expand_if_else! {($($just_fns)?){}{
            const IS_CHAINABLE_AND_ALWAYS_EMPTY: $crate::__private::bool = {
                $($($prepend_fn_and_const)*)?

                $crate::__expand_or!(
                    [$($($CUSTOM_IS_CHAINABLE_AND_ALWAYS_EMPTY)?)?]
                    [$IS_CHAINABLE_AND_ALWAYS_EMPTY]
                )
            };
        }}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_parse_with {
    (
        ( #$($attribute_expr_comma:tt)+ )
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {{
            kind($crate::ser::json_kinds::AnyValue)
            write_macro_bang($crate::__private_json_write!)
            write_rest( #$($attribute_expr_comma)+ )
            IS_CHAINABLE_AND_ALWAYS_EMPTY(false)
        } $($expand_macro_rest)* }
    };
    (
        ( ($runtime_expr:expr) $($as_type:tt)* )
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {{
            kind($crate::__private_impl_to_json_runtime_kind![$($as_type)*])
            write_macro_bang($crate::__private_json_write!)
            write_rest( ($runtime_expr) $($as_type)* )
            IS_CHAINABLE_AND_ALWAYS_EMPTY(
                $crate::__private_impl_to_json_runtime_const_val!($($as_type)*)
            )
        } $($expand_macro_rest)* }
    };
    (
        ( [$($array_content:tt)*] $(,)? )
        $expand:tt
    ) => {
        $crate::__private_json_after_array_start! {
            [
                prev[]
                current_compile_time[
                    left_bracket()
                ]
                after_value {
                    EOF_of_kind(json_array)
                    then_macro_bang($crate::__private_impl_to_json_eof!)
                    then_macro_rest(
                        $expand
                    )
                }
            ]
            $($array_content)*
        }
    };
    (
        ( {$($object_content:tt)*} $(,)? )
        $expand:tt
    ) => {
        $crate::__private_json_after_object_start! {
            [
                prev[]
                current_compile_time[
                    left_brace()
                ]
                after_value {
                    EOF_of_kind(json_object)
                    then_macro_bang($crate::__private_impl_to_json_eof!)
                    then_macro_rest(
                        $expand
                    )
                }
            ]
            $($object_content)*
        }
    };
    (
        ( $well_known_macro:ident $bang:tt $well_known_macro_body:tt $(,)? )
        $expand:tt
    ) => {
        $crate::__private_json_macro! {
            $well_known_macro $bang $well_known_macro_body
            [
                prev[]
                current_compile_time[]
                after_value {
                    EOF_of_kind($well_known_macro)
                    then_macro_bang($crate::__private_impl_to_json_eof!)
                    then_macro_rest(
                        $expand
                    )
                }
            ]
        }
    };
    (
        ($($json:tt)+)
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {{
            kind($crate::ser::json_kinds::AnyValue)
            write_macro_bang($crate::__private_json_write!)
            write_rest( $($json)+ )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( false )
        } $($expand_macro_rest)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_runtime_kind {
    (                         $(,)?) => {
        $crate::ser::json_kinds::AnyValue
    };
    (as & $lt:lifetime $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::ToJson>::ToJsonKind
    };
    (as &              $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::ToJson>::ToJsonKind
    };
    (as                $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::IntoJson>::JsonKind
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_runtime_const_val {
    ($(,)?) => {
        false
    };
    (as $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_eof {
    (
        $parsed1:tt $parsed2:tt
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {{
            kind( $crate::__private_impl_to_json_kind![$parsed1 $parsed2] )
            write_macro_bang($crate::__private_json_write_eof!)
            write_prev( $parsed1 $parsed2 )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( $crate::__private_impl_to_json_const_impl!($parsed1 $parsed2) )
        } $($expand_macro_rest)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_kind {
    (
        $_kind:ident {
            kind $kind:ident
            $($_rest:tt)*
        }
    ) => {
        $crate::__private::impl_to_json_kinds::chunks::$kind
    };
    (
        $kind:ident $parsed2:tt
    ) => {
        $crate::__private::impl_to_json_kinds::full::$kind
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_const_impl {
    (json_value_generic_const $json_value_generic_const_body:tt) => { false };
    (EmptyArray {}) => { true };
    (EmptyObject {}) => { true };
    (ArrayOfItems { ($runtime_expr:expr) $(as $RuntimeType:ty)? }) => {
        $crate::__expand_or![[$(<$RuntimeType as $crate::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY)?][false]]
    };
    (ObjectOfKvs { ($runtime_expr:expr) $(as $RuntimeType:ty)? }) => {
        $crate::__expand_or![[$(<$RuntimeType as $crate::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY)?][false]]
    };
    (
        only_compile_time $body:tt
    ) => {
        // TODO: compile-time empty string might return `true` here
        false
    };
    (
        runtime_chunks $body:tt
    ) => {
        // TODO: runtime string might return all_compile_chunks_are_empty && $RuntimeType::IS_CHAINABLE_AND_ALWAYS_EMPTY here
        false
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_match {
    (
        ($matched:tt)
        { $(
            $pat:pat $(if $pat_if:expr)? => json! $json:tt
        ),+ $(,)? }
        $on_match_parsed:tt
    ) => {
        $crate::__private_impl_to_json_match_variants! {
            // expanded
            {}
            [$({
                pat { $pat }
                pat_if { $(if $pat_if)? }
                json { $json }
            })+]
            {
                matched { $matched }
                on_match_parsed $on_match_parsed
            }
        }
    };
    (
        ($matched:tt)
        {} // match empty
        {
            expand_macro_bang($($on_match_parsed_macro_bang:tt)+)
            expand_macro_rest($($on_match_parsed_append:tt)*)
        }
    ) => {
        $($on_match_parsed_macro_bang)+ {{
            kind($crate::ser::json_kinds::AnyValue) // TODO: kind of Never
            write_macro_bang($crate::__private_impl_to_json_write_matched!)
            write_rest( $matched {} )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( false ) // TODO: ?
        } $($on_match_parsed_append)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_expand_matched {
    [($matched:expr)] => [ $matched ];
    [ $matched:expr ] => [ $matched ];
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_match_variants {
    (
        $expanded:tt
        // branches
        [
            {
                pat $pat:tt
                pat_if $pat_if:tt
                json { $json:tt }
            }
            $($rest_var:tt)*
        ]
        $then:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            $json
            {
                expand_macro_bang($crate::__private_impl_to_json_variant_expand!)
                expand_macro_rest(
                    expanded $expanded
                    cur_variant {
                        pat $pat
                        pat_if $pat_if
                    }
                    rest_variants [$($rest_var)*]
                    then $then
                )
            }
        }
    };
    (
        {
            match $match:tt
            kind $kind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        // branches
        []
        {
            matched { $matched:tt }
            on_match_parsed {
                expand_macro_bang($($on_match_parsed_macro_bang:tt)+)
                expand_macro_rest($($on_match_parsed_append:tt)*)
            }
        }
    ) => {
        $($on_match_parsed_macro_bang)+ {{
            kind $kind
            write_macro_bang($crate::__private_impl_to_json_write_matched!)
            write_rest(
                $matched
                $match
            )
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
        } $($on_match_parsed_append)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_write_matched {
    (
        $maybe_try:tt
        $consumer:tt
        $matched:tt
        {$(
            [$($pat:tt)+] {
                write_macro_bang($($write_macro_bang:tt)+)
                write_prev($($write_prev:tt)*)
                write_rest($($write_rest:tt)*)
            }
        )*}
    ) => {
        match $crate::__private_impl_to_json_expand_matched!($matched) {$(
            $($pat)+ => $($write_macro_bang)+ {
                $($write_prev)*
                $maybe_try
                $consumer
                $($write_rest)*
            },
        )*}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_variant_expand {
    (
        {
            kind($Kind:ty)
            write_macro_bang($($write_macro_bang:tt)+)
            $(write_prev($($write_prev:tt)*))?
            $(write_rest($($write_rest:tt)*))?
            IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        }
        expanded {}
        cur_variant {
            pat { $pat:pat }
            pat_if { $($pat_if:tt)* }
        }
        rest_variants $rest_variants:tt
        then $then:tt
    ) => {
        $crate::__private_impl_to_json_match_variants! {
            // expanded
            {
                match {
                    [$pat $($pat_if)*] {
                        write_macro_bang($($write_macro_bang)+)
                        write_prev($($($write_prev)*)?)
                        write_rest($($($write_rest)*)?)
                    }
                }
                kind($Kind)
                IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY)
            }
            $rest_variants
            $then
        }
    };
    (
        {
            kind($Kind:ty)
            write_macro_bang($($write_macro_bang:tt)+)
            $(write_prev($($write_prev:tt)*))?
            $(write_rest($($write_rest:tt)*))?
            IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        }
        expanded {
            match { $($expanded_match:tt)* }
            kind($expanded_Kind:ty)
            IS_CHAINABLE_AND_ALWAYS_EMPTY($expanded_IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        }
        cur_variant {
            pat { $pat:pat }
            pat_if { $($pat_if:tt)* }
        }
        rest_variants $rest_variants:tt
        then $then:tt
    ) => {
        $crate::__private_impl_to_json_match_variants! {
            // expanded
            {
                match {
                    $($expanded_match)*
                    [$pat $($pat_if)*] {
                        write_macro_bang($($write_macro_bang)+)
                        write_prev($($($write_prev)*)?)
                        write_rest($($($write_rest)*)?)
                    }
                }
                kind(<$expanded_Kind as $crate::ser::json_kinds::JsonKind>::Union<$Kind>)
                IS_CHAINABLE_AND_ALWAYS_EMPTY($expanded_IS_CHAINABLE_AND_ALWAYS_EMPTY && $IS_CHAINABLE_AND_ALWAYS_EMPTY)
            }
            $rest_variants
            $then
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_to_json_expand_if_else {
    { ()       $then:tt {$($else:tt)*} } => { $($else)* };
    { $pred:tt {$($then:tt)*} $else:tt } => { $($then)* };
}
