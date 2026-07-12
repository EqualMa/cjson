use core::marker::PhantomData;

use crate::{
    r#const::{HasConstState, states},
    ser::{IntoJson, traits::ConsumeTextChunk},
};

use super::{
    ConsumeArrayItemsAppendCommaIfNotEmpty, ConsumeChunksOfNonEmptyArray,
    ConsumeChunksOfNonEmptyObject, ConsumeJsonChunks, ConsumeJsonText, Consumed, OpenClose,
    json_kinds, open_close::GroupOrComma,
};

pub trait ReadyToConsumeJsonChunksOfNonEmptyArray:
    ConsumeJsonChunks<json_kinds::Array, CurrentState = states::Init>
{
    type LeftBracketValue: ConsumeJsonChunks<
            json_kinds::Array,
            CurrentState = states::LeftBracketValue,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_bracket_value(self, value: impl IntoJson) -> Self::LeftBracketValue;

    type LeftBracketItemsBeforeItem: ConsumeJsonChunks<
            json_kinds::Array,
            CurrentState = states::LeftBracketItemsBeforeItem,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_bracket_items_before_item(
        self,
        items: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::LeftBracketItemsBeforeItem;
}

pub trait ReadyToConsumeJsonChunksOfNonEmptyObject:
    ConsumeJsonChunks<json_kinds::Object, CurrentState = states::Init>
{
    type LeftBraceKvsBeforeKv: ConsumeJsonChunks<
            json_kinds::Object,
            CurrentState = states::LeftBraceKvsBeforeKv,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_brace_kvs_before_kv(
        self,
        kvs: impl IntoJson<JsonKind = json_kinds::Object>,
    ) -> Self::LeftBraceKvsBeforeKv;
}

impl<W: ConsumeTextChunk, InitialConsumer, const OC: u8> ReadyToConsumeJsonChunksOfNonEmptyArray
    for ConsumeChunksOfNonEmptyArray<W, InitialConsumer, states::Init, OC>
{
    type LeftBracketValue =
        ConsumeChunksOfNonEmptyArray<W, InitialConsumer, states::LeftBracketValue, OC>;

    fn left_bracket_value(mut self, value: impl IntoJson) -> Self::LeftBracketValue {
        match const { OpenClose::try_from_u8(OC).unwrap().open } {
            GroupOrComma::Nothing => {}
            GroupOrComma::Group => {
                self.0.consume_text_chunk("[");
            }
            GroupOrComma::Comma => {
                self.0.consume_text_chunk(",");
            }
        }

        value.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));

        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    type LeftBracketItemsBeforeItem =
        ConsumeChunksOfNonEmptyArray<W, InitialConsumer, states::LeftBracketItemsBeforeItem, OC>;

    fn left_bracket_items_before_item(
        mut self,
        items: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Self::LeftBracketItemsBeforeItem {
        match const { OpenClose::try_from_u8(OC).unwrap().open } {
            GroupOrComma::Nothing => {}
            GroupOrComma::Group => {
                self.0.consume_text_chunk("[");
            }
            GroupOrComma::Comma => {
                self.0.consume_text_chunk(",");
            }
        }

        items.json_provide_into(ConsumeArrayItemsAppendCommaIfNotEmpty(
            self.0.as_mut_consume_text_chunk(),
        ));

        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }
    // pub const fn
}

impl<W: ConsumeTextChunk, InitialConsumer, const OC: u8> ReadyToConsumeJsonChunksOfNonEmptyObject
    for ConsumeChunksOfNonEmptyObject<W, InitialConsumer, states::Init, OC>
{
    type LeftBraceKvsBeforeKv =
        ConsumeChunksOfNonEmptyObject<W, InitialConsumer, states::LeftBraceKvsBeforeKv, OC>;
    fn left_brace_kvs_before_kv(
        mut self,
        kvs: impl IntoJson<JsonKind = json_kinds::Object>,
    ) -> Self::LeftBraceKvsBeforeKv {
        match const { OpenClose::try_from_u8(OC).unwrap().open } {
            GroupOrComma::Nothing => {}
            GroupOrComma::Group => {
                self.0.consume_text_chunk("{");
            }
            GroupOrComma::Comma => {
                self.0.consume_text_chunk(",");
            }
        }

        kvs.json_provide_into(super::ConsumeObjectKvsAppendCommaIfNotEmpty(
            self.0.as_mut_consume_text_chunk(),
        ));

        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
    }
}

impl<W: ConsumeTextChunk, InitialConsumer, S: ?Sized + HasConstState, const OC: u8>
    ConsumeChunksOfNonEmptyArray<W, InitialConsumer, S, OC>
{
    pub(crate) fn impl_end_with_right_bracket(
        mut self,
        (): (),
    ) -> Consumed<json_kinds::Array, InitialConsumer> {
        const { S::STATE.right_bracket().assert_eof_of_non_empty_array() }
        match const { OpenClose::try_from_u8(OC).unwrap().close } {
            GroupOrComma::Nothing => {}
            GroupOrComma::Group => self.0.consume_text_chunk("]"),
            GroupOrComma::Comma => self.0.consume_text_chunk(","),
        }
        Consumed::ASSERT_ARRAY
    }
    pub(crate) fn impl_end_with_right_brace(
        self,
        yes: core::convert::Infallible,
    ) -> Consumed<json_kinds::Array, InitialConsumer> {
        match yes {}
    }
}

impl<W: ConsumeTextChunk, InitialConsumer, S: ?Sized + HasConstState, const OC: u8>
    ConsumeChunksOfNonEmptyObject<W, InitialConsumer, S, OC>
{
    pub(crate) fn impl_end_with_right_bracket(
        self,
        yes: core::convert::Infallible,
    ) -> Consumed<json_kinds::Object, InitialConsumer> {
        match yes {}
    }
    pub(crate) fn impl_end_with_right_brace(
        mut self,
        (): (),
    ) -> Consumed<json_kinds::Object, InitialConsumer> {
        const { S::STATE.right_brace().assert_eof_of_non_empty_object() }
        match const { OpenClose::try_from_u8(OC).unwrap().close } {
            GroupOrComma::Nothing => {}
            GroupOrComma::Group => self.0.consume_text_chunk("}"),
            GroupOrComma::Comma => self.0.consume_text_chunk(","),
        }
        Consumed::ASSERT_OBJECT
    }
}
