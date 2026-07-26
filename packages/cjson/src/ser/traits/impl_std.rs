use std::io;

use super::TryConsumeTextChunk;

pub struct IoWrite<W: ?Sized + io::Write>(pub W);

impl<W: ?Sized + io::Write> TryConsumeTextChunk for IoWrite<W> {
    type Err = io::Error;

    fn try_consume_text_chunk(&mut self, chunk: &str) -> Result<(), Self::Err> {
        self.0.write_all(chunk.as_bytes())
    }

    fn try_consume_2_text_chunks(&mut self, chunk1: &str, chunk2: &str) -> Result<(), Self::Err> {
        #[cfg(not(feature = "write_all_vectored"))]
        {
            self.try_consume_text_chunk(chunk1)?;
            self.try_consume_text_chunk(chunk2)?;
            Ok(())
        }

        #[cfg(feature = "write_all_vectored")]
        self.0.write_all_vectored(&mut [
            io::IoSlice::new(chunk1.as_bytes()),
            io::IoSlice::new(chunk2.as_bytes()),
        ])
    }

    fn as_mut_try_consume_text_chunk(&mut self) -> impl TryConsumeTextChunk<Err = Self::Err>
    where
        Self: Sized,
    {
        super::MutTryConsume(self)
    }
}
