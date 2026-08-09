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
    json_kinds::{JsonKind, JsonKindContains},
    never_consume::NeverConsume,
    open_close::{GroupOrComma, MakeChunks, OpenClose},
};

pub use self::consumed::Consumed;

#[rustfmt::skip]
macro_rules! not_any_value {
    () => {
        fn into_consume_json_text(
            self,
            yes: crate::ser::consumers::yes_or_no::No,
        ) -> crate::ser::consumers::ConsumeJsonText<Self::Writer> {
            match yes {}
        }

        fn consume_any_value_of_any_kind(
            self,
            _: impl crate::ser::IntoJson,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                json_kinds::AnyValue,
            >,
        ) -> trait_mod::Output![Consumed<json_kinds::AnyValue, Self>] {
            trait_mod::never_future!(match yes {})
        }
        fn consume_any_value(
            self,
            _: crate::ser::texts::Value<impl crate::ser::traits::IntoTextChunks>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                json_kinds::AnyValue,
            >,
        ) -> trait_mod::Output![Consumed<json_kinds::AnyValue, Self>] {
            trait_mod::never_future!(match yes {})
        }
    };
}

#[rustfmt::skip]
macro_rules! not_string {
    () => {
        fn consume_empty_string(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> trait_mod::Output![Consumed<json_kinds::JsonString, Self>] {
            trait_mod::never_future!(match yes {})
        }

        fn consume_json_string_as_str(
            self,
            _: crate::r#const::JsonStringAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> trait_mod::Output![Consumed<json_kinds::JsonString, Self>] {
            trait_mod::never_future!(match yes {})
        }

        fn consume_str(
            self,
            _: &str,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> trait_mod::Output![Consumed<json_kinds::JsonString, Self>] {
            trait_mod::never_future!(match yes {})
        }

        type EndJsonString = crate::ser::consumers::json_string_chunks::NeverEndJsonString;
        fn start_to_consume_chunks_of_json_string_with_first_chunk(
            self,
            _: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> trait_mod::Output![trait_mod::CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            trait_mod::never_future!(match yes {})
        }
        fn start_to_consume_chunks_of_json_string(
            self,
            _: impl IntoJson<JsonKind = json_kinds::JsonString>,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> trait_mod::Output![trait_mod::CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            trait_mod::never_future!(match yes {})
        }

        type ConsumeChainedStrings = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_strings(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::consumers::json_kinds::JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Self::ConsumeChainedStrings {
            match yes {}
        }
    };
}

#[rustfmt::skip]
macro_rules! not_array {
    () => {
        fn consume_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> trait_mod::Output![Consumed<crate::ser::json_kinds::Array, Self>] {
            trait_mod::never_future!(match yes {})
        }
        fn consume_non_empty_array_as_str(
            self,
            _: crate::r#const::NonEmptyArrayAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> trait_mod::Output![Consumed<crate::ser::json_kinds::Array, Self>] {
            trait_mod::never_future!(match yes {})
        }

        type ConsumeChunksOfNonEmptyArray =
            crate::ser::consumers::never_consume::NeverConsume<Self, crate::r#const::states::Init>;

        fn start_to_consume_chunks_of_non_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            match yes {}
        }

        type ConsumeChainedArrays = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_arrays(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> Self::ConsumeChainedArrays {
            match yes {}
        }

        fn consume_array_of_items(
            self,
            _: impl IntoIterator<Item: crate::ser::IntoJson>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Array,
            >,
        ) -> trait_mod::Output![Consumed<crate::ser::json_kinds::Array, Self>] {
            trait_mod::never_future!(match yes {})
        }
    };
}

#[rustfmt::skip]
macro_rules! not_object {
    () => {
        fn consume_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> trait_mod::Output![Consumed<json_kinds::Object, Self>] {
            trait_mod::never_future!(match yes {})
        }
        fn consume_non_empty_object_as_str(
            self,
            _: crate::r#const::NonEmptyObjectAsStr<'_>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> trait_mod::Output![Consumed<json_kinds::Object, Self>] {
            trait_mod::never_future!(match yes {})
        }

        type ConsumeChunksOfNonEmptyObject =
            crate::ser::consumers::never_consume::NeverConsume<Self, crate::r#const::states::Init>;
        fn start_to_consume_chunks_of_non_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            match yes {}
        }

        type ConsumeChainedObjects = crate::ser::consumers::never_consume::NeverConsume<Self>;
        fn start_to_consume_chained_objects(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                crate::ser::json_kinds::Object,
            >,
        ) -> Self::ConsumeChainedObjects {
            match yes {}
        }

        fn consume_object_of_iter(
            self,
            _: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKindContains>::Contains<
                json_kinds::Object,
            >,
        ) -> trait_mod::Output![Consumed<json_kinds::Object, Self>] {
            trait_mod::never_future!(match yes {})
        }
    };
}

pub(crate) mod define_traits;
mod trait_helpers;
mod trait_items;

macro_rules! define_traits {
    ({
        $( #[common_items] {
            $($common_items:item)*
        })?

        mod $mod_trait:ident {
            $vis:vis trait $Trait:ident
            $(<$($TraitParam1:ident $($TraitParam2:ident)? : $TraitParamBounds:path),* $(,)?>)?
            $(: $TraitBound:path)? {
                $($trait_items:tt)*
            }

            $($mod_trait_items:item)*
        }

        mod $mod_try:ident {
            $vis_try:vis trait $TraitTry:ident
            $(<$($TraitTryParam1:ident $($TraitTryParam2:ident)? : $TraitTryParamBounds:path),* $(,)?>)?
            $(: $TraitTryBound:path)? {
                $($trait_try_items:tt)*
            }

            $($mod_try_items:item)*
        }

        mod $mod_async_try:ident {
            $vis_async_try:vis trait $TraitAsyncTry:ident
            $(<$($TraitAsyncTryParam1:ident $($TraitAsyncTryParam2:ident)? : $TraitAsyncTryParamBounds:path),* $(,)?>)?
            $(: $TraitAsyncTryBound:path)? {
                $($trait_async_try_items:tt)*
            }

            $($mod_async_try_items:item)*
        }



        $($items:item)*
    }) => {
        mod $mod_trait {
            use super::*;
            use crate::ser::consumers::define_traits::base as trait_mod;

            $($($common_items)*)?
            $($mod_trait_items)*

            $vis trait $Trait
            $(<$($TraitParam1 $($TraitParam2)? : $TraitParamBounds),*>)?
            $(: $TraitBound)? {
                $($trait_items)*
                $($items)*
            }
        }
        $vis use self::$mod_trait::$Trait;

        mod $mod_try {
            use super::*;
            use crate::ser::consumers::define_traits::try_ as trait_mod;

            $($($common_items)*)?
            $($mod_try_items)*

            $vis_try trait $TraitTry
            $(<$($TraitParam1 $($TraitParam2)? : $TraitParamBounds),*>)?
            $(: $TraitTryBound)? {
                $($trait_try_items)*
                $($items)*
            }
        }
        $vis_try use self::$mod_try::$TraitTry;

        mod $mod_async_try {
            use super::*;
            use crate::ser::consumers::define_traits::async_try as trait_mod;

            $($($common_items)*)?
            $($mod_async_try_items)*

            $vis_async_try trait $TraitAsyncTry
            $(<$($TraitParam1 $($TraitParam2)? : $TraitParamBounds),*>)?
            $(: $TraitAsyncTryBound)? {
                $($trait_async_try_items)*
                $($items)*
            }
        }
        $vis_async_try use self::$mod_async_try::$TraitAsyncTry;
    };
}

mod help_ancestor;
mod yes_or_no;

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

pub(crate) mod writer_assert;

// TODO: seal
define_traits!({
    #[common_items]
    {
        use trait_mod::{
            CONSUME_CHAINED, CONSUME_IN_JSON_STRING, CONSUME_JSON, CONSUME_TEXT_CHUNK,
            END_JSON_STRING, Output, READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY,
            READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT,
        };
    }

    mod consume_json {
        pub trait ConsumeJson {}
    }
    mod try_consume_json {
        pub trait TryConsumeJson {}
    }
    mod async_try_consume_json {
        pub trait AsyncTryConsumeJson {}
    }

    type ConsumeJsonKind: JsonKind;
    type Writer: CONSUME_TEXT_CHUNK
        + writer_assert::WriterAssertIsFromConsumeJsonText<
            Self,
            <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
        >;

    fn into_consume_json_text(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
    ) -> ConsumeJsonText<Self::Writer>;

    fn consume_any_value_of_any_kind(
        self,
        value: impl IntoJson,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
    ) -> Output![Consumed<json_kinds::AnyValue, Self>];

    fn consume_any_value(
        self,
        value: texts::Value<impl traits::IntoTextChunks>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
    ) -> Output![Consumed<json_kinds::AnyValue, Self>];

    fn consume_empty_string(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Output![Consumed<json_kinds::JsonString, Self>];
    fn consume_json_string_as_str(
        self,
        v: crate::r#const::JsonStringAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Output![Consumed<json_kinds::JsonString, Self>];

    /// Consume a json string from `&str`.
    fn consume_str(
        self,
        s: &str,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Output![Consumed<json_kinds::JsonString, Self>];

    type EndJsonString: END_JSON_STRING;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>];
    fn start_to_consume_chunks_of_json_string(
        self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>];

    type ConsumeChainedStrings: CONSUME_CHAINED<json_kinds::JsonString, InitialConsumer = Self>;
    fn start_to_consume_chained_strings(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings;

    fn consume_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
    ) -> Output![ Consumed<json_kinds::Array, Self>];
    fn consume_non_empty_array_as_str(
        self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
    ) -> Output![Consumed<json_kinds::Array, Self>];

    type ConsumeChunksOfNonEmptyArray: READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY<
        InitialConsumer = Self,
    >;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray;

    type ConsumeChainedArrays: CONSUME_CHAINED<json_kinds::Array, InitialConsumer = Self>;

    fn start_to_consume_chained_arrays(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays;

    fn consume_array_of_items(
        self,
        items: impl IntoIterator<Item: IntoJson>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
    ) -> Output![Consumed<json_kinds::Array, Self>];

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
    ) -> Output![Consumed<json_kinds::Object, Self>];
    fn consume_non_empty_object_as_str(
        self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
    ) -> Output![Consumed<json_kinds::Object, Self>];

    type ConsumeChunksOfNonEmptyObject: READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT<
        InitialConsumer = Self,
    >;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject;

    type ConsumeChainedObjects: CONSUME_CHAINED<json_kinds::Object, InitialConsumer = Self>;
    fn start_to_consume_chained_objects(
        self,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects;

    fn consume_object_of_iter(
        self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        yes: <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
    ) -> Output![Consumed<json_kinds::Object, Self>];
});

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON, Output};
    }

    mod consume_chained {
        pub trait ConsumeChained<K: json_kinds::ChainableJsonKind> {}
    }
    mod try_consume_chained {
        pub trait TryConsumeChained<K: json_kinds::ChainableJsonKind> {}
    }
    mod async_try_consume_chained {
        pub trait AsyncTryConsumeChained<K: json_kinds::ChainableJsonKind> {}
    }

    fn extend<V: IntoJson<JsonKind = K>>(
        &mut self,
        s: V,
    ) -> Output![(), <Self::InitialConsumer as CONSUME_JSON>::Writer];

    type InitialConsumer: ?Sized + CONSUME_JSON;
    fn end_with<V: IntoJson<JsonKind = K>>(
        self,
        s: V,
    ) -> Output![Consumed<K, Self::InitialConsumer>, <Self::InitialConsumer as CONSUME_JSON>::Writer];
});

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON, CONSUME_JSON_CHUNKS, Output};
    }

    mod consume_chunks {
        pub trait ConsumeJsonChunks<K: json_kinds::ArrayOrObject> {}
    }
    mod try_consume_chunks {
        pub trait TryConsumeJsonChunks<K: json_kinds::ArrayOrObject> {}
    }
    mod async_try_consume_chunks {
        pub trait AsyncTryConsumeJsonChunks<K: json_kinds::ArrayOrObject> {}
    }

    type InitialConsumer: CONSUME_JSON;
    type CurrentState: ?Sized + HasConstState;

    #[cfg(remove)]
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk>: CONSUME_JSON_CHUNKS<K>;
    #[cfg(remove)]
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T>;

    #[cfg(remove)]
    type ConsumeRuntimeChunk<C: RuntimeChunks>: CONSUME_JSON_CHUNKS<K>;
    #[cfg(remove)]
    fn consume_runtime_chunk<C: RuntimeChunks>(self, chunk: C) -> Self::ConsumeRuntimeChunk<C>;

    type ConsumeIntermediateChunk<Next: ?Sized + HasConstState>: CONSUME_JSON_CHUNKS<K, InitialConsumer = Self::InitialConsumer, CurrentState = Next>;
    fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
        self,
        v: crate::r#const::IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
    ) -> Output![
        Self::ConsumeIntermediateChunk<Next>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    fn consume_contentful_last_chunk(
        self,
        v: K::ContentfulLastChunkAsStr<'_, Self::CurrentState>,
    ) -> Output![
        Consumed<K, Self::InitialConsumer>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonValue: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenValue<Self::CurrentState>,
        >;
    fn json_value<V: IntoJson>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonValue,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeCommaJsonValue: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenCommaValue<Self::CurrentState>,
        >;
    fn comma_json_value<V: IntoJson>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeCommaJsonValue,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonItemsAfterArrayStartBeforeItem: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenItemsAfterArrayStartBeforeItem<Self::CurrentState>,
        >;
    fn json_items_after_array_start_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonItemsAfterArrayStartBeforeItem,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonItemsAfterItem: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenItemsAfterItem<Self::CurrentState>,
        >;
    fn json_items_after_item<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonItemsAfterItem,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonKvsAfterObjectStartBeforeKv: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenKvsAfterObjectStartBeforeKv<Self::CurrentState>,
        >;
    fn json_kvs_after_object_start_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonKvsAfterObjectStartBeforeKv,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonKvsAfterFieldValue: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenKvsAfterFieldValue<Self::CurrentState>,
        >;
    fn json_kvs_after_field_value<V: IntoJson<JsonKind = json_kinds::Object>>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonKvsAfterFieldValue,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type ConsumeJsonStringFragment: CONSUME_JSON_CHUNKS<
            K,
            InitialConsumer = Self::InitialConsumer,
            CurrentState = states::ThenStringFragment<Self::CurrentState>,
        >;
    fn json_string_fragment<V: IntoJson<JsonKind = json_kinds::JsonString>>(
        self,
        v: V,
    ) -> Output![
        Self::ConsumeJsonStringFragment,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

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
    ) -> Output![
        Consumed<K, Self::InitialConsumer>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];
    fn end_with_right_brace(
        self,
        yes: K::Contains<json_kinds::Object>,
    ) -> Output![
        Consumed<K, Self::InitialConsumer>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];
    #[cfg(remove)]
    fn end(self) -> Consumed<K, Self::InitialConsumer>;
});

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON, CONSUME_JSON_CHUNKS, Output};
    }

    mod consume_from_init {
        pub trait ConsumeJsonChunksFromInit<K: json_kinds::ArrayOrObject>:
            CONSUME_JSON_CHUNKS<K, CurrentState = states::Init>
        {
        }
    }
    mod try_consume_from_init {
        pub trait TryConsumeJsonChunksFromInit<K: json_kinds::ArrayOrObject>:
            CONSUME_JSON_CHUNKS<K, CurrentState = states::Init>
        {
        }
    }
    mod async_try_consume_from_init {
        pub trait AsyncTryConsumeJsonChunksFromInit<K: json_kinds::ArrayOrObject>:
            CONSUME_JSON_CHUNKS<K, CurrentState = states::Init>
        {
        }
    }

    type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState>: CONSUME_JSON_CHUNKS<K, InitialConsumer = Self::InitialConsumer, CurrentState = Next>;
    fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
        self,
        v: K::ContentfulFirstChunkAsStr<'_, Next>,
    ) -> Output![
        Self::ConsumeContentfulFirstChunk<Next>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    fn consume_contentful_full_chunk(
        self,
        v: K::ContentfulFullChunkAsAtr<'_>,
    ) -> Output![
        Consumed<K, Self::InitialConsumer>,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];
});

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

pub struct ConsumeJsonText<W>(pub W);

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

impl_many!({
    {
        {
            use define_traits::base as trait_mod;
        }
        {
            use define_traits::try_ as trait_mod;
        }
        {
            use define_traits::async_try as trait_mod;
        }
    }

    use trait_mod::{
        CONSUME_IN_JSON_STRING, CONSUME_JSON, CONSUME_TEXT_CHUNK, Output, XHelpers as _,
        async_move_block, await_, await_try, de_async, de_async_move, last_expr, select,
        select_method,
    };

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeJsonText<W> {
        type ConsumeJsonKind = json_kinds::AnyValue;
        type Writer = W;

        fn into_consume_json_text(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
        ) -> ConsumeJsonText<Self::Writer> {
            self
        }

        fn consume_any_value_of_any_kind(
            self,
            value: impl IntoJson,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
        ) -> Output![Consumed<json_kinds::AnyValue, Self>] {
            value.json_provide_into_x(self).x_map_ok(Consumed::upcast)
        }

        fn consume_any_value(
            mut self,
            value: texts::Value<impl traits::IntoTextChunks>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::AnyValue>,
        ) -> Output![Consumed<json_kinds::AnyValue, Self>] {
            de_async_move!(async move {
                select_method!(
                    (value.into_inner())
                        .write_into(&mut self.0)
                        .try_write_into
                        .async_try_write_into
                        .await?
                );
                last_expr!(Consumed::ASSERT_ANY_VALUE)
            })
        }

        fn consume_empty_string(
            mut self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            de_async_move!(async move {
                select_method!(
                    (self.0)
                        .consume_text_chunk("\"\"")
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        fn consume_json_string_as_str(
            mut self,
            v: crate::r#const::JsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            de_async_move!(async move {
                select_method!(
                    (self.0)
                        .consume_text_chunk(v.as_str())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        fn consume_str(
            mut self,
            s: &str,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            de_async_move!(async move {
                if s.is_empty() {
                    await_!(<Self as CONSUME_JSON>::consume_empty_string(self, ()))
                } else {
                    select_method!(
                        (self.0)
                            .consume_text_chunk("\"")
                            .try_consume_text_chunk
                            .async_try_consume_text_chunk
                            .await?
                    );
                    select_method!(
                        (super::texts::StrToJsonStringFragment(s))
                            .write_into(&mut self.0)
                            .try_write_into
                            .async_try_write_into
                            .await?
                    );
                    select_method!(
                        (self.0)
                            .consume_text_chunk("\"")
                            .try_consume_text_chunk
                            .async_try_consume_text_chunk
                            .await?
                    );
                    last_expr!(Consumed::ASSERT_STRING)
                }
            })
        }

        type EndJsonString = json_string_chunks::EndJsonStringWithClose;
        fn start_to_consume_chunks_of_json_string_with_first_chunk(
            mut self,
            v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            de_async_move!(async move {
                () = select_method!(
                    (self.0)
                        .consume_text_chunk(v.as_str())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(CONSUME_IN_JSON_STRING::new(self.0))
            })
        }
        fn start_to_consume_chunks_of_json_string(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            de_async_move!(async move {
                // TODO: optimize with ConsumeJsonStringOpenFragment
                () = select_method!(
                    (self.0)
                        .consume_text_chunk("\"")
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                let Consumed { .. } = select_method!(
                    v.json_provide_into(consume_content::ConsumeStringFragment(select_method!(
                        (self.0)
                            .as_mut_consume_text_chunk()
                            .as_mut_try_consume_text_chunk
                            .as_mut_async_try_consume_text_chunk
                    )))
                    .json_provide_into_try
                    .json_provide_into_async_try
                    .await?
                );
                last_expr!(CONSUME_IN_JSON_STRING::new(self.0))
            })
        }

        type ConsumeChainedStrings = ConsumeChainedStringsFull<W>;
        fn start_to_consume_chained_strings(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Self::ConsumeChainedStrings {
            ConsumeChainedStringsFull::new(self.0)
        }

        fn consume_empty_array(
            mut self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk("[]"));
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
        fn consume_non_empty_array_as_str(
            mut self,
            v: crate::r#const::NonEmptyArrayAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>] {
            de_async_move!(async move {
                () = select_method!(
                    (self.0)
                        .consume_text_chunk(v.as_str())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }

        type ConsumeChunksOfNonEmptyArray =
            ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;
        fn start_to_consume_chunks_of_non_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
        }

        type ConsumeChainedArrays = ConsumeChainedArraysFull<W>;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            ConsumeChainedArraysFull::new(self.0)
        }

        fn consume_array_of_items(
            mut self,
            items: impl IntoIterator<Item: IntoJson>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>] {
            de_async_move!(async move {
                let mut items = items.into_iter();
                let Some(first) = items.next() else {
                    return await_!(<Self as CONSUME_JSON>::consume_empty_array(self, ()));
                };
                () = await_try!(self.0.x_consume_text_chunk("["));
                let Consumed { .. } = await_try!(
                    first
                        .json_provide_into_x(ConsumeJsonText(self.0.as_mut_x_consume_text_chunk()))
                );

                () = await_try!(items.x_into_for_each(de_async!(async |item| {
                    await_try!(self.0.x_consume_text_chunk(","));
                    let Consumed { .. } = await_try!(item.json_provide_into_x(ConsumeJsonText(
                        self.0.as_mut_x_consume_text_chunk()
                    )));
                    last_expr!(())
                })));

                () = await_try!(self.0.x_consume_text_chunk("]"));

                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }

        fn consume_empty_object(
            mut self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                () = select_method!(
                    (self.0)
                        .consume_text_chunk("{}")
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }

        fn consume_non_empty_object_as_str(
            mut self,
            v: crate::r#const::NonEmptyObjectAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                () = select_method!(
                    (self.0)
                        .consume_text_chunk(v.as_str())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await?
                );
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }

        type ConsumeChunksOfNonEmptyObject =
            ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;

        fn start_to_consume_chunks_of_non_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
        }

        type ConsumeChainedObjects = consume_chained_full::ConsumeChainedObjectsFull<W>;
        fn start_to_consume_chained_objects(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChainedObjects {
            consume_chained_full::ConsumeChainedObjectsFull::new(self.0)
        }

        fn consume_object_of_iter(
            mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                let mut items = kvs.into_iter();
                let Some(first) = items.next() else {
                    return await_!(<Self as CONSUME_JSON>::consume_empty_object(self, ()));
                };
                () = await_try!(self.0.x_consume_text_chunk("{\""));
                () = await_try!(self.0.x_write_key_frag_quote_colon_value(first)); // TODO:

                () = await_try!(items.x_into_for_each(de_async!(async |kv| {
                    () = await_try!(self.0.x_consume_text_chunk(",\""));
                    () = await_try!(self.0.x_write_key_frag_quote_colon_value(kv)); // TODO:
                    last_expr!(())
                })));

                () = await_try!(self.0.x_consume_text_chunk("}"));

                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
    }
});

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
    W,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<(InitialConsumer, S)>);
pub struct ConsumeChunksOfNonEmptyObject<
    W,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<(InitialConsumer, S)>);

impl_many!({
    {
        {
            use define_traits::base as trait_mod;

            use chunks::ImplEndWithRight as IMPL_END_WITH_RIGHT;
        }
        {
            use define_traits::try_ as trait_mod;

            use chunks::ImplTryEndWithRight as IMPL_END_WITH_RIGHT;
        }
        {
            use define_traits::async_try as trait_mod;

            use chunks::ImplAsyncTryEndWithRight as IMPL_END_WITH_RIGHT;
        }
    }

    use trait_mod::{
        CONSUME_CHAINED, CONSUME_JSON, CONSUME_JSON_CHUNKS, CONSUME_JSON_CHUNKS_FROM_INIT,
        CONSUME_TEXT_CHUNK, Output, XHelpers as _, async_, async_move_block, await_try,
        de_async_move, last_expr, only_expr, select, select_expr, select_method,
    };

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

        impl<
            W: CONSUME_TEXT_CHUNK,
            InitialConsumer: CONSUME_JSON<Writer = W>,
            S: ?Sized + HasConstState,
            const OC: u8,
        > CONSUME_JSON_CHUNKS<K> for CONSUME<W, InitialConsumer, S, OC>
        {
            type InitialConsumer = InitialConsumer;
            type CurrentState = S;

            type ConsumeIntermediateChunk<Next: ?Sized + HasConstState> =
                CONSUME<W, InitialConsumer, Next, OC>;
            fn consume_intermediate_chunk<Next: ?Sized + HasConstState>(
                mut self,
                chunk: crate::r#const::IntermediateChunkAsStr<'_, Self::CurrentState, Next>,
            ) -> Output![Self::ConsumeIntermediateChunk<Next>, W] {
                const {
                    assert!(!S::STATE.is_init());
                    assert!(!Next::STATE.is_eof());
                }

                de_async_move!(async move {
                    () = select_method!(
                        (self.0)
                            .consume_text_chunk(chunk.as_str())
                            .try_consume_text_chunk
                            .async_try_consume_text_chunk
                            .await?
                    );

                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            fn consume_contentful_last_chunk(
                mut self,
                v: <K as json_kinds::ArrayOrObject>::ContentfulLastChunkAsStr<
                    '_,
                    Self::CurrentState,
                >,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                const {
                    assert!(!S::STATE.is_init());
                }

                de_async_move!(async move {
                    () = match const { OpenClose::try_from_u8(OC).unwrap().close } {
                        open_close::GroupOrComma::Nothing => {
                            await_try!(self.0.x_consume_text_chunk(v.remove_group_close()))
                        }
                        open_close::GroupOrComma::Group => {
                            await_try!(self.0.x_consume_text_chunk(v.as_str()))
                        }
                        open_close::GroupOrComma::Comma => {
                            await_try!(self.0.x_consume_2_text_chunks(v.remove_group_close(), ","))
                        }
                    };

                    const { last_expr!(assert_consumed()) }
                })
            }

            type ConsumeJsonValue = CONSUME<W, InitialConsumer, states::ThenValue<S>, OC>;
            fn json_value<V: IntoJson>(mut self, v: V) -> Output![Self::ConsumeJsonValue, W] {
                const { _ = states::ThenValue::<S>::STATE }
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(ConsumeJsonText(
                        self.0.as_mut_x_consume_text_chunk()
                    )));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeCommaJsonValue = CONSUME<W, InitialConsumer, states::ThenCommaValue<S>, OC>;
            fn comma_json_value<V: IntoJson>(
                mut self,
                v: V,
            ) -> Output![
                Self::ConsumeCommaJsonValue,
                <Self::InitialConsumer as CONSUME_JSON>::Writer
            ] {
                const { _ = states::ThenCommaValue::<S>::STATE }
                de_async_move!(async move {
                    () = await_try!(self.0.x_consume_text_chunk(","));
                    let Consumed { .. } = await_try!(v.json_provide_into_x(ConsumeJsonText(
                        self.0.as_mut_x_consume_text_chunk()
                    )));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeJsonItemsAfterArrayStartBeforeItem =
                CONSUME<W, InitialConsumer, states::ThenItemsAfterArrayStartBeforeItem<S>, OC>;
            fn json_items_after_array_start_before_item<
                V: IntoJson<JsonKind = json_kinds::Array>,
            >(
                mut self,
                v: V,
            ) -> Output![Self::ConsumeJsonItemsAfterArrayStartBeforeItem, W] {
                const { _ = states::ThenItemsAfterArrayStartBeforeItem::<S>::STATE }

                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(
                        ConsumeArrayItemsAppendCommaIfNotEmpty(
                            self.0.as_mut_x_consume_text_chunk(),
                        )
                    ));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeJsonItemsAfterItem =
                CONSUME<W, InitialConsumer, states::ThenItemsAfterItem<S>, OC>;
            fn json_items_after_item<V: IntoJson<JsonKind = json_kinds::Array>>(
                mut self,
                v: V,
            ) -> Output![Self::ConsumeJsonItemsAfterItem, W] {
                const { _ = states::ThenItemsAfterItem::<S>::STATE }
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(
                        ConsumeArrayItemsPrependCommaIfNotEmpty(
                            self.0.as_mut_x_consume_text_chunk(),
                        )
                    ));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeJsonKvsAfterObjectStartBeforeKv =
                CONSUME<W, InitialConsumer, states::ThenKvsAfterObjectStartBeforeKv<S>, OC>;
            fn json_kvs_after_object_start_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
                mut self,
                v: V,
            ) -> Output![Self::ConsumeJsonKvsAfterObjectStartBeforeKv, W] {
                const { _ = states::ThenKvsAfterObjectStartBeforeKv::<S>::STATE }
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(
                        ConsumeObjectKvsAppendCommaIfNotEmpty(self.0.as_mut_x_consume_text_chunk())
                    ));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeJsonKvsAfterFieldValue =
                CONSUME<W, InitialConsumer, states::ThenKvsAfterFieldValue<S>, OC>;
            fn json_kvs_after_field_value<V: IntoJson<JsonKind = json_kinds::Object>>(
                mut self,
                v: V,
            ) -> Output![Self::ConsumeJsonKvsAfterFieldValue, W] {
                const { _ = states::ThenKvsAfterFieldValue::<S>::STATE }
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(
                        ConsumeObjectKvsPrependCommaIfNotEmpty(
                            self.0.as_mut_x_consume_text_chunk()
                        ),
                    ));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            type ConsumeJsonStringFragment =
                CONSUME<W, InitialConsumer, states::ThenStringFragment<S>, OC>;
            fn json_string_fragment<V: IntoJson<JsonKind = json_kinds::JsonString>>(
                mut self,
                v: V,
            ) -> Output![Self::ConsumeJsonStringFragment, W] {
                const { _ = states::ThenStringFragment::<S>::STATE }
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(v.json_provide_into_x(
                        consume_content::ConsumeStringFragment(
                            self.0.as_mut_x_consume_text_chunk(),
                        )
                    ));
                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            #[cfg(remove)]
            type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
                CONSUME<W, InitialConsumer, states::NextStateOf<T>, OC>;
            #[cfg(remove)]
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

            #[cfg(remove)]
            type ConsumeRuntimeChunk<C: RuntimeChunks> =
                CONSUME<W, InitialConsumer, C::NextState<S>, OC>;
            #[cfg(remove)]
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
                yes: <K as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                IMPL_END_WITH_RIGHT(self).impl_end_with_right_bracket(yes)
            }
            fn end_with_right_brace(
                self,
                yes: <K as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                IMPL_END_WITH_RIGHT(self).impl_end_with_right_brace(yes)
            }

            #[cfg(remove)]
            fn end(self) -> Consumed<K, Self::InitialConsumer> {
                const {
                    S::STATE.assert_eof();
                }
                assert_consumed()
            }
        }

        impl<W: CONSUME_TEXT_CHUNK, InitialConsumer: CONSUME_JSON<Writer = W>, const OC: u8>
            CONSUME_JSON_CHUNKS_FROM_INIT<K> for CONSUME<W, InitialConsumer, states::Init, OC>
        {
            type ConsumeContentfulFirstChunk<Next: ?Sized + HasConstState> =
                CONSUME<W, InitialConsumer, Next, OC>;
            fn consume_contentful_first_chunk<Next: ?Sized + HasConstState>(
                mut self,
                v: <K as json_kinds::ArrayOrObject>::ContentfulFirstChunkAsStr<'_, Next>,
            ) -> Output![Self::ConsumeContentfulFirstChunk<Next>, W] {
                const {
                    // make sure current state is Init
                    Self::CurrentState::STATE.assert_init();
                    assert!(!Next::STATE.is_eof());
                }

                de_async_move!(async move {
                    match const { OpenClose::try_from_u8(OC).unwrap().open } {
                        open_close::GroupOrComma::Nothing => {
                            () = await_try!(self.0.x_consume_text_chunk(v.remove_group_open()))
                        }
                        open_close::GroupOrComma::Group => {
                            () = await_try!(self.0.x_consume_text_chunk(v.as_str()))
                        }
                        open_close::GroupOrComma::Comma => {
                            () = await_try!(
                                self.0.x_consume_2_text_chunks(",", v.remove_group_open())
                            )
                        }
                    }

                    last_expr!(CONSUME(self.0, PhantomData))
                })
            }

            fn consume_contentful_full_chunk(
                mut self,
                v: <K as json_kinds::ArrayOrObject>::ContentfulFullChunkAsAtr<'_>,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                de_async_move!(async move {
                    match const { OpenClose::try_from_u8(OC).unwrap().into_tuple() } {
                        (GroupOrComma::Nothing, GroupOrComma::Nothing) => {
                            () = await_try!(
                                self.0.x_consume_text_chunk(v.remove_surrounding_group())
                            )
                        }
                        (GroupOrComma::Nothing, GroupOrComma::Group) => {
                            () = await_try!(self.0.x_consume_text_chunk(v.remove_group_open()))
                        }
                        (GroupOrComma::Nothing, GroupOrComma::Comma) => {
                            () = await_try!(
                                self.0.x_consume_2_text_chunks(v.remove_group_open(), ",")
                            )
                        }
                        (GroupOrComma::Group, GroupOrComma::Nothing) => {
                            () = await_try!(self.0.x_consume_text_chunk(v.remove_group_close()))
                        }
                        (GroupOrComma::Group, GroupOrComma::Group) => {
                            () = await_try!(self.0.x_consume_text_chunk(v.as_str()))
                        }
                        (GroupOrComma::Group, GroupOrComma::Comma) => {
                            () = await_try!(
                                self.0.x_consume_2_text_chunks(v.remove_group_close(), ",")
                            )
                        }
                        (GroupOrComma::Comma, GroupOrComma::Nothing) => {
                            () = await_try!(
                                self.0.x_consume_2_text_chunks(",", v.remove_group_close())
                            )
                        }
                        (GroupOrComma::Comma, GroupOrComma::Group) => {
                            () = await_try!(
                                self.0.x_consume_2_text_chunks(",", v.remove_group_open())
                            )
                        }
                        (GroupOrComma::Comma, GroupOrComma::Comma) => {
                            // TODO: consume_3_text_chunks
                            // actually this branch should not be used
                            () = await_try!(self.0.x_consume_text_chunk(","));
                            () = await_try!(
                                self.0.x_consume_text_chunk(v.remove_surrounding_group())
                            );
                            () = await_try!(self.0.x_consume_text_chunk(","));
                        }
                    }
                    const { last_expr!(assert_consumed()) }
                })
            }
        }
    });

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

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for CONSUME<W> {
            type ConsumeJsonKind = json_kinds::Array;
            type Writer = W;

            not_any_value! {}
            not_string! {}
            not_object! {}

            fn consume_empty_array(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Output![Consumed<json_kinds::Array, Self>, W] {
                only_expr!(Consumed::ASSERT_ARRAY)
            }
            fn consume_non_empty_array_as_str(
                mut self,
                v: crate::r#const::NonEmptyArrayAsStr<'_>,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Output![Consumed<json_kinds::Array, Self>, W] {
                let (chunk1, chunk2) = make_non_empty_array(v.items());
                de_async_move!(async move {
                    () = await_try!(self.0.x_consume_2_text_chunks(chunk1, chunk2));
                    last_expr!(Consumed::ASSERT_ARRAY)
                })
            }

            type ConsumeChunksOfNonEmptyArray =
                ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OC.as_u8() }>;
            fn start_to_consume_chunks_of_non_empty_array(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Self::ConsumeChunksOfNonEmptyArray {
                ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
            }

            type ConsumeChainedArrays = Self;
            fn start_to_consume_chained_arrays(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Self::ConsumeChainedArrays {
                self
            }

            fn consume_array_of_items(
                self,
                items: impl IntoIterator<Item: IntoJson>,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
            ) -> Output![Consumed<json_kinds::Array, Self>, W] {
                fn consume_array_of_items<W: CONSUME_TEXT_CHUNK, T: IntoJson>(
                    mut w: W,
                    items: impl IntoIterator<Item = T>,
                ) -> Output![Consumed<json_kinds::Array, CONSUME<W>>, W] {
                    let action = async_!(move |item: T| {
                        () = reorder_comma_item!({
                            {
                                () = await_try!(w.x_consume_text_chunk(","));
                            }
                            {
                                let Consumed { .. } = await_try!(item.json_provide_into_x(
                                    ConsumeJsonText(w.as_mut_x_consume_text_chunk()),
                                ));
                            }
                        });
                        last_expr!(())
                    });

                    select_expr!(
                        {
                            () = items.into_iter().for_each(action);
                            Consumed::ASSERT_ARRAY
                        },
                        items
                            .into_iter()
                            .try_for_each(action)
                            .map(|()| Consumed::ASSERT_ARRAY),
                        async move {
                            let mut action = action;
                            for item in items {
                                () = action(item).await?;
                            }
                            Ok(Consumed::ASSERT_ARRAY)
                        }
                    )
                }

                consume_array_of_items(self.0, items)
            }
        }

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<json_kinds::Array> for CONSUME<W> {
            fn extend<V: IntoJson<JsonKind = json_kinds::Array>>(
                &mut self,
                arr: V,
            ) -> Output![(), W] {
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(
                        arr.json_provide_into_x(CONSUME(self.0.as_mut_x_consume_text_chunk()))
                    );
                    last_expr!(())
                })
            }

            type InitialConsumer = Self;
            fn end_with<V: IntoJson<JsonKind = json_kinds::Array>>(
                self,
                arr: V,
            ) -> Output![Consumed<json_kinds::Array, Self::InitialConsumer>, W] {
                arr.json_provide_into_x(self)
            }
        }
    });

    impl_many!({
        {
            {
                use ConsumeObjectKvsPrependCommaIfNotEmpty as CONSUME;

                const OC: OpenClose = OpenClose::PREPEND_COMMA;
                const fn make_non_empty_object(kvs: &str) -> (&'static str, &str) {
                    (",", kvs)
                }

                macro_rules! write_iter {
                    ($e:expr, $args:expr) => {
                        $e.x_write_comma_kvs($args)
                    };
                }
            }
            {
                use ConsumeObjectKvsAppendCommaIfNotEmpty as CONSUME;

                const OC: OpenClose = OpenClose::APPEND_COMMA;
                const fn make_non_empty_object(kvs: &str) -> (&str, &'static str) {
                    (kvs, ",")
                }
                macro_rules! write_iter {
                    ($e:expr, $args:expr) => {
                        $e.x_write_kvs_comma($args)
                    };
                }
            }
        }

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for CONSUME<W> {
            type ConsumeJsonKind = json_kinds::Object;
            type Writer = W;

            not_any_value! {}
            not_string! {}
            not_array! {}

            fn consume_empty_object(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Output![Consumed<json_kinds::Object, Self>, W] {
                only_expr!(Consumed::ASSERT_OBJECT)
            }

            fn consume_non_empty_object_as_str(
                mut self,
                v: crate::r#const::NonEmptyObjectAsStr<'_>,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Output![Consumed<json_kinds::Object, Self>, W] {
                let (chunk1, chunk2) = make_non_empty_object(v.kvs());
                de_async_move!(async move {
                    () = await_try!(self.0.x_consume_2_text_chunks(chunk1, chunk2));
                    last_expr!(Consumed::ASSERT_OBJECT)
                })
            }

            type ConsumeChunksOfNonEmptyObject =
                ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OC.as_u8() }>;

            fn start_to_consume_chunks_of_non_empty_object(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Self::ConsumeChunksOfNonEmptyObject {
                ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
            }

            type ConsumeChainedObjects = Self;
            fn start_to_consume_chained_objects(
                self,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Self::ConsumeChainedObjects {
                self
            }

            fn consume_object_of_iter(
                mut self,
                kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
                (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
            ) -> Output![Consumed<json_kinds::Object, Self>] {
                de_async_move!(async move {
                    () = await_try!(write_iter!(self.0, kvs));
                    last_expr!(Consumed::ASSERT_OBJECT)
                })
            }
        }

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<json_kinds::Object> for CONSUME<W> {
            fn extend<V: IntoJson<JsonKind = json_kinds::Object>>(
                &mut self,
                arr: V,
            ) -> Output![(), W] {
                de_async_move!(async move {
                    let Consumed { .. } = await_try!(
                        arr.json_provide_into_x(CONSUME(self.0.as_mut_x_consume_text_chunk()))
                    );
                    last_expr!(())
                })
            }

            type InitialConsumer = Self;
            fn end_with<V: IntoJson<JsonKind = json_kinds::Object>>(
                self,
                arr: V,
            ) -> Output![Consumed<json_kinds::Object, Self::InitialConsumer>, W] {
                arr.json_provide_into_x(self)
            }
        }
    });
});

pub struct ConsumeArrayItemsPrependCommaIfNotEmpty<W>(pub W);
pub struct ConsumeArrayItemsAppendCommaIfNotEmpty<W>(pub W);

pub struct ConsumeObjectKvsPrependCommaIfNotEmpty<W>(pub W);
pub struct ConsumeObjectKvsAppendCommaIfNotEmpty<W>(pub W);
