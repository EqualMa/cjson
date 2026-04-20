use super::iter_text_chunk::{HasConstChunk, IterTextChunk};

#[cfg(feature = "alloc")]
mod impl_alloc;

#[cfg(feature = "std")]
pub(crate) mod impl_std;

pub trait ConsumeTextChunk {
    fn consume_text_chunk(&mut self, chunk: &str);
}

pub trait TryConsumeTextChunk {
    type Err;

    fn try_consume_text_chunk(&mut self, chunk: &str) -> Result<(), Self::Err>;
}

impl<C: ?Sized + ConsumeTextChunk> TryConsumeTextChunk for C {
    type Err = core::convert::Infallible;

    fn try_consume_text_chunk(&mut self, chunk: &str) -> Result<(), Self::Err> {
        self.consume_text_chunk(chunk);
        Ok(())
    }
}

// TODO: sealed
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

    fn write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W);

    fn try_write_into<W: ?Sized + TryConsumeTextChunk>(self, w: &mut W) -> Result<(), W::Err>;
}

macro_rules! proxy_IntoTextChunks {
    (|$self_:ident| -> $Proxy:ty $proxy:block ) => {
        type IntoTextChunks = <$Proxy as crate::ser::traits::IntoTextChunks>::IntoTextChunks;
        fn into_text_chunks($self_) -> Self::IntoTextChunks {
            <$Proxy as crate::ser::traits::IntoTextChunks>::into_text_chunks($proxy)
        }

        #[doc(hidden)]
        #[cfg(feature = "alloc")]
        fn _private_into_text_chunks_vec($self_) -> alloc::vec::Vec<u8> {
            <$Proxy as crate::ser::traits::IntoTextChunks>::_private_into_text_chunks_vec($proxy)
        }

        crate::ser::traits::proxy_IntoTextChunks_write! {
            |$self_| -> $Proxy $proxy
        }
    };
}

macro_rules! proxy_IntoTextChunks_write {
    (|$self_:ident| -> $Proxy:ty $proxy:block ) => {
        fn write_into<W: ?Sized + crate::ser::traits::ConsumeTextChunk>($self_, w: &mut W) {
            <$Proxy as crate::ser::traits::IntoTextChunks>::write_into($proxy, w)
        }

        fn try_write_into<W: ?Sized + crate::ser::traits::TryConsumeTextChunk>($self_, w: &mut W) -> Result<(), W::Err> {
            <$Proxy as crate::ser::traits::IntoTextChunks>::try_write_into($proxy, w)
        }
    };
}

pub(crate) use {proxy_IntoTextChunks, proxy_IntoTextChunks_write};

impl IntoTextChunks for &str {
    type IntoTextChunks = Self;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self
    }

    fn write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(self)
    }

    fn try_write_into<W: TryConsumeTextChunk + ?Sized>(self, w: &mut W) -> Result<(), W::Err> {
        w.try_consume_text_chunk(self)
    }
}

#[cfg(feature = "alloc")]
impl IntoTextChunks for alloc::string::String {
    type IntoTextChunks = Self;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self
    }

    fn write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(&self)
    }

    fn try_write_into<W: ?Sized + TryConsumeTextChunk>(self, w: &mut W) -> Result<(), W::Err> {
        w.try_consume_text_chunk(&self)
    }
}

pub(crate) mod sealed {
    pub trait Text {}
    pub trait Value {}
    pub trait JsonString {}
    pub trait Array {}
    pub trait Object {}
    pub trait EmptyOrCommaSeparatedElements {}
    pub trait NonEmptyCommaSeparatedElements {}
    pub trait EmptyOrLeadingCommaWithCommaSeparatedElements {}
    pub trait EmptyOrCommaSeparatedElementsWithTrailingComma {}
    pub trait JsonStringFragment {}

    pub trait Kvs {}
    pub trait NonEmptyKvs {}
    pub trait EmptyOrLeadingCommaWithNonEmptyKvs {}
    pub trait EmptyOrNonEmptyKvsWithTrailingComma {}
}

/// Json text.
pub trait Text: sealed::Text + IntoTextChunks {}

/// All json values are json texts without surrounding whitespaces.
pub trait Value: sealed::Value + Text {}

pub trait JsonString: sealed::JsonString + Value {
    type IntoJsonStringFragments: JsonStringFragment;
    fn into_json_string_fragments(self) -> Self::IntoJsonStringFragments;
}

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

macro_rules! impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements {
    () => {
        type PrependLeadingCommaIfNotEmpty =
            crate::ser::texts::Chain<crate::ser::texts::Comma, Self>;
        fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
            crate::ser::texts::Chain(crate::ser::texts::Comma, self)
        }

        type AppendTrailingCommaIfNotEmpty =
            crate::ser::texts::Chain<Self, crate::ser::texts::Comma>;
        fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
            crate::ser::texts::Chain(self, crate::ser::texts::Comma)
        }

        type ChainWithComma<Other: crate::ser::traits::EmptyOrCommaSeparatedElements> =
            crate::ser::texts::Chain<Self, Other::PrependLeadingCommaIfNotEmpty>;

        fn chain_with_comma<Other: crate::ser::traits::EmptyOrCommaSeparatedElements>(
            self,
            other: Other,
        ) -> Self::ChainWithComma<Other> {
            crate::ser::texts::Chain(self, other.prepend_leading_comma_if_not_empty())
        }
    };
}

pub(crate) use impl_EmptyOrCommaSeparatedElements_for_NonEmptyCommaSeparatedElements;

pub trait NonEmptyCommaSeparatedElements:
    EmptyOrCommaSeparatedElements + sealed::NonEmptyCommaSeparatedElements
{
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

pub trait Object: sealed::Object + Value {
    type IntoKvs: Kvs;
    fn into_kvs(self) -> Self::IntoKvs;
}

pub trait Kvs: sealed::Kvs + IntoTextChunks {
    type IntoEmptyOrLeadingCommaWithNonEmptyKvs: EmptyOrLeadingCommaWithNonEmptyKvs;
    fn into_kvs_with_leading_comma_if_not_empty(
        self,
    ) -> Self::IntoEmptyOrLeadingCommaWithNonEmptyKvs;

    type IntoEmptyOrNonEmptyKvsWithTrailingComma: EmptyOrNonEmptyKvsWithTrailingComma;
    fn into_kvs_with_trailing_comma_if_not_empty(
        self,
    ) -> Self::IntoEmptyOrNonEmptyKvsWithTrailingComma;

    type ChainOtherKvs<Other: Kvs>: Kvs;

    fn chain_other_kvs<Other: Kvs>(self, other: Other) -> Self::ChainOtherKvs<Other>;
}

macro_rules! impl_Kvs_for_NonEmptyKvs {
    () => {
        type IntoEmptyOrLeadingCommaWithNonEmptyKvs =
            crate::ser::texts::Chain<crate::ser::texts::Comma, Self>;
        fn into_kvs_with_leading_comma_if_not_empty(
            self,
        ) -> Self::IntoEmptyOrLeadingCommaWithNonEmptyKvs {
            crate::ser::texts::Chain(crate::ser::texts::Comma, self)
        }

        type IntoEmptyOrNonEmptyKvsWithTrailingComma =
            crate::ser::texts::Chain<Self, crate::ser::texts::Comma>;
        fn into_kvs_with_trailing_comma_if_not_empty(
            self,
        ) -> Self::IntoEmptyOrNonEmptyKvsWithTrailingComma {
            crate::ser::texts::Chain(self, crate::ser::texts::Comma)
        }

        type ChainOtherKvs<Other: crate::ser::traits::Kvs> =
            crate::ser::texts::Chain<Self, Other::IntoEmptyOrLeadingCommaWithNonEmptyKvs>;

        fn chain_other_kvs<Other: crate::ser::traits::Kvs>(
            self,
            other: Other,
        ) -> Self::ChainOtherKvs<Other> {
            crate::ser::texts::Chain(self, other.into_kvs_with_leading_comma_if_not_empty())
        }
    };
}

pub(crate) use impl_Kvs_for_NonEmptyKvs;

pub trait NonEmptyKvs: sealed::NonEmptyKvs + Kvs {}
pub trait EmptyOrLeadingCommaWithNonEmptyKvs:
    sealed::EmptyOrLeadingCommaWithNonEmptyKvs + IntoTextChunks
{
}
pub trait EmptyOrNonEmptyKvsWithTrailingComma:
    sealed::EmptyOrNonEmptyKvsWithTrailingComma + IntoTextChunks
{
}

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
