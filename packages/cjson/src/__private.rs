pub use str;

pub use crate::__expand_or;

pub mod well_known_ident {
    pub use crate::values::Null as null;
}

pub mod well_known_macro {
    pub use crate::__private_json_well_known_macro_json_string as json_string;
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

#[cfg(feature = "proc-macro")]
pub mod proc_macro {
    pub use ::core::{compile_error, primitive::str, stringify};
}
