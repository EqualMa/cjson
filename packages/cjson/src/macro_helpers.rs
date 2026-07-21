pub mod impl_json_options {
    pub use crate::{
        __private_impl_json_option_impl_generics as impl_generics,
        __private_impl_json_option_where_clause as where_clause,
        __private_impl_json_option_where_clause_into as where_clause_into,
        __private_impl_json_option_where_clause_to as where_clause_to,
    };
}

pub mod impl_json_auto_ref {
    pub mod to {
        pub use crate::__private_impl_json_auto_ref_to as auto_ref;
    }

    pub mod to_type {
        pub use crate::__private_impl_json_auto_ref_to_type as auto_ref;
    }

    pub mod into {
        pub use crate::__private_impl_json_auto_ref_into as auto_ref;
    }
}
