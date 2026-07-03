use core::marker::PhantomData;

use crate::r#const::HasConstCompileTimeChunk;

use super::{
    ConsumeChainedArrays, ConsumeChainedStrings, ConsumeJson, ConsumeJsonChunks, Consumed,
    json_kinds::JsonKind, runtime_chunks::RuntimeChunks,
};

pub enum NeverConsume {}

impl<K: JsonKind> ConsumeJsonChunks<K> for NeverConsume {
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> = Self;

    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        match self {}
    }

    type ConsumeRuntimeChunk<C: RuntimeChunks> = Self;

    fn consume_runtime_chunk<C: RuntimeChunks>(self, _: C) -> Self::ConsumeRuntimeChunk<C> {
        match self {}
    }

    fn end(self) -> Consumed<K, Self> {
        match self {}
    }
}

pub struct NeverConsumeChained<INITIAL: ConsumeJson>(NeverConsume, PhantomData<INITIAL>);

impl<INITIAL: ConsumeJson> ConsumeChainedArrays for NeverConsumeChained<INITIAL> {
    fn extend(&mut self, _: impl super::IntoJson<JsonKind = super::json_kinds::Array>) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;
    fn end_with(
        self,
        _: impl super::IntoJson<JsonKind = super::json_kinds::Array>,
    ) -> Consumed<super::json_kinds::Array, Self::InitialConsumer> {
        match self.0 {}
    }
}

impl<INITIAL: ConsumeJson> ConsumeChainedStrings for NeverConsumeChained<INITIAL> {
    fn extend(&mut self, _: impl crate::ser::IntoJson<JsonKind = super::json_kinds::JsonString>) {
        match self.0 {}
    }

    type InitialConsumer = INITIAL;

    fn end_with(
        self,
        _: impl crate::ser::IntoJson<JsonKind = super::json_kinds::JsonString>,
    ) -> Consumed<super::json_kinds::JsonString, Self::InitialConsumer> {
        match self.0 {}
    }
}
