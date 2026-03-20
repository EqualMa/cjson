#[macro_export]
macro_rules! __private_proc_macro_to_json {
    ((
        struct {
            $(
                $field:ident : $field_type:ty
            ),* $(,)?
        }
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            {
                $(
                    const {
                        $crate::__private::proc_macro::stringify!($field)
                    } =
                        (&self.$field) as &'cjson_lt_to_json $field_type
                    ;
                )*
            }
        }
    };
    ((
        struct ;
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            null
        }
    };
    ((
        struct () ;
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            []
        }
    };
    ((
        struct (
            $field_type:ty $(,)?
        ) ;
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            (&self.0) as &'cjson_lt_to_json $field_type
        }
    };
    ((
        struct (
            $(
                $field_type:ty
            ),+ $(,)?
        ) ;
    ) $data:tt ) => {
        $crate::__private::proc_macro::unnamed_fields! {
            [$crate::__private_proc_macro_to_json_unnamed_fields!]
            [
                ($(
                    ($field_type)
                )+)
                $data
            ]
            $(
                ($field_type)
            )+
        }
    };
    ((
        enum {}
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            match (*self) {}
        }
    };
    ((
        enum {
            $Var:ident $(,)?
        }
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            match self {
                Self::$Var => json!(
                    const { $crate::__private::proc_macro::stringify!($Var) }
                ),
            }
        }
    };
    ((
        enum {
            $Var:ident $var_body:tt $(,)?
        }
    ) $data:tt ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            match self {
                $crate::__private_proc_macro_to_json_enum_var_pat!(
                    $Var
                    $var_body
                ) => json!(
                    const { $crate::__private::proc_macro::stringify!($Var) }
                ),
            }
        }
    };
}

#[macro_export]
macro_rules! __private_proc_macro_to_json_item_resolved {
    (
        // data
        {
            impl_generics $impl_generics:tt
            ty_generics [$($ty_generics:tt)*]
            where_clause $where_clause:tt
            item_name($ItemName:ident)
        }
        $_self:tt
        $($json:tt)*
    ) => {
        $crate::impl_to_json! {
            impl_generics! $impl_generics,
            where_clause! $where_clause,
            |$_self: $ItemName<$($ty_generics)*>|
                $($json)*
        }
    };
}

#[macro_export]
macro_rules! __private_proc_macro_to_json_unnamed_fields {
    (
        ($(
            ($field_type:ty)
        )+)
        $data:tt
        $($field:tt)+
    ) => {
        $crate::__private_proc_macro_to_json_item_resolved! {
            $data
            self
            [
                $(
                    (&self.$field) as &'cjson_lt_to_json $field_type,
                )+
            ]
        }
    };
}

#[macro_export]
macro_rules! __private_proc_macro_to_json_enum_var_pat {
    [ $Var:ident () ] => [ Self::$Var() ];
    [ $Var:ident {} ] => [ Self::$Var{} ];
    ( $Var:ident ($($field_type:ty),+ $(,)?) ) => {
        Self::$Var()
    };
    ( $Var:ident {$($field:ident : $field_type:ty),+ $(,)? } ) => {
        Self::$Var {
            $($field,)+
        }
    };
}
