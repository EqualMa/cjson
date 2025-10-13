use crate::values::{False, Null, True};

use super::super::{iter_text_chunk::IterNonLending, traits};

macro_rules! impl_for_literal_name {
    ($For:ty = $Chunk:ident = $bytes:expr) => {
        #[derive(Debug)]
        pub struct $Chunk;

        impl AsRef<[u8]> for $Chunk {
            fn as_ref(&self) -> &[u8] {
                $bytes
            }
        }

        impl traits::IntoTextChunks for $For {
            type IntoTextChunks = IterNonLending<core::iter::Once<$Chunk>>;

            fn into_text_chunks(self) -> Self::IntoTextChunks {
                IterNonLending(core::iter::once($Chunk))
            }
        }

        impl traits::sealed::Text for $For {}
        impl traits::Text for $For {}
        impl traits::sealed::Value for $For {}
        impl traits::Value for $For {}
    };
}

impl_for_literal_name!(Null = NullChunk = b"null");
impl_for_literal_name!(False = FalseChunk = b"false");
impl_for_literal_name!(True = TrueChunk = b"true");
