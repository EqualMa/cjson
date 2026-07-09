use core::marker::PhantomData;

use crate::{
    r#const::{HasConstCompileTimeChunk, HasConstState, IntermediateChunkAsStr},
    ser::IntoJson,
};

use super::{
    ConsumeChainedArrays, ConsumeChainedStrings, ConsumeJson, ConsumeJsonChunks, Consumed,
    json_kinds::{self, JsonKind},
    runtime_chunks::RuntimeChunks,
};

enum Never {}
pub struct NeverConsume<INITIAL: ConsumeJson>(Never, PhantomData<INITIAL>);

impl<INITIAL: ConsumeJson, K: json_kinds::ArrayOrObject> ConsumeJsonChunks<K>
    for NeverConsume<INITIAL>
{
    type InitialConsumer = INITIAL;
    type CurrentState = crate::r#const::states::Init;

    type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState> = Self;
    fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
        self,
        _: <K as json_kinds::ArrayOrObject>::ContentfulFirstChunkAsStr<'_, Next>,
    ) -> Self::ConsumeContentfulFirstChunk<Next> {
        match self.0 {}
    }

    type ConsumeIntermediateChunk<Next: ?Sized + HasConstState> = Self;
    fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
        self,
        _: IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
    ) -> Self::ConsumeIntermediateChunk<Next> {
        match self.0 {}
    }

    fn consume_contentful_last_chunk(
        self,
        _: <K as json_kinds::ArrayOrObject>::ContentfulLastChunkAsStr<'_, Self::CurrentState>,
    ) -> Consumed<K, Self::InitialConsumer> {
        match self.0 {}
    }

    fn consume_contentful_full_chunk(
        self,
        _: <K as json_kinds::ArrayOrObject>::ContentfulFullChunkAsAtr<'_>,
    ) -> Consumed<K, Self::InitialConsumer> {
        match self.0 {}
    }

    type ConsumeJsonValue = Self;
    fn json_value(self, _: impl IntoJson) -> Self::ConsumeJsonValue {
        match self.0 {}
    }

    type ConsumeJsonItemsAfterArrayStartBeforeItem = Self;
    fn json_items_after_array_start_before_item(
        self,
        _: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::ConsumeJsonItemsAfterArrayStartBeforeItem {
        match self.0 {}
    }

    type ConsumeJsonItemsAfterItem = Self;
    fn json_items_after_item(
        self,
        _: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::ConsumeJsonItemsAfterItem {
        match self.0 {}
    }

    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> = Self;

    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        match self.0 {}
    }

    type ConsumeRuntimeChunk<C: RuntimeChunks> = Self;

    fn consume_runtime_chunk<C: RuntimeChunks>(self, _: C) -> Self::ConsumeRuntimeChunk<C> {
        match self.0 {}
    }

    fn end(self) -> Consumed<K, Self::InitialConsumer> {
        match self.0 {}
    }

    #[cfg(todo)]
    type ConsumeOpenContentBeforeContent = Self;

    #[cfg(todo)]
    fn consume_open_content_before_content(
        self,
        _: impl IntoJson<JsonKind = K>,
        _: <K as JsonKind>::ArrayOrObjectContainsSelf,
    ) -> Self::ConsumeOpenContentBeforeContent {
        match self {}
    }
}

#[cfg(todo)]
impl<INITIAL: ConsumeJson, K: JsonKind> ConsumeOpenContentBeforeContent<K>
    for NeverConsume<INITIAL>
{
    type InitialConsumer = INITIAL;

    fn extend(&mut self, _: impl IntoJson<JsonKind = K>) {
        match self.0 {}
    }

    type End<const PREV_STATE: u128, const NEXT_STATE: u128> = Self;

    fn end<const PREV_STATE: u128, const NEXT_STATE: u128>(
        self,
        _: IntermediateChunkAsStr<'_, PREV_STATE, NEXT_STATE>,
    ) -> Self::End<PREV_STATE, NEXT_STATE> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> ConsumeChainedArrays for NeverConsume<INITIAL> {
    fn extend(&mut self, _: impl super::IntoJson<JsonKind = json_kinds::Array>) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;
    fn end_with(
        self,
        _: impl super::IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> ConsumeChainedStrings for NeverConsume<INITIAL> {
    fn extend(&mut self, _: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;

    fn end_with(
        self,
        _: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        match self.0 {}
    }
}
