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
}

pub trait ImplToJsonHelper {
    type ImplToJsonHelper<'a>: crate::ser::ToJson
    where
        Self: 'a;
}

pub mod proc_macro {
    pub mod attrs {
        pub mod cjson {
            pub mod crate_path {}
        }
        pub mod cjson_struct {
            pub use super::cjson::*;
        }
    }
    pub use ::cjson_proc_macro::unnamed_fields;
    pub use ::core::{compile_error, stringify};
}
