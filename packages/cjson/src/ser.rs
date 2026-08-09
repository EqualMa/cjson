use crate::ser::traits::{AsyncTryConsumeTextChunk, TryConsumeTextChunk};

pub use self::consumers::{
    AsyncTryConsumeJson, ConsumeChained, ConsumeJson, ConsumeJsonChunks, ConsumeJsonChunksFromInit,
    ConsumeJsonText, Consumed, TryConsumeJson,
    chunks::{ReadyToConsumeJsonChunksOfNonEmptyArray, ReadyToConsumeJsonChunksOfNonEmptyObject},
    json_kinds,
    json_string_chunks::ConsumeInJsonString,
};

#[cfg(feature = "std")]
pub use self::traits::impl_std::IoWrite;

pub(crate) use self::consumers::{define_traits, writer_assert::WriterAssertIsFromConsumeJsonText};

use json_kinds::JsonKind;

mod consumers;
pub(crate) mod helpers;
pub mod iter_text_chunk;
pub mod open_close;
pub mod texts;
pub mod traits;
pub mod values;

pub mod exts;

pub trait IntoJson {
    type JsonKind: JsonKind;
    fn json_provide_into<W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>>(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W>;

    fn json_provide_into_try<
        W: TryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Result<Consumed<Self::JsonKind, W>, <W::Writer as TryConsumeTextChunk>::Err>;

    fn json_provide_into_async_try<
        W: AsyncTryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> impl Future<
        Output = Result<Consumed<Self::JsonKind, W>, <W::Writer as AsyncTryConsumeTextChunk>::Err>,
    >;

    /// If implemented as `true`, chaining with another json value is optimized.
    /// Note that it is always correct to implement as `false`.
    ///
    /// Wrong implementations will not affect json validity.
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool;
}

pub trait ToJson {
    type ToJsonKind: JsonKind;
    fn json_provide_to<W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>>(
        &self,
        w: W,
    ) -> Consumed<Self::ToJsonKind, W>;
    fn json_provide_to_try<
        W: TryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> Result<Consumed<Self::ToJsonKind, W>, <W::Writer as TryConsumeTextChunk>::Err>;
    fn json_provide_to_async_try<
        W: AsyncTryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> impl Future<
        Output = Result<
            Consumed<Self::ToJsonKind, W>,
            <W::Writer as AsyncTryConsumeTextChunk>::Err,
        >,
    >;
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool;
}

pub trait ToJsonByCopyIntoJson: Copy + IntoJson {}

mod into_json_key_colon_value;
pub trait IntoJsonKeyColonValue: into_json_key_colon_value::Sealed {}

impl<T: ?Sized + ToJson> IntoJson for &T {
    type JsonKind = T::ToJsonKind;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        T::json_provide_to(self, w)
    }
    fn json_provide_into_try<
        W: TryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Result<Consumed<Self::JsonKind, W>, <W::Writer as TryConsumeTextChunk>::Err> {
        T::json_provide_to_try(self, w)
    }
    fn json_provide_into_async_try<
        W: AsyncTryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> impl Future<
        Output = Result<Consumed<Self::JsonKind, W>, <W::Writer as AsyncTryConsumeTextChunk>::Err>,
    > {
        T::json_provide_to_async_try(self, w)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = T::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

impl<T: ToJsonByCopyIntoJson> ToJson for T {
    type ToJsonKind = T::JsonKind;

    fn json_provide_to<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> Consumed<Self::ToJsonKind, W> {
        T::json_provide_into(*self, w)
    }
    fn json_provide_to_try<
        W: TryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> Result<Consumed<Self::ToJsonKind, W>, <W::Writer as TryConsumeTextChunk>::Err> {
        T::json_provide_into_try(*self, w)
    }
    fn json_provide_to_async_try<
        W: AsyncTryConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> impl Future<
        Output = Result<
            Consumed<Self::ToJsonKind, W>,
            <W::Writer as AsyncTryConsumeTextChunk>::Err,
        >,
    > {
        T::json_provide_into_async_try(*self, w)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = <T as IntoJson>::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}

impl<T: ?Sized + ToJson> ToJsonByCopyIntoJson for &T {}

pub trait IntoJsonExt: IntoJson + Sized {
    fn into_json_as<W: Default + traits::ConsumeTextChunk>(self) -> W {
        let mut w = W::default();
        let Consumed { .. } =
            self.json_provide_into(ConsumeJsonText(w.as_mut_consume_text_chunk()));
        w
    }

    fn into_json_as_try<W: Default + traits::TryConsumeTextChunk>(self) -> Result<W, W::Err> {
        let mut w = W::default();
        let Consumed { .. } =
            self.json_provide_into_try(ConsumeJsonText(w.as_mut_try_consume_text_chunk()))?;
        Ok(w)
    }

    fn into_json_as_async_try<W: Default + traits::AsyncTryConsumeTextChunk>(
        self,
    ) -> impl Future<Output = Result<W, W::Err>> {
        async {
            let mut w = W::default();
            let Consumed { .. } = self
                .json_provide_into_async_try(ConsumeJsonText(
                    w.as_mut_async_try_consume_text_chunk(),
                ))
                .await?;
            Ok(w)
        }
    }

    #[cfg(feature = "alloc")]
    fn into_json_as_string(self) -> ::alloc::string::String {
        self.into_json_as()
    }
}

pub trait ToJsonExt: ToJson {
    fn to_json_as<W: Default + traits::ConsumeTextChunk>(&self) -> W {
        <&Self as IntoJsonExt>::into_json_as(self)
    }

    fn to_json_as_try<W: Default + traits::TryConsumeTextChunk>(&self) -> Result<W, W::Err> {
        <&Self as IntoJsonExt>::into_json_as_try(self)
    }

    fn to_json_as_async_try<W: Default + traits::AsyncTryConsumeTextChunk>(
        &self,
    ) -> impl Future<Output = Result<W, W::Err>> {
        <&Self as IntoJsonExt>::into_json_as_async_try(self)
    }

    #[cfg(feature = "alloc")]
    fn to_json_as_string(&self) -> ::alloc::string::String {
        <&Self as IntoJsonExt>::into_json_as_string(self)
    }
}

impl<T: IntoJson> IntoJsonExt for T {}
impl<T: ToJson + ?Sized> ToJsonExt for T {}

mod bool;
mod int;
mod string;

mod slice;

mod tuple;

#[cfg(feature = "alloc")]
mod alloc;

pub trait ToJsonArray: ToJson<ToJsonKind = json_kinds::Array> {}
impl<T: ?Sized + ToJson<ToJsonKind = json_kinds::Array>> ToJsonArray for T {}

pub trait ToJsonObject: ToJson<ToJsonKind = json_kinds::Object> {}
impl<T: ?Sized + ToJson<ToJsonKind = json_kinds::Object>> ToJsonObject for T {}

pub trait ToJsonString: ToJson<ToJsonKind = json_kinds::JsonString> {}
impl<T: ?Sized + ToJson<ToJsonKind = json_kinds::JsonString>> ToJsonString for T {}

pub trait IntoJsonArray: IntoJson<JsonKind = json_kinds::Array> {}
impl<T: ?Sized + IntoJson<JsonKind = json_kinds::Array>> IntoJsonArray for T {}

pub trait IntoJsonObject: IntoJson<JsonKind = json_kinds::Object> {}
impl<T: ?Sized + IntoJson<JsonKind = json_kinds::Object>> IntoJsonObject for T {}

pub trait IntoJsonString: IntoJson<JsonKind = json_kinds::JsonString> {}
impl<T: ?Sized + IntoJson<JsonKind = json_kinds::JsonString>> IntoJsonString for T {}
