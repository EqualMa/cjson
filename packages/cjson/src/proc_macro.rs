/// `rename = $name:expr`
///
/// `rename()` hyphen-separated paths
///
/// `rename(with($rename_func:expr))`
#[allow(non_snake_case)]
pub mod rename {
    pub mod with {}
    pub mod lowercase {}
    pub mod UPPERCASE {}
    pub mod PascalCase {}
    pub mod camelCase {}

    pub mod snake_case {}
    pub mod SCREAMING_SNAKE_CASE {}

    pub mod kebab {
        pub mod case {}
    }

    pub mod SCREAMING {
        pub mod KEBAB {
            pub mod CASE {}
        }
    }
}

pub mod attrs {
    pub mod r#struct {
        pub mod cjson {
            pub use super::super::common::cjson::{
                IS_CHAINABLE_AND_ALWAYS_EMPTY, JsonKind, crate_, derive_from, r#where, where_into,
                where_to,
            };

            pub mod rename {
                pub use super::super::super::super::rename::*;
            }
            pub mod tag {}
            pub mod rename_fields {}
            pub mod transparent {}
            pub mod to {}
        }
        pub mod field {
            pub mod cjson {
                pub mod to {}
            }
        }
    }

    pub mod r#enum {
        pub mod cjson {
            pub use super::super::common::cjson::{
                IS_CHAINABLE_AND_ALWAYS_EMPTY, JsonKind, crate_, derive_from, r#where, where_into,
                where_to,
            };

            /// See [`#[json_x(any_value)]`](crate::macro_helpers::json_x::any_value).
            pub mod any_value {}
        }
        pub mod variant {
            pub mod field {}
        }
    }

    pub mod common {
        pub mod cjson {
            /// Specifies the path to this crate
            ///
            /// ```
            /// # use cjson::ToJson; mod path { pub mod to { pub use ::cjson; }}
            /// #[derive(ToJson)]
            /// #[cjson(crate(path::to::cjson))]
            /// struct Obj {}
            /// ```
            pub mod crate_ {}
            pub mod r#where {}
            pub mod where_to {}
            pub mod where_into {}
            pub mod derive_from {}

            #[allow(non_snake_case)]
            pub mod JsonKind {}
            #[allow(non_snake_case)]
            pub mod IS_CHAINABLE_AND_ALWAYS_EMPTY {}
        }
    }
}
