use core::marker::PhantomData;

use crate::{
    r#const::{HasConstCompileTimeChunk, HasConstState, IntermediateChunkAsStr, states},
    ser::IntoJson,
};

use super::{
    ConsumeChainedArrays, ConsumeChainedObjects, ConsumeChainedStrings, ConsumeJson,
    ConsumeJsonChunks, ConsumeJsonChunksFromInit, Consumed,
    json_kinds::{self, JsonKind},
    runtime_chunks::RuntimeChunks,
};

enum Never {}
pub struct NeverConsume<INITIAL: ConsumeJson, S: ?Sized = ()>(Never, PhantomData<(INITIAL, S)>);

impl<INITIAL: ConsumeJson, S: ?Sized + HasConstState, K: json_kinds::ArrayOrObject>
    ConsumeJsonChunks<K> for NeverConsume<INITIAL, S>
{
    type InitialConsumer = INITIAL;
    type CurrentState = S;

    type ConsumeIntermediateChunk<Next: ?Sized + HasConstState> = NeverConsume<INITIAL, Next>;
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

    type ConsumeJsonValue = NeverConsume<INITIAL, states::ThenValue<S>>;
    fn json_value<V: IntoJson>(self, _: V) -> Self::ConsumeJsonValue {
        match self.0 {}
    }

    type ConsumeJsonItemsAfterArrayStartBeforeItem =
        NeverConsume<INITIAL, states::ThenItemsAfterArrayStartBeforeItem<S>>;
    fn json_items_after_array_start_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        _: V,
    ) -> Self::ConsumeJsonItemsAfterArrayStartBeforeItem {
        match self.0 {}
    }

    type ConsumeJsonItemsAfterItem = NeverConsume<INITIAL, states::ThenItemsAfterItem<S>>;
    fn json_items_after_item<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        _: V,
    ) -> Self::ConsumeJsonItemsAfterItem {
        match self.0 {}
    }

    type ConsumeJsonKvsAfterFieldValue = NeverConsume<INITIAL, states::ThenKvsAfterFieldValue<S>>;
    fn json_kvs_after_field_value<V: IntoJson<JsonKind = json_kinds::Object>>(
        self,
        _: V,
    ) -> Self::ConsumeJsonKvsAfterFieldValue {
        match self.0 {}
    }

    type ConsumeJsonStringFragment = NeverConsume<INITIAL, states::ThenStringFragment<S>>;
    fn json_string_fragment<V: IntoJson<JsonKind = json_kinds::JsonString>>(
        self,
        _: V,
    ) -> Self::ConsumeJsonStringFragment {
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

    fn end_with_right_bracket(
        self,
        _: K::Contains<json_kinds::Array>,
    ) -> Consumed<K, Self::InitialConsumer> {
        match self.0 {}
    }

    fn end_with_right_brace(
        self,
        _: K::Contains<json_kinds::Object>,
    ) -> Consumed<K, Self::InitialConsumer> {
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

impl<INITIAL: ConsumeJson, K: json_kinds::ArrayOrObject> ConsumeJsonChunksFromInit<K>
    for NeverConsume<INITIAL, states::Init>
{
    type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState> = NeverConsume<INITIAL, Next>;
    fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
        self,
        _: <K as json_kinds::ArrayOrObject>::ContentfulFirstChunkAsStr<'_, Next>,
    ) -> Self::ConsumeContentfulFirstChunk<Next> {
        match self.0 {}
    }

    fn consume_contentful_full_chunk(
        self,
        _: <K as json_kinds::ArrayOrObject>::ContentfulFullChunkAsAtr<'_>,
    ) -> Consumed<K, Self::InitialConsumer> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> super::chunks::ReadyToConsumeJsonChunksOfNonEmptyArray
    for NeverConsume<INITIAL, states::Init>
{
    type LeftBracketValue = NeverConsume<INITIAL, states::LeftBracketValue>;

    fn left_bracket_value(self, _: impl IntoJson) -> Self::LeftBracketValue {
        match self.0 {}
    }

    type LeftBracketItemsBeforeItem = NeverConsume<INITIAL, states::LeftBracketItemsBeforeItem>;

    fn left_bracket_items_before_item(
        self,
        _: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::LeftBracketItemsBeforeItem {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> super::chunks::ReadyToConsumeJsonChunksOfNonEmptyObject
    for NeverConsume<INITIAL, states::Init>
{
    type LeftBraceKvsBeforeKv = NeverConsume<INITIAL, states::LeftBraceKvsBeforeKv>;

    fn left_brace_kvs_before_kv(
        self,
        _: impl IntoJson<JsonKind = json_kinds::Object>,
    ) -> Self::LeftBraceKvsBeforeKv {
        match self.0 {}
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
    fn extend<V: IntoJson<JsonKind = json_kinds::Array>>(&mut self, _: V) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;
    fn end_with<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        _: V,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> ConsumeChainedObjects for NeverConsume<INITIAL> {
    fn extend<V: IntoJson<JsonKind = json_kinds::Object>>(&mut self, _: V) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;
    fn end_with<V: IntoJson<JsonKind = json_kinds::Object>>(
        self,
        _: V,
    ) -> Consumed<json_kinds::Object, Self::InitialConsumer> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> ConsumeChainedStrings for NeverConsume<INITIAL> {
    fn extend<V: IntoJson<JsonKind = json_kinds::JsonString>>(&mut self, _: V) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;

    fn end_with<V: IntoJson<JsonKind = json_kinds::JsonString>>(
        self,
        _: V,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        match self.0 {}
    }
}
