pub use self::non_empty_array as NonEmptyArray;
pub mod non_empty_array {
    pub use crate::r#const::{
        NonEmptyArrayAsArray as AsArray, NonEmptyArrayAsArrayVec as AsArrayVec,
    };
}
