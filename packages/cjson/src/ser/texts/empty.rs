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

    type AppendTrailingCommaIfNotEmpty = Self;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
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

impl traits::sealed::EmptyOrCommaSeparatedElementsWithTrailingComma for Empty {}
impl traits::EmptyOrCommaSeparatedElementsWithTrailingComma for Empty {}

// kvs
impl traits::sealed::Kvs for Empty {}
impl traits::Kvs for Empty {
    type IntoEmptyOrLeadingCommaWithNonEmptyKvs = Self;

    fn into_kvs_with_leading_comma_if_not_empty(
        self,
    ) -> Self::IntoEmptyOrLeadingCommaWithNonEmptyKvs {
        self
    }

    type IntoEmptyOrNonEmptyKvsWithTrailingComma = Self;

    fn into_kvs_with_trailing_comma_if_not_empty(
        self,
    ) -> Self::IntoEmptyOrNonEmptyKvsWithTrailingComma {
        self
    }

    type ChainOtherKvs<Other: traits::Kvs> = Other;

    fn chain_other_kvs<Other: traits::Kvs>(self, other: Other) -> Self::ChainOtherKvs<Other> {
        other
    }
}
impl traits::sealed::EmptyOrLeadingCommaWithNonEmptyKvs for Empty {}
impl traits::EmptyOrLeadingCommaWithNonEmptyKvs for Empty {}
impl traits::sealed::EmptyOrNonEmptyKvsWithTrailingComma for Empty {}
impl traits::EmptyOrNonEmptyKvsWithTrailingComma for Empty {}
