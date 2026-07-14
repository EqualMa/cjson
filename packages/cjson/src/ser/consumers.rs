use core::marker::PhantomData;

use crate::{
    r#const::{HasConstCompileTimeChunk, HasConstState, State, states},
    ser::{
        IntoJson,
        consumers::{
            consume_chained_full::ConsumeChainedStringsFull, runtime_chunks::RuntimeChunks,
        },
        into_json_key_colon_value::Sealed as _,
        texts,
        traits::{self, ConsumeTextChunk, IntoTextChunks as _},
    },
    utils::impl_many,
};

use self::{
    consume_chained_full::ConsumeChainedArraysFull,
    consume_open_content::ConsumeArrayOpenItemsIfNotEmpty,
    json_kinds::JsonKind,
    never_consume::NeverConsume,
    open_close::{GroupOrComma, MakeChunks, OpenClose},
};

pub use self::consumed::Consumed;

macro_rules! not_any_value {
    () => {
        fn consume_any_value(
            self,
            _: crate::ser::texts::Value<impl crate::ser::traits::IntoTextChunks>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::AnyValue,
            >,
        ) -> Consumed<json_kinds::AnyValue, Self> {
            match yes {}
        }
    };
}

macro_rules! not_string {
    () => {
        fn consume_empty_string(
            self,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Consumed<json_kinds::JsonString, Self> {
            match yes {}
        }

        fn consume_json_string_as_str(
            self,
            _: crate::r#const::JsonStringAsStr<'_>,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Consumed<json_kinds::JsonString, Self> {
            match yes {}
        }

        fn consume_str(
            self,
            _: &str,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Consumed<json_kinds::JsonString, Self> {
            match yes {}
        }

        type EndJsonString = crate::ser::consumers::json_string_chunks::NeverEndJsonString;
        fn start_to_consume_chunks_of_json_string_with_first_chunk(
            self,
            _: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> crate::ser::consumers::json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
            match yes {}
        }
        fn start_to_consume_chunks_of_json_string(
            self,
            _: impl IntoJson<JsonKind = json_kinds::JsonString>,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> crate::ser::consumers::json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
            match yes {}
        }

        type ConsumeChainedStrings = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_strings(
            self,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Self::ConsumeChainedStrings {
            match yes {}
        }
    };
}

macro_rules! not_array {
    () => {
        fn consume_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Consumed<crate::ser::json_kinds::Array, Self> {
            match yes {}
        }
        fn consume_non_empty_array_as_str(
            self,
            _: crate::r#const::NonEmptyArrayAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Consumed<crate::ser::json_kinds::Array, Self> {
            match yes {}
        }

        type ConsumeChunksOfNonEmptyArray =
            crate::ser::consumers::never_consume::NeverConsume<Self, crate::r#const::states::Init>;

        fn start_to_consume_chunks_of_non_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            match yes {}
        }

        type ConsumeChainedArrays = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_arrays(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Self::ConsumeChainedArrays {
            match yes {}
        }

        fn consume_array_of_items(
            self,
            _: impl IntoIterator<Item: crate::ser::IntoJson>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Consumed<crate::ser::json_kinds::Array, Self> {
            match yes {}
        }
    };
}

macro_rules! not_object {
    () => {
        fn consume_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Consumed<json_kinds::Object, Self> {
            match yes {}
        }
        fn consume_non_empty_object_as_str(
            self,
            _: crate::r#const::NonEmptyObjectAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Consumed<json_kinds::Object, Self> {
            match yes {}
        }

        type ConsumeChunksOfNonEmptyObject =
            crate::ser::consumers::never_consume::NeverConsume<Self, crate::r#const::states::Init>;
        fn start_to_consume_chunks_of_non_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            match yes {}
        }

        type ConsumeChainedObjects = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_objects(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Self::ConsumeChainedObjects {
            match yes {}
        }

        fn consume_object_of_iter(
            self,
            _: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Object,
            >,
        ) -> Consumed<json_kinds::Object, Self> {
            match yes {}
        }
    };
}

// TODO: refactor these methods out of trait
macro_rules! consume_fragment {
    ($writer:tt) => {
        fn consume_fragment_as_str(&mut self, v: crate::r#const::JsonStringFragmentAsStr<'_>) {
            self.$writer.consume_text_chunk(v.as_str())
        }

        fn consume_fragment(
            &mut self,
            v: impl crate::ser::IntoJson<JsonKind = json_kinds::JsonString>,
        ) {
            let crate::ser::Consumed { .. } = v.json_provide_into(
                crate::ser::consumers::consume_content::ConsumeStringFragment(
                    self.$writer.as_mut_consume_text_chunk(),
                ),
            );
        }
    };
}

pub mod json_kinds;
pub mod runtime_chunks;

pub(super) mod chunks;
mod consume_chained_content;
mod consume_chained_full;
mod consume_comma_content_close;
mod consume_content;
mod consume_content_and_record;
mod consume_content_close;
mod consume_open_content;
mod consume_open_content_comma;
mod consumed;
pub(super) mod json_string_chunks;
mod never_consume;

// TODO: seal
pub trait ConsumeJson {
    type ConsumeJsonKind: JsonKind;
    type Writer: ConsumeTextChunk;

    fn consume_any_value(
        self,
        value: texts::Value<impl traits::IntoTextChunks>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::AnyValue>,
    ) -> Consumed<json_kinds::AnyValue, Self>;

    fn consume_empty_string(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self>;
    fn consume_json_string_as_str(
        self,
        v: crate::r#const::JsonStringAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self>;

    /// Consume a json string from `&str`.
    fn consume_str(
        self,
        s: &str,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self>;

    type EndJsonString: json_string_chunks::EndJsonString;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self>;
    fn start_to_consume_chunks_of_json_string(
        self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self>;

    type ConsumeChainedStrings: ConsumeChainedStrings<InitialConsumer = Self>;
    fn start_to_consume_chained_strings(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings;

    fn consume_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self>;
    fn consume_non_empty_array_as_str(
        self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self>;

    type ConsumeChunksOfNonEmptyArray: chunks::ReadyToConsumeJsonChunksOfNonEmptyArray<InitialConsumer = Self>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray;

    type ConsumeChainedArrays: ConsumeChainedArrays<InitialConsumer = Self>;

    fn start_to_consume_chained_arrays(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays;

    fn consume_array_of_items(
        self,
        items: impl IntoIterator<Item: IntoJson>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self>;

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self>;
    fn consume_non_empty_object_as_str(
        self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self>;

    type ConsumeChunksOfNonEmptyObject: chunks::ReadyToConsumeJsonChunksOfNonEmptyObject;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject;

    type ConsumeChainedObjects: ConsumeChainedObjects<InitialConsumer = Self>;
    fn start_to_consume_chained_objects(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects;

    fn consume_object_of_iter(
        self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self>;
}

pub trait ConsumeChainedStrings {
    fn extend(&mut self, s: impl IntoJson<JsonKind = json_kinds::JsonString>);

    type InitialConsumer: ?Sized;
    fn end_with(
        self,
        s: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer>;
}

pub trait ConsumeChainedArrays {
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>);

    type InitialConsumer: ?Sized;
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer>;
}

pub trait ConsumeChainedObjects {
    fn extend(&mut self, obj: impl IntoJson<JsonKind = json_kinds::Object>);

    type InitialConsumer: ?Sized;
    fn end_with(
        self,
        obj: impl IntoJson<JsonKind = json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self::InitialConsumer>;
}

pub trait ConsumeJsonChunks<K: json_kinds::ArrayOrObject> {
    type InitialConsumer;
    type CurrentState: ?Sized + HasConstState;

    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk>: ConsumeJsonChunks<K>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T>;

    type ConsumeRuntimeChunk<C: RuntimeChunks>: ConsumeJsonChunks<K>;
    fn consume_runtime_chunk<C: RuntimeChunks>(self, chunk: C) -> Self::ConsumeRuntimeChunk<C>;

    type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState>: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
        self,
        v: K::ContentfulFirstChunkAsStr<'_, Next>,
    ) -> Self::ConsumeContentfulFirstChunk<Next>;

    type ConsumeIntermediateChunk<Next: ?Sized + HasConstState>: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
        self,
        v: crate::r#const::IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
    ) -> Self::ConsumeIntermediateChunk<Next>;

    fn consume_contentful_last_chunk(
        self,
        v: K::ContentfulLastChunkAsStr<'_, Self::CurrentState>,
    ) -> Consumed<K, Self::InitialConsumer>;

    fn consume_contentful_full_chunk(
        self,
        v: K::ContentfulFullChunkAsAtr<'_>,
    ) -> Consumed<K, Self::InitialConsumer>;

    type ConsumeJsonValue: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn json_value(self, v: impl IntoJson) -> Self::ConsumeJsonValue;

    type ConsumeJsonItemsAfterArrayStartBeforeItem: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn json_items_after_array_start_before_item(
        self,
        v: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::ConsumeJsonItemsAfterArrayStartBeforeItem;

    type ConsumeJsonItemsAfterItem: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn json_items_after_item(
        self,
        v: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::ConsumeJsonItemsAfterItem;

    type ConsumeJsonKvsAfterFieldValue: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn json_kvs_after_field_value(
        self,
        v: impl IntoJson<JsonKind = json_kinds::Object>,
    ) -> Self::ConsumeJsonKvsAfterFieldValue;

    type ConsumeJsonStringFragment: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn json_string_fragment(
        self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Self::ConsumeJsonStringFragment;

    #[cfg(remove)]
    type ConsumeOpenContentBeforeContent: ConsumeOpenContentBeforeContent<K, InitialConsumer = Self::InitialConsumer>;
    #[cfg(remove)]
    fn consume_open_content_before_content(
        self,
        content: impl IntoJson<JsonKind = K>,
        yes: K::ArrayOrObjectContainsSelf,
    ) -> Self::ConsumeOpenContentBeforeContent;

    fn end_with_right_bracket(
        self,
        yes: K::Contains<json_kinds::Array>,
    ) -> Consumed<K, Self::InitialConsumer>;
    fn end_with_right_brace(
        self,
        yes: K::Contains<json_kinds::Object>,
    ) -> Consumed<K, Self::InitialConsumer>;
    fn end(self) -> Consumed<K, Self::InitialConsumer>;
}

pub trait ConsumeArrayOfItems {
    type InitialConsumer;
    fn consume_non_first_item(&mut self, item: impl IntoJson);

    fn end(self) -> Consumed<json_kinds::Array, Self::InitialConsumer>;
}

#[cfg(remove)]
pub trait ConsumeOpenContentBeforeContent<K: JsonKind> {
    type InitialConsumer;

    fn extend(&mut self, content: impl IntoJson<JsonKind = K>);

    type End<const PREV_STATE: u128, const NEXT_STATE: u128>: ConsumeJsonChunks<K, InitialConsumer = Self::InitialConsumer>;
    fn end<const PREV_STATE: u128, const NEXT_STATE: u128>(
        self,
        v: crate::r#const::IntermediateChunkAsStr<'_, PREV_STATE, NEXT_STATE>,
    ) -> Self::End<PREV_STATE, NEXT_STATE>;
}

pub struct ConsumeJsonText<W: ConsumeTextChunk>(pub W);

// TODO: remove
#[cfg(todo)]
pub struct ConsumeChunksOfJsonArray<W: ConsumeTextChunk, S: ?Sized + HasConstState>(
    W,
    PhantomData<S>,
);
#[cfg(todo)]
pub struct ConsumeChunksOfJsonObject<W: ConsumeTextChunk, S: ?Sized + HasConstState>(
    W,
    PhantomData<S>,
);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeJsonText<W> {
    type ConsumeJsonKind = json_kinds::AnyValue;
    type Writer = W;

    fn consume_any_value(
        mut self,
        value: texts::Value<impl traits::IntoTextChunks>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::AnyValue>,
    ) -> Consumed<json_kinds::AnyValue, Self> {
        value.into_inner().write_into(&mut self.0);
        Consumed::ASSERT_ANY_VALUE
    }

    fn consume_empty_string(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk("\"\"");
        Consumed::ASSERT_STRING
    }

    fn consume_json_string_as_str(
        mut self,
        v: crate::r#const::JsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk(v.as_str());
        Consumed::ASSERT_STRING
    }

    fn consume_str(
        mut self,
        s: &str,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        if s.is_empty() {
            self.consume_empty_string(())
        } else {
            self.0.consume_text_chunk("\"");
            super::texts::StrToJsonStringFragment(s).write_into(&mut self.0);
            self.0.consume_text_chunk("\"");
            Consumed::ASSERT_STRING
        }
    }

    type EndJsonString = json_string_chunks::EndJsonStringWithClose;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        mut self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        self.0.consume_text_chunk(v.as_str());
        json_string_chunks::ConsumeInJsonString::new(self.0)
    }
    fn start_to_consume_chunks_of_json_string(
        mut self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        self.0.consume_text_chunk("\""); // TODO: optimize with ConsumeJsonStringOpenFragment
        let Consumed { .. } = v.json_provide_into(consume_content::ConsumeStringFragment(
            self.0.as_mut_consume_text_chunk(),
        ));
        json_string_chunks::ConsumeInJsonString::new(self.0)
    }

    type ConsumeChainedStrings = ConsumeChainedStringsFull<W>;
    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        ConsumeChainedStringsFull::new(self.0)
    }

    fn consume_empty_array(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk("[]");
        Consumed::ASSERT_ARRAY
    }
    fn consume_non_empty_array_as_str(
        mut self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk(v.as_str());
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    type ConsumeChainedArrays = ConsumeChainedArraysFull<W>;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        ConsumeChainedArraysFull::new(self.0)
    }

    fn consume_array_of_items(
        mut self,
        items: impl IntoIterator<Item: IntoJson>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        let mut items = items.into_iter();
        let Some(first) = items.next() else {
            return self.consume_empty_array(());
        };
        self.0.consume_text_chunk("[");
        first.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
        items.for_each(|item| {
            self.0.consume_text_chunk(",");
            item.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
        });
        self.0.consume_text_chunk("]");
        Consumed::ASSERT_ARRAY
    }

    fn consume_empty_object(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk("{}");
        Consumed::ASSERT_OBJECT
    }

    fn consume_non_empty_object_as_str(
        mut self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk(v.as_str());
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject =
        ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;

    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
    }

    type ConsumeChainedObjects = consume_chained_full::ConsumeChainedObjectsFull<W>;
    fn start_to_consume_chained_objects(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects {
        consume_chained_full::ConsumeChainedObjectsFull::new(self.0)
    }

    fn consume_object_of_iter(
        mut self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        let mut items = kvs.into_iter();
        let Some(first) = items.next() else {
            return self.consume_empty_object(());
        };
        self.0.consume_text_chunk("{\"");
        write_key_frag_quote_colon_value(&mut self.0, first);

        items.for_each(|kv| {
            self.0.consume_text_chunk(",\"");
            write_key_frag_quote_colon_value(&mut self.0, kv);
        });
        self.0.consume_text_chunk("}");
        Consumed::ASSERT_OBJECT
    }
}

#[cfg(todo)]
impl<W: ConsumeTextChunk, S: ?Sized + HasConstState> ConsumeJsonChunks<json_kinds::Array>
    for ConsumeChunksOfJsonArray<W, S>
{
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
        ConsumeChunksOfJsonArray<W, states::NextStateOf<T>>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        const {
            S::STATE.assert_same(T::CHUNK.prev_state());

            if S::STATE.is_init() {
                T::CHUNK
                    .remove_group_open()
                    .prev_state()
                    .assert_same(&State::INIT_AFTER_ARRAY_START)
            }
        }
        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    type ConsumeRuntimeChunk<C: RuntimeChunks> = ConsumeChunksOfJsonArray<W, C::NextState<S>>;
    fn consume_runtime_chunk<C: RuntimeChunks>(mut self, chunk: C) -> Self::ConsumeRuntimeChunk<C> {
        const { _ = C::NextState::<S>::STATE }
        chunk.runtime_chunks_write_into(&mut self.0);
        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    type ConsumeOpenContentBeforeContent =
        ConsumeArrayOpenContentComma<W, { OpenClose::BOTH_GROUP.as_u8() }>;

    fn consume_open_content_before_content(
        self,
        content: impl IntoJson<JsonKind = json_kinds::Array>,
        (): <json_kinds::Array as JsonKind>::ArrayOrObjectContainsSelf,
    ) {
        ConsumeArrayOpenContentComma::new(self.0, content)
    }

    fn end(self) -> Consumed<json_kinds::Array, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed::ASSERT_ARRAY
    }
}

#[cfg(todo)]
impl<W: ConsumeTextChunk, S: ?Sized + HasConstState> ConsumeJsonChunks<json_kinds::Object>
    for ConsumeChunksOfJsonObject<W, S>
{
    type InitialConsumer = _;
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
        ConsumeChunksOfJsonObject<W, states::NextStateOf<T>>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        const {
            S::STATE.assert_same(T::CHUNK.prev_state());

            if S::STATE.is_init() {
                T::CHUNK
                    .remove_group_open()
                    .prev_state()
                    .assert_same(&State::INIT_AFTER_OBJECT_START)
            }
        }
        ConsumeChunksOfJsonObject(self.0, PhantomData)
    }

    type ConsumeRuntimeChunk<C: RuntimeChunks> = ConsumeChunksOfJsonObject<W, C::NextState<S>>;
    fn consume_runtime_chunk<C: RuntimeChunks>(mut self, chunk: C) -> Self::ConsumeRuntimeChunk<C> {
        const { _ = C::NextState::<S>::STATE }
        chunk.runtime_chunks_write_into(&mut self.0);
        ConsumeChunksOfJsonObject(self.0, PhantomData)
    }

    type ConsumeOpenContentBeforeContent = NeverConsume; // TODO:
    fn consume_open_content_before_content(
        self,
        content: impl IntoJson<JsonKind = json_kinds::Object>,
        yes: <json_kinds::Object as JsonKind>::ArrayOrObjectContainsSelf,
    ) -> Self::ConsumeOpenContentBeforeContent {
    }

    fn end(self) -> Consumed<json_kinds::Object, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed::ASSERT_OBJECT
    }
}

mod open_close;

pub struct ConsumeChunksOfNonEmptyArray<
    W: ConsumeTextChunk,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<(InitialConsumer, S)>);
pub struct ConsumeChunksOfNonEmptyObject<
    W: ConsumeTextChunk,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<(InitialConsumer, S)>);

impl_many!({
    {
        {
            use ConsumeChunksOfNonEmptyArray as CONSUME;
            use consume_open_content_comma::ConsumeArrayOpenContentComma as TConsumeOpenContentBeforeContent;
            use json_kinds::Array as K;

            const fn assert_consumed<W: ?Sized>() -> Consumed<K, W> {
                Consumed::ASSERT_ARRAY
            }

            const fn assert_start(state: State) {
                state.assert_is_top_level_after_array_start()
            }
            const fn assert_end(state: State) {
                state.assert_is_before_top_level_right_bracket()
            }
        }
        {
            use ConsumeChunksOfNonEmptyObject as CONSUME;
            use consume_open_content_comma::ConsumeObjectOpenContentComma as TConsumeOpenContentBeforeContent;
            use json_kinds::Object as K;

            const fn assert_consumed<W: ?Sized>() -> Consumed<K, W> {
                Consumed::ASSERT_OBJECT
            }

            const fn assert_start(state: State) {
                state.assert_is_top_level_after_object_start()
            }
            const fn assert_end(state: State) {
                state.assert_is_before_top_level_right_brace()
            }
        }
    }

    impl<W: ConsumeTextChunk, InitialConsumer, S: ?Sized + HasConstState, const OC: u8>
        ConsumeJsonChunks<K> for CONSUME<W, InitialConsumer, S, OC>
    {
        type InitialConsumer = InitialConsumer;
        type CurrentState = S;

        type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState> =
            CONSUME<W, InitialConsumer, Next, OC>;
        fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
            mut self,
            v: <K as json_kinds::ArrayOrObject>::ContentfulFirstChunkAsStr<'_, Next>,
        ) -> Self::ConsumeContentfulFirstChunk<Next> {
            const {
                // make sure current state is Init
                S::STATE.assert_init();
                assert!(!Next::STATE.is_eof());
            }

            match const { OpenClose::try_from_u8(OC).unwrap().open } {
                open_close::GroupOrComma::Nothing => {
                    self.0.consume_text_chunk(v.remove_group_open())
                }
                open_close::GroupOrComma::Group => self.0.consume_text_chunk(v.as_str()),
                open_close::GroupOrComma::Comma => {
                    self.0.consume_2_text_chunks(",", v.remove_group_open())
                }
            }

            CONSUME(self.0, PhantomData)
        }

        type ConsumeIntermediateChunk<Next: ?Sized + HasConstState> =
            CONSUME<W, InitialConsumer, Next, OC>;
        fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
            mut self,
            chunk: crate::r#const::IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
        ) -> Self::ConsumeIntermediateChunk<Next> {
            const {
                assert!(!S::STATE.is_init());
                assert!(!Next::STATE.is_eof());
            }

            self.0.consume_text_chunk(chunk.as_str());

            CONSUME(self.0, PhantomData)
        }

        fn consume_contentful_last_chunk(
            mut self,
            v: <K as json_kinds::ArrayOrObject>::ContentfulLastChunkAsStr<'_, Self::CurrentState>,
        ) -> Consumed<K, Self::InitialConsumer> {
            const {
                assert!(!S::STATE.is_init());
            }

            match const { OpenClose::try_from_u8(OC).unwrap().close } {
                open_close::GroupOrComma::Nothing => {
                    self.0.consume_text_chunk(v.remove_group_close())
                }
                open_close::GroupOrComma::Group => self.0.consume_text_chunk(v.as_str()),
                open_close::GroupOrComma::Comma => {
                    self.0.consume_2_text_chunks(v.remove_group_close(), ",")
                }
            }

            const { assert_consumed() }
        }

        fn consume_contentful_full_chunk(
            mut self,
            v: <K as json_kinds::ArrayOrObject>::ContentfulFullChunkAsAtr<'_>,
        ) -> Consumed<K, Self::InitialConsumer> {
            match const { OpenClose::try_from_u8(OC).unwrap().into_tuple() } {
                (GroupOrComma::Nothing, GroupOrComma::Nothing) => {
                    self.0.consume_text_chunk(v.remove_surrounding_group())
                }
                (GroupOrComma::Nothing, GroupOrComma::Group) => {
                    self.0.consume_text_chunk(v.remove_group_open())
                }
                (GroupOrComma::Nothing, GroupOrComma::Comma) => {
                    self.0.consume_2_text_chunks(v.remove_group_open(), ",")
                }
                (GroupOrComma::Group, GroupOrComma::Nothing) => {
                    self.0.consume_text_chunk(v.remove_group_close())
                }
                (GroupOrComma::Group, GroupOrComma::Group) => self.0.consume_text_chunk(v.as_str()),
                (GroupOrComma::Group, GroupOrComma::Comma) => {
                    self.0.consume_2_text_chunks(v.remove_group_close(), ",")
                }
                (GroupOrComma::Comma, GroupOrComma::Nothing) => {
                    self.0.consume_2_text_chunks(",", v.remove_group_close())
                }
                (GroupOrComma::Comma, GroupOrComma::Group) => {
                    self.0.consume_2_text_chunks(",", v.remove_group_open())
                }
                (GroupOrComma::Comma, GroupOrComma::Comma) => {
                    // TODO: consume_3_text_chunks
                    // actually this branch should not be used
                    self.0.consume_text_chunk(",");
                    self.0.consume_text_chunk(v.remove_surrounding_group());
                    self.0.consume_text_chunk(",");
                }
            }
            const { assert_consumed() }
        }

        type ConsumeJsonValue = CONSUME<W, InitialConsumer, states::ThenValue<S>, OC>;
        fn json_value(mut self, v: impl IntoJson) -> Self::ConsumeJsonValue {
            const { _ = states::ThenValue::<S>::STATE }
            let Consumed { .. } =
                v.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
            CONSUME(self.0, PhantomData)
        }

        type ConsumeJsonItemsAfterArrayStartBeforeItem =
            CONSUME<W, InitialConsumer, states::ThenItemsAfterArrayStartBeforeItem<S>, OC>;
        fn json_items_after_array_start_before_item(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::Array>,
        ) -> Self::ConsumeJsonItemsAfterArrayStartBeforeItem {
            const { _ = states::ThenItemsAfterArrayStartBeforeItem::<S>::STATE }
            let Consumed { .. } = v.json_provide_into(ConsumeArrayItemsAppendCommaIfNotEmpty(
                self.0.as_mut_consume_text_chunk(),
            ));
            CONSUME(self.0, PhantomData)
        }

        type ConsumeJsonItemsAfterItem =
            CONSUME<W, InitialConsumer, states::ThenItemsAfterItem<S>, OC>;
        fn json_items_after_item(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::Array>,
        ) -> Self::ConsumeJsonItemsAfterItem {
            const { _ = states::ThenItemsAfterItem::<S>::STATE }
            let Consumed { .. } = v.json_provide_into(ConsumeArrayItemsPrependCommaIfNotEmpty(
                self.0.as_mut_consume_text_chunk(),
            ));
            CONSUME(self.0, PhantomData)
        }

        type ConsumeJsonKvsAfterFieldValue =
            CONSUME<W, InitialConsumer, states::ThenKvsAfterFieldValue<S>, OC>;
        fn json_kvs_after_field_value(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::Object>,
        ) -> Self::ConsumeJsonKvsAfterFieldValue {
            const { _ = states::ThenKvsAfterFieldValue::<S>::STATE }
            let Consumed { .. } = v.json_provide_into(ConsumeObjectKvsPrependCommaIfNotEmpty(
                self.0.as_mut_consume_text_chunk(),
            ));
            CONSUME(self.0, PhantomData)
        }

        type ConsumeJsonStringFragment =
            CONSUME<W, InitialConsumer, states::ThenStringFragment<S>, OC>;
        fn json_string_fragment(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Self::ConsumeJsonStringFragment {
            const { _ = states::ThenStringFragment::<S>::STATE }
            let Consumed { .. } = v.json_provide_into(consume_content::ConsumeStringFragment(
                self.0.as_mut_consume_text_chunk(),
            ));
            CONSUME(self.0, PhantomData)
        }

        type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
            CONSUME<W, InitialConsumer, states::NextStateOf<T>, OC>;
        fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
            mut self,
        ) -> Self::ConsumeConstChunk<T> {
            const {
                S::STATE.assert_same(T::CHUNK.prev_state());

                if T::CHUNK.prev_state().is_init() {
                    assert_start(T::CHUNK.remove_group_open().into_prev_state());
                }

                if T::CHUNK.into_next_state().is_eof() {
                    assert_end(T::CHUNK.remove_group_close().into_next_state());
                }
            }

            if const { <T as MakeChunks<OC>>::MADE_CHUNKS.prepend_comma } {
                self.0.consume_text_chunk(",");
            }

            if const { !(<T as MakeChunks<OC>>::MADE_CHUNKS.chunk.is_empty()) } {
                self.0
                    .consume_text_chunk(const { <T as MakeChunks<OC>>::MADE_CHUNKS.chunk });
            }

            if const { <T as MakeChunks<OC>>::MADE_CHUNKS.append_comma } {
                self.0.consume_text_chunk(",");
            }

            CONSUME(self.0, PhantomData)
        }

        type ConsumeRuntimeChunk<C: RuntimeChunks> =
            CONSUME<W, InitialConsumer, C::NextState<S>, OC>;
        fn consume_runtime_chunk<C: RuntimeChunks>(
            mut self,
            chunk: C,
        ) -> Self::ConsumeRuntimeChunk<C> {
            const { _ = C::NextState::<S>::STATE }
            chunk.runtime_chunks_write_into(&mut self.0);
            CONSUME(self.0, PhantomData)
        }

        #[cfg(remove)]
        type ConsumeOpenContentBeforeContent =
            TConsumeOpenContentBeforeContent<W, InitialConsumer, S, OPEN_CLOSE>;
        #[cfg(remove)]
        fn consume_open_content_before_content(
            self,
            content: impl IntoJson<JsonKind = K>,
            (): <K as JsonKind>::ArrayOrObjectContainsSelf,
        ) -> Self::ConsumeOpenContentBeforeContent {
            let mut w = TConsumeOpenContentBeforeContent::new(self.0);
            w.extend(content);
            w
        }

        fn end_with_right_bracket(
            self,
            yes: <K as JsonKind>::Contains<json_kinds::Array>,
        ) -> Consumed<K, Self::InitialConsumer> {
            self.impl_end_with_right_bracket(yes)
        }
        fn end_with_right_brace(
            self,
            yes: <K as JsonKind>::Contains<json_kinds::Object>,
        ) -> Consumed<K, Self::InitialConsumer> {
            self.impl_end_with_right_brace(yes)
        }

        fn end(self) -> Consumed<K, Self::InitialConsumer> {
            const {
                S::STATE.assert_eof();
            }
            assert_consumed()
        }
    }
});

pub struct ConsumeArrayItemsPrependCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);
pub struct ConsumeArrayItemsAppendCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);

impl_many!({
    {
        {
            use ConsumeArrayItemsPrependCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::PREPEND_COMMA;
            const fn make_non_empty_array(items: &str) -> (&'static str, &str) {
                (",", items)
            }

            macro_rules! reorder_comma_item {
                ({
                    {$($comma:tt)*}
                    {$($item:tt)*}
                }) => {{
                    $($comma)*
                    $($item)*
                }};
            }
        }
        {
            use ConsumeArrayItemsAppendCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::APPEND_COMMA;
            const fn make_non_empty_array(items: &str) -> (&str, &'static str) {
                (items, ",")
            }

            macro_rules! reorder_comma_item {
                ({
                    {$($comma:tt)*}
                    {$($item:tt)*}
                }) => {{
                    $($item)*
                    $($comma)*
                }};
            }
        }
    }

    impl<W: ConsumeTextChunk> ConsumeJson for CONSUME<W> {
        type ConsumeJsonKind = json_kinds::Array;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_object! {}

        fn consume_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self> {
            Consumed::ASSERT_ARRAY
        }
        fn consume_non_empty_array_as_str(
            mut self,
            v: crate::r#const::NonEmptyArrayAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self> {
            let (chunk1, chunk2) = make_non_empty_array(v.items());
            self.0.consume_2_text_chunks(chunk1, chunk2);
            Consumed::ASSERT_ARRAY
        }

        type ConsumeChunksOfNonEmptyArray =
            ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OC.as_u8() }>;
        fn start_to_consume_chunks_of_non_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
        }

        type ConsumeChainedArrays = Self;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            self
        }

        fn consume_array_of_items(
            mut self,
            items: impl IntoIterator<Item: IntoJson>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self> {
            items.into_iter().for_each(|item| {
                reorder_comma_item!({
                    {
                        self.0.consume_text_chunk(",");
                    }
                    {
                        let Consumed { .. } = item
                            .json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
                    }
                })
            });
            Consumed::ASSERT_ARRAY
        }
    }

    impl<W: ConsumeTextChunk> ConsumeChainedArrays for CONSUME<W> {
        fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
            let Consumed { .. } =
                arr.json_provide_into(CONSUME(self.0.as_mut_consume_text_chunk()));
        }

        type InitialConsumer = Self;
        fn end_with(
            self,
            arr: impl IntoJson<JsonKind = json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
            arr.json_provide_into(self)
        }
    }
});

pub struct ConsumeObjectKvsPrependCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);
pub struct ConsumeObjectKvsAppendCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);

impl_many!({
    {
        {
            use ConsumeObjectKvsPrependCommaIfNotEmpty as CONSUME;
            use write_comma_kvs as write_iter;
            const OC: OpenClose = OpenClose::PREPEND_COMMA;
            const fn make_non_empty_object(kvs: &str) -> (&'static str, &str) {
                (",", kvs)
            }
        }
        {
            use ConsumeObjectKvsAppendCommaIfNotEmpty as CONSUME;
            use write_kvs_comma as write_iter;
            const OC: OpenClose = OpenClose::APPEND_COMMA;
            const fn make_non_empty_object(kvs: &str) -> (&str, &'static str) {
                (kvs, ",")
            }
        }
    }

    impl<W: ConsumeTextChunk> ConsumeJson for CONSUME<W> {
        type ConsumeJsonKind = json_kinds::Object;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_array! {}

        fn consume_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Consumed<json_kinds::Object, Self> {
            Consumed::ASSERT_OBJECT
        }

        fn consume_non_empty_object_as_str(
            mut self,
            v: crate::r#const::NonEmptyObjectAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Consumed<json_kinds::Object, Self> {
            let (chunk1, chunk2) = make_non_empty_object(v.kvs());
            self.0.consume_2_text_chunks(chunk1, chunk2);
            Consumed::ASSERT_OBJECT
        }

        type ConsumeChunksOfNonEmptyObject =
            ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OC.as_u8() }>;

        fn start_to_consume_chunks_of_non_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
        }

        type ConsumeChainedObjects = Self;
        fn start_to_consume_chained_objects(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChainedObjects {
            self
        }

        fn consume_object_of_iter(
            mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Consumed<json_kinds::Object, Self> {
            write_iter(&mut self.0, kvs);
            Consumed::ASSERT_OBJECT
        }
    }

    impl<W: ConsumeTextChunk> ConsumeChainedObjects for CONSUME<W> {
        fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Object>) {
            let Consumed { .. } =
                arr.json_provide_into(CONSUME(self.0.as_mut_consume_text_chunk()));
        }

        type InitialConsumer = Self;
        fn end_with(
            self,
            arr: impl IntoJson<JsonKind = json_kinds::Object>,
        ) -> Consumed<json_kinds::Object, Self::InitialConsumer> {
            arr.json_provide_into(self)
        }
    }
});

fn write_key_frag_quote_colon_value(
    w: &mut impl ConsumeTextChunk,
    kv: impl crate::ser::IntoJsonKeyColonValue,
) {
    let (key, value) = kv.into_json_key_value();
    key.json_provide_into(consume_content::ConsumeStringFragment(
        w.as_mut_consume_text_chunk(),
    ));
    w.consume_text_chunk("\":");
    value.json_provide_into(ConsumeJsonText(w.as_mut_consume_text_chunk()));
}

fn write_comma_kvs(
    w: &mut impl ConsumeTextChunk,
    kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
) {
    // ,"key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value
    kvs.into_iter().for_each(|kv| {
        w.consume_text_chunk(",\"");
        write_key_frag_quote_colon_value(w, kv);
    });
}

fn write_non_empty_kvs(
    w: &mut impl ConsumeTextChunk,
    first: impl crate::ser::IntoJsonKeyColonValue,
    kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
) {
    // "key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value
    w.consume_text_chunk("\""); // TODO: ConsumeStringOpenFragment
    write_key_frag_quote_colon_value(w, first);

    write_comma_kvs(w, kvs);
}

fn write_kvs(
    w: &mut impl ConsumeTextChunk,
    kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
) {
    let mut kvs = kvs.into_iter();
    let Some(first) = kvs.next() else { return };

    write_non_empty_kvs(w, first, kvs);
}

fn write_kvs_comma(
    w: &mut impl ConsumeTextChunk,
    kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
) {
    // "key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value,
    let mut kvs = kvs.into_iter();
    let Some(first) = kvs.next() else { return };

    write_non_empty_kvs(w, first, kvs);

    w.consume_text_chunk(",");
}
