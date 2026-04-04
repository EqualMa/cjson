use super::iter_text_chunk::IterTextChunk;

pub trait IntoTextChunks {
    type IntoTextChunks: IterTextChunk;
    fn into_text_chunks(self) -> Self::IntoTextChunks;

    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8>
    where
        Self: Sized,
    {
        IterTextChunk::_private_collect_into_vec(self.into_text_chunks())
    }
}

impl<T: IterTextChunk> IntoTextChunks for T {
    type IntoTextChunks = Self;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self
    }
}

pub(crate) mod sealed {
    pub trait Text {}
    pub trait Value {}
    pub trait Array {}
    pub trait EmptyOrCommaSeparatedElements {}
    pub trait EmptyOrLeadingCommaWithCommaSeparatedElements {}
    pub trait EmptyOrCommaSeparatedElementsWithTrailingComma {}
    pub trait JsonStringFragment {}
}

/// Json text.
pub trait Text: sealed::Text + IntoTextChunks {}

/// All json values are json texts without surrounding whitespaces.
pub trait Value: sealed::Value + Text {}

pub trait Array: sealed::Array + Value {
    type IntoCommaSeparatedElements: EmptyOrCommaSeparatedElements;
    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements;
}

/// Conforms to `ws [ value *( value-separator value ) ] ws`
pub trait EmptyOrCommaSeparatedElements:
    sealed::EmptyOrCommaSeparatedElements + IntoTextChunks
{
    type PrependLeadingCommaIfNotEmpty: EmptyOrLeadingCommaWithCommaSeparatedElements;
    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty;

    type AppendTrailingCommaIfNotEmpty: EmptyOrCommaSeparatedElementsWithTrailingComma;
    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty;

    type ChainWithComma<Other: EmptyOrCommaSeparatedElements>: EmptyOrCommaSeparatedElements;
    fn chain_with_comma<Other: EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other>;
}

pub trait EmptyOrLeadingCommaWithCommaSeparatedElements:
    sealed::EmptyOrLeadingCommaWithCommaSeparatedElements + IntoTextChunks
{
}

pub trait EmptyOrCommaSeparatedElementsWithTrailingComma:
    sealed::EmptyOrCommaSeparatedElementsWithTrailingComma + IntoTextChunks
{
}

/// `s` is [`JsonStringFragment`] if and only if `s` surrounded with `"` is a valid json string that
/// contains only bit sequences of encoded Unicode characters.
///
/// - `b"\\u0000"` is a `JsonStringFragment`.
/// - `b"\\uD834\\uDD1E"` and `b"\xF0\x9D\x84\x9E"` are both `JsonStringFragment`s and
///   they can be decoded to the same utf-8 string if surrounded with `"`.
/// - `b"\\uDEAD"` (a single unpaired UTF-16 surrogate) is not a `JsonStringFragment`.
pub trait JsonStringFragment: sealed::JsonStringFragment + IntoTextChunks {}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    #[cfg(feature = "alloc")]
    use alloc::{string::String, vec::Vec};

    const _: () = {
        struct AssertAsRefU8Slice<T: ?Sized + AsRef<[u8]>>(PhantomData<T>);

        AssertAsRefU8Slice::<[u8]>(PhantomData);
        AssertAsRefU8Slice::<[u8; 0]>(PhantomData);
        AssertAsRefU8Slice::<[u8; 4096]>(PhantomData);
        AssertAsRefU8Slice::<&[u8]>(PhantomData);
        AssertAsRefU8Slice::<&str>(PhantomData);
        #[cfg(feature = "alloc")]
        {
            AssertAsRefU8Slice::<String>(PhantomData);
            AssertAsRefU8Slice::<Vec<u8>>(PhantomData);
        }
    };
}
