use crate::ser::traits::{self, IntoTextChunks};

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
}

impl<'a> IntoTextChunks for StrToJsonStringFragment<'a> {
    type IntoTextChunks = Chunks<'a>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        Chunks::new(self.0)
    }

    fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, w: &mut W) {
        let mut iter_bytes = self.0.as_bytes().iter();

        while iter_bytes.len() > 0 {
            let bytes = iter_bytes.as_slice();

            match iter_bytes.position(escape::needs_escape) {
                Some(i) => {
                    let byte = bytes[i];

                    let escaped = unsafe { escape::escape_to_bytes_unchecked(byte) };

                    if i > 0 {
                        let prev = bytes.split_at(i).0;
                        w.consume_text_chunk(unsafe { str::from_utf8_unchecked(prev) });
                    }

                    w.consume_text_chunk(unsafe { str::from_utf8_unchecked(escaped) });
                }
                None => w.consume_text_chunk(unsafe { str::from_utf8_unchecked(bytes) }),
            }
        }
    }

    fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        let mut iter_bytes = self.0.as_bytes().iter();

        while iter_bytes.len() > 0 {
            let bytes = iter_bytes.as_slice();

            match iter_bytes.position(escape::needs_escape) {
                Some(i) => {
                    let byte = bytes[i];

                    let escaped = unsafe { escape::escape_to_bytes_unchecked(byte) };

                    if i > 0 {
                        let prev = bytes.split_at(i).0;
                        w.try_consume_text_chunk(unsafe { str::from_utf8_unchecked(prev) })?;
                    }

                    w.try_consume_text_chunk(unsafe { str::from_utf8_unchecked(escaped) })?;
                }
                None => w.try_consume_text_chunk(unsafe { str::from_utf8_unchecked(bytes) })?,
            }
        }

        Ok(())
    }

    async fn async_try_write_into<W: ?Sized + traits::AsyncTryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        let mut iter_bytes = self.0.as_bytes().iter();

        while iter_bytes.len() > 0 {
            let bytes = iter_bytes.as_slice();

            match iter_bytes.position(escape::needs_escape) {
                Some(i) => {
                    let byte = bytes[i];

                    let escaped = unsafe { escape::escape_to_bytes_unchecked(byte) };

                    if i > 0 {
                        let prev = bytes.split_at(i).0;
                        w.async_try_consume_text_chunk(unsafe { str::from_utf8_unchecked(prev) })
                            .await?;
                    }

                    w.async_try_consume_text_chunk(unsafe { str::from_utf8_unchecked(escaped) })
                        .await?;
                }
                None => {
                    w.async_try_consume_text_chunk(unsafe { str::from_utf8_unchecked(bytes) })
                        .await?
                }
            }
        }

        Ok(())
    }
}

impl traits::sealed::JsonStringFragment for StrToJsonStringFragment<'_> {}
impl traits::JsonStringFragment for StrToJsonStringFragment<'_> {}

mod r#const;

impl<'a> StrToJsonStringFragment<'a> {
    pub(crate) const fn const_into_text_chunks(self) -> r#const::Chunks<'a> {
        r#const::Chunks::new(self.0)
    }
}
