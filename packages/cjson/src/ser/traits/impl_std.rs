use std::io::{self};

use super::TryConsumeTextChunk;

pub struct IoWrite<W: ?Sized + io::Write>(pub W);

impl<W: ?Sized + io::Write> TryConsumeTextChunk for IoWrite<W> {
    type Err = io::Error;

    fn try_consume_text_chunk(&mut self, chunk: &str) -> Result<(), Self::Err> {
        self.0.write_all(chunk.as_bytes())
    }
}
