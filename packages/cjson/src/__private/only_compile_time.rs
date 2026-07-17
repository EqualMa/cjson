pub use self::{
    //
    non_empty_array as NonEmptyArray,
    non_empty_object as NonEmptyObject,
};

macro_rules! define_chunk_mod {
    ($mod_name:ident, ($($absolute_path_of_array:tt)+), ($($absolute_path_of_array_vec:tt)+) $(,)?) => {
        pub mod $mod_name {
            pub use $($absolute_path_of_array)+ as ChunkType;
            define_chunk_mod! {}
            pub mod json_value_generic_const {
                pub use $($absolute_path_of_array_vec)+ as ChunkType;
                define_chunk_mod! {}
            }
        }
    };
    () => {
        pub mod left_bracket {
            pub use super::*;
        }
        pub mod right_bracket {
            pub use super::*;
        }
        pub mod left_brace {
            pub use super::*;
        }
        pub mod right_brace {
            pub use super::*;
        }
        pub mod comma {
            pub use super::*;
        }
        pub mod colon {
            pub use super::*;
        }
        pub mod json_value {
            pub use super::*;
        }
        pub mod double_quote {
            pub use super::*;
        }
        pub mod json_string_fragment {
            pub use super::*;
        }
    };
}

pub mod non_empty_array {
    pub use crate::r#const::{
        ContentfulFirstChunkOfArrayAsArray as ContentfulFirstChunkAsArray,
        ContentfulLastChunkOfArrayAsArray as ContentfulLastChunkAsArray,
        NonEmptyArrayAsArray as AsArray, NonEmptyArrayAsArrayVec as AsArrayVec,
    };

    define_chunk_mod!(
        contentful_first_chunk,
        (crate::r#const::ContentfulFirstChunkOfArrayAsArray),
        (crate::r#const::ContentfulFirstChunkOfArrayAsArrayVec),
    );
    define_chunk_mod!(
        contentful_last_chunk,
        (crate::r#const::ContentfulLastChunkOfArrayAsArray),
        (crate::r#const::ContentfulLastChunkOfArrayAsArrayVec),
    );
}

pub mod non_empty_object {
    pub use crate::r#const::{
        ContentfulFirstChunkOfObjectAsArray as ContentfulFirstChunkAsArray,
        ContentfulLastChunkOfObjectAsArray as ContentfulLastChunkAsArray,
        NonEmptyObjectAsArray as AsArray, NonEmptyObjectAsArrayVec as AsArrayVec,
    };

    define_chunk_mod!(
        contentful_first_chunk,
        (crate::r#const::ContentfulFirstChunkOfObjectAsArray),
        (crate::r#const::ContentfulFirstChunkOfObjectAsArrayVec),
    );
    define_chunk_mod!(
        contentful_last_chunk,
        (crate::r#const::ContentfulLastChunkOfObjectAsArray),
        (crate::r#const::ContentfulLastChunkOfObjectAsArrayVec),
    );
}

pub mod json_string {
    pub use crate::r#const::{JsonStringAsArray as AsArray, JsonStringAsArrayVec as AsArrayVec};
}
