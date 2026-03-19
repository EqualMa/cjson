pub mod attrs {
    pub mod r#struct {
        pub mod cjson {
            pub use super::super::common::cjson::crate_;
        }
    }

    pub mod r#enum {
        pub mod cjson {
            pub use super::super::common::cjson::crate_;
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
        }
    }
}
