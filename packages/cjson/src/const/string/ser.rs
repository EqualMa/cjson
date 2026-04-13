use crate::{
    r#const::{RuntimeChunkSurroundedWithCompileTime, State, TextChunksReadyToUngroup},
    ser::{
        texts,
        traits::{self, IntoTextChunks},
    },
};

use super::JsonString;

impl<S: RuntimeChunkSurroundedWithCompileTime> JsonString<S> {
    pub(super) const ASSERT: () = {
        let (prev_state, next_state) = S::UNGROUPED_STATES;
        prev_state.assert_same(State::INIT_IN_STRING);
        next_state.assert_same(State::INIT_IN_STRING);
    };
}

pub struct JsonStringSer<S: TextChunksReadyToUngroup>(S);

impl<S: TextChunksReadyToUngroup> JsonStringSer<S> {
    pub(super) fn from_json_string<
        'a,
        C: RuntimeChunkSurroundedWithCompileTime<ChunksReadyToUngroup<'a> = S>,
    >(
        arr: &'a JsonString<C>,
    ) -> Self {
        Self(arr.0.inner().to_text_chunks_ready_to_ungroup())
    }
}

impl<S: TextChunksReadyToUngroup> IntoTextChunks for JsonStringSer<S> {
    type IntoTextChunks = <S as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.into_text_chunks()
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0._private_into_text_chunks_vec()
    }
}

impl<S: TextChunksReadyToUngroup> traits::sealed::Text for JsonStringSer<S> {}
impl<S: TextChunksReadyToUngroup> traits::Text for JsonStringSer<S> {}
impl<S: TextChunksReadyToUngroup> traits::sealed::Value for JsonStringSer<S> {}
impl<S: TextChunksReadyToUngroup> traits::Value for JsonStringSer<S> {}
impl<S: TextChunksReadyToUngroup> traits::sealed::JsonString for JsonStringSer<S> {}
impl<S: TextChunksReadyToUngroup> traits::JsonString for JsonStringSer<S> {
    type IntoJsonStringFragments = texts::JsonStringFragment<S::Ungroup>;

    /// `S` has been validated in [`JsonString::new`].
    fn into_json_string_fragments(self) -> Self::IntoJsonStringFragments {
        let frag = S::ungroup(self.0);
        texts::JsonStringFragment::new_without_validation(frag)
    }
}
