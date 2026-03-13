use crate::ser::{
    ToJsonStringFragment, iter_text_chunk,
    traits::{self, IntoTextChunks},
};

use super::Chain;

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

impl<A: ToJsonStringFragment, B: ToJsonStringFragment> ToJsonStringFragment for Chain<A, B> {
    type ToJsonStringFragment<'a>
        = Chain<A::ToJsonStringFragment<'a>, B::ToJsonStringFragment<'a>>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        Chain(
            self.0.to_json_string_fragment(),
            self.1.to_json_string_fragment(),
        )
    }
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
