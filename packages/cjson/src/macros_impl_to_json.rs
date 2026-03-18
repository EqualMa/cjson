#[macro_export]
macro_rules! impl_to_json {
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
        ( $lit:literal $(,)? )
        {} // used_const_generics
        $data:tt
    ) => {
        $crate::__private_impl_to_json_parse!(
            ( const { $lit } )
            {}
            $data
        )
    };
    (
        ( const $const_block:block )
        $used_const_generics:tt
        $data:tt
    ) => {
        $crate::__private_impl_to_json_const!(
            then_bang($crate::__private_impl_to_json_expand!)
            then_rest($data)
            vis(pub)
            $used_const_generics
            const $const_block
        )
    };
    (
        ( $well_known_ident:ident $(,)? )
        {}
        $data:tt
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod()
            type($crate::__private::well_known_ident::$well_known_ident)
            value($crate::__private::well_known_ident::$well_known_ident)
            $data
        }
    };
    (
        ( ($runtime_expr:expr) as $RuntimeType:ty $(,)? )
        {}
        $data:tt
    ) => {
        $crate::__private_impl_to_json_expand! {
            mod()
            type($RuntimeType)
            value($runtime_expr)
            $data
        }
    };
    // TODO:
    (
        ( [$($array_content:tt)*] $(,)? )
        $used_const_generics:tt
        $data:tt
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
        // $crate::__private_impl_to_json_mod! $macro_body ;

        impl< $($impl_generics)* > $crate::ser::ToJson
            for $Type
            $($where_clause)*
        {
            type ToJson<'cjson_lt_to_json> = <
                $ToJsonType as $crate::ser::ToJson
            >::ToJson<'cjson_lt_to_json>
            where Self: 'cjson_lt_to_json;
            // <$crate::__private_impl_to_json_type! $macro_body
            fn to_json(&$_self) -> Self::ToJson<'_> {
                // $crate::__private_impl_to_json_value! $macro_body
                $to_json_value.to_json()
            }
        }
    }; };
}

#[macro_export]
macro_rules! __private_impl_to_json_mod {
    () => {};
}

#[macro_export]
macro_rules! __private_impl_to_json_type {
    () => {};
}

#[macro_export]
macro_rules! __private_impl_to_json_value {
    () => {};
}

#[macro_export]
macro_rules! __private_impl_to_json_const {
    (
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
        vis($($vis:tt)*)
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
        then_bang($($then_bang:tt)+)
        then_rest($($then_rest:tt)*)
        vis($($vis:tt)*)
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
}
