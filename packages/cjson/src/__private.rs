pub use str;

/// used by [`crate::impl_to_json!`]
pub use bool;

/// used by [`crate::json_to!`]
pub use Default;

#[cfg(feature = "alloc")]
/// used by [`crate::json_to!`]
pub use ::alloc::string::String;

pub use crate::__expand_or;

// TODO: move to pub module
pub mod well_known_ident {
    pub use crate::values::Null as null;
}

pub mod well_known_macro {
    pub use crate::__private_json_well_known_macro_json_string as json_string;
    pub use crate::__private_json_well_known_macro_json_value_generic_const as json_value_generic_const;
}

pub mod state_then_runtime {
    pub use crate::r#const::states::{
        ThenItemsAfterArrayStartBeforeItem as json_items_after_array_start_before_item,
        ThenItemsAfterItem as json_items_after_item,
        ThenKvsAfterFieldValue as json_kvs_after_field_value,
        ThenKvsAfterObjectStartBeforeKv as json_kvs_after_object_start_before_kv,
        ThenStringFragment as json_string_fragment, ThenValue as json_value,
    };
}

pub mod runtime_kinds {
    pub use crate::r#const::ChunkConcatJsonStringFragment as json_string_fragment;
    pub use crate::r#const::ChunkConcatJsonValue as json_value;

    pub use crate::r#const::ChunkConcatJsonItemsAfterArrayStartBeforeItem as json_items_after_array_start_before_item;
    pub use crate::r#const::ChunkConcatJsonItemsAfterItem as json_items_after_item;
    pub use crate::r#const::ChunkConcatJsonItemsBetweenBrackets as json_items_between_brackets;
}

pub mod only_compile_time_kinds {
    pub use crate::r#const::{
        //
        array::NonEmptyArray as JSON_ARRAY_NON_EMPTY,
        object::NonEmptyObject as JSON_OBJECT_NON_EMPTY,
        string::JsonString as JSON_STRING,
        value::Value as JSON_VALUE,
    };
}

pub trait ImplToJsonHelper {
    type ImplToJsonHelper<'a>: crate::ser::ToJson
    where
        Self: 'a;
}

pub mod impl_to_json_options {
    pub use crate::{
        __private_impl_to_json_option_impl_generics as impl_generics,
        __private_impl_to_json_option_vis as vis,
        __private_impl_to_json_option_where_clause as where_clause,
    };
}

pub mod impl_to_json_kinds {
    pub mod full {
        pub use crate::ser::json_kinds::{
            //
            AnyValue as json_value_generic_const,
            Array as EmptyArray,
            Array as ArrayOfItems,
            Object as EmptyObject,
            Object as ObjectOfKvs,
        };
    }
    pub mod chunks {
        pub use crate::ser::json_kinds::{
            //
            Array as NonEmptyArray,
            JsonString as json_string,
            Object as NonEmptyObject,
        };
    }
}

// TODO: rename to write
pub mod only_compile_time;
pub use only_compile_time as write;

pub mod refed;

#[cfg(feature = "proc-macro")]
pub mod proc_macro {
    pub use ::core::{compile_error, primitive::str, stringify};
}
