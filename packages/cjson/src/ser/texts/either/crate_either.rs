use crate::{ser::traits, values::Either};

derive_either!(
    //
    EmptyOrLeadingCommaWithCommaSeparatedElements,
    EmptyOrCommaSeparatedElementsWithTrailingComma,
);

impl<A: traits::EmptyOrCommaSeparatedElements, B: traits::EmptyOrCommaSeparatedElements>
    traits::sealed::EmptyOrCommaSeparatedElements for Either<A, B>
{
}
impl<A: traits::EmptyOrCommaSeparatedElements, B: traits::EmptyOrCommaSeparatedElements>
    traits::EmptyOrCommaSeparatedElements for Either<A, B>
{
    type PrependLeadingCommaIfNotEmpty =
        Either<A::PrependLeadingCommaIfNotEmpty, B::PrependLeadingCommaIfNotEmpty>;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        match self {
            Self::A(this) => Either::A(A::prepend_leading_comma_if_not_empty(this)),
            Self::B(this) => Either::B(B::prepend_leading_comma_if_not_empty(this)),
        }
    }

    type AppendTrailingCommaIfNotEmpty =
        Either<A::AppendTrailingCommaIfNotEmpty, B::AppendTrailingCommaIfNotEmpty>;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
        match self {
            Self::A(this) => Either::A(A::append_trailing_comma_if_not_empty(this)),
            Self::B(this) => Either::B(B::append_trailing_comma_if_not_empty(this)),
        }
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> =
        Either<A::ChainWithComma<Other>, B::ChainWithComma<Other>>;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other> {
        match self {
            Self::A(this) => Either::A(A::chain_with_comma(this, other)),
            Self::B(this) => Either::B(B::chain_with_comma(this, other)),
        }
    }
}
