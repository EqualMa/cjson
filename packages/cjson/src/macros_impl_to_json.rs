///
/// ```
/// ```
///
/// `vis` default to `pub` if not specified.
///
/// ```compile_error
/// pub enum Private {
///     A,
/// }
///
/// ::cjson::impl_to_json!(
///     vis![],
///     impl_generics![],
///     where_clause![],
///     |self: Private| match self {
///         #[cjson(match_branch_name(A))]
///         Self::A => json!("A"),
///     }
/// );
/// ```
#[macro_export]
macro_rules! impl_to_json {
    ($($t:tt)+) => {
        $crate::__private_impl_to_json_options! {
            {
                vis[] // not specified
                impl_generics[] // empty
                where_clause[] // empty
            }
            {$($t)+}
            {$($t)+}
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_options {
    (
        $options:tt
        { $_option_name:ident !        $_option_bracketed:tt , $($_rest:tt)+ }
        {  $option_name:ident $bang:tt  $option_bracketed:tt ,  $($rest:tt)+ }
    ) => {
        $crate::__private::impl_to_json_options::$option_name $bang {
            $options
            $option_bracketed
            {$($_rest)+}
            { $($rest)+}
        }
    };
    (
        $options:tt
        $_rest:tt
         $rest:tt
    ) => {
        $crate::__private_impl_to_json_options_resolved! {
            $options
            $rest
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_option_vis {
    (
        {
            vis[ /* not specified */ ]
            impl_generics $impl_generics:tt
            where_clause $where_clause:tt
        }
        $option_bracketed:tt
        $_rest:tt
         $rest:tt
    ) => {
        $crate::__private_impl_to_json_options! {
            {
                vis[ $option_bracketed ]
                impl_generics $impl_generics
                where_clause $where_clause
            }
            $_rest
             $rest
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_option_impl_generics {
    (
        {
            vis $vis:tt
            impl_generics[]
            where_clause $where_clause:tt
        }
        $option_bracketed:tt
        $_rest:tt
         $rest:tt
    ) => {
        $crate::__private_impl_to_json_options! {
            {
                vis $vis
                impl_generics $option_bracketed
                where_clause $where_clause
            }
            $_rest
             $rest
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_option_where_clause {
    (
        {
            vis $vis:tt
            impl_generics $impl_generics:tt
            where_clause[]
        }
        $option_bracketed:tt
        $_rest:tt
         $rest:tt
    ) => {
        $crate::__private_impl_to_json_options! {
            {
                vis $vis
                impl_generics $impl_generics
                where_clause $option_bracketed
            }
            $_rest
             $rest
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_options_resolved {
    (
        {
            vis[] // not specified
            $($options:tt)+
        }
        $rest:tt
    ) => {
        $crate::__private_impl_to_json_options_resolved! {
            {
                // vis default to pub
                vis[ [pub] ]
                $($options)+
            }
            $rest
        }
    };
    (
        {
            vis[ [$($vis:tt)*] ]
            impl_generics[ $($impl_generics:tt)* ]
            where_clause[$($where_clause:tt)*]
        }{
        $({$($used_const_generics:tt)*},)?
        |$_self:ident : $Type:ty|
        match $matched:tt $match_body:tt
        }
    ) => {
        $crate::__private_impl_to_json_match! {
            ($($vis)*)
            ($matched)
            $match_body
            {$($($used_const_generics)*)?}
            ({
                trait ToJson2
                provide json_provide_to
                try_provide json_try_provide_to
                ref(&)
            }{
                impl_generics($($impl_generics)*)
                where_clause($($where_clause)*)
                self($_self)
                type($Type)
            })
        }
    };
    (
        {
            // TODO: not respected
            vis[ [$($vis:tt)*] ]
            impl_generics[ $($impl_generics:tt)* ]
            where_clause[$($where_clause:tt)*]
        }{
        $({$($used_const_generics:tt)*},)?
        |$_self:ident : $Type:ty| $($macro_body:tt)*
        }
    ) => {
        $crate::__private_impl_to_json_parse! {
            ( $($macro_body)* )
            {$($($used_const_generics)*)?}
            {
                impl_generics($($impl_generics)*)
                where_clause($($where_clause)*)
                self($_self)
                type($Type)
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_parse {
    (
        $json:tt
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            $json
            {
                expand_macro_bang($crate::__private_impl_to_json_parsed!)
                expand_macro_rest(
                    ({
                        trait ToJson2
                        provide json_provide_to
                        try_provide json_try_provide_to
                        ref(&)
                    } $data)
                )
            }

        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_parsed {
    (
        kind($Kind:ty)
        write_macro_bang($($write_macro_bang:tt)+)
        $(write_prev($($write_prev:tt)*))?
        $(write_rest($($write_rest:tt)*))?
        IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        ({
            trait $Trait:ident
            provide $provide:ident
            try_provide $try_provide:ident
            ref($($ref:tt)?)
        }{
            impl_generics($($impl_generics:tt)*)
            where_clause($($where_clause:tt)*)
            self($_self:ident)
            type($Type:ty)
        })
    ) => { const _: () = {
        impl< $($impl_generics)* > $crate::ser::$Trait
            for $Type
            $($where_clause)*
        {
            type ToJsonKind = $Kind;
            fn $provide<W: $crate::ser::ConsumeJson<ConsumeJsonKind: $crate::ser::json_kinds::JsonKind<Contains<Self::ToJsonKind> = ()>>>(
                $($ref)? $_self,
                w: W,
            ) -> $crate::ser::Consumed<Self::ToJsonKind, W> {
                $($write_macro_bang)+ {
                    $($($write_prev)*)?
                    { no_try }
                    (w)
                    $($($write_rest)*)?
                }
            }

            const IS_CHAINABLE_AND_ALWAYS_EMPTY: $crate::__private::bool = $IS_CHAINABLE_AND_ALWAYS_EMPTY;
        }
    }; };
}

#[macro_export]
macro_rules! __private_impl_to_json_parse_with {
    (
        ( ($runtime_expr:expr) $($as_type:tt)* )
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            kind($crate::__private_impl_to_json_runtime_kind![$($as_type)*])
            write_macro_bang($crate::__private_json_write!)
            write_rest( ($runtime_expr) ) // TODO: write as Type
            IS_CHAINABLE_AND_ALWAYS_EMPTY(
                $crate::__private_impl_to_json_runtime_const_val!($($as_type)*)
            )
            $($expand_macro_rest)*
        }
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
        $($expand_macro_bang)+ {
            kind($crate::ser::json_kinds::AnyValue)
            write_macro_bang($crate::__private_json_write!)
            write_rest( $($json)+ )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( false )
            $($expand_macro_rest)*
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_runtime_kind {
    (                         $(,)?) => {
        $crate::ser::json_kinds::AnyValue
    };
    (as & $lt:lifetime $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::ToJson2>::ToJsonKind
    };
    (as &              $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::ToJson2>::ToJsonKind
    };
    (as                $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::IntoJson>::JsonKind
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_runtime_const_val {
    ($(,)?) => {
        false
    };
    (as $Ty:ty $(,)?) => {
        <$Ty as $crate::ser::IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY
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
macro_rules! __private_impl_to_json_expand_verbatim {
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

        impl< $($impl_generics)* > $crate::ser::ToJson
            for $Type
            $($where_clause)*
        {
            type ToJson<'cjson_lt_to_json> = $ToJsonType
            where Self: 'cjson_lt_to_json;

            fn to_json(&$_self) -> Self::ToJson<'_> {
                $to_json_value
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
        $path:tt
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
        $path:tt
        $(($($next_list:tt)*))?
    ) => {
        $crate::__private_impl_to_json_impl_resolve! {
            $compile_runtime
            $last_compile_time
            // prev_state
            ($crate::r#const::State::INIT)
            // impl_generics
            ($( const $CONST: $ConstTy, )*)
            // used_const_names
            ($($CONST,)*)
            // next_paths
            ( $($($next_list)*)? )
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_impl_resolve {
    (
        []
        $compile_time:tt
        $prev_state:tt
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
        $impl_generics:tt
        $used_const_names:tt
        ($($next_list:tt)*)
    ) => {
        $crate::__private_impl_to_json_impl_resolve! {
            []
            $compile_time
            $prev_state
            $impl_generics
            $used_const_names
            ($($next_list)*)
        }

        $crate::__private_impl_to_json_impl_resolve! {
            [$($rest_compile_runtime)*]
            $last_compile_time
            (<
                $crate::__private_impl_to_json_for_type![
                    used_const_names $used_const_names
                    prefix_path(cjson_macro_generated_types:: $($next_list::)*)
                ] as $crate::r#const::HasConstCompileTimeChunk
            >::CHUNK.into_next_state().$runtime_kind())
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
        ($($path:tt)+)
        $(($($next_list:tt)*))?
    ) => {
        $($path)+ <
            $crate::__private_impl_to_json_type_resolve! {
                $compile_runtime
                $last_compile_time
                ($( $CONST, )*)
                next_list( $($($next_list)*)? )
            }
        >
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_type_resolve {
    (
        [$compile_runtime:tt]
        $last_compile_time:tt
        $used_const_names:tt
        next_list($($next_list:tt)*)
    ) => {
        $crate::r#const::ChunkConcat<
            $crate::__private_impl_to_json_type_compile_runtime![
                $compile_runtime
                $used_const_names
                ($($next_list)*)
            ],
            $crate::r#const::CompileTimeChunk<
                $crate::__private_impl_to_json_for_type![
                    used_const_names $used_const_names
                    prefix_path(cjson_macro_generated_types:: $($next_list::)* next::)
                ]
            >,
        >
    };
    (
        [$compile_runtime:tt $($rest_compile_runtime:tt)+]
        $last_compile_time:tt
        $used_const_names:tt
        next_list($($next_list:tt)*)
    ) => {
        $crate::r#const::ChunkConcat<
            $crate::__private_impl_to_json_type_compile_runtime![
                $compile_runtime
                $used_const_names
                ($($next_list)*)
            ],
            $crate::__private_impl_to_json_type_resolve![
                [$($rest_compile_runtime)+]
                $last_compile_time
                $used_const_names
                next_list($($next_list)* next)
            ]
        >
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_type_compile_runtime {
    (
        {
            compile_time $compile_time:tt
            runtime[
                $runtime_kind:ident $runtime_expr:tt
                $(as $runtime_type:ty)?
            ]
        }
        $used_const_names:tt
        ($($next_list:tt)*)
    ) => {
        $crate::__private::runtime_kinds::$runtime_kind<
            $crate::r#const::CompileTimeChunk<
                $crate::__private_impl_to_json_for_type![
                    used_const_names $used_const_names
                    prefix_path(cjson_macro_generated_types:: $($next_list::)*)
                ]
            >,
            $crate::__expand_or![[$($runtime_type)?][_]]
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
        ($($path:tt)+)
    ) => {
        $($path)+ ::new($crate::r#const::value::Value::new(
            $crate::__private_impl_to_json_value_resolve!(
                $compile_runtime
                $last_compile_time
            )
        ))
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
        vis($($vis:tt)*)
        branch_name_or_empty()
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
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
        vis($($vis:tt)*)
        branch_name_or_empty()
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
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
    (
        vis $vis:tt
        branch_name_or_empty($($branch_name:ident)+)
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
        {$(const $CONST:ident : $ConstTy:ty $(= $const_value:expr)?;)*} // used const generics
        $const_block:expr
    ) => { $($then_bang)+ {
        mod(
            pub enum HasConstJsonValue {}

            impl
                <$(const $CONST: $ConstTy),*>
                $crate::r#const::HasConstJsonValue for
                CjsonMacroGeneratedChunk<
                    cjson_macro_generated_types:: $($branch_name ::)+ HasConstJsonValue,
                    $($CONST),*
                >
            {
                const JSON_VALUE: $crate::ser::texts::Value<&'static $crate::__private::str> = {
                    $crate::r#const::ConstAsJsonValueStr(
                        $crate::__private_impl_to_json_expand_if_else! {
                            (
                                $({$CONST})*
                            ){
                                $crate::r#const::ConstIntoJsonValueString(
                                    $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                                ).const_into_json_value_string_without_const_len()
                                // TODO: rust limitation: generic parameters may not be used in const operations
                                // .const_into_json_value_string::<LEN>()
                            }{
                                $crate::r#const::ConstIntoJsonValueString(
                                    $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                                ).const_into_json_value_string::<{
                                    $crate::r#const::ConstIntoJsonValueString(
                                        $crate::r#const::ConstIntoJson($const_block).const_into_json(),
                                    )
                                    .const_into_json_value_string_len()
                                }>()
                            }
                        }
                    )
                    .const_as_json_value_str()
                };
            }
        )
        type(
            $crate::r#const::ConstJsonValue::<CjsonMacroGeneratedChunk::<
                cjson_macro_generated_types:: $($branch_name ::)+ HasConstJsonValue,
                $({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),*
            >>
        )
        value(
            $crate::r#const::ConstJsonValue::<CjsonMacroGeneratedChunk::<
                cjson_macro_generated_types:: $($branch_name ::)+ HasConstJsonValue,
                $({$crate::__private::__expand_or!([$($const_value)?][$CONST])}),*
            >>::DEFAULT
        )
        $($then_rest)*
    } };
}

#[macro_export]
macro_rules! __private_impl_to_json_eof {
    (
        $parsed1:tt $parsed2:tt
        {
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            kind( $crate::__private_impl_to_json_kind![$parsed1 $parsed2] )
            write_macro_bang($crate::__private_json_write_eof!)
            write_prev( $parsed1 $parsed2 )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( $crate::__private_impl_to_json_const_impl!($parsed1 $parsed2) )
            $($expand_macro_rest)*
        }
    };
}

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

#[macro_export]
macro_rules! __private_impl_to_json_const_impl {
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
        ($last_compile_time:tt $path:tt ($used_const_generics:tt $data:tt))
    ) => {
        $crate::__private_impl_to_json_after_value_mixed_expand! {
            {
                $parsed
                $last_compile_time
                $used_const_generics
                $path
            }
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_after_value_mixed_expand {
    (
        $t:tt
        {
            vis $vis:tt
            branch_name_or_empty()
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            mod(
                $crate::__private_impl_to_json_mod! $t
                $crate::__private_impl_to_json_impl! $t
            )
            type(
                $crate::__private_impl_to_json_type! $t
            )
            value(
                $crate::__private_impl_to_json_value! $t
            )
            $($expand_macro_rest)*
        }
    };
    (
        {
            $compile_runtime:tt
            $last_compile_time:tt
            $used_const_generics:tt
            $path:tt
        }
        {
            vis $vis:tt
            branch_name_or_empty($($branch_name:ident)+)
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            mod(
                $crate::__private_impl_to_json_mod_resolve! {
                    $compile_runtime
                    $last_compile_time
                }
                $crate::__private_impl_to_json_impl! {
                    $compile_runtime
                    $last_compile_time
                    $used_const_generics
                    __unused_path__
                    ($($branch_name)+)
                }
            )
            type(
                $crate::__private_impl_to_json_type! {
                    $compile_runtime
                    $last_compile_time
                    $used_const_generics
                    $path
                    ($($branch_name)+)
                }
            )
            value(
                $crate::__private_impl_to_json_value! {
                    $compile_runtime
                    $last_compile_time
                    $used_const_generics
                    $path
                }
            )
            $($expand_macro_rest)*
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

#[macro_export]
macro_rules! __private_impl_to_json_match {
    (
        $vis:tt
        ($matched:tt)
        // match only one
        { $(
            #[cjson(match_branch_name($match_branch_name:ident))]
            $pat:pat $(if $pat_if:expr)? => json! $json:tt
        ),+ $(,)? }
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_match_variants! {
            // expanded
            {}
            [$({
                match_branch_name { $match_branch_name }
                pat { $pat }
                pat_if { $(if $pat_if)? }
                json { $json }
            })+]
            $used_const_generics
            {
                vis $vis
                matched { $matched }
                data $data
            }
        }
    };
    (
        $vis:tt
        ($matched:tt)
        {} // match empty
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parsed! {
            kind($crate::ser::json_kinds::AnyValue) // TODO: kind of Never
            write_macro_bang($crate::__private_impl_to_json_write_matched!)
            write_rest( $matched {} )
            IS_CHAINABLE_AND_ALWAYS_EMPTY( false ) // TODO: ?
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_expect_empty {
    () => {};
}

#[macro_export]
macro_rules! __private_impl_to_json_expand_matched {
    [($matched:expr)] => [ $matched ];
    [ $matched:expr ] => [ $matched ];
}

#[macro_export]
macro_rules! __private_impl_to_json_match_variants {
    (
        $expanded:tt
        // branches
        [
            {
                match_branch_name { $match_branch_name:ident }
                pat $pat:tt
                pat_if $pat_if:tt
                json { $json:tt }
            }
            $($rest_var:tt)*
        ]
        $used_const_generics:tt
        $then:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            $json
            {
                expand_macro_bang($crate::__private_impl_to_json_variant_expand!)
                expand_macro_rest(
                    expanded $expanded
                    cur_variant {
                        match_branch_name { $match_branch_name }
                        pat $pat
                        pat_if $pat_if
                    }
                    rest_variants [$($rest_var)*]
                    used_const_generics $used_const_generics
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
        $used_const_generics:tt
        {
            vis($($vis:tt)*)
            matched { $matched:tt }
            data $data:tt
        }
    ) => {
        $crate::__private_impl_to_json_parsed! {
            kind $kind
            write_macro_bang($crate::__private_impl_to_json_write_matched!)
            write_rest(
                $matched
                $match
            )
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            $data
        }
    };
}

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

#[macro_export]
macro_rules! __private_impl_to_json_variant_expand {
    (
        kind($Kind:ty)
        write_macro_bang($($write_macro_bang:tt)+)
        $(write_prev($($write_prev:tt)*))?
        $(write_rest($($write_rest:tt)*))?
        IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        expanded {}
        cur_variant {
            match_branch_name { $match_branch_name:ident }
            pat { $pat:pat }
            pat_if { $($pat_if:tt)* }
        }
        rest_variants $rest_variants:tt
        used_const_generics $used_const_generics:tt
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
            $used_const_generics
            $then
        }
    };
    (
        kind($Kind:ty)
        write_macro_bang($($write_macro_bang:tt)+)
        $(write_prev($($write_prev:tt)*))?
        $(write_rest($($write_rest:tt)*))?
        IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        expanded {
            match { $($expanded_match:tt)* }
            kind($expanded_Kind:ty)
            IS_CHAINABLE_AND_ALWAYS_EMPTY($expanded_IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)
        }
        cur_variant {
            match_branch_name { $match_branch_name:ident }
            pat { $pat:pat }
            pat_if { $($pat_if:tt)* }
        }
        rest_variants $rest_variants:tt
        used_const_generics $used_const_generics:tt
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
            $used_const_generics
            $then
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_match_either_expr {
    { () $e:expr } => { $e };
    { ($prev:tt $Branch:ident) $e:expr } => {
        $crate::__private_impl_to_json_match_either_expr! {
            $prev
            $crate::values::Either::$Branch($e)
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_expand_if_else {
    { ()       $then:tt {$($else:tt)*} } => { $($else)* };
    { $pred:tt {$($then:tt)*} $else:tt } => { $($then)* };
}

#[macro_export]
macro_rules! __private_impl_to_json_match_impl_helper {
    (
        ($($branch_name:ident)+)
        ($ToJsonType:ty)
        {
            vis $vis:tt
            matched $matched:tt
            data {
                impl_generics($($impl_generics:tt)*)
                where_clause($($where_clause:tt)*)
                self($_self:ident)
                type($Type:ty)
            }
        }
    ) => {
        impl< $($impl_generics)* >
            cjson_macro_generated_types::$($branch_name)+::ImplToJsonHelper
            for $Type
            $($where_clause)*
        {
            type ImplToJsonHelper<'cjson_lt_to_json> = $ToJsonType
            where Self: 'cjson_lt_to_json;
        }
    };
}
