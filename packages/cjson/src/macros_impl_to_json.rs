#[macro_export]
macro_rules! impl_to_json {
    (
        $(vis![$($vis:tt)*],)?
        $(impl_generics![$($impl_generics:tt)*],)?
        $(where_clause![$($where_clause:tt)*],)?
        $({$($used_const_generics:tt)*},)?
        |$_self:ident : $Type:ty|
        match $matched:tt $match_body:tt
    ) => {
        $crate::__private_impl_to_json_match! {
            ($($($vis)*)?)
            ($matched)
            $match_body
            {$($($used_const_generics)*)?}
            {
                impl_generics($($($impl_generics)*)?)
                where_clause($($($where_clause)*)?)
                self($_self)
                type($Type)
            }
        }
    };
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
        $json:tt
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            $json
            $used_const_generics
            {
                vis(pub) // TODO: parameterize
                branch_name_or_empty()
                expand_macro_bang($crate::__private_impl_to_json_expand!)
                expand_macro_rest($data)
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_parse_with {
    (
        ( $lit:literal $(,)? )
        {} // used_const_generics
        $expand:tt
    ) => {
        $crate::__private_impl_to_json_parse_with! {
            ( const { $lit } )
            {}
            $expand
        }
    };
    (
        ( const $const_block:block )
        $used_const_generics:tt
        {
            vis $vis:tt
            branch_name_or_empty $branch_name_or_empty:tt
            expand_macro_bang $expand_macro_bang:tt
            expand_macro_rest $expand_macro_rest:tt
        }
    ) => {
        $crate::__private_impl_to_json_const! {
            vis $vis
            branch_name_or_empty $branch_name_or_empty
            then_bang $expand_macro_bang
            then_rest $expand_macro_rest
            $used_const_generics
            const $const_block
        }
    };
    (
        ( $well_known_ident:ident $(,)? )
        {}
        {
            vis $vis:tt
            branch_name_or_empty $branch_name_or_empty:tt
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            mod()
            type($crate::__private::well_known_ident::$well_known_ident)
            value($crate::__private::well_known_ident::$well_known_ident)
            $($expand_macro_rest)*
        }
    };
    (
        ( ($runtime_expr:expr) as $RuntimeType:ty $(,)? )
        {}
        {
            vis $vis:tt
            branch_name_or_empty $branch_name_or_empty:tt
            expand_macro_bang($($expand_macro_bang:tt)+)
            expand_macro_rest($($expand_macro_rest:tt)*)
        }
    ) => {
        $($expand_macro_bang)+ {
            mod()
            type($RuntimeType)
            value($runtime_expr)
            $($expand_macro_rest)*
        }
    };
    (
        ( [$($array_content:tt)*] $(,)? )
        $used_const_generics:tt
        $expand:tt
    ) => {
        $crate::__private_json_after_array_start! {
            [
                prev[]
                current_compile_time[
                    left_bracket()
                ]
                after_value {
                    EOF_impl_to_json(
                        $used_const_generics
                        $expand
                    )
                }
            ]
            $($array_content)*
        }
    };
    (
        ( {$($object_content:tt)*} $(,)? )
        $used_const_generics:tt
        $expand:tt
    ) => {
        $crate::__private_json_after_object_start! {
            [
                prev[]
                current_compile_time[
                    left_brace()
                ]
                after_value {
                    EOF_impl_to_json(
                        $used_const_generics
                        $expand
                    )
                }
            ]
            $($object_content)*
        }
    };
    (
        ( $well_known_macro:ident $bang:tt $well_known_macro_body:tt $(,)? )
        $used_const_generics:tt
        $expand:tt
    ) => {
        $crate::__private_json_macro! {
            $well_known_macro $bang $well_known_macro_body
            [
                prev[]
                current_compile_time[]
                after_value {
                    EOF_impl_to_json(
                        $used_const_generics
                        $expand
                    )
                }
            ]
        }
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
            >::CHUNK.next_state().$runtime_kind())
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
        $(($($next_list:tt)*))?
    ) => {
        $crate::r#const::AssertJsonValueChunks<
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
macro_rules! __private_impl_to_json_after_value {
    (
        [
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
        (
            // used_const_generics
            {$( const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ;)*}
            {
                vis($($vis:tt)*)
                branch_name_or_empty()
                expand_macro_bang($($expand_macro_bang:tt)+)
                expand_macro_rest($($expand_macro_rest:tt)*)
            }
        )
    ) => {
        $($expand_macro_bang)+ {
            mod(
                $($vis)* enum HasConstCompileTimeChunk<$( const $CONST: $ConstTy ),*> {}

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
            $($expand_macro_rest)*
        }
    };
    (
        [
            prev_compile_runtime[]
            last_compile_time $only_compile_time:tt
        ]
        (
            // used_const_generics
            {$( const $CONST:ident: $ConstTy:ty $(= $const_value:expr)? ;)*}
            {
                vis $vis:tt
                branch_name_or_empty($($branch_name:ident)+)
                expand_macro_bang($($expand_macro_bang:tt)+)
                expand_macro_rest($($expand_macro_rest:tt)*)
            }
        )
    ) => {
        $($expand_macro_bang)+ {
            mod(
                pub enum HasConstCompileTimeChunk {}

                $crate::__private_impl_for_only_compile_time_tokens! {
                    prev_state($crate::r#const::State::INIT)
                    tokens $only_compile_time
                    impl_generics($( const $CONST: $ConstTy, )*)
                    for(
                        CjsonMacroGeneratedChunk::<
                            cjson_macro_generated_types:: $($branch_name::)+ HasConstCompileTimeChunk,
                            $( $CONST ),*
                        >
                    )
                }
            )
            type(
                $crate::r#const::ConstJsonValue::<
                    $crate::r#const::CompileTimeChunkIsJsonValue<
                        CjsonMacroGeneratedChunk::<
                            cjson_macro_generated_types:: $($branch_name::)+ HasConstCompileTimeChunk,
                            $( $CONST ),*
                        >
                    >
                >
            )
            value(
                $crate::r#const::CompileTimeChunk::<
                    CjsonMacroGeneratedChunk::<
                        cjson_macro_generated_types:: $($branch_name::)+ HasConstCompileTimeChunk,
                        $( $CONST ),*
                    >
                >::JSON_VALUE
            )
            $($expand_macro_rest)*
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
                    ($($branch_name)+)
                }
            )
            type(
                $crate::__private_impl_to_json_type! {
                    $compile_runtime
                    $last_compile_time
                    $used_const_generics
                    ($($branch_name)+)
                }
            )
            value(
                $crate::__private_impl_to_json_value! {
                    $compile_runtime
                    $last_compile_time
                    $used_const_generics
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
        $crate::__private_impl_to_json_expand_verbatim! {
            mod(
                $crate::__private_impl_to_json_expect_empty! $used_const_generics
            )
            type($crate::values::Never)
            value(match $crate::__private_impl_to_json_expand_matched!($matched) {})
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
            $used_const_generics
            {
                vis(pub)
                branch_name_or_empty($match_branch_name)
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
            mod $expanded_mod:tt
            impl { $($expanded_impl:tt)* }
            type {
                [$($expanded_type_prefix:tt)*]
                [$($expanded_type_postfix:tt)*]
            }
            match { $($expanded_match:tt)* }
            prev_branch {
                name($prev_branch_name:ident)
                type($prev_branch_type:ty)
                value($prev_branch_value:expr)
                either_paths $prev_either_paths:tt
            }
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
        // TODO: call to_json in either
        $crate::__private_impl_to_json_expand! {
            mod(
                #[allow(non_snake_case)]
                mod cjson_macro_generated_types $expanded_mod

                $($vis)* struct CjsonMacroGeneratedChunk<T>(T);

                $($expanded_impl)*
            )
            type(
                $($expanded_type_prefix)*
                $prev_branch_type
                $($expanded_type_postfix)*
            )
            value(
                match $crate::__private_impl_to_json_expand_matched!($matched) {
                    $($expanded_match)*
                    $crate::__private_impl_to_json_match_either_expr! {
                        $prev_either_paths
                        $prev_branch_value
                    }
                }
            )
            $data
        }
    };
}

#[macro_export]
macro_rules! __private_impl_to_json_variant_expand {
    (
        mod($(
            $define_item:item
            $impl_item:item
        )?)
        type $type:tt
        value $value:tt
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
                mod {
                    $(
                        pub mod $match_branch_name {
                            $define_item
                        }
                    )?
                }
                impl {
                    $($impl_item)?
                }
                type {
                    []
                    []
                }
                match {
                    $pat $($pat_if)* =>
                }
                prev_branch {
                    name ($match_branch_name)
                    type $type
                    value $value
                    either_paths()
                }
            }
            $rest_variants
            $used_const_generics
            $then
        }
    };
    (
        mod($(
            $define_item:item
            $impl_item:item
        )?)
        type $type:tt
        value $value:tt
        expanded {
            mod { $($expanded_mod:tt)* }
            impl { $($expanded_impl:tt)* }
            type {
                [$($expanded_type_prefix:tt)*]
                [$($expanded_type_postfix:tt)*]
            }
            match { $($expanded_match:tt)* }
            prev_branch {
                name($prev_branch_name:ident)
                type($prev_branch_type:ty)
                value($prev_branch_value:expr)
                either_paths $prev_either_paths:tt
            }
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
                mod {
                    $($expanded_mod)*
                    $(
                        pub mod $match_branch_name {
                            $define_item
                        }
                    )?
                }
                impl {
                    $($expanded_impl)*
                    $($impl_item)?
                }
                type {
                    [
                        $($expanded_type_prefix)*
                        $crate::values::Either<
                            $prev_branch_type,
                    ]
                    [
                        >
                        $($expanded_type_postfix)*
                    ]
                }
                match {
                    $($expanded_match)*
                        $crate::__private_impl_to_json_match_either_expr! {
                            $prev_either_paths
                            $crate::values::Either::A($prev_branch_value)
                        },
                    $pat $($pat_if)* =>
                }
                prev_branch {
                    name ($match_branch_name)
                    type $type
                    value $value
                    either_paths($prev_either_paths B)
                }
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
