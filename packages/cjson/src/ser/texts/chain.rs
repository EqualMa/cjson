use crate::ser::{
    iter_text_chunk,
    traits::{self, IntoTextChunks},
};

use super::{Chain, Comma};

impl<A: IntoTextChunks, B: IntoTextChunks> IntoTextChunks for Chain<A, B> {
    type IntoTextChunks = iter_text_chunk::Chain<A::IntoTextChunks, B::IntoTextChunks>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        let Self(a, b) = self;
        iter_text_chunk::Chain::new(a.into_text_chunks(), b.into_text_chunks())
    }
}

impl<A: traits::JsonStringFragment, B: traits::JsonStringFragment>
    traits::sealed::JsonStringFragment for Chain<A, B>
{
}
impl<A: traits::JsonStringFragment, B: traits::JsonStringFragment> traits::JsonStringFragment
    for Chain<A, B>
{
}

impl<
    A: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
    B: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::sealed::EmptyOrLeadingCommaWithCommaSeparatedElements for Chain<A, B>
{
}
impl<
    A: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
    B: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::EmptyOrLeadingCommaWithCommaSeparatedElements for Chain<A, B>
{
}

impl<
    A: traits::EmptyOrCommaSeparatedElementsWithTrailingComma,
    B: traits::EmptyOrCommaSeparatedElementsWithTrailingComma,
> traits::sealed::EmptyOrCommaSeparatedElementsWithTrailingComma for Chain<A, B>
{
}
impl<
    A: traits::EmptyOrCommaSeparatedElementsWithTrailingComma,
    B: traits::EmptyOrCommaSeparatedElementsWithTrailingComma,
> traits::EmptyOrCommaSeparatedElementsWithTrailingComma for Chain<A, B>
{
}

impl<T: traits::NonEmptyCommaSeparatedElements>
    traits::sealed::EmptyOrLeadingCommaWithCommaSeparatedElements for Chain<Comma, T>
{
}
impl<T: traits::NonEmptyCommaSeparatedElements>
    traits::EmptyOrLeadingCommaWithCommaSeparatedElements for Chain<Comma, T>
{
}

impl<T: traits::NonEmptyCommaSeparatedElements>
    traits::sealed::EmptyOrCommaSeparatedElementsWithTrailingComma for Chain<T, Comma>
{
}
impl<T: traits::NonEmptyCommaSeparatedElements>
    traits::EmptyOrCommaSeparatedElementsWithTrailingComma for Chain<T, Comma>
{
}

impl<
    T: traits::NonEmptyCommaSeparatedElements,
    Other: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::sealed::EmptyOrCommaSeparatedElements for Chain<T, Other>
{
}
impl<
    A: traits::NonEmptyCommaSeparatedElements,
    B: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::EmptyOrCommaSeparatedElements for Chain<A, B>
{
    traits::impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements! {}
}

impl<
    A: traits::NonEmptyCommaSeparatedElements,
    B: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::sealed::NonEmptyCommaSeparatedElements for Chain<A, B>
{
}
impl<
    A: traits::NonEmptyCommaSeparatedElements,
    B: traits::EmptyOrLeadingCommaWithCommaSeparatedElements,
> traits::NonEmptyCommaSeparatedElements for Chain<A, B>
{
}

impl<A: traits::EmptyOrLeadingCommaWithNonEmptyKvs, B: traits::EmptyOrLeadingCommaWithNonEmptyKvs>
    traits::sealed::EmptyOrLeadingCommaWithNonEmptyKvs for Chain<A, B>
{
}
impl<A: traits::EmptyOrLeadingCommaWithNonEmptyKvs, B: traits::EmptyOrLeadingCommaWithNonEmptyKvs>
    traits::EmptyOrLeadingCommaWithNonEmptyKvs for Chain<A, B>
{
}

impl<A: traits::EmptyOrNonEmptyKvsWithTrailingComma, B: traits::EmptyOrNonEmptyKvsWithTrailingComma>
    traits::sealed::EmptyOrNonEmptyKvsWithTrailingComma for Chain<A, B>
{
}
impl<A: traits::EmptyOrNonEmptyKvsWithTrailingComma, B: traits::EmptyOrNonEmptyKvsWithTrailingComma>
    traits::EmptyOrNonEmptyKvsWithTrailingComma for Chain<A, B>
{
}

impl<T: traits::NonEmptyKvs> traits::sealed::EmptyOrLeadingCommaWithNonEmptyKvs
    for Chain<Comma, T>
{
}
impl<T: traits::NonEmptyKvs> traits::EmptyOrLeadingCommaWithNonEmptyKvs for Chain<Comma, T> {}

impl<T: traits::NonEmptyKvs> traits::sealed::EmptyOrNonEmptyKvsWithTrailingComma
    for Chain<T, Comma>
{
}
impl<T: traits::NonEmptyKvs> traits::EmptyOrNonEmptyKvsWithTrailingComma for Chain<T, Comma> {}

impl<T: traits::NonEmptyKvs, Other: traits::EmptyOrLeadingCommaWithNonEmptyKvs> traits::sealed::Kvs
    for Chain<T, Other>
{
}
impl<A: traits::NonEmptyKvs, B: traits::EmptyOrLeadingCommaWithNonEmptyKvs> traits::Kvs
    for Chain<A, B>
{
    traits::impl_Kvs_for_NonEmptyKvs! {}
}

impl<A: traits::NonEmptyKvs, B: traits::EmptyOrLeadingCommaWithNonEmptyKvs>
    traits::sealed::NonEmptyKvs for Chain<A, B>
{
}
impl<A: traits::NonEmptyKvs, B: traits::EmptyOrLeadingCommaWithNonEmptyKvs> traits::NonEmptyKvs
    for Chain<A, B>
{
}
