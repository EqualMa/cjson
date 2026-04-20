use crate::values::{False, Null, True};

use super::super::{iter_text_chunk::IterNonLending, traits};

macro_rules! impl_for_literal_name {
    ($For:ty = $Chunk:ident = $v_str:expr) => {
        #[derive(Debug)]
        pub struct $Chunk;

        impl $Chunk {
            pub(crate) const JSON_STR: crate::ser::texts::Value<&'static str> =
                crate::ser::texts::Value::new_without_validation($v_str);
        }

        impl AsRef<[u8]> for $Chunk {
            fn as_ref(&self) -> &[u8] {
                Self::JSON_STR.inner().as_bytes()
            }
        }

        impl traits::IntoTextChunks for $For {
            type IntoTextChunks = IterNonLending<core::iter::Once<$Chunk>>;

            fn into_text_chunks(self) -> Self::IntoTextChunks {
                IterNonLending(core::iter::once($Chunk))
            }

            fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, w: &mut W) {
                w.consume_text_chunk($Chunk::JSON_STR.inner())
            }

            fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
                self,
                w: &mut W,
            ) -> Result<(), W::Err> {
                w.try_consume_text_chunk($Chunk::JSON_STR.inner())
            }
        }

        impl traits::sealed::Text for $For {}
        impl traits::Text for $For {}
        impl traits::sealed::Value for $For {}
        impl traits::Value for $For {}
    };
}

impl_for_literal_name!(Null = NullChunk = "null");
impl_for_literal_name!(False = FalseChunk = super::boolean::Chunk(false).as_ref_str());
impl_for_literal_name!(True = TrueChunk = super::boolean::Chunk(true).as_ref_str());
