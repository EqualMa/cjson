use polonius_the_crab::ඞ::core::iter;

use crate::ser::{iter_text_chunk::IterNonLending, traits::IntoTextChunks};

use super::Comma;

impl Comma {
    const STR: &'static str = ",";
}

impl AsRef<[u8]> for Comma {
    fn as_ref(&self) -> &[u8] {
        b","
    }
}

impl IntoTextChunks for Comma {
    type IntoTextChunks = IterNonLending<iter::Once<Self>>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        IterNonLending(iter::once(self))
    }

    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        alloc::vec![b',']
    }

    fn write_into<W: ?Sized + crate::ser::traits::ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(Comma::STR)
    }

    fn try_write_into<W: ?Sized + crate::ser::traits::TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        w.try_consume_text_chunk(Comma::STR)
    }
}
