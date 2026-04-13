use crate::{
    r#const::{RuntimeChunkSurroundedWithCompileTime, State, TextChunksReadyToUngroup},
    ser::{
        texts,
        traits::{self, IntoTextChunks},
    },
};

use super::NonEmptyObject;

impl<OBJ: RuntimeChunkSurroundedWithCompileTime> NonEmptyObject<OBJ> {
    pub(super) const ASSERT: () = {
        let (prev_state, next_state) = OBJ::UNGROUPED_STATES;
        prev_state.assert_same(State::INIT_AFTER_OBJECT_START);
        next_state.assert_same(State::INIT_AFTER_OBJECT_FIELD_VALUE);
    };
}

pub struct NonEmptyObjectSer<OBJ: TextChunksReadyToUngroup>(OBJ);

impl<OBJ: TextChunksReadyToUngroup> NonEmptyObjectSer<OBJ> {
    pub(super) fn from_non_empty_object<
        'a,
        C: RuntimeChunkSurroundedWithCompileTime<ChunksReadyToUngroup<'a> = OBJ>,
    >(
        arr: &'a NonEmptyObject<C>,
    ) -> Self {
        Self(arr.0.inner().to_text_chunks_ready_to_ungroup())
    }
}

impl<OBJ: TextChunksReadyToUngroup> IntoTextChunks for NonEmptyObjectSer<OBJ> {
    type IntoTextChunks = <OBJ as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.into_text_chunks()
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0._private_into_text_chunks_vec()
    }
}

impl<OBJ: TextChunksReadyToUngroup> traits::sealed::Text for NonEmptyObjectSer<OBJ> {}
impl<OBJ: TextChunksReadyToUngroup> traits::Text for NonEmptyObjectSer<OBJ> {}
impl<OBJ: TextChunksReadyToUngroup> traits::sealed::Value for NonEmptyObjectSer<OBJ> {}
impl<OBJ: TextChunksReadyToUngroup> traits::Value for NonEmptyObjectSer<OBJ> {}
impl<OBJ: TextChunksReadyToUngroup> traits::sealed::Object for NonEmptyObjectSer<OBJ> {}
impl<OBJ: TextChunksReadyToUngroup> traits::Object for NonEmptyObjectSer<OBJ> {
    type IntoKvs = texts::NonEmptyKvs<OBJ::Ungroup>;

    /// `OBJ` has been validated in [`NonEmptyObject::new`].
    fn into_kvs(self) -> Self::IntoKvs {
        let items = OBJ::ungroup(self.0);
        texts::NonEmptyKvs::new_without_validation(items)
    }
}
