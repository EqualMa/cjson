use crate::ser::{
    iter_text_chunk::IterNonLending,
    traits::{self, IntoTextChunks},
};

use super::Boolean;

pub struct Chunk(pub(crate) bool);

impl Chunk {
    const fn as_ref_str(&self) -> &'static str {
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

impl IntoTextChunks for Boolean {
    type IntoTextChunks = IterNonLending<core::iter::Once<Chunk>>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        IterNonLending(core::iter::once(Chunk(self.0)))
    }
}

impl traits::sealed::Text for Boolean {}
impl traits::Text for Boolean {}
impl traits::sealed::Value for Boolean {}
impl traits::Value for Boolean {}
