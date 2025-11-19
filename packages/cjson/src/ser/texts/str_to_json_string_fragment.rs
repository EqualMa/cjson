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
}

impl traits::sealed::JsonStringFragment for StrToJsonStringFragment<'_> {}
impl traits::JsonStringFragment for StrToJsonStringFragment<'_> {}

mod r#const;

impl<'a> StrToJsonStringFragment<'a> {
    pub(crate) const fn const_into_text_chunks(self) -> r#const::Chunks<'a> {
        r#const::Chunks::new(self.0)
    }
}
