use crate::{
    never_future::NeverFuture,
    ser::{
        iter_text_chunk::NeverTextChunk,
        traits::{self, IntoTextChunks},
    },
    values::Never,
};

impl IntoTextChunks for Never {
    type IntoTextChunks = NeverTextChunk;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        match self {}
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        match self {}
    }

    fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, _: &mut W) {
        match self {}
    }

    fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
        self,
        _: &mut W,
    ) -> Result<(), W::Err> {
        match self {}
    }

    fn async_try_write_into<W: ?Sized + traits::AsyncTryConsumeTextChunk>(
        self,
        _: &mut W,
    ) -> impl Future<Output = Result<(), W::Err>> {
        (match self {}) as NeverFuture<Result<(), W::Err>>
    }
}

impl traits::sealed::Text for Never {}
impl traits::Text for Never {}
impl traits::sealed::Value for Never {}
impl traits::Value for Never {}
impl traits::sealed::JsonString for Never {}
impl traits::JsonString for Never {
    type IntoJsonStringFragments = Self;

    fn into_json_string_fragments(self) -> Self::IntoJsonStringFragments {
        self
    }
}

impl traits::sealed::JsonStringFragment for Never {}
impl traits::JsonStringFragment for Never {}

pub enum NeverElements {}

impl traits::sealed::Array for Never {}
impl traits::Array for Never {
    type IntoCommaSeparatedElements = NeverElements;

    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        match self {}
    }
}

impl traits::IntoTextChunks for NeverElements {
    type IntoTextChunks = NeverTextChunk;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        match self {}
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        match self {}
    }

    fn write_into<W: ?Sized + traits::ConsumeTextChunk>(self, _: &mut W) {
        match self {}
    }

    fn try_write_into<W: ?Sized + traits::TryConsumeTextChunk>(
        self,
        _: &mut W,
    ) -> Result<(), W::Err> {
        match self {}
    }

    fn async_try_write_into<W: ?Sized + traits::AsyncTryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> impl Future<Output = Result<(), W::Err>> {
        (match self {}) as NeverFuture<Result<(), W::Err>>
    }
}

impl traits::sealed::EmptyOrLeadingCommaWithCommaSeparatedElements for NeverElements {}
impl traits::EmptyOrLeadingCommaWithCommaSeparatedElements for NeverElements {}
impl traits::sealed::EmptyOrCommaSeparatedElementsWithTrailingComma for NeverElements {}
impl traits::EmptyOrCommaSeparatedElementsWithTrailingComma for NeverElements {}
impl traits::sealed::EmptyOrCommaSeparatedElements for NeverElements {}
impl traits::EmptyOrCommaSeparatedElements for NeverElements {
    type PrependLeadingCommaIfNotEmpty = Self;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        match self {}
    }

    type AppendTrailingCommaIfNotEmpty = Self;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
        match self {}
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> = Self;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        _: Other,
    ) -> Self::ChainWithComma<Other> {
        match self {}
    }
}
