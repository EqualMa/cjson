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
///         Self::A => json!("A"),
///     }
/// );
/// ```
#[macro_export]
macro_rules! impl_json {
    ($($t:tt)+) => {
        $crate::__private_impl_json_options! {
            {
                parse_mod($crate::macro_helpers::impl_json_options)
                on_parsed(($crate::__private_impl_json_options_resolved!))
            }
            {
                impl_generics[] // empty
                derive_from[] // empty
                where_clause[] // empty
                where_clause_to[] // empty
                where_clause_into[] // empty
                JsonKind[] // empty
                IS_CHAINABLE_AND_ALWAYS_EMPTY[] // empty
            }
            {$($t)+}
            {$($t)+}
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_options {
    // []
    (
        {
            parse_mod($($parse_mod:tt)+)
            on_parsed $on_parsed:tt
        }
        $options:tt
        { $_option_name:ident !        [$($_option_bracketed:tt)*] , $($_rest:tt)+ }
        {  $option_name:ident $bang:tt     $option_bracketed:tt    ,  $($rest:tt)+ }
    ) => {
        $($parse_mod)+::$option_name $bang {
            (($crate::__private_impl_json_options!)[{
                parse_mod($($parse_mod)+)
                on_parsed $on_parsed
            }]{
                {$($_rest)+}
                { $($rest)+}
            })
            $options
            $option_bracketed
        }
    };
    // {}
    (
        {
            parse_mod($($parse_mod:tt)+)
            on_parsed $on_parsed:tt
        }
        $options:tt
        { $_option_name:ident !        {$($_option:tt)*} , $($_rest:tt)+ }
        {  $option_name:ident $bang:tt  {$($option:tt)*} ,  $($rest:tt)+ }
    ) => {
        $($parse_mod)+::$option_name $bang {
            (($crate::__private_impl_json_options!)[{
                parse_mod($($parse_mod)+)
                on_parsed $on_parsed
            }]{
                {$($_rest)+}
                { $($rest)+}
            })
            [$($option)*]
            $option_bracketed
        }
    };
    // ()
    (
        {
            parse_mod($($parse_mod:tt)+)
            on_parsed $on_parsed:tt
        }
        $options:tt
        { $_option_name:ident !        ($($_option:tt)*) , $($_rest:tt)+ }
        {  $option_name:ident $bang:tt  ($($option:tt)*) ,  $($rest:tt)+ }
    ) => {
        $($parse_mod)+::$option_name $bang {
            (($crate::__private_impl_json_options!)[{
                parse_mod($($parse_mod)+)
                on_parsed $on_parsed
            }]{
                {$($_rest)+}
                { $($rest)+}
            })
            [$($option)*]
            $option_bracketed
        }
    };
    (
        {
            parse_mod $parse_mod:tt
            on_parsed(
                ($($on_parsed_macro_bang:tt)+)
                $([$($on_parsed_prepend:tt)*])?
                $({$($on_parsed_append:tt)*})?
            )
        }
        $options:tt
        $_rest:tt
         $rest:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            $options
            $rest
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_impl_generics {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics[] // this forbids multiple impl_generics![]
            derive_from $derive_from:tt
            where_clause $where_clause:tt
            where_clause_to $where_clause_to:tt
            where_clause_into $where_clause_into:tt
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $option_bracketed
                derive_from $derive_from
                where_clause $where_clause
                where_clause_to $where_clause_to
                where_clause_into $where_clause_into
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_derive_from {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from[] // this forbids multiple derive_from![]
            where_clause $where_clause:tt
            where_clause_to $where_clause_to:tt
            where_clause_into $where_clause_into:tt
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $option_bracketed
                where_clause $where_clause
                where_clause_to $where_clause_to
                where_clause_into $where_clause_into
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_where_clause {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from $derive_from:tt
            where_clause[] // this forbids multiple where_clause![]
            where_clause_to $where_clause_to:tt
            where_clause_into $where_clause_into:tt
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $derive_from
                where_clause $option_bracketed
                where_clause_to $where_clause_to
                where_clause_into $where_clause_into
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_where_clause_to {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from $derive_from:tt
            where_clause $where_clause:tt
            where_clause_to[] // this forbids multiple where_clause_to![]
            where_clause_into $where_clause_into:tt
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $derive_from
                where_clause $where_clause
                where_clause_to $option_bracketed
                where_clause_into $where_clause_into
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_where_clause_into {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from $derive_from:tt
            where_clause $where_clause:tt
            where_clause_to $where_clause_to:tt
            where_clause_into[] // this forbids multiple where_clause_to![]
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $derive_from
                where_clause $where_clause
                where_clause_to $where_clause_to
                where_clause_into $option_bracketed
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_JsonKind {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from $derive_from:tt
            where_clause $where_clause:tt
            where_clause_to $where_clause_to:tt
            where_clause_into $where_clause_into:tt
            JsonKind[] // this forbids multiple JsonKind![]
            IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY:tt
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $derive_from
                where_clause $where_clause
                where_clause_to $where_clause_to
                where_clause_into $where_clause_into
                JsonKind $option_bracketed
                IS_CHAINABLE_AND_ALWAYS_EMPTY $IS_CHAINABLE_AND_ALWAYS_EMPTY
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_option_IS_CHAINABLE_AND_ALWAYS_EMPTY {
    (
        (
            ($($on_parsed_macro_bang:tt)+)
            $([$($on_parsed_prepend:tt)*])?
            $({$($on_parsed_append:tt)*})?
        )
        {
            impl_generics $impl_generics:tt
            derive_from $derive_from:tt
            where_clause $where_clause:tt
            where_clause_to $where_clause_to:tt
            where_clause_into $where_clause_into:tt
            JsonKind $JsonKind:tt
            IS_CHAINABLE_AND_ALWAYS_EMPTY[] // this forbids multiple IS_CHAINABLE_AND_ALWAYS_EMPTY![]
        }
        $option_bracketed:tt
    ) => {
        $($on_parsed_macro_bang)+ {
            $($($on_parsed_prepend)*)?
            {
                impl_generics $impl_generics
                derive_from $derive_from
                where_clause $where_clause
                where_clause_to $where_clause_to
                where_clause_into $where_clause_into
                JsonKind $JsonKind
                IS_CHAINABLE_AND_ALWAYS_EMPTY $option_bracketed
            }
            $($($on_parsed_append)*)?
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_options_resolved {
    (
        $options:tt
        {
            |$_self:ident : $Type:ty|
            $($json_comma:tt)*
        }
    ) => {
        $crate::__private_impl_to_json_parse! {
            ($($json_comma)*)
            {
                expand_macro_bang($crate::__private_impl_json_on_parsed!)
                expand_macro_rest($options {
                    self($_self)
                    Self($Type)
                })
            }
        }
    };
}

#[macro_export]
macro_rules! __private_impl_json_auto_ref_to   { ($($t:tt)*) => { &$($t)* }; }
#[macro_export]
macro_rules! __private_impl_json_auto_ref_into { ($($t:tt)*) => {  $($t)* }; }

/// This macro exists because elided lifetimes are not allowed when defining a `type`.
///
/// <details><summary>
/// Expand this section to see the tests.
/// </summary>
///
/// ```compile_fail
/// trait HasAssocType {
///     type Type;
/// }
///
/// impl HasAssocType for &str {
///     type Type = ();
/// }
///
/// type AssocTypeOfStr = <&str as HasAssocType>::Type;
/// ```
///
/// ```
/// trait HasAssocType {
///     type Type;
/// }
///
/// impl HasAssocType for &str {
///     type Type = ();
/// }
///
/// type AssocTypeOfStr = <&'static str as HasAssocType>::Type;
/// ```
///
/// </details>
#[doc(hidden)]
#[macro_export]
macro_rules! __private_impl_json_auto_ref_to_type {
    ($t:ty) => {
        $crate::__private::refed::Refed::<$t>
    };
}

#[macro_export]
macro_rules! __private_impl_json_auto_deref_to   { ($($t:tt)*) => { *$($t)* }; }
#[macro_export]
macro_rules! __private_impl_json_auto_deref_into { ($($t:tt)*) => {  $($t)* }; }

#[macro_export]
macro_rules! __private_impl_json_on_parsed {
    (
        $parsed:tt
        {
            impl_generics[ $($impl_generics:tt)* ]
            derive_from[
                $($DeriveFrom:ty $(= $DeriveFromKind:ident)?),* $(,)?
            ]
            where_clause[$($where_clause:tt)*]
            where_clause_to[$($where_clause_to:tt)*]
            where_clause_into[$($where_clause_into:tt)*]
            JsonKind[$($JsonKind:ty)?]
            IS_CHAINABLE_AND_ALWAYS_EMPTY[$($IS_CHAINABLE_AND_ALWAYS_EMPTY:expr)?]
        }
        {
            self($_self:tt)
            Self($Type:ty)
        }
    ) => {
        #[automatically_derived] // TODO: is this needed?
        const _: () = {
            #[allow(unused_imports)]
            use $crate::macro_helpers::impl_json_auto_ref::into::auto_ref;
            impl< $($impl_generics)* > $crate::ser::IntoJson
                for $Type
                where
                    $($DeriveFrom: $crate::ser::IntoJson$(<JsonKind = $crate::ser::json_kinds::$DeriveFromKind>)?,)*
                    $($where_clause)*
                    $($where_clause_into)*
            {
                $crate::__private_impl_to_json_parsed_as_into_body! {
                    $parsed
                    {
                        $(JsonKind($JsonKind))?
                        $(IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY))?
                        receiver($_self)
                        self($_self)
                        prepend_fn_and_const(
                            #[allow(unused_imports)]
                            use $crate::macro_helpers::impl_json_auto_ref::into::auto_deref;
                        )
                    }
                }
            }
        };

        #[automatically_derived] // TODO: is this needed?
        const _: () = {
            #[allow(unused_imports)]
            use $crate::macro_helpers::impl_json_auto_ref::to_type::auto_ref;

            impl< $($impl_generics)* > $crate::ser::ToJson2
                for $Type
                where
                    $($DeriveFrom: $crate::ser::ToJson2$(<ToJsonKind = $crate::ser::json_kinds::$DeriveFromKind>)?,)*
                    $($where_clause)*
                    $($where_clause_to)*
            {
                $crate::__private_impl_to_json_parsed_as_to_body! {
                    $parsed
                    {
                        $(JsonKind($JsonKind))?
                        $(IS_CHAINABLE_AND_ALWAYS_EMPTY($IS_CHAINABLE_AND_ALWAYS_EMPTY))?
                        receiver(&$_self)
                        self($_self)
                        prepend_fn_and_const(
                            #[allow(unused_imports)]
                            use $crate::macro_helpers::impl_json_auto_ref::to::{auto_ref, auto_deref};
                        )
                    }
                }
            }
        };
    };
}
