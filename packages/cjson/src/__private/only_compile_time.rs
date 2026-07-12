pub use self::{
    //
    non_empty_array as NonEmptyArray,
    non_empty_object as NonEmptyObject,
};

pub mod non_empty_array {
    pub use crate::r#const::{
        ContentfulFirstChunkOfArrayAsArray as ContentfulFirstChunkAsArray,
        ContentfulFirstChunkOfArrayAsArrayVec as ContentfulFirstChunkAsArrayVec,
        ContentfulLastChunkOfArrayAsArray as ContentfulLastChunkAsArray,
        ContentfulLastChunkOfArrayAsArrayVec as ContentfulLastChunkAsArrayVec,
        NonEmptyArrayAsArray as AsArray, NonEmptyArrayAsArrayVec as AsArrayVec,
    };
}

pub mod non_empty_object {
    pub use crate::r#const::{
        ContentfulFirstChunkOfObjectAsArray as ContentfulFirstChunkAsArray,
        ContentfulFirstChunkOfObjectAsArrayVec as ContentfulFirstChunkAsArrayVec,
        ContentfulLastChunkOfObjectAsArray as ContentfulLastChunkAsArray,
        ContentfulLastChunkOfObjectAsArrayVec as ContentfulLastChunkAsArrayVec,
        NonEmptyObjectAsArray as AsArray, NonEmptyObjectAsArrayVec as AsArrayVec,
    };
}
