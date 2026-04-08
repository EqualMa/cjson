use crate::{
    r#const::{RuntimeChunkSurroundedWithCompileTime, State, TextChunksReadyToUngroup},
    ser::{
        texts,
        traits::{self, IntoTextChunks},
    },
};

use super::NonEmptyArray;

impl<ARR: RuntimeChunkSurroundedWithCompileTime> NonEmptyArray<ARR> {
    pub(super) const ASSERT: () = {
        let (prev_state, next_state) = ARR::UNGROUPED_STATES;
        prev_state.assert_same(State::INIT_AFTER_ARRAY_START);
        next_state.assert_same(State::INIT_AFTER_ARRAY_ITEM);
    };
}

pub struct NonEmptyArraySer<ARR: TextChunksReadyToUngroup>(ARR);

impl<ARR: TextChunksReadyToUngroup> NonEmptyArraySer<ARR> {
    pub(super) fn from_non_empty_array<
        'a,
        C: RuntimeChunkSurroundedWithCompileTime<ChunksReadyToUngroup<'a> = ARR>,
    >(
        arr: &'a NonEmptyArray<C>,
    ) -> Self {
        Self(arr.0.inner().to_text_chunks_ready_to_ungroup())
    }
}

impl<ARR: TextChunksReadyToUngroup> IntoTextChunks for NonEmptyArraySer<ARR> {
    type IntoTextChunks = <ARR as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.into_text_chunks()
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0._private_into_text_chunks_vec()
    }
}

impl<ARR: TextChunksReadyToUngroup> traits::sealed::Text for NonEmptyArraySer<ARR> {}
impl<ARR: TextChunksReadyToUngroup> traits::Text for NonEmptyArraySer<ARR> {}
impl<ARR: TextChunksReadyToUngroup> traits::sealed::Value for NonEmptyArraySer<ARR> {}
impl<ARR: TextChunksReadyToUngroup> traits::Value for NonEmptyArraySer<ARR> {}
impl<ARR: TextChunksReadyToUngroup> traits::sealed::Array for NonEmptyArraySer<ARR> {}
impl<ARR: TextChunksReadyToUngroup> traits::Array for NonEmptyArraySer<ARR> {
    type IntoCommaSeparatedElements = texts::NonEmptyCommaSeparatedItems<ARR::Ungroup>;

    /// `ARR` has been validated in [`NonEmptyArray::new`].
    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        let items = ARR::ungroup(self.0);
        texts::NonEmptyCommaSeparatedItems::new_without_validation(items)
    }
}
