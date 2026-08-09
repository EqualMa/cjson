use core::marker::PhantomData;

use crate::{
    r#const::{HasConstState, IntermediateChunkAsStr, states},
    ser::IntoJson,
    utils::impl_many,
};

use super::{Consumed, json_kinds};

enum Never {}
pub struct NeverConsume<INITIAL, S: ?Sized = ()>(Never, PhantomData<(INITIAL, S)>);

impl_many!({
    {
        {
            use crate::ser::consumers::define_traits::base as trait_mod;
        }
        {
            use crate::ser::consumers::define_traits::try_ as trait_mod;
        }
        {
            use crate::ser::consumers::define_traits::async_try as trait_mod;
        }
    }

    use trait_mod::{
        CONSUME_CHAINED, CONSUME_JSON, CONSUME_JSON_CHUNKS, CONSUME_JSON_CHUNKS_FROM_INIT, Output,
        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY, READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT,
        never_future,
    };

    impl<INITIAL: CONSUME_JSON, S: ?Sized + HasConstState, K: json_kinds::ArrayOrObject>
        CONSUME_JSON_CHUNKS<K> for NeverConsume<INITIAL, S>
    {
        type InitialConsumer = INITIAL;
        type CurrentState = S;

        type ConsumeIntermediateChunk<Next: ?Sized + HasConstState> = NeverConsume<INITIAL, Next>;
        fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
            self,
            _: IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
        ) -> Output![Self::ConsumeIntermediateChunk<Next>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        fn consume_contentful_last_chunk(
            self,
            _: <K as json_kinds::ArrayOrObject>::ContentfulLastChunkAsStr<'_, Self::CurrentState>,
        ) -> Output![Consumed<K, Self::InitialConsumer>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonValue = NeverConsume<INITIAL, states::ThenValue<S>>;
        fn json_value<V: IntoJson>(self, _: V) -> Output![Self::ConsumeJsonValue, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        type ConsumeCommaJsonValue = NeverConsume<INITIAL, states::ThenCommaValue<S>>;
        fn comma_json_value<V: IntoJson>(
            self,
            _: V,
        ) -> Output![
            Self::ConsumeCommaJsonValue,
            <Self::InitialConsumer as CONSUME_JSON>::Writer
        ] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonItemsAfterArrayStartBeforeItem =
            NeverConsume<INITIAL, states::ThenItemsAfterArrayStartBeforeItem<S>>;
        fn json_items_after_array_start_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
            self,
            _: V,
        ) -> Output![
            Self::ConsumeJsonItemsAfterArrayStartBeforeItem,
            INITIAL::Writer
        ] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonItemsAfterItem = NeverConsume<INITIAL, states::ThenItemsAfterItem<S>>;
        fn json_items_after_item<V: IntoJson<JsonKind = json_kinds::Array>>(
            self,
            _: V,
        ) -> Output![Self::ConsumeJsonItemsAfterItem, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonKvsAfterObjectStartBeforeKv =
            NeverConsume<INITIAL, states::ThenKvsAfterObjectStartBeforeKv<S>>;
        fn json_kvs_after_object_start_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
            self,
            _: V,
        ) -> Output![
            Self::ConsumeJsonKvsAfterObjectStartBeforeKv,
            INITIAL::Writer
        ] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonKvsAfterFieldValue =
            NeverConsume<INITIAL, states::ThenKvsAfterFieldValue<S>>;
        fn json_kvs_after_field_value<V: IntoJson<JsonKind = json_kinds::Object>>(
            self,
            _: V,
        ) -> Output![Self::ConsumeJsonKvsAfterFieldValue, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        type ConsumeJsonStringFragment = NeverConsume<INITIAL, states::ThenStringFragment<S>>;
        fn json_string_fragment<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            self,
            _: V,
        ) -> Output![Self::ConsumeJsonStringFragment, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        #[cfg(remove)]
        type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> = Self;

        #[cfg(remove)]
        fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
            self,
        ) -> Self::ConsumeConstChunk<T> {
            never_future!(match self.0 {})
        }

        #[cfg(remove)]
        type ConsumeRuntimeChunk<C: RuntimeChunks> = Self;

        #[cfg(remove)]
        fn consume_runtime_chunk<C: RuntimeChunks>(self, _: C) -> Self::ConsumeRuntimeChunk<C> {
            never_future!(match self.0 {})
        }

        fn end_with_right_bracket(
            self,
            _: K::Contains<json_kinds::Array>,
        ) -> Output![Consumed<K, Self::InitialConsumer>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        fn end_with_right_brace(
            self,
            _: K::Contains<json_kinds::Object>,
        ) -> Output![Consumed<K, Self::InitialConsumer>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        #[cfg(remove)]
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

    impl<INITIAL: CONSUME_JSON, K: json_kinds::ArrayOrObject> CONSUME_JSON_CHUNKS_FROM_INIT<K>
        for NeverConsume<INITIAL, states::Init>
    {
        type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState> =
            NeverConsume<INITIAL, Next>;
        fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
            self,
            _: <K as json_kinds::ArrayOrObject>::ContentfulFirstChunkAsStr<'_, Next>,
        ) -> Output![Self::ConsumeContentfulFirstChunk<Next>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        fn consume_contentful_full_chunk(
            self,
            _: <K as json_kinds::ArrayOrObject>::ContentfulFullChunkAsAtr<'_>,
        ) -> Output![Consumed<K, Self::InitialConsumer>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }
    }

    impl<INITIAL: CONSUME_JSON> READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY
        for NeverConsume<INITIAL, states::Init>
    {
        type LeftBracketValue = NeverConsume<INITIAL, states::LeftBracketValue>;

        fn left_bracket_value<V: IntoJson>(
            self,
            _: V,
        ) -> Output![
            Self::LeftBracketValue,
            <Self::InitialConsumer as CONSUME_JSON>::Writer
        ] {
            never_future!(match self.0 {})
        }

        type LeftBracketItemsBeforeItem = NeverConsume<INITIAL, states::LeftBracketItemsBeforeItem>;

        fn left_bracket_items_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
            self,
            _: V,
        ) -> Output![
            Self::LeftBracketItemsBeforeItem,
            <Self::InitialConsumer as CONSUME_JSON>::Writer
        ] {
            never_future!(match self.0 {})
        }
    }

    impl<INITIAL: CONSUME_JSON> READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT
        for NeverConsume<INITIAL, states::Init>
    {
        type LeftBraceKvsBeforeKv = NeverConsume<INITIAL, states::LeftBraceKvsBeforeKv>;

        fn left_brace_kvs_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
            self,
            _: V,
        ) -> Output![
            Self::LeftBraceKvsBeforeKv,
            <Self::InitialConsumer as CONSUME_JSON>::Writer
        ] {
            never_future!(match self.0 {})
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

    impl<INITIAL: CONSUME_JSON, K: json_kinds::ChainableJsonKind> CONSUME_CHAINED<K>
        for NeverConsume<INITIAL>
    {
        fn extend<V: IntoJson<JsonKind = K>>(&mut self, _: V) -> Output![(), INITIAL::Writer] {
            never_future!(match self.0 {})
        }

        type InitialConsumer = INITIAL;
        fn end_with<V: IntoJson<JsonKind = K>>(
            self,
            _: V,
        ) -> Output![Consumed<K, Self::InitialConsumer>, INITIAL::Writer] {
            never_future!(match self.0 {})
        }
    }
});
