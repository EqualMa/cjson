use crate::ser::traits::{self, IntoTextChunks};

use super::{Chain, CommaSeparated};

impl<A: traits::EmptyOrCommaSeparatedElements, B: traits::EmptyOrCommaSeparatedElements>
    IntoTextChunks for CommaSeparated<A, B>
{
    type IntoTextChunks = <A::ChainWithComma<B> as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        let Self(a, b) = self;
        a.chain_with_comma(b).into_text_chunks()
    }
}

impl<A: traits::EmptyOrCommaSeparatedElements, B: traits::EmptyOrCommaSeparatedElements>
    traits::sealed::EmptyOrCommaSeparatedElements for CommaSeparated<A, B>
{
}

impl<A: traits::EmptyOrCommaSeparatedElements, B: traits::EmptyOrCommaSeparatedElements>
    traits::EmptyOrCommaSeparatedElements for CommaSeparated<A, B>
{
    type PrependLeadingCommaIfNotEmpty =
        Chain<A::PrependLeadingCommaIfNotEmpty, B::PrependLeadingCommaIfNotEmpty>;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        let Self(a, b) = self;
        Chain(
            a.prepend_leading_comma_if_not_empty(),
            b.prepend_leading_comma_if_not_empty(),
        )
    }

    type AppendTrailingCommaIfNotEmpty =
        Chain<A::AppendTrailingCommaIfNotEmpty, B::AppendTrailingCommaIfNotEmpty>;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
        let Self(a, b) = self;
        Chain(
            a.append_trailing_comma_if_not_empty(),
            b.append_trailing_comma_if_not_empty(),
        )
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> =
        <A::ChainWithComma<B> as traits::EmptyOrCommaSeparatedElements>::ChainWithComma<Other>;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other> {
        let Self(a, b) = self;
        a.chain_with_comma(b).chain_with_comma(other)
    }
}
