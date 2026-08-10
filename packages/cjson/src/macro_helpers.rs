pub mod impl_json_options {
    pub use crate::{
        __private_impl_json_option_IS_CHAINABLE_AND_ALWAYS_EMPTY as IS_CHAINABLE_AND_ALWAYS_EMPTY,
        __private_impl_json_option_JsonKind as JsonKind,
        __private_impl_json_option_derive_from as derive_from,
        __private_impl_json_option_impl_generics as impl_generics,
        __private_impl_json_option_where_clause as where_clause,
        __private_impl_json_option_where_clause_into as where_clause_into,
        __private_impl_json_option_where_clause_to as where_clause_to,
    };
}

pub mod impl_into_or_to_json_options {
    #[doc(no_inline)]
    pub use super::impl_json_options::{
        IS_CHAINABLE_AND_ALWAYS_EMPTY, JsonKind, derive_from, impl_generics, where_clause,
    };
}

pub mod impl_json_auto_ref {
    pub mod to {
        pub use crate::__private_impl_json_auto_deref_to as auto_deref;
        pub use crate::__private_impl_json_auto_ref_to as auto_ref;
    }

    pub mod to_type {
        #[doc(inline)]
        pub use crate::__private_impl_json_auto_ref_to_type as auto_ref;
    }

    pub mod into {
        pub use crate::__private_impl_json_auto_deref_into as auto_deref;
        pub use crate::__private_impl_json_auto_ref_into as auto_ref;
    }
}

// TODO: move to pub module
pub mod well_known_ident {
    pub use crate::values::Null as null;
}

pub mod well_known_attribute {
    pub use crate::__private_json_x as json_x;
}

pub mod json_x {
    use crate::ser::{
        ConsumeJsonText, Consumed, WriterAssertIsFromConsumeJsonText,
        json_kinds::{self, JsonKind},
    };

    // TODO: document
    pub fn any_value<C>(
        consumed: Consumed<
            impl JsonKind,
            ConsumeJsonText<impl WriterAssertIsFromConsumeJsonText<C, ()>>,
        >,
    ) -> Consumed<json_kinds::AnyValue, C> {
        consumed.assert_consume_json_text_and_upcast_to_any_value()
    }
}
