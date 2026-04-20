use crate::ser::{
    iter_text_chunk::{self, HasConstChunk},
    traits::IntoTextChunks,
};

use super::ConstChunk;

impl<T: ?Sized + HasConstChunk> ConstChunk<T> {
    pub const DEFAULT: Self = Self(core::marker::PhantomData);
}

impl<T: ?Sized + HasConstChunk> core::fmt::Debug for ConstChunk<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ConstChunk")
    }
}

impl<T: ?Sized + HasConstChunk> Clone for ConstChunk<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + HasConstChunk> Copy for ConstChunk<T> {}

impl<T: ?Sized + HasConstChunk> IntoTextChunks for ConstChunk<T> {
    type IntoTextChunks = iter_text_chunk::ConstChunk<T>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        iter_text_chunk::ConstChunk::DEFAULT
    }

    fn write_into<W: ?Sized + crate::ser::traits::ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(T::CHUNK)
    }

    fn try_write_into<W: ?Sized + crate::ser::traits::TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        w.try_consume_text_chunk(T::CHUNK)
    }
}
