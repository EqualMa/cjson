use crate::ser::{iter_text_chunk, traits::IntoTextChunks};

use super::{super::traits, Empty};

impl IntoTextChunks for Empty {
    type IntoTextChunks = iter_text_chunk::Empty;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        iter_text_chunk::Empty
    }
}
impl traits::sealed::JsonStringFragment for Empty {}
impl traits::JsonStringFragment for Empty {}

impl traits::sealed::EmptyOrCommaSeparatedElements for Empty {}
impl traits::EmptyOrCommaSeparatedElements for Empty {
    type PrependLeadingCommaIfNotEmpty = Self;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        self
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> = Other;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other> {
        other
    }
}

impl traits::sealed::EmptyOrLeadingCommaWithCommaSeparatedElements for Empty {}
impl traits::EmptyOrLeadingCommaWithCommaSeparatedElements for Empty {}
