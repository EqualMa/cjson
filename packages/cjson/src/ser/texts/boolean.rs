use crate::ser::{
    iter_text_chunk::IterNonLending,
    traits::{self, IntoTextChunks},
};

use super::Boolean;

pub struct Chunk(pub(crate) bool);

impl Chunk {
    pub(crate) const fn as_ref_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }
    pub(crate) const fn as_ref_u8_slice(&self) -> &'static [u8] {
        self.as_ref_str().as_bytes()
    }
}

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        self.as_ref_u8_slice()
    }
}

impl Boolean {
    const fn as_ref_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }
}

impl IntoTextChunks for Boolean {
    type IntoTextChunks = IterNonLending<core::iter::Once<Chunk>>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        IterNonLending(core::iter::once(Chunk(self.0)))
    }

    fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(self.as_ref_str())
    }

    fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        w.try_consume_text_chunk(self.as_ref_str())
    }
}

impl traits::sealed::Text for Boolean {}
impl traits::Text for Boolean {}
impl traits::sealed::Value for Boolean {}
impl traits::Value for Boolean {}

mod r#const;
