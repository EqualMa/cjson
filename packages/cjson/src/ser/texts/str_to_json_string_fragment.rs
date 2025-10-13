use crate::ser::{
    iter_text_chunk::IterTextChunk,
    traits::{self, IntoTextChunks},
};

use super::StrToJsonStringFragment;

mod escape;

#[derive(Debug)]
pub struct Chunks<'a> {
    iter_bytes: core::slice::Iter<'a, u8>,
    escaped: Option<&'static [u8]>,
}

impl<'a> Chunks<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter_bytes: s.as_bytes().iter(),
            escaped: None,
        }
    }

    fn next(&mut self) -> Option<&'a [u8]> {
        if let Some(ch) = self.escaped.take() {
            return Some(ch);
        }

        if self.iter_bytes.len() == 0 {
            return None;
        }

        let bytes = self.iter_bytes.as_slice();

        match self.iter_bytes.position(escape::needs_escape) {
            Some(i) => {
                let byte = bytes[i];

                let escaped = unsafe { escape::escape_to_bytes_unchecked(byte) };

                if i == 0 {
                    Some(escaped)
                } else {
                    self.escaped = Some(escaped);

                    Some(&bytes[..i])
                }
            }
            None => Some(bytes),
        }
    }
}

impl<'this> IterTextChunk for Chunks<'this> {
    type Chunk<'a>
        = &'this [u8]
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.next()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let escaped_len = self.escaped.map_or(0, <[u8]>::len);
        let bytes_len = self.iter_bytes.len();
        (
            escaped_len.saturating_add(bytes_len),
            match bytes_len.checked_mul(6) {
                Some(v) => escaped_len.checked_add(v),
                None => None,
            },
        )
    }
}

impl<'a> IntoTextChunks for StrToJsonStringFragment<'a> {
    type IntoTextChunks = Chunks<'a>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        Chunks::new(self.0)
    }
}

impl traits::sealed::JsonStringFragment for StrToJsonStringFragment<'_> {}
impl traits::JsonStringFragment for StrToJsonStringFragment<'_> {}
